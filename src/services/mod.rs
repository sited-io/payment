use stripe::{ParseIdError, StripeError};
use tonic::Status;
use uuid::Uuid;

pub mod countries;
mod stripe_service;
pub use stripe_service::StripeService;

pub fn parse_uuid(uuid_string: &str, field: &str) -> Result<Uuid, Status> {
    uuid_string.parse().map_err(|_| {
        Status::invalid_argument(format!(
            "field {field} is not a valid UUID v4"
        ))
    })
}

pub fn stripe_error_to_status(err: StripeError) -> Status {
    tracing::log::error!("{err}");
    Status::internal("")
}

pub fn parse_id_error_to_status(err: ParseIdError) -> Status {
    tracing::log::error!("{err}");
    Status::internal("")
}
