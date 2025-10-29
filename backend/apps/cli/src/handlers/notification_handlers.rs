use crate::{
    commands::NotificationCommands,
    controllers::notification_controller::CliNotificationController,
};

pub async fn handle_notification_command(
    command: NotificationCommands,
    controller: &CliNotificationController,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        NotificationCommands::Get { notification_id } => {
            controller.get_notification(&notification_id).await;
            Ok(())
        },
        cmd @ NotificationCommands::Create { .. } => {
            controller.create_notification(&cmd).await;
            Ok(())
        },
        NotificationCommands::List => {
            controller.list_notifications().await;
            Ok(())
        },
        NotificationCommands::ListActiveNotifications { active_id } => {
            controller.list_active_notifications(&active_id).await;
            Ok(())
        },
        cmd @ NotificationCommands::Update { .. } => {
            controller.update_notification(&cmd).await;
            Ok(())
        },
        NotificationCommands::Delete { notification_id } => {
            controller.delete_notification(&notification_id).await;
            Ok(())
        },
    }
}
