#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

    #[get("/hello/<name>")]
    pub fn hello(name: &RawStr) -> String {
        format!("Hello, {}!", name.as_str())
    }

use rocket::http::RawStr;

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
