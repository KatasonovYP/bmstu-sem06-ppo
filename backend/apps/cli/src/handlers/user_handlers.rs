// handlers/user_handlers.rs
use crate::{
    commands::UserCommands,
    controllers::user_controller::CliUserController,
};

pub async fn handle_user_command(
    command: UserCommands,
    controller: &CliUserController,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        UserCommands::Get { user_id } => {
            controller.get_user(&user_id).await;
            Ok(())
        },
        cmd @ UserCommands::Create { .. } => {
            controller.create_user(&cmd).await;
            Ok(())
        },
        cmd @ UserCommands::Update { .. } => {
            controller.update_user(&cmd).await;
            Ok(())
        },
        UserCommands::List => {
            controller.list_users().await;
            Ok(())
        },
        UserCommands::Delete { user_id } => {
            controller.delete_user(&user_id).await;
            Ok(())
        },
    }
}
