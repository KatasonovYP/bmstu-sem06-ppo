use std::sync::Arc;

use domain::{
    models::UserEntity,
    ports::domain::AbstractUserService,
};

use crate::commands::UserCommands;

pub struct CliUserController {
    user_service: Arc<dyn AbstractUserService>,
}

impl CliUserController {
    pub fn new(user_service: Arc<dyn AbstractUserService>) -> Self {
        Self { user_service }
    }

    pub async fn get_user(&self, user_id: &u32) {
        self.user_service.get_user(*user_id).await.unwrap();
    }

    pub async fn create_user(&self, command: &UserCommands) {
        let entity = UserEntity::try_from(command).unwrap();
        self.user_service.create_user(entity).await.unwrap();
    }

    pub async fn update_user(&self, command: &UserCommands) {
        let entity = UserEntity::try_from(command).unwrap();
        self.user_service.update_user(entity).await.unwrap();
    }

    pub async fn list_users(&self) {
        self.user_service.list_users().await.unwrap();
    }

    pub async fn delete_user(&self, user_id: &u32) {
        self.user_service.delete_user(*user_id).await.unwrap();
    }
}
