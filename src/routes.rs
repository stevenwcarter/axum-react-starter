use crate::api::api_routes;
use crate::context::GraphQLContext;
use crate::graphql::{create_schema, Schema};

use axum::extract::{Request, WebSocketUpgrade};
use axum::http::{self, HeaderValue};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{get, on, MethodFilter};
use axum::{Extension, Router};
use axum_embed::{FallbackBehavior, ServeEmbed};
use http::Method;
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_axum::{graphiql, playground, subscriptions};
use juniper_graphql_ws::ConnectionConfig;
use rust_embed::RustEmbed;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};

#[derive(RustEmbed, Clone)]
#[folder = "site/build/"]
struct SiteAssets;

pub fn app(context: GraphQLContext) -> Router {
    let qm_schema = create_schema();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let middleware = ServiceBuilder::new()
        .layer(cors)
        .layer(CompressionLayer::new());
    let graphql_routes = Router::new()
        .route(
            "/",
            on(MethodFilter::GET.or(MethodFilter::POST), custom_graphql),
        )
        .route("/subscriptions", get(custom_subscriptions))
        .route(
            "/graphiql",
            get(graphiql("/graphql", "/graphql/subscriptions")),
        )
        .route(
            "/playground",
            get(playground("/graphql", "/graphql/subscriptions")),
        )
        .route("/test", get(root))
        .layer(Extension(context.clone()))
        .layer(Extension(Arc::new(qm_schema)));

    let serve_assets = ServeEmbed::<SiteAssets>::with_parameters(
        Some("/index.html".to_owned()),
        FallbackBehavior::Ok,
        None,
    );

    let fallback_serve_assets = serve_assets.clone();

    Router::new()
        .route_service("/assets/{*uri}", serve_assets)
        .layer(middleware::from_fn(set_static_cache_control))
        .nest("/graphql", graphql_routes)
        .nest("/api/v1", api_routes(context.clone()))
        .fallback_service(fallback_serve_assets)
        .layer(Extension(context.clone()))
        .layer(middleware)
}

async fn root() -> &'static str {
    "Hello world!"
}

#[axum::debug_handler]
async fn custom_subscriptions(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(context): Extension<GraphQLContext>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.protocols(["graphql-transport-ws", "graphql-ws"])
        .on_upgrade(move |socket| {
            let connection_config =
                ConnectionConfig::new(context.clone()).with_max_in_flight_operations(10);
            subscriptions::serve_ws(socket, schema, connection_config)
        })
}

#[axum::debug_handler]
async fn custom_graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(context): Extension<GraphQLContext>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(request.execute(&*schema, &context).await)
}

async fn set_static_cache_control(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        http::header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000"),
    );
    response
}
