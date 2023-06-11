use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[macro_use] extern crate rocket;

#[derive(Debug)]
struct Hogehoge(bool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Hogehoge {
    type Error = std::convert::Infallible;

    async fn from_request(_: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Hogehoge(true))
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn news_index() -> &'static str {
    "News List"
}

#[get("/<id>")]
fn news_view(id:u32) -> String {
    format!("News View {}", id)
}

#[get("/<name>/<age>/<cool>")]
fn hoge(name: &str, age: u8, cool:bool) -> String {
    if cool {
        format!("COOl {} {}", name, age)
    } else {
        format!("HOGE {} {}", name, age)
    }
}

#[get("/<name>")]
fn taro(name: &str, _hogehoge: Hogehoge) -> String {
    format!("Hoge2 {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![index])
      .mount("/news", routes![news_index, news_view])
      .mount("/hoge", routes![taro, hoge])
}