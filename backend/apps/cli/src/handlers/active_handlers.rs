use crate::{
    commands::ActiveCommands,
    controllers::active_controller::CliActiveController,
};

pub async fn handle_active_command(
    command: ActiveCommands,
    controller: &CliActiveController,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ActiveCommands::Get { active_id } => {
            controller.get_active(&active_id).await;
            Ok(())
        },
        cmd @ ActiveCommands::Create { .. } => {
            controller.create_active(&cmd).await;
            Ok(())
        },
        cmd @ ActiveCommands::Update { .. } => {
            controller.update_active(&cmd).await;
            Ok(())
        },
        ActiveCommands::List => {
            controller.list_actives().await;
            Ok(())
        },
        ActiveCommands::ListUserActives { user_id } => {
            controller.list_user_actives(&user_id).await;
            Ok(())
        },
        ActiveCommands::Delete { active_id } => {
            controller.delete_active(&active_id).await;
            Ok(())
        },
    }
}
