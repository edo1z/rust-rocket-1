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

#[get("/<name>/<age>/<cool>")]
fn hoge(name: &str, age: u8, cool:bool) -> String {
    if cool {
        format!("COOl {} {}", name, age)
    } else {
        format!("HOGE {} {}", name, age)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![index])
      .mount("/news", routes![news_index, news_view])
      .mount("/hoge", routes![hoge])
}