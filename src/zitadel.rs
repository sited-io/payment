use std::fmt::Display;

use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use tonic::Status;

pub struct ZitadelService {
    client: Client,
    base_url: String,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Clone, Serialize)]
struct AuthenticationRequest {
    grant_type: &'static str,
    scope: &'static str,
}

const AUTH_REQUEST: AuthenticationRequest = AuthenticationRequest {
    grant_type: "client_credentials",
    scope: "openid profile email urn:zitadel:iam:org:project:id:zitadel:aud",
};

#[derive(Debug, Clone, Serialize)]
struct MetadataValueRequest<'a> {
    value: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticationResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl ZitadelService {
    pub fn new(
        base_url: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url,
            client_id,
            client_secret,
        }
    }

    pub async fn update_stripe_id(
        &self,
        user_id: &String,
        account_id: &String,
    ) -> Result<(), Status> {
        let access_token = self.get_access_token().await?.access_token;

        self.client
            .request(
                Method::POST,
                self.user_metadata_url(user_id, "stripe_account_id"),
            )
            .bearer_auth(access_token)
            .json(&MetadataValueRequest { value: account_id })
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[ZitadelService.update_stripe_id]: '{err}'"
                );
                Status::internal("")
            })?;

        Ok(())
    }

    async fn get_access_token(&self) -> Result<AuthenticationResponse, Status> {
        self.client
            .request(Method::POST, self.auth_url())
            .basic_auth(
                self.client_id.clone(),
                Some(self.client_secret.clone()),
            )
            .form(&AUTH_REQUEST)
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[ZitadelService.get_access_token] - sending auth request: '{err}'"
                );
                Status::internal("")
            })?
            .json()
            .await
            .map_err(|err| {
                tracing::log::error!(
                    "[ZitadelService.get_access_token] - parsing auth reponse: '{err}'"
                );
                Status::internal("")
            })
    }

    fn auth_url(&self) -> String {
        format!("{}/oauth/v2/token", self.base_url)
    }

    fn user_metadata_url<K: Display>(
        &self,
        user_id: &String,
        key: K,
    ) -> String {
        format!(
            "{}/management/v1/users/{}/metadata/{}",
            self.base_url, user_id, key
        )
    }
}
