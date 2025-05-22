use clap::{
    Parser,
    Subcommand,
    ValueEnum,
};

use super::{
    ActiveCommands,
    NotificationCommands,
    SentCommands,
    UserCommands,
};

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Json,
    Console,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Console)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    Active {
        #[command(subcommand)]
        command: ActiveCommands,
    },
    Notification {
        #[command(subcommand)]
        command: NotificationCommands,
    },
    Sent {
        #[command(subcommand)]
        command: SentCommands,
    },
    RefreshPrices,
    SendExeedingMessages,
}
