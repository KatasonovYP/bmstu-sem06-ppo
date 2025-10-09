use std::net::SocketAddr;

use adapters::{
    di_domain_module::di_domain_module,
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
    // templates: Tera,
    connection: DatabaseConnection,
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

const DEMO_USER: &str = "demo@sea-ql.org";
const DEMO_USER_PID: &str = "79a6243b-088d-5d95-9b16-a2d1689e291f";
const DEMO_USER_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJwaWQiOiI2MjRhOWMxZi1hMTQ5LTQ0Y2MtYjBhM03OTMzNDViZTlkOyTMiLCJleHAiOjE3MzY4NDc1OTcsImNsYWltcyI6bnVsbH0.w2dJzWUw343eAt_sWrngb065uwJK-SOgJ8gDBls7XHSKILNKGzh-ZG9VFEBwVl4356-vD1MM8Qo8Y2TcO5V-NA";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PasswordLoginParams {
    pub email: String,
    pub password: String,
}

async fn user_login(
    params: Json<PasswordLoginParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    if params.email != DEMO_USER {
        panic!("unauthorized!");
    }
    if params.password != DEMO_USER {
        panic!("unauthorized!");
    }

    Ok(Json(serde_json::json!({
        "token": DEMO_USER_TOKEN,
        "pid": DEMO_USER_PID,
        "name": "Demo User",
        "is_verified": true,
    })))
}

fn check_user_auth(headers: &HeaderMap) -> Result<(), (StatusCode, &'static str)> {
    let auth_header = headers.get("AUTHORIZATION");

    let Some(auth_header) = auth_header else {
        panic!("unauthorized!");
    };
    if !auth_header.to_str().unwrap().ends_with(DEMO_USER_TOKEN) {
        panic!("unauthorized!");
    }

    Ok(())
}

async fn current_user(
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, &'static str)> {
    check_user_auth(&headers)?;

    Ok(Json(serde_json::json!({
        "pid": DEMO_USER_PID,
        "name": "Demo User",
        "email": DEMO_USER,
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
    // Construct GraphQL schema
    let builder = Builder::new(&CONTEXT, database.clone());
    let builder = register_entity_modules(builder);
    builder
        // Maximum depth of the constructed query
        .set_depth_limit(depth)
        // Maximum complexity of the constructed query
        .set_complexity_limit(complexity)
        .schema_builder()
        // GraphQL schema with database connection
        .data(database)
        .finish()
}

async fn graphql_handler(
    state: State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, (StatusCode, &'static str)> {
    check_user_auth(&headers)?;
    // Maximum depth of the constructed query
    const DEPTH: Option<usize> = None;
    // Maximum complexity of the constructed query
    const COMPLEXITY: Option<usize> = None;
    // GraphQL schema
    let schema = schema(state.connection.clone(), DEPTH, COMPLEXITY).unwrap();
    // GraphQL handler
    let res = schema.execute(req.into_inner()).await.into();
    Ok(res)
}

use aws_config::{
    Region,
    SdkConfig as AwsSdkConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::new().unwrap();
    di_domain_module(settings.clone()).await;

    let s3_config = AwsSdkConfig::builder()
        .endpoint_url("https://storage.yandexcloud.net".to_string())
        .region(Region::new("ru-central1".to_string()))
        .build();

    let s3_origin = S3OriginBuilder::new()
        .config(s3_config.clone())
        .bucket("static.admin.stocks-tracker.ru")
        .build()
        .expect("Failed to build S3 origin");

    let s3_origin_home = S3OriginBuilder::new()
        .config(s3_config)
        .bucket("static.admin.stocks-tracker.ru")
        .prefix("admin/index.html")
        .prune_path(1)
        .build()
        .expect("Failed to build S3 origin");

    let connection = Database::connect(settings.postgres_connection_string)
        .await
        .expect("Database connection failed");

    // let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
    // .expect("Tera initialization failed");
    let state = AppState {
        // templates,
        connection,
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
