use std::sync::Arc;

use clap::{
    Parser,
    ValueEnum,
};
use shaku::HasComponent;
use stocks_tracker::{
    app::{
        cli::{
            commands::*,
            controllers::{
                active_controller::CliActiveController,
                notification_controller::CliNotificationController,
                sent_controller::CliSentController,
                user_controller::CliUserController,
            },
        },
        di_domain_module::di_domain_module,
    },
    domain::ports::domain::{
        AbstractLimitMonitorService,
        AbstractPriceCacheService,
    },
};

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Console,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let module = di_domain_module().await;
    let active_cntr = CliActiveController::new(module.resolve());
    let notification_cntr = CliNotificationController::new(module.resolve());
    let user_cntr = CliUserController::new(module.resolve());
    let sent_cntr = CliSentController::new(module.resolve());
    let limit_monitor_service: Arc<dyn AbstractLimitMonitorService> = module.resolve();
    let price_cache_service: Arc<dyn AbstractPriceCacheService> = module.resolve();

    match &cli.command {
        Commands::SendExeedingMessages => {
            limit_monitor_service
                .send_exeeding_messages()
                .await
                .unwrap();
        },
        Commands::RefreshPrices => {
            price_cache_service.refresh_all_prices().await.unwrap();
        },
        Commands::User {
            command: UserCommands::Get { user_id },
        } => user_cntr.get_user(user_id).await,
        Commands::User {
            command: cmd @ UserCommands::Create { .. },
        } => user_cntr.create_user(cmd).await,
        Commands::User {
            command: cmd @ UserCommands::Update { .. },
        } => user_cntr.update_user(cmd).await,
        Commands::User {
            command: UserCommands::List,
        } => user_cntr.list_users().await,
        Commands::User {
            command: UserCommands::Delete { user_id },
        } => user_cntr.delete_user(user_id).await,

        Commands::Active {
            command: ActiveCommands::Get { active_id },
        } => active_cntr.get_active(active_id).await,
        Commands::Active {
            command: cmd @ ActiveCommands::Create { .. },
        } => active_cntr.create_active(cmd).await,
        Commands::Active {
            command: cmd @ ActiveCommands::Update { .. },
        } => active_cntr.update_active(cmd).await,
        Commands::Active {
            command: ActiveCommands::List,
        } => active_cntr.list_actives().await,
        Commands::Active {
            command: ActiveCommands::ListUserActives { user_id },
        } => active_cntr.list_user_actives(user_id).await,
        Commands::Active {
            command: ActiveCommands::Delete { active_id },
        } => active_cntr.delete_active(active_id).await,

        Commands::Notification {
            command: NotificationCommands::Get { notification_id },
        } => notification_cntr.get_notification(notification_id).await,
        Commands::Notification {
            command: cmd @ NotificationCommands::Create { .. },
        } => notification_cntr.create_notification(cmd).await,
        Commands::Notification {
            command: NotificationCommands::List,
        } => notification_cntr.list_notifications().await,
        Commands::Notification {
            command: cmd @ NotificationCommands::Update { .. },
        } => notification_cntr.update_notification(cmd).await,
        Commands::Notification {
            command: NotificationCommands::Delete { notification_id },
        } => notification_cntr.delete_notification(notification_id).await,

        Commands::Sent {
            command: SentCommands::Get { notification_id },
        } => sent_cntr.get_sent(notification_id).await,
        Commands::Sent {
            command: cmd @ SentCommands::Create { .. },
        } => sent_cntr.create_sent(cmd).await,
        Commands::Sent {
            command: cmd @ SentCommands::Update { .. },
        } => sent_cntr.update_sent(cmd).await,
        Commands::Sent {
            command: SentCommands::List,
        } => sent_cntr.list_sent().await,
        Commands::Sent {
            command: SentCommands::Delete { notification_id },
        } => sent_cntr.delete_sent(notification_id).await,
    }
}
