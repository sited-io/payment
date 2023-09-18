mod auth;
mod commerce;
mod model;
mod services;

pub mod api;
pub mod db;
pub mod logging;

pub use auth::init_jwks_verifier;
pub use commerce::CommerceService;
pub use services::*;

pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| {
        panic!("ERROR: Missing environment variable '{var}'")
    })
}
