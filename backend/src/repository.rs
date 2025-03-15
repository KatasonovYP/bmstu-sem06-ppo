use crate::db::DbPool;
use crate::models::{NewUser, UpdateUser, User};
use crate::schema::users;
use diesel::prelude::*;

pub struct UserRepository;

impl UserRepository {
    pub fn find_all(pool: &DbPool) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        users::table.select(User::as_select()).load(&mut conn)
    }

    pub fn find_by_id(id: i32, pool: &DbPool) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().unwrap();
        users::table
            .find(id)
            .select(User::as_select())
            .first(&mut conn)
    }

    pub fn create(new_user: NewUser, pool: &DbPool) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_select())
            .get_result(&mut conn)
    }

    pub fn update(id: i32, user: UpdateUser, pool: &DbPool) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        diesel::update(users::table.find(id))
            .set(user)
            .returning(User::as_select())
            .get_result(&mut conn)
    }

    pub fn delete(id: i32, pool: &DbPool) -> Result<usize, diesel::result::Error> {
        let mut conn = pool.get().unwrap();

        diesel::delete(users::table.find(id)).execute(&mut conn)
    }
}
