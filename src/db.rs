use std::ops::DerefMut;

use deadpool_postgres::tokio_postgres::error::SqlState;
use deadpool_postgres::{
    tokio_postgres::NoTls, Config, CreatePoolError, Pool, PoolError, Runtime,
    SslMode,
};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use refinery::Target;
use tonic::Status;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[derive(Debug)]
pub enum DbError {
    TokioPostgres(deadpool_postgres::tokio_postgres::Error),
    Pool(PoolError),
    CreatePool(CreatePoolError),
    SeaQuery(sea_query::error::Error),
}

impl DbError {
    pub fn ignore_to_ts_query<T>(self, default: T) -> Result<T, Self> {
        if let Self::TokioPostgres(err) = &self {
            if let Some(err) = err.as_db_error() {
                if *err.code() == SqlState::SYNTAX_ERROR
                    && err.routine() == Some("toTSQuery")
                {
                    tracing::log::warn!("{err:?}");
                    return Ok(default);
                }
            }
        }

        Err(self)
    }
}

impl From<deadpool_postgres::tokio_postgres::Error> for DbError {
    fn from(err: deadpool_postgres::tokio_postgres::Error) -> Self {
        Self::TokioPostgres(err)
    }
}

impl From<PoolError> for DbError {
    fn from(err: PoolError) -> Self {
        Self::Pool(err)
    }
}

impl From<CreatePoolError> for DbError {
    fn from(err: CreatePoolError) -> Self {
        Self::CreatePool(err)
    }
}

impl From<sea_query::error::Error> for DbError {
    fn from(err: sea_query::error::Error) -> Self {
        Self::SeaQuery(err)
    }
}

impl From<DbError> for Status {
    fn from(err: DbError) -> Self {
        match err {
            DbError::TokioPostgres(tp_err) => {
                if let Some(err) = tp_err.as_db_error() {
                    match *err.code() {
                        SqlState::UNIQUE_VIOLATION => {
                            Status::already_exists(err.message())
                        }
                        SqlState::SYNTAX_ERROR => {
                            tracing::log::error!("{err:?}");
                            Status::internal("")
                        }
                        SqlState::FOREIGN_KEY_VIOLATION => {
                            Status::failed_precondition(err.message())
                        }
                        _ => {
                            tracing::log::error!("{err:?}");
                            Status::internal("")
                        }
                    }
                } else {
                    tracing::log::error!("{:?}", tp_err);
                    Status::internal("")
                }
            }
            DbError::Pool(pool_err) => {
                tracing::log::error!("{pool_err:?}");
                Status::internal("")
            }
            DbError::CreatePool(create_pool_err) => {
                tracing::log::error!("{create_pool_err:?}");
                Status::internal("")
            }
            DbError::SeaQuery(sea_query_err) => {
                tracing::log::error!("{sea_query_err:?}");
                Status::internal("")
            }
        }
    }
}

pub fn init_db_pool(
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    root_cert: Option<String>,
) -> Result<Pool, CreatePoolError> {
    let mut config = Config::new();
    config.host = Some(host);
    config.port = Some(port);
    config.user = Some(user);
    config.password = Some(password);
    config.dbname = Some(dbname);

    if let Some(root_cert) = root_cert {
        println!("Using root cert {}", root_cert);
        config.ssl_mode = Some(SslMode::Require);
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_ca_file(root_cert).unwrap();
        let connector = MakeTlsConnector::new(builder.build());
        config.create_pool(Some(Runtime::Tokio1), connector)
    } else {
        config.ssl_mode = Some(SslMode::Prefer);
        config.create_pool(Some(Runtime::Tokio1), NoTls)
    }
}

pub async fn migrate(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = pool.get().await?;

    let runner = embedded::migrations::runner();
    runner
        .set_target(Target::Latest)
        .run_async(client.deref_mut().deref_mut())
        .await?;

    Ok(())
}
