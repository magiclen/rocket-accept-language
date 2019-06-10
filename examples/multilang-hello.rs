#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate rocket_accept_language;

use rocket::State;

use rocket_accept_language::{AcceptLanguage, Locale};

struct SupportLanguages {
    locales: Vec<Locale>
}

impl SupportLanguages {
    fn as_locales(&self) -> &[Locale] {
        &self.locales
    }
}

#[get("/")]
fn hello(accept_language: &AcceptLanguage, support_languages: State<SupportLanguages>) -> &'static str {
    let (language, region) = accept_language.get_appropriate_language_region(support_languages.as_locales()).unwrap_or(("en", None));

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
    let support_languages = SupportLanguages {
        locales: vec![Locale::new("zh_TW", None).unwrap(), Locale::new("zh_CN", None).unwrap(), Locale::new("jp", None).unwrap()]
    };

    rocket::ignite()
        .manage(support_languages)
        .mount("/", routes![hello])
        .launch();
}