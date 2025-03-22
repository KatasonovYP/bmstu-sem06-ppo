use axum::{http::StatusCode, Json};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PingResponse {
    status_code: u16,
}

#[derive(Clone)]
pub struct HealthController {}

impl HealthController {
    #[axum_macros::debug_handler]
    pub async fn get_ping() -> Json<PingResponse> {
        Json(PingResponse {
            status_code: StatusCode::OK.as_u16(),
        })
    }
}
