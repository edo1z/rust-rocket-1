#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<_hoge>")]
fn hoge(_hoge: u8) -> &'static str {
    "Hoge!"
}

#[get("/<_hoge>")]
fn hoge2(_hoge: bool) -> &'static str {
    "Hoge2!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hoge, hoge2])
}
