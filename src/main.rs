#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;


mod other {
    #[get("/world")]
    pub fn world() -> &'static str {
        "Hello, world!"
    }
}

use other::world;

fn main() {
    rocket::ignite().mount("/hello", routes![other::world]).launch();
}
