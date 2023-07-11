use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct GuardA;
pub struct GuardB;
pub struct GuardC;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardA {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("GuardA");
        match req.headers().get_one("hoge") {
            Some(val) if val == "100" => Outcome::Success(GuardA),
            _ => Outcome::Forward(()),
            // _ => Outcome::Failure((Status::BadRequest, "Bad Request".to_string())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardB {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("GuardB");
        match req.headers().get_one("hoge") {
            Some(val) if val == "200" => Outcome::Success(GuardB),
            _ => Outcome::Forward(()),
            // _ => Outcome::Failure((Status::BadRequest, "Bad Request".to_string())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardC {
    type Error = String;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("GuardC");
        match req.headers().get_one("hoge") {
            Some(val) if val == "300" => Outcome::Success(GuardC),
            // _ => Outcome::Forward(()),
            _ => Outcome::Failure((Status::BadRequest, "Bad Request".to_string())),
        }
    }
}