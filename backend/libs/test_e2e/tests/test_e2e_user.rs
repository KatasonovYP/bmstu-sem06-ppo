use domain::{
    models::UserEntity,
    value_objects::Username,
};
use http_client_stocks_tracker::{
    apis,
    models::ActiveRequest,
};
use secrecy::ExposeSecret;
use test_utils::TestManager;

#[tokio::test(flavor = "multi_thread")]
async fn test_e2e_user() {
    let tm = TestManager::default().await;

    tm.user_repo
        .create_user(&UserEntity {
            user_id: 1,
            tg_id: 413776157,
            chat_id: 413776157,
            username: Username::new("KatasonovYP").unwrap(),
            first_name: Some("".to_string()),
            second_name: Some("".to_string()),
        })
        .await
        .unwrap();

    let configuration = apis::configuration::Configuration {
        base_path: tm.settings.e2e_backend_taget_url,
        user_agent: Option::None,
        client: reqwest::Client::new(),
        basic_auth: Option::None,
        oauth_access_token: Option::None,
        bearer_access_token: Option::None,
        api_key: Some(apis::configuration::ApiKey {
            prefix: Some("Bearer".to_string()),
            key: tm.settings.e2e_token.expose_secret().into(),
        }),
    };

    apis::health_api::get_ping(&configuration).await.unwrap();

    let response = apis::active_api::list_user_actives(&configuration)
        .await
        .unwrap();
    let actives_count = response.len();

    apis::active_api::create_active(
        &configuration,
        ActiveRequest {
            bought_price: 20.0,
            count: 10,
            currency: "RUB".to_string(),
            security_id: "AFLT".to_string(),
            user_id: 0,
        },
    )
    .await
    .unwrap();

    let response = apis::active_api::list_user_actives(&configuration)
        .await
        .unwrap();

    assert_eq!(actives_count + 1, response.len());
}
