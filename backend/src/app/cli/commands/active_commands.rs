use clap::Subcommand;

use crate::domain::{
    errors::DomainError,
    models::ActiveEntity,
    value_objects::Price,
};

#[derive(Subcommand)]
pub enum ActiveCommands {
    Get {
        #[arg(long)]
        active_id: u32,
    },
    Create {
        #[arg(long)]
        user_id: u32,
        #[arg(long)]
        security_id: String,
        #[arg(long)]
        bought_price: f64,
        #[arg(long)]
        currency: String,
        #[arg(long)]
        count: u32,
    },
    Update {
        #[arg(long)]
        active_id: u32,
        #[arg(long)]
        user_id: u32,
        #[arg(long)]
        security_id: String,
        #[arg(long)]
        bought_price: f64,
        #[arg(long)]
        currency: String,
        #[arg(long)]
        count: u32,
    },
    List,
    ListUserActives {
        #[arg(long)]
        user_id: u32,
    },
    Delete {
        #[arg(long)]
        active_id: u32,
    },
}

impl TryFrom<&ActiveCommands> for ActiveEntity {
    type Error = DomainError;

    fn try_from(command: &ActiveCommands) -> Result<Self, Self::Error> {
        match command {
            ActiveCommands::Create {
                user_id,
                security_id,
                bought_price,
                currency,
                count,
            } => Ok(ActiveEntity {
                user_id: *user_id,
                security_id: security_id.clone(),
                bought_price: Price::new(*bought_price, currency.clone()),
                count: *count,
                ..Default::default()
            }),
            ActiveCommands::Update {
                active_id,
                user_id,
                security_id,
                bought_price,
                currency,
                count,
            } => Ok(ActiveEntity {
                active_id: *active_id,
                user_id: *user_id,
                security_id: security_id.clone(),
                bought_price: Price::new(*bought_price, currency.clone()),
                count: *count,
            }),
            _ => Err(Self::Error::ValidationError(
                "Command can't be converted into ActiveEntity".into(),
            )),
        }
    }
}
