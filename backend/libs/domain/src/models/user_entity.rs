use crate::value_objects::Username;

#[derive(Debug, Clone, Default, fake::Dummy)]
pub struct UserEntity {
    pub user_id: u32,
    pub tg_id: i64,
    pub chat_id: i64,
    pub username: Username,
    pub first_name: Option<String>,
    pub second_name: Option<String>,
}

impl PartialEq for UserEntity {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}
