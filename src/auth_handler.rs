use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;

use crate::errors::ServiceError;
use crate::models::{Pool, SlimUser, User};
use crate::utils::{hash, verify};
use futures::future::err;
use futures::future::Either;
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePassword {
    old_password: String,
    new_password: String,
}

pub fn update_password(
    auth_data: web::Json<UpdatePassword>,
    id: Identity,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    match id.identity().as_ref() {
        Some(identity) => {
            let user: SlimUser = serde_json::from_str(&identity).unwrap();
            id.forget();
            Either::A(
                web::block(move || query_update(user, auth_data.into_inner(), pool)).then(
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
                ),
            )
        }
        _ => Either::B(err(ServiceError::Unauthorized)),
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

pub fn login(
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
    let mut items = users
        .filter(username.eq(&auth_data.username))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &auth_data.password) {
            if matching {
                let slim_user = SlimUser {
                    username: user.username,
                };
                return Ok(slim_user);
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

fn query_update(
    auth_data: SlimUser,
    upwd: UpdatePassword,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{password, username, users};
    let conn: &SqliteConnection = &pool.get().unwrap();
    let mut items = users
        .filter(username.eq(&auth_data.username))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &upwd.old_password) {
            if matching {
                let hash_password: String = hash(&upwd.new_password)?;
                let _ = diesel::update(users.find(&auth_data.username))
                    .set(password.eq(hash_password))
                    .execute(conn)?;
                let slim_user = SlimUser {
                    username: user.username,
                };
                return Ok(slim_user);
            }
        }
    }
    Err(ServiceError::BadRequest("Username not exist !".into()))
}
