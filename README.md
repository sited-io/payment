# Payments service for peoplesmarkets.com

## Prerequesites

Ensure `service-apis` git submodule is initialized. If not yet done run:

```sh
git submodule update --init
```

If `service-apis` git submodule was already initialized, ensure to pull the newest changes:

```sh
git submodule update --remote
```

## Build

```sh
cargo build
```

## Run locally

Ensure environment variables are set.

```sh
export RUST_LOG=info
export RUST_BACKTRACE=0

export HOST="[::1]:10000"

export JWKS_URL='https://auth-dev.peoplesmarkets.com/oauth/v2/keys'
export JWKS_HOST='auth-dev.peoplesmarkets.com'

export STRIPE_SECRET_KEY="xxxx"
```

Then run:

```sh
cargo run
```
