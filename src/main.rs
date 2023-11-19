#[macro_use]
extern crate rocket;

mod auth;
mod policy;
mod dashboard;
mod db;
mod user;

use rocket::Request;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(dashboard::PATH, dashboard::admin))
}

#[catch(default)]
fn error(status: Status, _: &Request) -> Template {
    Template::render("error", context! { status, reason: status.reason_lossy() })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![error])
        .mount("/", routes![index])
        .mount(auth::PATH, auth::routes())
        .mount(dashboard::PATH, dashboard::routes())
        .mount(user::PATH, user::routes())
        .mount("/", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(db::stage())
}
