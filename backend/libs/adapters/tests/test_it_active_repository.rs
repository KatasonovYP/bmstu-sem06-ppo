#[path = "./utils.rs"]
mod utils;

use domain::{
    errors::DomainError,
    models::ActiveEntity,
};
use fake::{
    Fake,
    Faker,
};
use utils::TestManager;

#[tokio::test(flavor = "multi_thread")]
async fn test_it_create_active_success() {
    // Arrange
    let tm = TestManager::default().await;
    // Act
    let expected_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };

    let result_active = tm
        .active_repo
        .create_active(&expected_active)
        .await
        .unwrap();

    // Assert
    assert_eq!(result_active, expected_active);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_create_active_error() {
    // Arrange
    let tm = TestManager::default().await;
    // Act
    let active_error = tm
        .active_repo
        .create_active(&Faker.fake())
        .await
        .unwrap_err();

    // Assert
    match active_error {
        DomainError::RepositoryError(msg) => {
            assert!(msg.contains("violates foreign key constraint"))
        },
        _ => panic!("Expected RepositoryError"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_get_active_success() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let expected_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };

    let created_active = tm
        .active_repo
        .create_active(&expected_active)
        .await
        .unwrap();

    let got_active = tm
        .active_repo
        .get_active(created_active.active_id)
        .await
        .unwrap();

    // Assert
    assert_eq!(expected_active, got_active);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_get_not_existing_active_error() {
    // Arrange
    let tm = TestManager::default().await;
    let active_id = Faker.fake();

    // Act
    let active_error = tm.active_repo.get_active(active_id).await.unwrap_err();

    // Assert
    match active_error {
        DomainError::EntityNotFound { entity, id } => {
            assert_eq!(active_id.to_string(), id);
            assert_eq!(entity, "Active");
        },
        _ => panic!("Expected EntityNotFound"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_get_deleted_active_error() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let expected_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };

    let created_active = ActiveEntity {
        active_id: expected_active.active_id,
        ..tm.active_repo
            .create_active(&expected_active)
            .await
            .unwrap()
    };

    let deleted_active = tm
        .active_repo
        .delete_active(created_active.active_id)
        .await
        .unwrap();

    let got_active_error = tm
        .active_repo
        .get_active(deleted_active.active_id)
        .await
        .unwrap_err();

    // Assert
    match got_active_error {
        DomainError::EntityNotFound { entity, id } => {
            assert_eq!(deleted_active.active_id.to_string(), id);
            assert_eq!(entity, "Active");
        },
        _ => panic!("Expected EntityNotFound"),
    }
    // Assert
    assert_eq!(expected_active, deleted_active);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_delete_active_success() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let expected_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };

    let created_active = ActiveEntity {
        active_id: expected_active.active_id,
        ..tm.active_repo
            .create_active(&expected_active)
            .await
            .unwrap()
    };

    let deleted_active = tm
        .active_repo
        .delete_active(created_active.active_id)
        .await
        .unwrap();

    // Assert
    assert_eq!(expected_active, deleted_active);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_delete_active_error() {
    // Arrange
    let tm = TestManager::default().await;
    let active_id = Faker.fake();

    // Act
    let active_error = tm.active_repo.delete_active(active_id).await.unwrap_err();

    // Assert
    match active_error {
        DomainError::EntityNotFound { entity, id } => {
            assert_eq!(active_id.to_string(), id);
            assert_eq!(entity, "Active");
        },
        _ => panic!("Expected EntityNotFound"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_update_active_success() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let expected_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };

    let active_to_create = ActiveEntity {
        active_id: expected_active.active_id,
        user_id: expected_active.user_id,
        ..Faker.fake()
    };

    let created_active = tm
        .active_repo
        .create_active(&active_to_create)
        .await
        .unwrap();

    let updated_active = tm
        .active_repo
        .update_active(&expected_active)
        .await
        .unwrap();

    // Assert
    assert_eq!(expected_active, created_active);
    assert_eq!(expected_active, updated_active);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_update_not_existing_active_error() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let active_error = tm
        .active_repo
        .update_active(&Faker.fake())
        .await
        .unwrap_err();

    // Assert
    match active_error {
        DomainError::RepositoryError(msg) => {
            assert!(msg.contains("None of the records are updated"))
        },
        _ => panic!("Expected RepositoryError"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_it_update_deleted_active_error() {
    // Arrange
    let tm = TestManager::default().await;

    // Act
    let expected_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };

    let created_active = tm
        .active_repo
        .create_active(&expected_active)
        .await
        .unwrap();

    let _ = tm
        .active_repo
        .delete_active(created_active.active_id)
        .await
        .unwrap();

    let update_error = tm
        .active_repo
        .update_active(&expected_active)
        .await
        .unwrap_err();

    // Assert
    match update_error {
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
async fn test_it_update_active_with_invalid_user_id_error() {
    // Arrange
    let tm = TestManager::default().await;

    let valid_active = ActiveEntity {
        user_id: tm
            .user_repo
            .create_user(&Faker.fake())
            .await
            .unwrap()
            .user_id,
        ..Faker.fake()
    };
    let created_active = tm.active_repo.create_active(&valid_active).await.unwrap();

    let mut invalid_active = created_active.clone();
    invalid_active.user_id = Faker.fake();

    // Act
    let update_error = tm
        .active_repo
        .update_active(&invalid_active)
        .await
        .unwrap_err();

    // Assert
    match update_error {
        DomainError::RepositoryError(msg) => {
            assert!(
                msg.contains("violates foreign key constraint"),
                "Unexpected error message: {msg}",
            )
        },
        _ => panic!("Expected RepositoryError"),
    }
}
