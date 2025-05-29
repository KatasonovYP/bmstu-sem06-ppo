use clap::Subcommand;
use domain::{
    errors::DomainError,
    models::NotificationEntity,
    value_objects::Price,
};

#[derive(Subcommand)]
pub enum NotificationCommands {
    Get {
        #[arg(long)]
        notification_id: u32,
    },
    Create {
        #[arg(long)]
        portfolio_id: u32,
        #[arg(long)]
        active_id: u32,
        #[arg(long)]
        limit_upper: f64,
        #[arg(long)]
        limit_lower: f64,
        #[arg(long)]
        limit_type: String,
    },
    Update {
        #[arg(long)]
        notification_id: u32,
        #[arg(long)]
        portfolio_id: u32,
        #[arg(long)]
        active_id: u32,
        #[arg(long)]
        limit_upper: f64,
        #[arg(long)]
        limit_lower: f64,
        #[arg(long)]
        limit_type: String,
    },
    List,
    ListActiveNotifications {
        #[arg(long)]
        active_id: u32,
    },
    Delete {
        #[arg(long)]
        notification_id: u32,
    },
}

impl TryFrom<&NotificationCommands> for NotificationEntity {
    type Error = DomainError;

    fn try_from(command: &NotificationCommands) -> Result<Self, Self::Error> {
        match command {
            NotificationCommands::Update {
                notification_id,
                portfolio_id,
                active_id,
                limit_upper,
                limit_lower,
                limit_type,
            } => Ok(NotificationEntity {
                notification_id: *notification_id,
                portfolio_id: *portfolio_id,
                active_id: *active_id,
                limit_upper: Price::new(*limit_upper, limit_type.clone()),
                limit_lower: Price::new(*limit_lower, limit_type.clone()),
            }),
            NotificationCommands::Create {
                portfolio_id,
                active_id,
                limit_upper,
                limit_lower,
                limit_type,
            } => Ok(NotificationEntity {
                notification_id: Default::default(),
                portfolio_id: *portfolio_id,
                active_id: *active_id,
                limit_upper: Price::new(*limit_upper, limit_type.clone()),
                limit_lower: Price::new(*limit_lower, limit_type.clone()),
            }),
            _ => Err(Self::Error::ValidationError(
                "Command can't be converted into NotificationEntity".into(),
            )),
        }
    }
}
