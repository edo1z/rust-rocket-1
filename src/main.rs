#[macro_use] extern crate rocket;
use serde::Deserialize;
use dotenv::dotenv;
use rocket::figment::value::Map;
use rocket::{State, fairing::AdHoc};

#[derive(Deserialize, Debug)]
struct AppConfig {
    hoge: String,
    databases: Map<String, Map<String, String>>
}

#[get("/")]
fn index(config: &State<AppConfig>) -> String {
    // config.hoge.to_string()
    config.databases.get("hoge").unwrap().get("url").unwrap().to_string()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", routes![index])
        .attach(AdHoc::config::<AppConfig>())
}
