use std::sync::Arc;

use domain::{
    models::ActiveEntity,
    ports::domain::AbstractActiveService,
};

use crate::commands::ActiveCommands;

pub struct CliActiveController {
    active_service: Arc<dyn AbstractActiveService>,
}

impl CliActiveController {
    pub fn new(active_service: Arc<dyn AbstractActiveService>) -> Self {
        Self { active_service }
    }

    pub async fn get_active(&self, active_id: &u32) {
        self.active_service.get_active(*active_id).await.unwrap();
    }

    pub async fn create_active(&self, command: &ActiveCommands) {
        let entity = ActiveEntity::try_from(command).unwrap();
        self.active_service.create_active(entity).await.unwrap();
    }

    pub async fn update_active(&self, command: &ActiveCommands) {
        let entity = ActiveEntity::try_from(command).unwrap();
        self.active_service.update_active(entity).await.unwrap();
    }

    pub async fn list_actives(&self) {
        self.active_service.list_actives().await.unwrap();
    }

    pub async fn list_user_actives(&self, user_id: &u32) {
        self.active_service
            .list_user_actives(*user_id)
            .await
            .unwrap();
    }

    pub async fn delete_active(&self, active_id: &u32) {
        self.active_service.delete_active(*active_id).await.unwrap();
    }
}
