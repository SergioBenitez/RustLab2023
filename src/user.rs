use rocket::Route;
use rocket::http::uri::Origin;
use rocket::response::{Redirect, Debug};
use rocket_db_pools::Connection;

use crate::db::Db;
use crate::policy::{Admin, PolicyError};

pub const PATH: Origin<'static> = uri!("/user");

#[put("/<id>/active?<value>")]
async fn set_active(
    id: i64,
    value: bool,
) -> Result<Redirect, Debug<PolicyError>> {
    todo!()
}

pub fn routes() -> Vec<Route> {
    routes![set_active]
}
