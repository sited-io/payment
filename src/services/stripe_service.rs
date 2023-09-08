use deadpool_postgres::Pool;
use std::str::FromStr;
use tonic::transport::Channel;

use jwtk::jwk::RemoteJwksVerifier;
use stripe::{
    Account, AccountId, AccountLink, AccountLinkType, AccountType, Client,
    CreateAccount, CreateAccountLink,
};
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::commerce::v1::market_booth_service_client::MarketBoothServiceClient;
use crate::api::peoplesmarkets::commerce::v1::GetMarketBoothRequest;
use crate::api::peoplesmarkets::payment::v1::stripe_service_server::{
    self, StripeServiceServer,
};
use crate::api::peoplesmarkets::payment::v1::{
    CreateAccountLinkRequest, CreateAccountLinkResponse, CreateAccountRequest,
    CreateAccountResponse, GetAccountDetailsRequest, GetAccountDetailsResponse,
    GetAccountRequest, GetAccountResponse, StripeAccount as StripeAccountMsg,
    StripeAccountDetails,
};
use crate::auth::get_user_id;
use crate::model::StripeAccount;
use crate::{parse_id_error_to_status, parse_uuid, stripe_error_to_status};

pub struct StripeService {
    pool: Pool,
    verifier: RemoteJwksVerifier,
    stripe_client: Client,
    market_booth_service_client: MarketBoothServiceClient<Channel>,
}

impl StripeService {
    fn new(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        market_booth_service_client: MarketBoothServiceClient<Channel>,
    ) -> Self {
        Self {
            pool,
            verifier,
            stripe_client,
            market_booth_service_client,
        }
    }

    pub fn build(
        pool: Pool,
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        market_booth_service_client: MarketBoothServiceClient<Channel>,
    ) -> StripeServiceServer<Self> {
        StripeServiceServer::new(Self::new(
            pool,
            verifier,
            stripe_client,
            market_booth_service_client,
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
}
