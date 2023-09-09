use deadpool_postgres::Pool;
use std::str::FromStr;
use tonic::transport::Channel;

use jwtk::jwk::RemoteJwksVerifier;
use stripe::{
    Account, AccountId, AccountLink, AccountLinkType, AccountType,
    CheckoutSession, CheckoutSessionMode, Client, CreateAccount,
    CreateAccountLink, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCheckoutSessionLineItemsAdjustableQuantity,
    CreateCheckoutSessionLineItemsPriceData,
    CreateCheckoutSessionLineItemsPriceDataProductData,
    CreateCheckoutSessionPaymentIntentData, Currency as StripeCurrency,
};
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::commerce::v1::market_booth_service_client::MarketBoothServiceClient;
use crate::api::peoplesmarkets::commerce::v1::offer_service_client::OfferServiceClient;
use crate::api::peoplesmarkets::commerce::v1::{
    Currency, GetMarketBoothRequest, GetOfferRequest,
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
use crate::{parse_id_error_to_status, parse_uuid, stripe_error_to_status};

pub struct StripeService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    stripe_client: Client,
    market_booth_service_client: MarketBoothServiceClient<Channel>,
    offer_service_client: OfferServiceClient<Channel>,
}

impl StripeService {
    fn new(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        market_booth_service_client: MarketBoothServiceClient<Channel>,
        offer_service_client: OfferServiceClient<Channel>,
    ) -> Self {
        Self {
            pool,
            verifier,
            stripe_client,
            market_booth_service_client,
            offer_service_client,
        }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        market_booth_service_client: MarketBoothServiceClient<Channel>,
        offer_service_client: OfferServiceClient<Channel>,
    ) -> StripeServiceServer<Self> {
        StripeServiceServer::new(Self::new(
            pool,
            verifier,
            stripe_client,
            market_booth_service_client,
            offer_service_client,
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
        min_fee: u32,
    ) -> i64 {
        i64::from(((unit_amount * fee_pct) / 100).max(min_fee))
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

        let mut client = self.market_booth_service_client.clone();

        let market_booth_uuid =
            parse_uuid(&market_booth_id, "market_booth_id")?;

        let found_market_booth = client
            .get_market_booth(Request::new(GetMarketBoothRequest {
                market_booth_id,
            }))
            .await
            .map_err(|_| Status::not_found(""))?
            .into_inner()
            .market_booth
            .ok_or_else(|| Status::not_found(""))?;

        if found_market_booth.user_id != user_id {
            return Err(Status::not_found(""));
        }

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
        let CreateCheckoutSessionRequest {
            offer_id,
            success_url,
            cancel_url,
        } = request.into_inner();

        let mut offer_service_client = self.offer_service_client.clone();
        let mut market_booth_service_client =
            self.market_booth_service_client.clone();

        let found_offer = offer_service_client
            .get_offer(Request::new(GetOfferRequest { offer_id }))
            .await
            .map_err(|_| Status::not_found(""))?
            .into_inner()
            .offer
            .ok_or_else(|| Status::not_found(""))?;

        let price = found_offer.price.ok_or_else(|| Status::internal(""))?;

        let market_booth_uuid = parse_uuid(&found_offer.market_booth_id, "")?;

        let found_market_booth = market_booth_service_client
            .get_market_booth(Request::new(GetMarketBoothRequest {
                market_booth_id: found_offer.market_booth_id,
            }))
            .await
            .map_err(|_| Status::not_found("market_booth"))?
            .into_inner()
            .market_booth
            .ok_or_else(|| Status::not_found("market_booth"))?;

        let stripe_account = StripeAccount::get(&self.pool, &market_booth_uuid)
            .await
            .map_err(|_| Status::not_found(""))?
            .ok_or_else(|| Status::not_found(""))?;

        let stripe_account_id =
            AccountId::from_str(&stripe_account.stripe_account_id)
                .map_err(parse_id_error_to_status)?;

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

        let price_data = CreateCheckoutSessionLineItemsPriceData {
            currency: Self::get_currency(price.currency)?,
            product_data: Some(product),
            unit_amount: Some(i64::from(price.unit_amont)),
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

        let payment_intent = CreateCheckoutSessionPaymentIntentData {
            application_fee_amount: Some(Self::calculate_fee_amount(
                price.unit_amont,
                found_market_booth.platform_fee_percent,
                found_market_booth.minimum_platform_fee_cent,
            )),
            ..Default::default()
        };

        let mut session = CreateCheckoutSession::new(&success_url);
        session.cancel_url = Some(&cancel_url);
        session.mode = Some(CheckoutSessionMode::Payment);
        session.line_items = Some(line_items);
        session.payment_intent_data = Some(payment_intent);

        let stripe_client = self.stripe_client.clone();

        let link = CheckoutSession::create(
            &stripe_client.with_stripe_account(stripe_account_id),
            session,
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
