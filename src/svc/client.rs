use crate::{context::GraphQLContext, db::get_conn, models::Client, schema::clients};
use anyhow::{Context, Result};
use diesel::prelude::*;

pub struct ClientSvc {}

impl ClientSvc {
    pub fn get(context: &GraphQLContext, client_uuid: &str) -> Result<Client> {
        clients::table
            .filter(clients::uuid.eq(client_uuid))
            .select(Client::as_select())
            .first(&mut get_conn(context))
            .context("Could not find client")
    }
    pub fn list(context: &GraphQLContext, limit: i32, offset: i32) -> Result<Vec<Client>> {
        let limit: i64 = limit.into();
        let offset: i64 = offset.into();

        clients::table
            .select(Client::as_select())
            .order_by(clients::name.asc())
            .limit(limit)
            .offset(offset)
            .load::<Client>(&mut get_conn(context))
            .context("Could not load rates")
    }
    pub fn create(context: &GraphQLContext, client: &Client) -> Result<Client> {
        diesel::insert_into(clients::table)
            .values(client)
            .execute(&mut get_conn(context))
            .context("Could not update client")?;

        Self::get(context, &client.uuid)
    }
    pub fn update(context: &GraphQLContext, client: &Client) -> Result<Client> {
        diesel::update(clients::table)
            .filter(clients::uuid.eq(&client.uuid))
            .set(client)
            .execute(&mut get_conn(context))
            .context("Could not update client")?;

        Self::get(context, &client.uuid)
    }
    pub fn delete(context: &GraphQLContext, client_uuid: &str) -> Result<()> {
        diesel::delete(clients::table)
            .filter(clients::uuid.eq(client_uuid))
            .execute(&mut get_conn(context))
            .context("Could not delete client")?;

        Ok(())
    }
}
