use std::marker::PhantomData;
use std::ops::Deref;

use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::outcome::{IntoOutcome, try_outcome};
use rocket::request::{Outcome, Request, FromRequest};
use rocket::serde;
use rocket_db_pools::{Database, Connection};

use crate::db::Db;
use crate::policy::{Role, PolicyError};

pub struct User<'r> {
    id: i64,
    name: String,
    role: Role,
    _req: PhantomData<&'r ()>,
}

pub struct Admin<'r>(User<'r>);

pub struct Doctor<'r>(User<'r>);

pub struct Patient<'r>(User<'r>);

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo<'r> {
    name: &'r str,
    role: &'r str,
}

impl User<'_> {
    pub fn info(&self) -> UserInfo<'_> {
        UserInfo { name: &self.name, role: self.role.as_str() }
    }
}

impl<'r> Deref for Admin<'r> {
    type Target = User<'r>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> Deref for Doctor<'r> {
    type Target = User<'r>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'r> Deref for Patient<'r> {
    type Target = User<'r>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User<'r> {
    type Error = PolicyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let conn = Db::fetch(req.rocket())
            .or_forward(Status::ServiceUnavailable);

        let token = req.cookies()
            .get_private("token")
            .or_forward(Status::Unauthorized);

        let (cookie, conn) = (try_outcome!(token), try_outcome!(conn));
        let id = match cookie.value().parse::<i64>() {
            Ok(id) => id,
            Err(_) => return Outcome::Error((Status::InternalServerError, PolicyError::InvalidData))
        };

        let user = sqlx::query!("SELECT name, active, role FROM users WHERE id=?", id)
            .fetch_one(&**conn)
            .await
            .map_err(|e| PolicyError::Db(e))
            .or_error(Status::InternalServerError);

        let user = try_outcome!(user);
        let role = Role::try_from(user.role).or_error(Status::InternalServerError);
        Outcome::Success(User { id, name: user.name, role: try_outcome!(role), _req: PhantomData })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin<'r> {
    type Error = PolicyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user: User<'r>  = try_outcome!(req.guard().await);
        match user.role {
            Role::Admin => Outcome::Success(Admin(user)),
            _ => Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Doctor<'r> {
    type Error = PolicyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user: User<'r>  = try_outcome!(req.guard().await);
        match user.role {
            Role::Doctor => Outcome::Success(Doctor(user)),
            _ => Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Patient<'r> {
    type Error = PolicyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user: User<'r>  = try_outcome!(req.guard().await);
        match user.role {
            Role::Patient => Outcome::Success(Patient(user)),
            _ => Outcome::Forward(Status::Unauthorized)
        }
    }
}

#[derive(serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData<'r> {
    id: i64,
    name: String,
    role: Option<Role>,
    email: Option<String>,
    active: Option<bool>,
    _req: PhantomData<&'r ()>,
}

impl serde::Serialize for Role {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'r> Admin<'r> {
    pub async fn fetch_all_users(
        &self,
        conn: &'r mut Connection<Db>
    ) -> Result<Vec<UserData<'r>>, PolicyError> {
        let users = sqlx::query!("
                SELECT id, name, role, email, active FROM users WHERE id != ?
            ", self.id)
            .fetch(conn.as_mut())
            .map_ok(|r| UserData {
                id: r.id,
                name: r.name,
                role: Role::try_from(r.role).ok(),
                email: Some(r.email),
                active: Some(r.active),
                _req: PhantomData
            })
            .try_collect::<Vec<_>>()
            .await?;

        Ok(users)
    }

    pub async fn set_status(
        &self,
        id: i64,
        status: bool,
        conn: &'r mut Connection<Db>
    ) -> Result<(), PolicyError> {
        let query = sqlx::query!("UPDATE users SET active = ? WHERE id = ?", status, id);
        query.execute(conn.as_mut()).await?;
        Ok(())
    }
}
