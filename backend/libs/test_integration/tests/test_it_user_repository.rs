use domain::{
    errors::DomainError,
    models::UserEntity,
};
use fake::{
    Fake,
    Faker,
};
use test_utils::TestManager;

#[tokio::test(flavor = "multi_thread")]
async fn test_it_create_user_success() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let expected_user: UserEntity = Faker.fake();
    let result_user = tm.user_repo.create_user(&expected_user).await.unwrap();

    // Assert
    assert_eq!(result_user, expected_user);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_create_user_error_duplicate() {
    // Arrange
    let tm = TestManager::default().await;
    let user = Faker.fake();
    let _ = tm.user_repo.create_user(&user).await.unwrap();

    // Act
    let error = tm.user_repo.create_user(&user).await.unwrap_err();

    // Assert
    match error {
        DomainError::RepositoryError(msg) => {
            assert!(
                msg.contains("duplicate key value") || msg.contains("UNIQUE constraint failed"),
                "Unexpected error message: {msg}"
            )
        },
        _ => panic!("Expected RepositoryError"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_get_user_success() {
    // Arrange
    let tm = TestManager::default().await;
    let user: UserEntity = Faker.fake();
    let created_user = tm.user_repo.create_user(&user).await.unwrap();

    // Act
    let fetched_user = tm.user_repo.get_user(created_user.user_id).await.unwrap();

    // Assert
    assert_eq!(created_user, fetched_user);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_get_not_existing_user_error() {
    // Arrange
    let tm = TestManager::default().await;
    let user_id = Faker.fake();

    // Act
    let error = tm.user_repo.get_user(user_id).await.unwrap_err();

    // Assert
    match error {
        DomainError::EntityNotFound { entity, id } => {
            assert_eq!(user_id.to_string(), id);
            assert_eq!(entity, "User");
        },
        _ => panic!("Expected EntityNotFound"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_get_deleted_user_error() {
    // Arrange
    let tm = TestManager::default().await;
    let user: UserEntity = Faker.fake();

    // Act
    let created_user = tm.user_repo.create_user(&user).await.unwrap();

    let deleted_user = tm
        .user_repo
        .delete_user(created_user.user_id)
        .await
        .unwrap();
    let fetch_error = tm
        .user_repo
        .get_user(deleted_user.user_id)
        .await
        .unwrap_err();

    // Assert
    match fetch_error {
        DomainError::EntityNotFound { entity, id } => {
            assert_eq!(deleted_user.user_id.to_string(), id);
            assert_eq!(entity, "User");
        },
        _ => panic!("Expected EntityNotFound"),
    }
    assert_eq!(deleted_user, created_user);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_delete_user_success() {
    // Arrange
    let tm = TestManager::default().await;
    let user: UserEntity = Faker.fake();

    // Act
    let created_user = tm.user_repo.create_user(&user).await.unwrap();

    let deleted_user = tm
        .user_repo
        .delete_user(created_user.user_id)
        .await
        .unwrap();

    // Assert
    assert_eq!(created_user, deleted_user);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_delete_user_error() {
    // Arrange
    let tm = TestManager::default().await;
    let user_id = Faker.fake();

    // Act
    let error = tm.user_repo.delete_user(user_id).await.unwrap_err();

    // Assert
    match error {
        DomainError::EntityNotFound { entity, id } => {
            assert_eq!(user_id.to_string(), id);
            assert_eq!(entity, "User");
        },
        _ => panic!("Expected EntityNotFound"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_update_user_success() {
    // Arrange
    let tm = TestManager::default().await;
    let orig_user: UserEntity = Faker.fake();

    // Act
    let created_user = tm.user_repo.create_user(&orig_user).await.unwrap();

    let expected_user = UserEntity {
        user_id: created_user.user_id,
        ..Faker.fake()
    };

    let updated_user = tm.user_repo.update_user(&expected_user).await.unwrap();

    // Assert
    assert_eq!(expected_user, updated_user);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_update_not_existing_user_error() {
    // Arrange
    let tm = TestManager::default().await;
    let user: UserEntity = Faker.fake();

    // Act
    let error = tm.user_repo.update_user(&user).await.unwrap_err();

    // Assert
    match error {
        DomainError::RepositoryError(msg) => {
            assert!(
                msg.contains("None of the records are updated"),
                "Unexpected error message: {msg}"
            )
        },
        _ => panic!("Expected RepositoryError"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_update_deleted_user_error() {
    // Arrange
    let tm = TestManager::default().await;
    let user: UserEntity = Faker.fake();
    let created_user = tm.user_repo.create_user(&user).await.unwrap();
    let _ = tm
        .user_repo
        .delete_user(created_user.user_id)
        .await
        .unwrap();

    // Act
    let error = tm.user_repo.update_user(&created_user).await.unwrap_err();

    // Assert
    match error {
        DomainError::RepositoryError(msg) => {
            assert!(
                msg.contains("None of the records are updated"),
                "Unexpected error message: {msg}"
            )
        },
        _ => panic!("Expected RepositoryError"),
    }
}
