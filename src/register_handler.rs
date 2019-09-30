use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;

use crate::errors::ServiceError;
use crate::models::{Pool, SlimUser, User};
use crate::utils::hash;

use futures::Future;

pub fn create_user(
    auth_data: web::Json<User>,
    id: Identity,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || query(auth_data.into_inner(), pool)).then(
        move |res: Result<SlimUser, BlockingError<ServiceError>>| match res {
            Ok(user) => {
                let user_string = serde_json::to_string(&user).unwrap();
                id.remember(user_string);
                Ok(HttpResponse::Ok().finish())
            }
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        },
    )
}

/// Diesel query
fn query(auth_data: User, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{username, users};
    let conn: &SqliteConnection = &pool.get().unwrap();
    let items = users
        .filter(username.eq(&auth_data.username))
        .load::<User>(conn)?;
    if items.is_empty() {
        let password: String = hash(&auth_data.password)?;
        let new_user = User {
            username: auth_data.username,
            password,
        };
        diesel::insert_into(users).values(&new_user).execute(conn)?;

        let slim_user = SlimUser {
            username: new_user.username,
        };
        return Ok(slim_user);
    }
    Err(ServiceError::BadRequest("Username already exist !".into()))
}
