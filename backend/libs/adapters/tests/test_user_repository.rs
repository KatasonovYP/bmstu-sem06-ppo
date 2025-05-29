use std::sync::Arc;

use adapters::{
    di_domain_module::di_domain_module,
    settings::Settings,
};
use domain::{
    errors::DomainError,
    models::UserEntity,
    ports::storage::UserRepository,
};
use fake::{
    Fake,
    Faker,
};
use shaku::HasComponent;

#[tokio::test]
async fn test_user_repository() {
    let settings = Settings::new().unwrap();
    let module = di_domain_module(settings).await;
    let user_repo: Arc<dyn UserRepository> = module.resolve();

    let users_on_start_test = user_repo.list_users().await.unwrap();

    let test_user: UserEntity = Faker.fake();

    let result_user = user_repo.create_user(test_user.clone()).await.unwrap();

    let wrong_user: UserEntity = Faker.fake();

    let mut expected_user = test_user.clone();
    expected_user.user_id = result_user.user_id;
    assert_eq!(result_user, expected_user);

    let mut updated_test_user: UserEntity = Faker.fake();
    updated_test_user.user_id = result_user.user_id;
    let expected_updated_user = updated_test_user.clone();

    let result_user = user_repo.get_user(result_user.user_id).await.unwrap();
    assert_eq!(result_user, expected_user);

    let user_error = user_repo.get_user(wrong_user.user_id).await.err().unwrap();
    assert!(matches!(user_error, DomainError::EntityNotFound { .. }));

    let result_user = user_repo.update_user(updated_test_user).await.unwrap();
    assert_eq!(result_user, expected_updated_user);

    let user_error = user_repo
        .delete_user(wrong_user.user_id)
        .await
        .err()
        .unwrap();
    assert!(matches!(user_error, DomainError::EntityNotFound { .. }));

    let result_user = user_repo.delete_user(result_user.user_id).await.unwrap();
    assert_eq!(result_user, expected_user);

    let users_on_end_test = user_repo.list_users().await.unwrap();
    assert_eq!(users_on_start_test, users_on_end_test);
}
