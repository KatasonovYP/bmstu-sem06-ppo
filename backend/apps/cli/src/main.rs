mod commands;
mod controllers;
mod handlers;

use std::sync::Arc;

use adapters::{
    di_domain_module::BuildAppModule,
    settings::Settings,
};
use clap::{
    Parser,
    ValueEnum,
};

use crate::{
    commands::*,
    controllers::{
        active_controller::CliActiveController,
        notification_controller::CliNotificationController,
        sent_controller::CliSentController,
        user_controller::CliUserController,
    },
    handlers::{
        handle_active_command,
        handle_notification_command,
        handle_refresh_prices,
        handle_send_exceeding_messages,
        handle_sent_command,
        handle_user_command,
    },
};

#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Console,
}

struct AppContext {
    controllers: Controllers,
    services: Services,
}

struct Controllers {
    active: controllers::active_controller::CliActiveController,
    notification: controllers::notification_controller::CliNotificationController,
    user: controllers::user_controller::CliUserController,
    sent: controllers::sent_controller::CliSentController,
}

struct Services {
    limit_monitor: Arc<dyn domain::ports::domain::AbstractLimitMonitorService>,
    price_cache: Arc<dyn domain::ports::domain::AbstractPriceCacheService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let context = initialize_app().await?;

    execute_command(cli.command, context).await?;

    Ok(())
}

async fn initialize_app() -> Result<AppContext, Box<dyn std::error::Error>> {
    let settings = Settings::new("config/app.default.yaml").unwrap();

    let module = BuildAppModule::new(&settings).build().await;

    let controllers = Controllers {
        active: CliActiveController::new(module.resolve()),
        notification: CliNotificationController::new(module.resolve()),
        user: CliUserController::new(module.resolve()),
        sent: CliSentController::new(module.resolve()),
    };

    let services = Services {
        limit_monitor: module.resolve(),
        price_cache: module.resolve(),
    };

    Ok(AppContext {
        controllers,
        services,
    })
}

async fn execute_command(
    command: Commands,
    context: AppContext,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::SendExeedingMessages => handle_send_exceeding_messages(&context.services).await,
        Commands::RefreshPrices => handle_refresh_prices(&context.services).await,
        Commands::User { command } => handle_user_command(command, &context.controllers.user).await,
        Commands::Active { command } => {
            handle_active_command(command, &context.controllers.active).await
        },
        Commands::Notification { command } => {
            handle_notification_command(command, &context.controllers.notification).await
        },
        Commands::Sent { command } => handle_sent_command(command, &context.controllers.sent).await,
    }
}
