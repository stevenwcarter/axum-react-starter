use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};

use crate::{
    context::GraphQLContext,
    models::{Client, ClientInput},
    svc::ClientSvc,
};

pub struct Query;

#[juniper::graphql_object(context = GraphQLContext)]
impl Query {
    // Clients
    pub async fn get_client(context: &GraphQLContext, client_uuid: String) -> FieldResult<Client> {
        graphql_translate_anyhow(ClientSvc::get(context, &client_uuid))
    }
    pub fn list_clients(
        context: &GraphQLContext,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> FieldResult<Vec<Client>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        graphql_translate_anyhow(ClientSvc::list(context, limit, offset))
    }
}

pub struct Mutation;

#[juniper::graphql_object(context = GraphQLContext)]
impl Mutation {
    // Clients
    pub async fn create_client(
        context: &GraphQLContext,
        client: ClientInput,
    ) -> FieldResult<Client> {
        graphql_translate_anyhow(ClientSvc::create(context, &client.into()))
    }
    pub async fn update_client(
        context: &GraphQLContext,
        client: ClientInput,
    ) -> FieldResult<Client> {
        graphql_translate_anyhow(ClientSvc::update(context, &client.into()))
    }
    pub async fn delete_client(context: &GraphQLContext, client_uuid: String) -> FieldResult<bool> {
        graphql_translate_anyhow(ClientSvc::delete(context, &client_uuid))?;
        Ok(true)
    }
}

// type LimiterResultStream = Pin<Box<dyn Stream<Item = Vec<LimiterResult>> + Send>>;

// pub struct Subscription;

// #[juniper::graphql_subscription(context = GraphQLContext)]
// impl Subscription {
//     /// Provides the ability to view your rate limits in real time
//     async fn limits_viewer(
//         context: &GraphQLContext,
//         // API key to view limits for. Privileged accounts can see
//         // all rate limits
//         apikey: String,
//     ) -> LimiterResultStream {
//         let _context = context.clone();
//         let auth_apikey = ApiKey::from_hex(&apikey).expect("could not deserialize apikey");
//         let mut interval = tokio::time::interval(Duration::from_secs(5));
//         let stream = async_stream::stream! {
//             loop {
//                 interval.tick().await;
//                 yield LimiterResult::from_pools(&auth_apikey);
//             }
//         };
//
//         Box::pin(stream)
//     }
// }

pub type Schema = RootNode<Query, Mutation, EmptySubscription<GraphQLContext>>;
pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn graphql_translate_anyhow<T>(res: anyhow::Result<T>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}
