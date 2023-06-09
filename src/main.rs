#[macro_use] extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![index])
      .mount("/news", routes![news_index, news_view])
}