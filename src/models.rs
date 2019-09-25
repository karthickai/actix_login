use super::schema::*;
use diesel::{r2d2::ConnectionManager, SqliteConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub username: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            username: user.username,
        }
    }
}
