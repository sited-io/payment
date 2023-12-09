use chrono::Utc;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::str::FromStr;

use jwtk::jwk::RemoteJwksVerifier;
use stripe::{
    Account, AccountId, AccountLink, AccountLinkType, AccountType,
    CancelSubscription, CheckoutSession, CheckoutSessionMode, Client,
    CreateAccount, CreateAccountLink, CreateCheckoutSession,
    CreateCheckoutSessionLineItems,
    CreateCheckoutSessionLineItemsAdjustableQuantity,
    CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData,
    CreateCheckoutSessionLineItemsPriceDataRecurring,
    CreateCheckoutSessionLineItemsPriceDataRecurringInterval,
    CreateCheckoutSessionPaymentIntentData,
    CreateCheckoutSessionShippingAddressCollection,
    CreateCheckoutSessionShippingAddressCollectionAllowedCountries,
    CreateCheckoutSessionShippingOptions,
    CreateCheckoutSessionShippingOptionsShippingRateData,
    CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount,
    CreateCheckoutSessionShippingOptionsShippingRateDataType,
    CreateCheckoutSessionSubscriptionData, Currency as StripeCurrency,
    ResumeSubscription, Subscription as StripeSubscription, SubscriptionId,
    UpdateSubscription,
};
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::commerce::v1::{
    Currency, OfferType, PriceType, RecurringInterval,
};
use crate::api::peoplesmarkets::payment::v1::stripe_service_server::{
    self, StripeServiceServer,
};
use crate::api::peoplesmarkets::payment::v1::{
    CancelSubscriptionRequest, CancelSubscriptionResponse,
    CreateAccountLinkRequest, CreateAccountLinkResponse, CreateAccountRequest,
    CreateAccountResponse, CreateCheckoutSessionRequest,
    CreateCheckoutSessionResponse, GetAccountDetailsRequest,
    GetAccountDetailsResponse, GetAccountRequest, GetAccountResponse,
    ResumeSubscriptionRequest, ResumeSubscriptionResponse,
    StripeAccount as StripeAccountMsg, StripeAccountDetails,
};
use crate::auth::{get_user_id, verify_service_user};
use crate::countries::{to_stripe_country, ALL_STRIPE_COUNTRIES};
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

    fn metadata_key_shop_id() -> String {
        String::from("shop_id")
    }

    fn metadata_key_offer_id() -> String {
        String::from("offer_id")
    }

    fn shipping_rate_key() -> String {
        String::from("SHIPPING")
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
            shop_id: stripe_account.shop_id.to_string(),
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

    fn get_shipping_address_countries(
        all_countries: bool,
        countries: Vec<i32>,
    ) -> Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>
    {
        if all_countries {
            ALL_STRIPE_COUNTRIES.to_vec()
        } else {
            countries.iter().map(to_stripe_country).collect()
        }
    }
}

#[async_trait]
impl stripe_service_server::StripeService for StripeService {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let metadata = request.metadata().clone();

        let user_id = get_user_id(&metadata, &self.verifier).await?;

        let CreateAccountRequest { shop_id } = request.into_inner();

        let shop_uuid = parse_uuid(&shop_id, "shop_id")?;

        self.commerce_service
            .check_shop_and_owner(&shop_id, &user_id, &metadata)
            .await?;

        match StripeAccount::get_for_user(&self.pool, &shop_uuid, &user_id)
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
                    &shop_uuid,
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
            shop_id,
            refresh_url,
            return_url,
        } = request.into_inner();

        let shop_id = parse_uuid(&shop_id, "shop_id")?;

        let found_stripe_account =
            StripeAccount::get_for_user(&self.pool, &shop_id, &user_id)
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
        let GetAccountRequest { shop_id } = request.into_inner();

        let shop_id = parse_uuid(&shop_id, "shop_id")?;

        if let Some(found_stripe_account) =
            StripeAccount::get(&self.pool, &shop_id).await?
        {
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
        } else {
            Ok(Response::new(GetAccountResponse { account: None }))
        }
    }

    async fn get_account_details(
        &self,
        request: Request<GetAccountDetailsRequest>,
    ) -> Result<Response<GetAccountDetailsResponse>, Status> {
        let user_id = get_user_id(request.metadata(), &self.verifier).await?;

        let GetAccountDetailsRequest { shop_id } = request.into_inner();

        let shop_id = parse_uuid(&shop_id, "shop_id")?;

        let found_stripe_account =
            StripeAccount::get_for_user(&self.pool, &shop_id, &user_id)
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
        let metadata = request.metadata().clone();

        let user_id = get_user_id(&metadata, &self.verifier).await.ok();

        let CreateCheckoutSessionRequest {
            offer_id,
            success_url,
            cancel_url,
        } = request.into_inner();

        let found_offer = self.commerce_service.get_offer(&offer_id).await?;

        let price = found_offer
            .price
            .as_ref()
            .ok_or_else(|| Status::internal("offer.price missing"))?;

        let shop_uuid = parse_uuid(&found_offer.shop_id, "offer.shop_id")?;

        let stripe_account = StripeAccount::get(&self.pool, &shop_uuid)
            .await
            .map_err(|_| {
                Status::not_found(format!("stripe account for '{}'", shop_uuid))
            })?
            .ok_or_else(|| {
                Status::not_found(format!("stripe account for '{}'", shop_uuid))
            })?;

        let found_shop = self
            .commerce_service
            .get_shop(&found_offer.shop_id, &metadata)
            .await?;

        let stripe_account_id =
            AccountId::from_str(&stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?;

        // Create checkout session request
        let mut checkout_session = CreateCheckoutSession::new(&success_url);
        checkout_session.cancel_url = Some(&cancel_url);

        // Add shop_id and offer_id to metadata of stripe checkout session
        // this is used in stripe webhook handler to assign offers to payments
        let mut metadata = HashMap::from([
            (Self::metadata_key_shop_id(), shop_uuid.to_string()),
            (
                Self::metadata_key_offer_id(),
                found_offer.offer_id.to_string(),
            ),
        ]);

        match price.price_type() {
            PriceType::Unspecified => {
                return Err(Status::internal("price_type unspecified"))
            }
            PriceType::OneTime => {
                checkout_session.mode = Some(CheckoutSessionMode::Payment);
                checkout_session.payment_intent_data =
                    Some(CreateCheckoutSessionPaymentIntentData {
                        application_fee_amount: Some(
                            Self::calculate_fee_amount(
                                price.unit_amount,
                                found_shop.platform_fee_percent,
                                found_shop.minimum_platform_fee_cent,
                            ),
                        ),
                        ..Default::default()
                    });

                // Get shipping address collection based on configured shipping rates
                let found_shipping_rate =
                    self.commerce_service.get_shipping_rate(&offer_id).await;

                checkout_session.shipping_address_collection =
                    found_shipping_rate.to_owned().map(|s| {
                        CreateCheckoutSessionShippingAddressCollection {
                            allowed_countries:
                                Self::get_shipping_address_countries(
                                    s.all_countries,
                                    s.specific_countries,
                                ),
                        }
                    });

                // Add shipping rate to checkout session
                if let Some(shipping_rate) = found_shipping_rate {
                    checkout_session.shipping_options = Some(vec![
                        CreateCheckoutSessionShippingOptions {
                            shipping_rate_data: Some(CreateCheckoutSessionShippingOptionsShippingRateData {
                                type_: Some(CreateCheckoutSessionShippingOptionsShippingRateDataType::FixedAmount),
                                display_name: Self::shipping_rate_key(),
                                fixed_amount: Some(CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
                                    amount: shipping_rate.amount.into(),
                                    currency: Self::get_currency(shipping_rate.currency)?,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
                    ]);
                }
            }
            PriceType::Recurring => {
                // If offer is a digital subscription, we need to provide the user_id to the payment
                // in order to assing ownership of the subscription to the buyer.
                // In other cases customers should be able buy without authentication.
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
                                found_shop.platform_fee_percent,
                                found_shop.minimum_platform_fee_cent,
                            ),
                        ),
                        trial_period_days: price
                            .recurring
                            .as_ref()
                            .and_then(|r| r.trial_period_days),
                        ..Default::default()
                    });
            }
        }

        // Add metadata to checkout session
        checkout_session.metadata = Some(metadata);

        // Add line items to checkout session
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

        checkout_session.line_items =
            Some(vec![CreateCheckoutSessionLineItems {
                quantity: Some(1),
                adjustable_quantity: Some(adjustable_quantity),
                price_data: Some(price_data),
                ..Default::default()
            }]);

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

    async fn cancel_subscription(
        &self,
        request: Request<CancelSubscriptionRequest>,
    ) -> Result<Response<CancelSubscriptionResponse>, Status> {
        verify_service_user(request.metadata(), &self.verifier).await?;

        let CancelSubscriptionRequest {
            stripe_subscription_id,

            shop_id,
        } = request.into_inner();

        let shop_uuid = parse_uuid(&shop_id, "shop_id")?;

        let stripe_account = StripeAccount::get(&self.pool, &shop_uuid)
            .await
            .map_err(|_| Status::not_found(""))?
            .ok_or_else(|| Status::not_found(""))?;

        let stripe_account_id =
            AccountId::from_str(&stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?;

        let subscription_id = SubscriptionId::from_str(&stripe_subscription_id)
            .map_err(parse_id_error_to_status)?;

        let stripe_client = self.stripe_client.clone();
        let stripe_client =
            stripe_client.with_stripe_account(stripe_account_id);

        let mut update_subscription = UpdateSubscription::new();

        update_subscription.cancel_at_period_end = Some(true);

        StripeSubscription::update(
            &stripe_client,
            &subscription_id,
            update_subscription,
        )
        .await
        .map_err(|err| {
            tracing::log::error!("{err}");
            Status::internal("")
        })?;

        Ok(Response::new(CancelSubscriptionResponse {}))
    }

    async fn resume_subscription(
        &self,
        request: Request<ResumeSubscriptionRequest>,
    ) -> Result<Response<ResumeSubscriptionResponse>, Status> {
        verify_service_user(request.metadata(), &self.verifier).await?;

        let ResumeSubscriptionRequest {
            stripe_subscription_id,
            shop_id,
        } = request.into_inner();

        let shop_uuid = parse_uuid(&shop_id, "shop_id")?;

        let stripe_account = StripeAccount::get(&self.pool, &shop_uuid)
            .await
            .map_err(|_| Status::not_found(""))?
            .ok_or_else(|| Status::not_found(""))?;

        let stripe_account_id =
            AccountId::from_str(&stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?;

        let subscription_id = SubscriptionId::from_str(&stripe_subscription_id)
            .map_err(parse_id_error_to_status)?;

        let stripe_client = self.stripe_client.clone();
        let stripe_client =
            stripe_client.with_stripe_account(stripe_account_id);

        StripeSubscription::resume(
            &stripe_client,
            &subscription_id,
            ResumeSubscription::new(),
        )
        .await
        .map_err(|err| {
            tracing::log::error!("{err}");
            Status::internal("")
        })?;

        Ok(Response::new(ResumeSubscriptionResponse {}))
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
