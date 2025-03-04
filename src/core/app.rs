use std::sync::Arc;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{ApiKeyLocation, OpenApi, SecurityScheme},
    swagger::Swagger,
    transform::TransformOpenApi,
};
use axum::{response::IntoResponse, Extension, Router};
use tokio::net::TcpListener;

use crate::account;

use super::{extractors::AppJson, AppConfig, AppState};

pub struct App;

impl App {
    pub async fn serve() {
        tracing_subscriber::fmt::init();

        let config = AppConfig::new();
        let state = AppState::new(config.clone()).await;

        let host = config.host;
        let port = config.port;

        let address = format!("{host}:{port}");
        let listener = TcpListener::bind(address.clone())
            .await
            .expect(format!("Failed to bind to {address}").as_str());

        let app = setup_router(vec![account::router(state)]);

        axum::serve(listener, app.into_make_service())
            .await
            .expect("Failed to serve app");
    }
}

fn setup_router<R>(routers: Vec<R>) -> Router
where
    R: Into<ApiRouter>,
{
    if super::utils::should_add_openapi_routes() {
        let mut api = OpenApi::default();
        let mut app: ApiRouter = ApiRouter::new();

        for router in routers {
            app = app.merge(router.into())
        }

        app.merge(openapi_route())
            .finish_api_with(&mut api, api_docs)
            .layer(Extension(Arc::new(api)))
    } else {
        let mut app = Router::new();

        for router in routers {
            app = app.merge(router.into())
        }

        app
    }
}

fn openapi_route() -> ApiRouter {
    aide::generate::infer_responses(true);

    let router = ApiRouter::new()
        .route("/", get(Swagger::new("/docs/openapi.json").axum_handler()))
        .route("/docs/openapi.json", get(openapi_docs));

    aide::generate::infer_responses(false);

    router
}

#[axum::debug_handler]
async fn openapi_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    AppJson(api).into_response()
}

fn api_docs(docs: TransformOpenApi) -> TransformOpenApi {
    docs.security_scheme(
        crate::core::constants::openapi::DEFAULT_SECURITY_SCHEME,
        SecurityScheme::ApiKey {
            location: ApiKeyLocation::Header,
            name: "X-Session-Id".into(),
            description: None,
            extensions: Default::default(),
        },
    )
}
