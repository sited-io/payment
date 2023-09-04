use base64::Engine;
use std::str::FromStr;

use jwtk::jwk::RemoteJwksVerifier;
use stripe::{
    Account, AccountId, AccountLink, AccountLinkType, AccountType, Client,
    CreateAccount, CreateAccountLink,
};
use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::payment::v1::stripe_service_server::{
    self, StripeServiceServer,
};
use crate::api::peoplesmarkets::payment::v1::{
    CreateAccountLinkRequest, CreateAccountLinkResponse, CreateAccountRequest,
    CreateAccountResponse, GetAccountRequest, GetAccountResponse,
};
use crate::auth::{verify_token, Metadata};
use crate::zitadel::ZitadelService;
use crate::{parse_id_error_to_status, stripe_error_to_status};

pub struct StripeService {
    verifier: RemoteJwksVerifier,
    stripe_client: Client,
    zitadel_service: ZitadelService,
}

impl StripeService {
    fn new(
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        zitadel_service: ZitadelService,
    ) -> Self {
        Self {
            verifier,
            stripe_client,
            zitadel_service,
        }
    }

    pub fn build(
        verifier: RemoteJwksVerifier,
        stripe_client: Client,
        zitadel_service: ZitadelService,
    ) -> StripeServiceServer<Self> {
        let service = Self::new(verifier, stripe_client, zitadel_service);
        StripeServiceServer::new(service)
    }

    fn decode_account_id<'a>(
        metadata: Option<Metadata>,
    ) -> Result<String, Status> {
        let account_id = metadata
            .and_then(|m| m.stripe_account_id)
            .and_then(|id| {
                base64::engine::general_purpose::STANDARD.decode(id).ok()
            })
            .ok_or(Status::not_found("stripe_account_id"))?;

        Ok(std::str::from_utf8(&account_id)
            .map_err(|_| Status::not_found("stripe_account_id"))?
            .to_owned())
    }
}

#[async_trait]
impl stripe_service_server::StripeService for StripeService {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let (user_id, metadata) =
            verify_token(request.metadata(), &self.verifier).await?;

        match metadata.and_then(|m| m.stripe_account_id) {
            Some(_) => Ok(Response::new(CreateAccountResponse {})),
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

                // set stripe_account_id in user authentication
                let account_id = base64::engine::general_purpose::STANDARD
                    .encode(account.id.as_str());
                self.zitadel_service
                    .update_stripe_id(&user_id, &account_id)
                    .await?;

                Ok(Response::new(CreateAccountResponse {}))
            }
        }
    }

    async fn create_account_link(
        &self,
        request: Request<CreateAccountLinkRequest>,
    ) -> Result<Response<CreateAccountLinkResponse>, Status> {
        let (_, metadata) =
            verify_token(request.metadata(), &self.verifier).await?;

        let account_id = Self::decode_account_id(metadata)?;

        let CreateAccountLinkRequest {
            refresh_url,
            return_url,
        } = request.into_inner();

        let link = AccountLink::create(
            &self.stripe_client,
            CreateAccountLink {
                account: AccountId::from_str(&account_id)
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
        let (_, metadata) =
            verify_token(request.metadata(), &self.verifier).await?;

        let account_id = Self::decode_account_id(metadata)?;

        let account = Account::retrieve(
            &self.stripe_client,
            &AccountId::from_str(&account_id)
                .map_err(parse_id_error_to_status)?,
            &[],
        )
        .await
        .map_err(stripe_error_to_status)?;

        Ok(Response::new(GetAccountResponse {
            charges_enabled: account.charges_enabled.unwrap_or(false),
            details_submitted: account.details_submitted.unwrap_or(false),
        }))
    }
}
