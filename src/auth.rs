use rocket::Route;
use rocket::http::CookieJar;
use rocket::response::{Redirect, Debug};
use rocket::form::{Form, Contextual};
use rocket::http::uri::Origin;

use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use either::{Either, Left, Right};

use crate::db::Db;
use crate::policy::{FirstAccount, PolicyError};

pub const PATH: Origin<'static> = uri!("/auth");

#[derive(FromForm)]
pub struct Registration<'r> {
    #[field(validate = len(2..128).or_else(msg!("enter a valid name")))]
    pub name: &'r str,
    #[field(validate = len(1..128))]
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'r str,
    #[field(validate = len(8..).or_else(msg!("password is too short")))]
    #[field(validate = len(1..128))]
    pub password: &'r str,
    pub doctor: bool,
}

#[derive(Debug, FromForm)]
pub struct Login<'r> {
    #[field(validate = len(1..128))]
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    pub email: &'r str,
    #[field(validate = len(8..).or_else(msg!("password is too short")))]
    #[field(validate = len(1..128))]
    pub password: &'r str,
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(PATH, login))
}

#[get("/register")]
fn register() -> Template {
    Template::render("register", context!{})
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context!{})
}

#[post("/register", data = "<form>")]
async fn register_admin(
    token: FirstAccount<'_>,
    form: Form<Contextual<'_, Registration<'_>>>
) -> Result<Either<Redirect, Template>, Debug<PolicyError>> {
    todo!()
}

#[post("/register", data = "<form>", rank = 1)]
async fn register_other(
    form: Form<Contextual<'_, Registration<'_>>>,
    db: Connection<Db>,
) -> Result<Template, Debug<PolicyError>> {
    todo!()
}

#[post("/login", data = "<form>")]
async fn login_form(
    jar: &CookieJar<'_>,
    db: Connection<Db>,
    form: Form<Contextual<'_, Login<'_>>>
) -> Result<Redirect, Template> {
    todo!()
}

#[get("/logout")]
async fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private("token");
    Redirect::to(uri!(crate::index))
}

pub fn routes() -> Vec<Route> {
    routes![index, login, register, register_admin, register_other, login_form, logout]
}
