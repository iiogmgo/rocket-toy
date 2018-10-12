#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::request::{Form, LenientForm};
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct Task {
    complete: bool,
    description: String,
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}