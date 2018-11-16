#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rocket_accept_language;

use rocket_accept_language::AcceptLanguage;

#[get("/")]
fn hello(accept_language: AcceptLanguage) -> &'static str {
    let (language, region) = accept_language.get_first_language_region().unwrap_or(("en", None));

    match language {
        "zh" => match region.unwrap_or("") {
            "CN" => "哈罗！",
            _ => "哈囉！",
        }
        "jp" => "ハロー",
        _ => "Hello!"
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}