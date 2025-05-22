use std::{
    net::SocketAddr,
    sync::Arc,
};

use aide::{
    axum::{
        routing::{
            get,
            get_with,
            post_with,
        },
        ApiRouter,
        IntoApiResponse,
    },
    openapi::{
        OpenApi,
        Tag,
    },
    scalar::Scalar,
    swagger::Swagger,
    transform::TransformOpenApi,
};
use axum::{
    extract::Request,
    response::IntoResponse,
    Extension,
    Json,
    ServiceExt,
};
use shaku::HasComponent;
use stocks_tracker::app::{
    api::controllers::{
        health_controller::ApiHealthController,
        user_controller::ApiUserController,
    },
    di_domain_module::di_domain_module,
};
use tower_http::{
    self,
    normalize_path::NormalizePathLayer,
};
use tower_layer::Layer;

pub fn docs_routes() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::generate::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Scalar::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .api_route_with(
            "/swagger",
            get_with(
                Swagger::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    aide::generate::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}

// async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
//     Json(api)
// }

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
}

#[tokio::main]
async fn main() {
    let module = di_domain_module().await;

    let user_controller = ApiUserController::new(module.resolve());
    // let active_controller = ApiActiveController::new(module.resolve());
    // let notification_controller = ApiNotificationController::new(module.resolve());
    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);
    let mut api = OpenApi::default();
    let app = ApiRouter::new()
        .api_route("/ping", get(ApiHealthController::get_ping))
        .api_route("/users/{user_id}", get(ApiUserController::get_user))
        .api_route(
            "/users",
            post_with(
                ApiUserController::create_user,
                ApiUserController::create_user_docs,
            ),
        )
        .with_state(user_controller);
    // .api_route("/actives/{id}", get(ApiActiveController::get_active))
    // .api_route("/actives", post(ApiActiveController::create_active))
    // .api_route("/actives", get(ApiActiveController::list_actives))
    // .api_route(
    //     "/users/{user_id}/actives",
    //     get(ApiActiveController::list_user_actives),
    // )
    // .with_state(active_controller)
    // .route(
    //     "/notifications/{id}",
    //     get(ApiNotificationController::get_notification),
    // )
    // .route(
    //     "/notifications",
    //     post(ApiNotificationController::create_notification),
    // )
    // .with_state(notification_controller);
    // .route("/api.json", get(serve_api));

    // let app = app
    //     .layer(
    //         TraceLayer::new_for_http()
    //             .on_request(())
    //             .on_response(())
    //             .on_failure(())
    //             .make_span_with(
    //                 DefaultMakeSpan::new()
    //                     .include_headers(true)
    //                     .level(tracing::Level::INFO),
    //             ),
    //     )
    //     .layer(middleware::from_fn(tma_middleware));

    let app = app
        .nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server runs on {addr}");

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
