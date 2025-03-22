use crate::app::adapters::db::schema::users;
use crate::domain::models::UserEntity;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrmSelectUser {
    pub tg_id: i32,
    pub username: String,
    pub first_name: String,
    pub second_name: String,
}

impl From<OrmSelectUser> for UserEntity {
    fn from(orm_user: OrmSelectUser) -> Self {
        UserEntity {
            tg_id: orm_user.tg_id,
            username: orm_user.username,
            first_name: orm_user.first_name,
            second_name: orm_user.second_name,
        }
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct OrmInsertUser {
    pub tg_id: i32,
    pub username: String,
    pub first_name: String,
    pub second_name: String,
}

impl From<UserEntity> for OrmInsertUser {
    fn from(user_entity: UserEntity) -> Self {
        OrmInsertUser {
            tg_id: user_entity.tg_id,
            username: user_entity.username,
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
