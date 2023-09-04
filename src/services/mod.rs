mod stripe_service;

use stripe::{ParseIdError, StripeError};
pub use stripe_service::StripeService;
use tonic::Status;

pub fn stripe_error_to_status(err: StripeError) -> Status {
    tracing::log::error!("{err}");
    Status::internal("")
}

pub fn parse_id_error_to_status(err: ParseIdError) -> Status {
    tracing::log::error!("{err}");
    Status::internal("")
}
