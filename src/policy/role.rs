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
        todo!()
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
