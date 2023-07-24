#[macro_use] extern crate rocket;
use rocket::Config;
use serde::Deserialize;
use dotenv::dotenv;
use rocket::figment::value::Map;

#[derive(Deserialize, Debug)]
struct MyConfig {
    hoge: String,
    apple: Map<String, String>
}

#[get("/")]
fn index() -> String {
    let config = Config::figment().extract::<MyConfig>();
    format!("{:?}", config)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
}
