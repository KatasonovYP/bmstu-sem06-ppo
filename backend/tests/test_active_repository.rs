use std::sync::Arc;

use fake::{
    Fake,
    Faker,
};
use shaku::HasComponent;
use stocks_tracker::{
    app::di_domain_module::di_domain_module,
    domain::{
        errors::DomainError,
        models::{
            ActiveEntity,
            UserEntity,
        },
        ports::storage::{
            ActiveRepository,
            UserRepository,
        },
    },
};

#[tokio::test]
async fn test_active_repository() {
    let module = di_domain_module().await;
    let active_repo: Arc<dyn ActiveRepository> = module.resolve();
    let user_repo: Arc<dyn UserRepository> = module.resolve();

    let actives_on_start_test = active_repo.list_actives().await.unwrap();

    let test_user: UserEntity = Faker.fake();
    let result_user = user_repo.create_user(test_user.clone()).await.unwrap();
    let mut test_active: ActiveEntity = Faker.fake();
    test_active.user_id = result_user.user_id;

    let result_active = active_repo
        .create_active(test_active.clone())
        .await
        .unwrap();
    test_active.active_id = result_active.active_id;

    let wrong_active: ActiveEntity = Faker.fake();

    let mut expected_active = test_active.clone();
    expected_active.active_id = result_active.active_id;
    assert_eq!(result_active, expected_active);

    let mut updated_test_active: ActiveEntity = Faker.fake();
    updated_test_active.active_id = result_active.active_id;
    updated_test_active.user_id = result_active.user_id;
    let expected_updated_active = updated_test_active.clone();

    let result_active = active_repo
        .get_active(result_active.active_id)
        .await
        .unwrap();
    assert_eq!(result_active, expected_active);

    let active_error = active_repo
        .get_active(wrong_active.active_id)
        .await
        .err()
        .unwrap();
    assert!(matches!(active_error, DomainError::EntityNotFound { .. }));

    let result_active = active_repo
        .update_active(updated_test_active)
        .await
        .unwrap();
    assert_eq!(result_active, expected_updated_active);

    let active_error = active_repo
        .delete_active(wrong_active.active_id)
        .await
        .err()
        .unwrap();
    assert!(matches!(active_error, DomainError::EntityNotFound { .. }));

    let result_active = active_repo
        .delete_active(result_active.active_id)
        .await
        .unwrap();
    assert_eq!(result_active, expected_active);

    let actives_on_end_test = active_repo.list_actives().await.unwrap();
    assert_eq!(actives_on_start_test, actives_on_end_test);
}
