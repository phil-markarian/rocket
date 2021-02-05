#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/new")]
fn new() -> &'static str {
    "Hello, new!"
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/test/<test>")]
fn test(test: String) -> String {
    format!("This is a test {}!", test)
}

fn main() {
    rocket::ignite().mount("/", routes![index, new, hello, test]).launch();
}