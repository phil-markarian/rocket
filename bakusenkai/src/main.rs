#![feature(proc_macro_hygiene, decl_macro)]

//use std::collections::HashMap;

extern crate handlebars;

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
//use rocket::Request;
//use rocket::response::Redirect;

/* #[get("/")]
fn index() -> &'static str {
    "縛仙会のホームページ"
} */

#[get("/")]
fn index() -> Template {
    //let mut context = HashMap::new();
    let context: HashMap<String, String> = HashMap::new();
    let template = Template::render( "index", context);
    template
    //Template::render("index", &context)
}

#[get("/about")]
fn about() -> Template {
    //let mut context = HashMap::new();
    let context: HashMap<String, String> = HashMap::new();
    let template = Template::render( "about", context);
    template
    //Template::render("index", &context)
}

#[get("/katsudou")]
fn katsudou() -> Template {
    //let mut context = HashMap::new();
    let context: HashMap<String, String> = HashMap::new();
    let template = Template::render( "katsudou", context);
    template
    //Template::render("index", &context)
}

#[get("/shashin")]
fn shashin() -> Template {
    //let mut context = HashMap::new();
    let context: HashMap<String, String> = HashMap::new();
    let template = Template::render( "shashin", context);
    template
    //Template::render("index", &context)
}

#[get("/contact")]
fn contact() -> Template {
    //let mut context = HashMap::new();
    let context: HashMap<String, String> = HashMap::new();
    let template = Template::render( "contact", context);
    template
    //Template::render("index", &context)
}
/* #[get("/")]
fn index() -> Template {
    let context = context();
    Template::render("index", &context)
} */


/* #[get("/about")]
fn about() -> &'static str {
    "縛仙会について"
} */

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, about, katsudou, shashin,contact])
        .attach(Template::fairing())
        .launch();
}