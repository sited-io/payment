use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{Asterisk, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::db::DbError;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "stripe_accounts")]
pub enum StripeAccountIden {
    Table,
    StripeAccountId,
    ShopId,
    UserId,
    CreatedAt,
    UpdatedAt,
}

pub struct StripeAccount {
    pub stripe_account_id: String,
    pub shop_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl StripeAccount {
    pub async fn create(
        pool: &Pool,
        shop_id: &Uuid,
        stripe_account_id: &String,
        user_id: &String,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(StripeAccountIden::Table)
            .columns([
                StripeAccountIden::ShopId,
                StripeAccountIden::StripeAccountId,
                StripeAccountIden::UserId,
            ])
            .values([
                (*shop_id).into(),
                stripe_account_id.into(),
                user_id.into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn get(
        pool: &Pool,
        shop_id: &Uuid,
    ) -> Result<Option<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(StripeAccountIden::Table)
            .and_where(Expr::col(StripeAccountIden::ShopId).eq(*shop_id))
            .build_postgres(PostgresQueryBuilder);

        Ok(client
            .query_opt(sql.as_str(), &values.as_params())
            .await?
            .map(Self::from))
    }

    pub async fn get_for_user(
        pool: &Pool,
        shop_id: &Uuid,
        user_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(StripeAccountIden::Table)
            .and_where(Expr::col(StripeAccountIden::ShopId).eq(*shop_id))
            .and_where(Expr::col(StripeAccountIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        Ok(client
            .query_opt(sql.as_str(), &values.as_params())
            .await?
            .map(Self::from))
    }
}

impl From<&Row> for StripeAccount {
    fn from(row: &Row) -> Self {
        Self {
            stripe_account_id: row
                .get(StripeAccountIden::StripeAccountId.to_string().as_str()),
            shop_id: row.get(StripeAccountIden::ShopId.to_string().as_str()),
            user_id: row.get(StripeAccountIden::UserId.to_string().as_str()),
            created_at: row
                .get(StripeAccountIden::CreatedAt.to_string().as_str()),
            updated_at: row
                .get(StripeAccountIden::UpdatedAt.to_string().as_str()),
        }
    }
}

impl From<Row> for StripeAccount {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}
