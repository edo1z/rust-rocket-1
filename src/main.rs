#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use serde::Deserialize;

#[derive(Deserialize)]
struct Hoge {
    num: u8,
}

#[post("/", format = "json", data = "<hoge>")]
fn index(hoge: Json<Hoge>) -> String {
    format!("hoge {}", hoge.num)
}

#[catch(default)]
fn default_catcher(status: Status, _request: &Request) -> String {
    let msg = match status.code {
        404 => "Not Found",
        _ => "Internal Server Error",
    };
    format!("{}: {}", status.code, msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![default_catcher])
}
