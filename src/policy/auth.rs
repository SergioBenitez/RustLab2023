use password_hash::PasswordHash;
use rocket::{Sentinel, Rocket, Ignite};
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::{Outcome, IntoOutcome, try_outcome};
use rocket::http::Status;

use rocket::futures::TryFutureExt;
use rocket_db_pools::{Error, Database, Connection};

use sqlx::query;
use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString, PasswordVerifier};

use crate::policy::PolicyError;
use crate::auth::{Registration, Login};
use crate::db::Db;

pub struct FirstAccount<'r>(sqlx::Transaction<'r, sqlx::sqlite::Sqlite>);

impl Sentinel for FirstAccount<'_> {
    fn abort(rocket: &Rocket<Ignite>) -> bool {
        Db::fetch(rocket).is_none()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for FirstAccount<'r> {
    type Error = Option<Error<sqlx::Error>>;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = Db::fetch(req.rocket()).or_error((Status::InternalServerError, None));
        let tx = try_outcome!(db)
            .begin()
            .await
            .map_err(|e| Some(Error::Get(e)))
            .or_error(Status::ServiceUnavailable);

        let mut tx = try_outcome!(tx);
        let user_exists = query!("SELECT EXISTS (SELECT 1 from users) AS nonempty")
            .fetch_one(&mut *tx)
            .map_ok(|r| r.nonempty.map(|v| v == 1))
            .await;

        match user_exists {
            Ok(Some(false)) => Outcome::Success(FirstAccount(tx)),
            Ok(Some(true)) => Outcome::Forward(Status::Unauthorized),
            Ok(None) => Outcome::Error((Status::InternalServerError, None)),
            Err(e) => Outcome::Error((Status::InternalServerError, Some(Error::Get(e)))),
        }
    }
}

pub(in crate::policy) enum Role {
    Admin = 0,
    Doctor = 1,
    Patient = 2,
}

impl FirstAccount<'_> {
    pub async fn create(mut self, user: Registration<'_>) -> Result<(), PolicyError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let phash = argon2.hash_password(user.password.as_bytes(), &salt)?.to_string();

        let insert = query!(
            "INSERT INTO users (name, email, password, role, active) VALUES (?, ?, ?, ?, ?)",
            user.name, user.email, phash, Role::Admin as u8, true
        );

        insert.execute(&mut *self.0).await?;
        self.0.commit().await?;
        Ok(())
    }
}

impl Registration<'_> {
    pub async fn make(&self, mut conn: Connection<Db>) -> Result<(), PolicyError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let phash = argon2.hash_password(self.password.as_bytes(), &salt)?.to_string();

        let role = if self.doctor { Role::Doctor } else { Role::Patient } as u8;
        let insert = query!(
            "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)",
            self.name, self.email, phash, role
        );

        insert.execute(conn.as_mut()).await?;
        Ok(())
    }
}

impl Login<'_> {
    pub async fn verified(&self, mut conn: Connection<Db>) -> Result<(), PolicyError> {
        todo!()
    }
}
