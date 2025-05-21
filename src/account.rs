use crate::{
    context::GraphQLContext, db::get_conn, models::Account, product::ProductSvc, schema::account,
};
use anyhow::{Context, Result};
use diesel::prelude::*;

pub struct AccountSvc {}

impl AccountSvc {
    pub fn list(context: &GraphQLContext) -> Result<Vec<Account>> {
        let conn = &mut get_conn(context);

        account::table
            .load::<Account>(conn)
            .context("Could not load accounts")
    }
    pub fn get(context: &GraphQLContext, account_uuid: &str) -> Result<Account> {
        let conn = &mut get_conn(context);

        account::table
            .filter(account::uuid.eq(account_uuid))
            .first(conn)
            .context("Could not find account")
    }

    pub fn update(context: &GraphQLContext, account: &Account) -> Result<Account> {
        let conn = &mut get_conn(context);

        diesel::replace_into(account::table)
            .values(account)
            .execute(conn)
            .context("Could not update account")?;

        Self::get(context, account.uuid.as_str())
    }

    pub fn delete(context: &GraphQLContext, account_uuid: &str) -> Result<()> {
        let conn = &mut get_conn(context);

        diesel::delete(account::table)
            .filter(account::uuid.eq(account_uuid))
            .execute(conn)
            .context("could not delete account")?;

        ProductSvc::delete_for_account(context, account_uuid)?;

        Ok(())
    }
}
