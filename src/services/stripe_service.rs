use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::str::FromStr;

use jwtk::jwk::RemoteJwksVerifier;
use stripe::{
    Account, AccountId, AccountLink, AccountLinkType, AccountType,
    CheckoutSession, CheckoutSessionMode, Client, CreateAccount,
    CreateAccountLink, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCheckoutSessionLineItemsAdjustableQuantity,
    CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData,
    CreateCheckoutSessionLineItemsPriceDataRecurring,
    CreateCheckoutSessionLineItemsPriceDataRecurringInterval,
    CreateCheckoutSessionPaymentIntentData,
    CreateCheckoutSessionSubscriptionData, Currency as StripeCurrency,
};
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::commerce::v1::{
    Currency, OfferType, PriceType, RecurringInterval,
};
use crate::api::peoplesmarkets::payment::v1::stripe_service_server::{
    self, StripeServiceServer,
};
use crate::api::peoplesmarkets::payment::v1::{
    CreateAccountLinkRequest, CreateAccountLinkResponse, CreateAccountRequest,
    CreateAccountResponse, CreateCheckoutSessionRequest,
    CreateCheckoutSessionResponse, GetAccountDetailsRequest,
    GetAccountDetailsResponse, GetAccountRequest, GetAccountResponse,
    StripeAccount as StripeAccountMsg, StripeAccountDetails,
};
use crate::auth::get_user_id;
use crate::model::StripeAccount;
use crate::{
    parse_id_error_to_status, parse_uuid, stripe_error_to_status,
    CommerceService,
};

pub struct StripeService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    stripe_client: Client,
    commerce_service: CommerceService,
}

impl StripeService {
    fn metadata_key_user_id() -> String {
        String::from("user_id")
    }

    fn metadata_key_offer_id() -> String {
        String::from("offer_id")
    }

    fn new(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        commerce_service: CommerceService,
    ) -> Self {
        Self {
            pool,
            verifier,
            stripe_client,
            commerce_service,
        }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        commerce_service: CommerceService,
    ) -> StripeServiceServer<Self> {
        StripeServiceServer::new(Self::new(
            pool,
            verifier,
            stripe_client,
            commerce_service,
        ))
    }

    pub fn to_response(
        stripe_account: StripeAccount,
        enabled: bool,
    ) -> StripeAccountMsg {
        StripeAccountMsg {
            market_booth_id: stripe_account.market_booth_id.to_string(),
            stripe_account_id: stripe_account.stripe_account_id,
            enabled,
        }
    }

    fn get_currency(currency: i32) -> Result<StripeCurrency, Status> {
        let currency: Currency =
            Currency::try_from(currency).map_err(|_| Status::internal(""))?;

        match currency {
            Currency::Unspecified => Err(Status::internal("")),
            Currency::Eur => Ok(StripeCurrency::EUR),
        }
    }

    fn calculate_fee_amount(
        unit_amount: u32,
        fee_pct: u32,
        min_fee_amount: u32,
    ) -> i64 {
        i64::from(((unit_amount * fee_pct) / 100).max(min_fee_amount))
    }

    fn calculate_fee_percent(
        unit_amount: u32,
        fee_pct: u32,
        min_fee_amount: u32,
    ) -> f64 {
        let fee_amount = (unit_amount * fee_pct) / 100;
        if fee_amount < min_fee_amount {
            let fee_percent =
                f64::from(min_fee_amount * 100) / f64::from(unit_amount);
            // rouds f64 to second decimal
            (fee_percent * 100.0).round() / 100.0
        } else {
            f64::from(fee_pct)
        }
    }

    fn get_recurring_interval(
        interval: RecurringInterval,
    ) -> Result<CreateCheckoutSessionLineItemsPriceDataRecurringInterval, Status>
    {
        use CreateCheckoutSessionLineItemsPriceDataRecurringInterval::*;

        match interval {
            RecurringInterval::Unspecified => Err(Status::internal("")),
            RecurringInterval::Day => Ok(Day),
            RecurringInterval::Week => Ok(Week),
            RecurringInterval::Month => Ok(Month),
            RecurringInterval::Year => Ok(Year),
        }
    }
}

#[async_trait]
impl stripe_service_server::StripeService for StripeService {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let CreateAccountRequest { market_booth_id } = request.into_inner();

        let market_booth_uuid =
            parse_uuid(&market_booth_id, "market_booth_id")?;

        self.commerce_service
            .check_market_booth_and_owner(&market_booth_id, &user_id)
            .await?;

        match StripeAccount::get_for_user(
            &self.pool,
            &market_booth_uuid,
            &user_id,
        )
        .await?
        {
            Some(stripe_account) => {
                let account = Account::retrieve(
                    &self.stripe_client,
                    &AccountId::from_str(&stripe_account.stripe_account_id)
                        .map_err(parse_id_error_to_status)?,
                    &[],
                )
                .await
                .map_err(stripe_error_to_status)?;

                let enabled = account.charges_enabled.unwrap_or(false)
                    && account.details_submitted.unwrap_or(false);

                Ok(Response::new(CreateAccountResponse {
                    account: Some(Self::to_response(stripe_account, enabled)),
                }))
            }
            None => {
                let account = Account::create(
                    &self.stripe_client,
                    CreateAccount {
                        type_: Some(AccountType::Standard),
                        ..Default::default()
                    },
                )
                .await
                .map_err(stripe_error_to_status)?;

                let created_stripe_account = StripeAccount::create(
                    &self.pool,
                    &market_booth_uuid,
                    &account.id.to_string(),
                    &user_id,
                )
                .await?;

                Ok(Response::new(CreateAccountResponse {
                    account: Some(Self::to_response(
                        created_stripe_account,
                        false,
                    )),
                }))
            }
        }
    }

    async fn create_account_link(
        &self,
        request: Request<CreateAccountLinkRequest>,
    ) -> Result<Response<CreateAccountLinkResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let CreateAccountLinkRequest {
            market_booth_id,
            refresh_url,
            return_url,
        } = request.into_inner();

        let market_booth_id = parse_uuid(&market_booth_id, "market_booth_id")?;

        let found_stripe_account =
            StripeAccount::get_for_user(&self.pool, &market_booth_id, &user_id)
                .await?
                .ok_or(Status::not_found(""))?;

        let link = AccountLink::create(
            &self.stripe_client,
            CreateAccountLink {
                account: AccountId::from_str(
                    &found_stripe_account.stripe_account_id,
                )
                .map_err(parse_id_error_to_status)?,
                refresh_url: Some(&refresh_url),
                return_url: Some(&return_url),
                type_: AccountLinkType::AccountOnboarding,
                collect: None,
                expand: &[],
            },
        )
        .await
        .map_err(stripe_error_to_status)?
        .url;

        Ok(Response::new(CreateAccountLinkResponse { link }))
    }

    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        let GetAccountRequest { market_booth_id } = request.into_inner();

        let market_booth_id = parse_uuid(&market_booth_id, "market_booth_id")?;

        let found_stripe_account =
            StripeAccount::get(&self.pool, &market_booth_id)
                .await?
                .ok_or(Status::not_found(""))?;

        let account = Account::retrieve(
            &self.stripe_client,
            &AccountId::from_str(&found_stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?,
            &[],
        )
        .await
        .map_err(stripe_error_to_status)?;

        let enabled = account.charges_enabled.unwrap_or(false)
            && account.details_submitted.unwrap_or(false);

        Ok(Response::new(GetAccountResponse {
            account: Some(Self::to_response(found_stripe_account, enabled)),
        }))
    }

    async fn get_account_details(
        &self,
        request: Request<GetAccountDetailsRequest>,
    ) -> Result<Response<GetAccountDetailsResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let GetAccountDetailsRequest { market_booth_id } = request.into_inner();

        let market_booth_id = parse_uuid(&market_booth_id, "market_booth_id")?;

        let found_stripe_account =
            StripeAccount::get_for_user(&self.pool, &market_booth_id, &user_id)
                .await?
                .ok_or(Status::not_found(""))?;

        let account = Account::retrieve(
            &self.stripe_client,
            &AccountId::from_str(&found_stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?,
            &[],
        )
        .await
        .map_err(stripe_error_to_status)?;

        let enabled = account.charges_enabled.unwrap_or(false)
            && account.details_submitted.unwrap_or(false);

        Ok(Response::new(GetAccountDetailsResponse {
            account: Some(Self::to_response(found_stripe_account, enabled)),
            details: Some(StripeAccountDetails {
                charges_enabled: account.charges_enabled.unwrap_or(false),
                details_submitted: account.details_submitted.unwrap_or(false),
            }),
        }))
    }

    async fn create_checkout_session(
        &self,
        request: Request<CreateCheckoutSessionRequest>,
    ) -> Result<Response<CreateCheckoutSessionResponse>, Status> {
        let user_id =
            get_user_id(request.metadata(), &self.verifier).await.ok();

        let CreateCheckoutSessionRequest {
            offer_id,
            success_url,
            cancel_url,
        } = request.into_inner();

        let found_offer = self.commerce_service.get_offer(&offer_id).await?;

        let price = found_offer
            .price
            .as_ref()
            .ok_or_else(|| Status::internal(""))?;

        let market_booth_uuid = parse_uuid(&found_offer.market_booth_id, "")?;

        let stripe_account = StripeAccount::get(&self.pool, &market_booth_uuid)
            .await
            .map_err(|_| Status::not_found(""))?
            .ok_or_else(|| Status::not_found(""))?;

        let found_market_booth = self
            .commerce_service
            .get_market_booth(&found_offer.market_booth_id)
            .await?;

        let stripe_account_id =
            AccountId::from_str(&stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?;

        // create checkout session request
        let mut checkout_session = CreateCheckoutSession::new(&success_url);
        checkout_session.cancel_url = Some(&cancel_url);

        // adding offer_id to metadata of stripe checkout session
        // this is used in stripe webhook handler to assign payments to offers
        let mut metadata = HashMap::from([(
            Self::metadata_key_offer_id(),
            found_offer.offer_id.to_string(),
        )]);

        match price.price_type() {
            PriceType::Unspecified => return Err(Status::internal("")),
            PriceType::OneTime => {
                checkout_session.mode = Some(CheckoutSessionMode::Payment);
                checkout_session.payment_intent_data =
                    Some(CreateCheckoutSessionPaymentIntentData {
                        application_fee_amount: Some(
                            Self::calculate_fee_amount(
                                price.unit_amount,
                                found_market_booth.platform_fee_percent,
                                found_market_booth.minimum_platform_fee_cent,
                            ),
                        ),
                        ..Default::default()
                    });
            }
            PriceType::Recurring => {
                // If subscription and the offer is digital we need to provide the
                // user_id to the payment in order to assing ownership of the product to the user
                if found_offer.r#type() == OfferType::Digital {
                    if let Some(user_id) = user_id {
                        metadata.insert(Self::metadata_key_user_id(), user_id);
                    } else {
                        return Err(Status::unauthenticated(""));
                    }
                }

                checkout_session.mode = Some(CheckoutSessionMode::Subscription);
                checkout_session.subscription_data =
                    Some(CreateCheckoutSessionSubscriptionData {
                        application_fee_percent: Some(
                            Self::calculate_fee_percent(
                                price.unit_amount,
                                found_market_booth.platform_fee_percent,
                                found_market_booth.minimum_platform_fee_cent,
                            ),
                        ),
                        ..Default::default()
                    });
            }
        }

        let product = CreateCheckoutSessionLineItemsPriceDataProductData {
            name: found_offer.name,
            description: (!found_offer.description.is_empty())
                .then_some(found_offer.description),
            images: (!found_offer.images.is_empty()).then_some(
                found_offer
                    .images
                    .into_iter()
                    .map(|i| i.image_url)
                    .collect(),
            ),
            ..Default::default()
        };

        let recurring = match price.recurring.as_ref() {
            Some(r) => Some(CreateCheckoutSessionLineItemsPriceDataRecurring {
                interval: Self::get_recurring_interval(r.interval())?,
                interval_count: Some(u64::from(r.interval_count)),
            }),
            None => None,
        };

        let price_data = CreateCheckoutSessionLineItemsPriceData {
            currency: Self::get_currency(price.currency)?,
            product_data: Some(product),
            unit_amount: Some(i64::from(price.unit_amount)),
            recurring,
            ..Default::default()
        };

        let adjustable_quantity =
            CreateCheckoutSessionLineItemsAdjustableQuantity {
                enabled: true,
                minimum: Some(1),
                ..Default::default()
            };

        let line_items = vec![CreateCheckoutSessionLineItems {
            quantity: Some(1),
            adjustable_quantity: Some(adjustable_quantity),
            price_data: Some(price_data),
            ..Default::default()
        }];

        checkout_session.line_items = Some(line_items);

        checkout_session.metadata = Some(metadata);

        let stripe_client = self.stripe_client.clone();

        let link = CheckoutSession::create(
            &stripe_client.with_stripe_account(stripe_account_id),
            checkout_session,
        )
        .await
        .map_err(|err| {
            tracing::log::error!("{err}");
            Status::internal("")
        })?
        .url
        .ok_or_else(|| Status::internal(""))?;

        Ok(Response::new(CreateCheckoutSessionResponse { link }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fee_amount() {
        assert_eq!(StripeService::calculate_fee_amount(588, 2, 50), 50);
        assert_eq!(StripeService::calculate_fee_amount(1499, 2, 50), 50);
        assert_eq!(StripeService::calculate_fee_amount(5000, 2, 50), 100);
        assert_eq!(StripeService::calculate_fee_amount(4444, 2, 50), 88);
        assert_eq!(StripeService::calculate_fee_amount(4444, 3, 50), 133);
    }

    #[test]
    fn test_calculate_fee_percent() {
        assert_eq!(StripeService::calculate_fee_percent(588, 2, 50), 8.50);
        assert_eq!(StripeService::calculate_fee_percent(1499, 2, 50), 3.34);
        assert_eq!(StripeService::calculate_fee_percent(5000, 2, 50), 2.00);
        assert_eq!(StripeService::calculate_fee_percent(4444, 2, 50), 2.00);
        assert_eq!(StripeService::calculate_fee_percent(4444, 3, 50), 3.00);
    }
}
