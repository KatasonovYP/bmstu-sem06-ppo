use crate::{
    commands::SentCommands,
    controllers::sent_controller::CliSentController,
};

pub async fn handle_sent_command(
    command: SentCommands,
    controller: &CliSentController,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        SentCommands::Get { notification_id } => {
            controller.get_sent(&notification_id).await;
            Ok(())
        },
        cmd @ SentCommands::Create { .. } => {
            controller.create_sent(&cmd).await;
            Ok(())
        },
        cmd @ SentCommands::Update { .. } => {
            controller.update_sent(&cmd).await;
            Ok(())
        },
        SentCommands::List => {
            controller.list_sent().await;
            Ok(())
        },
        SentCommands::Delete { notification_id } => {
            controller.delete_sent(&notification_id).await;
            Ok(())
        },
    }
}
