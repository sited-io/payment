use std::time::Duration;

use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderName, Method};
use jwtk::jwk::RemoteJwksVerifier;
use payments::logging::{LogOnFailure, LogOnRequest, LogOnResponse};
use payments::zitadel::ZitadelService;
use stripe::Client;
use tonic::transport::Server;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use payments::api::peoplesmarkets::payments::v1::stripe_service_server::StripeServiceServer;
use payments::{get_env_var, StripeService};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging
    tracing_subscriber::fmt::init();

    // get required environment variables
    let host = get_env_var("HOST");

    let jwks_url = get_env_var("JWKS_URL");
    let jwks_host = get_env_var("JWKS_HOST");

    // initialize stripe client
    let stripe_client = Client::new(get_env_var("STRIPE_SECRET_KEY"));

    // initialize zitadel client service
    let zitadel_service = ZitadelService::new(
        get_env_var("ZITADEL_BASE_URL"),
        get_env_var("ZITADEL_CLIENT_ID"),
        get_env_var("ZITADEL_CLIENT_SECRET"),
    );

    // initialize client for JWT verification against public JWKS
    //   adding host header in order to work in private network
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::HOST,
        reqwest::header::HeaderValue::from_str(&jwks_host)?,
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // configure gRPC health reporter
    let (mut health_reporter, health_service) =
        tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<StripeServiceServer<StripeService>>()
        .await;

    // configure gRPC reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            tonic_health::pb::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            payments::api::peoplesmarkets::FILE_DESCRIPTOR_SET,
        )
        .build()
        .unwrap();

    let stripe_service = StripeService::build(
        RemoteJwksVerifier::new(
            jwks_url,
            Some(client),
            Duration::from_secs(120),
        ),
        stripe_client,
        zitadel_service,
    );

    tracing::log::info!("gRPC+web server listening on {}", host);

    Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .on_request(LogOnRequest::default())
                .on_response(LogOnResponse::default())
                .on_failure(LogOnFailure::default()),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([
                    AUTHORIZATION,
                    ACCEPT,
                    CONTENT_TYPE,
                    HeaderName::from_static("grpc-status"),
                    HeaderName::from_static("grpc-message"),
                    HeaderName::from_static("x-grpc-web"),
                ])
                .allow_methods([Method::POST])
                .allow_origin(AllowOrigin::any())
                .allow_private_network(true),
        )
        .accept_http1(true)
        .add_service(tonic_web::enable(reflection_service))
        .add_service(tonic_web::enable(health_service))
        .add_service(tonic_web::enable(stripe_service))
        .serve(host.parse().unwrap())
        .await?;

    Ok(())
}
