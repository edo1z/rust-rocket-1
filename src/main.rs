#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::Responder;
use rocket::Request;
use serde::Deserialize;

#[derive(Deserialize)]
struct Hoge {
    num: u8,
}

#[derive(Responder)]
enum Error {
    // #[response(status = 404)]
    // NotFound(String),
    #[response(status = 500)]
    InternalServerError(String),
}

#[post("/", format = "json", data = "<hoge>")]
fn index(hoge: Json<Hoge>) -> String {
    format!("hoge {}", hoge.num)
}

#[get("/", format = "json")]
fn hoge() -> Result<&'static str, Error> {
    // Ok("Hello, world!")
    // Err(Error::NotFound("hoge".to_string()))
    Err(Error::InternalServerError("hoge!!".to_string()))
}

#[catch(default)]
fn default_catcher(status: Status, _request: &Request) -> String {
    let msg = match status.code {
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Error",
    };
    format!("{} {}", status.code, msg)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hoge])
        .register("/", catchers![default_catcher])
}
