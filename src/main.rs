#[macro_use]
extern crate rocket;

mod auth;
mod policy;
mod db;

use rocket::Request;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(auth::PATH, auth::index))
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
        .mount("/", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(db::stage())
}
