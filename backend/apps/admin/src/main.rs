use std::net::SocketAddr;

use adapters::{
    di_domain_module::BuildAppModule,
    postgres::schema::*,
    settings::Settings,
};
use async_graphql::http::{
    GraphQLPlaygroundConfig,
    playground_source,
};
use async_graphql_axum::{
    GraphQLRequest,
    GraphQLResponse,
};
use axum::{
    Json,
    Router,
    extract::State,
    http::StatusCode,
    response::{
        Html,
        IntoResponse,
    },
    routing::{
        get,
        post,
    },
};
use axum_static_s3::S3OriginBuilder;
use http::HeaderMap;
use sea_orm::{
    Database,
    DatabaseConnection,
};
use sea_orm_pro::{
    ConfigParser,
    JsonCfg,
};
use seaography::{
    Builder,
    BuilderContext,
    async_graphql::{
        self,
        dynamic::{
            Schema,
            SchemaError,
        },
    },
    lazy_static,
};

#[derive(Clone)]
struct AppState {
    connection: DatabaseConnection,
    settings: Settings,
}

async fn admin_panel_config() -> Result<Json<JsonCfg>, (StatusCode, &'static str)> {
    let config = ConfigParser::new()
        .load_config("pro_admin")
        .expect("Invalid TOML Config");
    Ok(Json(config))
}

async fn graphql_playground() -> impl IntoResponse {
    // Setup GraphQL playground web and specify the endpoint for GraphQL resolver
    let config = GraphQLPlaygroundConfig::new("/api/graphql").with_header("Authorization", "");

    let res = playground_source(config).replace(
        r#""Authorization":"""#,
        r#""Authorization":`Bearer ${localStorage.getItem('auth_token')}`"#,
    );

    Html(res)
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PasswordLoginParams {
    pub email: String,
    pub password: String,
}

async fn user_login(
    state: State<AppState>,
    params: Json<PasswordLoginParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    if params.email != state.settings.admin_user_login {
        panic!("unauthorized!");
    }
    if params.password != state.settings.admin_user_password.expose_secret() {
        panic!("unauthorized!");
    }

    Ok(Json(serde_json::json!({
        "token": state.settings.admin_user_token.expose_secret(),
        "pid": state.settings.admin_user_pid,
        "name": "Demo User",
        "is_verified": true,
    })))
}

fn check_user_auth(
    headers: &HeaderMap,
    admin_user_token: &str,
) -> Result<(), (StatusCode, &'static str)> {
    let auth_header = headers.get("AUTHORIZATION");

    let Some(auth_header) = auth_header else {
        panic!("unauthorized!");
    };
    if !auth_header.to_str().unwrap().ends_with(admin_user_token) {
        panic!("unauthorized!");
    }

    Ok(())
}

async fn current_user(
    state: State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    check_user_auth(&headers, state.settings.admin_user_token.expose_secret())?;

    Ok(Json(serde_json::json!({
        "pid": &state.settings.admin_user_pid,
        "name": "Demo User",
        "email": &state.settings.admin_user_login,
    })))
}

seaography::register_entity_modules!([users, actives, notifications, sent]);

lazy_static::lazy_static! {
    static ref CONTEXT: BuilderContext = BuilderContext::default();
}

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    let builder = Builder::new(&CONTEXT, database.clone());
    let builder = register_entity_modules(builder);
    builder
        .set_depth_limit(depth)
        .set_complexity_limit(complexity)
        .schema_builder()
        .data(database)
        .finish()
}

async fn graphql_handler(
    state: State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, (StatusCode, &'static str)> {
    check_user_auth(&headers, state.settings.admin_user_token.expose_secret())?;
    const DEPTH: Option<usize> = None;
    const COMPLEXITY: Option<usize> = None;
    let schema = schema(state.connection.clone(), DEPTH, COMPLEXITY).unwrap();
    let res = schema.execute(req.into_inner()).await.into();
    Ok(res)
}

use aws_config::{
    BehaviorVersion,
    Region,
    SdkConfig as AwsSdkConfig,
};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::new("config/app.default.yaml").unwrap();
    BuildAppModule::new(&settings).build().await;

    let s3_config = AwsSdkConfig::builder()
        .endpoint_url("https://storage.yandexcloud.net".to_string())
        .region(Region::new("ru-central1".to_string()))
        .behavior_version(BehaviorVersion::v2025_08_07())
        .build();

    let s3_origin = S3OriginBuilder::new()
        .config(s3_config.clone())
        .bucket(&settings.admin_static_s3_bucket)
        .build()
        .expect("Failed to build S3 origin");

    let s3_origin_home = S3OriginBuilder::new()
        .config(s3_config)
        .bucket(&settings.admin_static_s3_bucket)
        .prefix("admin/index.html")
        .prune_path(1)
        .build()
        .expect("Failed to build S3 origin");

    let connection = Database::connect(settings.build_postgres_connection_string().expose_secret())
        .await
        .expect("Database connection failed");

    let state = AppState {
        connection,
        settings: settings.clone(),
    };
    let app = Router::new()
        .route("/api/admin/config", get(admin_panel_config))
        .route("/api/auth/login", post(user_login))
        .route("/api/user/current", get(current_user))
        .route("/api/graphql", get(graphql_playground))
        .route("/api/graphql", post(graphql_handler))
        .route_service("/admin", s3_origin_home.clone())
        .route_service("/admin/{*path}", s3_origin.clone())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.api_server_port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
