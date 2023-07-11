#[macro_use]
extern crate rocket;

mod guard;
use guard::{GuardA, GuardB, GuardC};

#[get("/<hoge>")]
fn hoge(hoge: u8, _a: GuardA, _b: GuardB, _c:GuardC) -> String {
    format!("Hoge! {}", hoge)
}

#[get("/<hoge>", rank = 2)]
fn hoge2(hoge: u8, _a: GuardA) -> String {
    format!("Hoge 2! {}", hoge)
}

#[get("/<hoge>", rank = 3)]
fn hoge3(hoge: u8, _a: GuardB) -> String {
    format!("Hoge 3! {}", hoge)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hoge, hoge2, hoge3])
}
