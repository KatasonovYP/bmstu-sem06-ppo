use chrono::NaiveDateTime;
use clap::Subcommand;
use domain::{
    errors::DomainError,
    models::SentEntity,
};

#[derive(Subcommand)]
pub enum SentCommands {
    Get {
        #[arg(long)]
        notification_id: u32,
    },
    Create {
        #[arg(long)]
        notification_id: u32,
        #[arg(long)]
        last_message_time: String, // Строка с датой и временем в формате DD.MM.YYYY HH:MM:SS
    },
    Update {
        #[arg(long)]
        notification_id: u32,
        #[arg(long)]
        last_message_time: String, // Строка с датой и временем в формате DD.MM.YYYY HH:MM:SS
    },
    List,
    Delete {
        #[arg(long)]
        notification_id: u32,
    },
}

impl TryFrom<&SentCommands> for SentEntity {
    type Error = DomainError;

    fn try_from(command: &SentCommands) -> Result<Self, Self::Error> {
        match command {
            SentCommands::Update {
                notification_id,
                last_message_time,
            } => Ok(SentEntity {
                notification_id: *notification_id,
                last_message_time: parse_datetime_string(last_message_time)?,
            }),
            SentCommands::Create {
                notification_id,
                last_message_time,
            } => Ok(SentEntity {
                notification_id: *notification_id,
                last_message_time: parse_datetime_string(last_message_time)?,
            }),
            _ => Err(Self::Error::ValidationError(
                "Command can't be converted into SentEntity".into(),
            )),
        }
    }
}

fn parse_datetime_string(datetime_str: &str) -> Result<NaiveDateTime, DomainError> {
    if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, "%d.%m.%Y %H:%M:%S") {
        return Ok(dt);
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, "%d.%m.%Y %H:%M") {
        return Ok(dt);
    }

    Err(DomainError::ValidationError(format!(
        "Неверный формат даты/времени: {datetime_str}. Используйте формат DD.MM.YYYY HH:MM:SS или DD.MM.YYYY HH:MM"
    )))
}
