use rocket::Route;
use rocket::http::uri::Origin;
use rocket::response::{Redirect, Debug};

use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};

use crate::auth;
use crate::db::Db;
use crate::policy::PolicyError;

pub const PATH: Origin<'static> = uri!("/dashboard");

#[get("/", rank = 1)]
async fn admin(
    mut conn: Connection<Db>
) -> Result<Template, Debug<PolicyError>> {
    todo!()
}

#[get("/", rank = 2)]
fn doctor() -> Template {
    todo!()
}

#[get("/", rank = 3)]
fn patient() -> Template {
    todo!()
}

#[get("/", rank = 4)]
fn unauthorized() -> Redirect {
    Redirect::to(uri!(auth::PATH, auth::index))
}

pub fn routes() -> Vec<Route> {
    routes![admin, doctor, patient, unauthorized]
}
