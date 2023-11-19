use rocket::Route;
use rocket::http::uri::Origin;
use rocket::response::{Redirect, Debug};

use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};

use crate::auth;
use crate::db::Db;
use crate::policy::{Admin, Doctor, Patient, PolicyError};

pub const PATH: Origin<'static> = uri!("/dashboard");

#[get("/", rank = 1)]
async fn admin(
    admin: Admin<'_>,
    mut conn: Connection<Db>
) -> Result<Template, Debug<PolicyError>> {
    Ok(Template::render("dashboard", context! {
        user: admin.info(),
        data: admin.fetch_all_users(&mut conn).await?,
    }))
}

#[get("/", rank = 2)]
fn doctor(user: Doctor<'_>) -> Template {
    Template::render("dashboard", context! { user: user.info() })
}

#[get("/", rank = 3)]
fn patient(user: Patient<'_>) -> Template {
    Template::render("dashboard", context! { user: user.info() })
}

#[get("/", rank = 4)]
fn unauthorized() -> Redirect {
    Redirect::to(uri!(auth::PATH, auth::index))
}

pub fn routes() -> Vec<Route> {
    routes![admin, doctor, patient, unauthorized]
}
