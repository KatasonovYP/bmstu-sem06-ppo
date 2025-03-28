use crate::app::adapters::db::schema::actives;
use crate::app::adapters::db::schema::notifications;
use crate::app::adapters::db::schema::users;
use crate::domain::errors::DomainError;
use crate::domain::models::ActiveEntity;
use crate::domain::models::NotificationEntity;
use crate::domain::models::UserEntity;
use crate::domain::value_objects::Username;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrmSelectUser {
    pub tg_id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
}

impl TryFrom<OrmSelectUser> for UserEntity {
    type Error = DomainError;
    fn try_from(orm_user: OrmSelectUser) -> Result<Self, Self::Error> {
        let username = Username::new(orm_user.username)?;
        Ok(UserEntity {
            tg_id: orm_user.tg_id,
            username,
            first_name: orm_user.first_name,
            second_name: orm_user.second_name,
        })
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct OrmInsertUser {
    pub tg_id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
}

impl From<UserEntity> for OrmInsertUser {
    fn from(user_entity: UserEntity) -> Self {
        OrmInsertUser {
            tg_id: user_entity.tg_id,
            username: user_entity.username.value,
            first_name: user_entity.first_name,
            second_name: user_entity.second_name,
        }
    }
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct OrmUpdateUser {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = actives)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrmSelectActive {
    pub active_id: i32,
    pub user_id: i32,
    pub security_id: i32,
    pub bought_price: i32,
    pub count: i32,
}

impl From<OrmSelectActive> for ActiveEntity {
    fn from(orm_active: OrmSelectActive) -> Self {
        ActiveEntity {
            user_id: orm_active.user_id,
            security_id: orm_active.security_id,
            bought_price: orm_active.bought_price,
            count: orm_active.count,
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = actives)]
pub struct OrmInsertActive {
    pub user_id: i32,
    pub security_id: i32,
    pub bought_price: i32,
    pub count: i32,
}

impl From<ActiveEntity> for OrmInsertActive {
    fn from(active_entity: ActiveEntity) -> Self {
        OrmInsertActive {
            user_id: active_entity.user_id,
            security_id: active_entity.security_id,
            bought_price: active_entity.bought_price,
            count: active_entity.count,
        }
    }
}

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrmSelectNotification {
    pub notification_id: i32,
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: i32,
    pub limit_lower: i32,
    pub limit_type: i16,
}

impl From<OrmSelectNotification> for NotificationEntity {
    fn from(orm_notification: OrmSelectNotification) -> Self {
        NotificationEntity {
            portfolio_id: orm_notification.portfolio_id,
            active_id: orm_notification.active_id,
            limit_upper: orm_notification.limit_upper,
            limit_lower: orm_notification.limit_lower,
            limit_type: orm_notification.limit_type,
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = notifications)]
pub struct OrmInsertNotification {
    pub portfolio_id: i32,
    pub active_id: i32,
    pub limit_upper: i32,
    pub limit_lower: i32,
    pub limit_type: i16,
}

impl From<NotificationEntity> for OrmInsertNotification {
    fn from(notification_entity: NotificationEntity) -> Self {
        OrmInsertNotification {
            portfolio_id: notification_entity.portfolio_id,
            active_id: notification_entity.active_id,
            limit_upper: notification_entity.limit_upper,
            limit_lower: notification_entity.limit_lower,
            limit_type: notification_entity.limit_type,
        }
    }
}
