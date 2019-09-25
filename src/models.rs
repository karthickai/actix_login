use super::schema::*;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use std::error::Error;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

impl CreateUser {
    pub fn new(username: String, password: String) -> CreateUser {
        CreateUser { username, password }
    }

    pub fn save(self, conn: &SqliteConnection) -> Result<(), Box<dyn Error>> {
        diesel::insert_into(users::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}
