#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<hoge>")]
fn hoge(hoge: u8) -> String {
    format!("Hoge! {}", hoge)
}

#[get("/<hoge>", rank = 2)]
fn hoge2(hoge: &str) -> String {
    format!("Hoge2! {}", hoge)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hoge, hoge2])
}
