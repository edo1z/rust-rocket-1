#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Error {
    code: u16,
    msg: String,
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = Status::from_code(self.code).unwrap_or(Status::InternalServerError);
        Response::build_from(Json(self).respond_to(req)?)
            .status(status)
            .ok()
    }
}


#[get("/", format = "json")]
fn index() -> Result<&'static str, Error> {
    Err(Error {
        code: 400,
        msg: "hoge!".to_string(),
    })
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
        .mount("/", routes![index])
        .register("/", catchers![default_catcher])
}
