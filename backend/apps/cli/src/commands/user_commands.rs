use clap::Subcommand;
use domain::{
    errors::DomainError,
    models::UserEntity,
    value_objects::Username,
};

#[derive(Subcommand)]
pub enum UserCommands {
    Get {
        #[arg(long)]
        user_id: u32,
    },
    Create {
        #[arg(long)]
        tg_id: i64,
        #[arg(long)]
        chat_id: i64,
        #[arg(long)]
        username: String,
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        second_name: Option<String>,
    },
    Update {
        #[arg(long)]
        user_id: u32,
        #[arg(long)]
        chat_id: i64,
        #[arg(long)]
        tg_id: i64,
        #[arg(long)]
        username: String,
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        second_name: Option<String>,
    },
    List,
    Delete {
        #[arg(long)]
        user_id: u32,
    },
}

impl TryFrom<&UserCommands> for UserEntity {
    type Error = DomainError;

    fn try_from(command: &UserCommands) -> Result<Self, Self::Error> {
        match command {
            UserCommands::Create {
                tg_id,
                chat_id,
                username,
                first_name,
                second_name,
            } => Ok(UserEntity {
                tg_id: *tg_id,
                chat_id: *chat_id,
                username: Username {
                    value: username.clone(),
                },
                first_name: first_name.clone(),
                second_name: second_name.clone(),
                ..Default::default()
            }),
            UserCommands::Update {
                user_id,
                tg_id,
                chat_id,
                username,
                first_name,
                second_name,
            } => Ok(UserEntity {
                user_id: *user_id,
                tg_id: *tg_id,
                chat_id: *chat_id,
                username: Username {
                    value: username.clone(),
                },
                first_name: first_name.clone(),
                second_name: second_name.clone(),
            }),
            _ => Err(Self::Error::ValidationError(
                "Command can't be converted into UserEntity".into(),
            )),
        }
    }
}
