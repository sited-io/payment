use http::header::AUTHORIZATION;
use jwtk::jwk::RemoteJwksVerifier;
use jwtk::Claims;
use serde::Deserialize;
use tonic::metadata::MetadataMap;
use tonic::Status;

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub stripe_account_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtraClaims {
    #[serde(rename = "urn:zitadel:iam:user:metadata")]
    pub metadata: Option<Metadata>,
}

pub async fn verify_token(
    metadata: &MetadataMap,
    verifier: &RemoteJwksVerifier,
) -> Result<(String, Option<Metadata>), Status> {
    let token = metadata
        .get(AUTHORIZATION.as_str())
        .and_then(|v| v.to_str().ok())
        .and_then(|header_value| header_value.split_once(' '))
        .map(|(_, token)| token.to_string())
        .ok_or_else(|| Status::unauthenticated(""))?;

    let header_and_claims = verifier
        .verify::<Claims<ExtraClaims>>(&token)
        .await
        .map_err(|err| Status::unauthenticated(err.to_string()))?;

    let user_id = header_and_claims
        .claims()
        .sub
        .clone()
        .ok_or_else(|| Status::unauthenticated(""))?;

    let metadata = header_and_claims.claims().extra.extra.metadata.clone();

    Ok((user_id, metadata))
}
