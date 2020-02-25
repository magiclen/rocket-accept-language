#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

use rocket::State;

use rocket_accept_language::{AcceptLanguage, LanguageIdentifier};

struct SupportLanguages {
    language_identifiers: Vec<LanguageIdentifier>,
}

impl SupportLanguages {
    fn as_language_identifiers(&self) -> &[LanguageIdentifier] {
        &self.language_identifiers
    }
}

#[get("/")]
fn hello(
    accept_language: &AcceptLanguage,
    support_languages: State<SupportLanguages>,
) -> &'static str {
    let (language, region) = accept_language
        .get_appropriate_language_region(support_languages.as_language_identifiers())
        .unwrap_or(("en", None));

    match language {
        "zh" => {
            match region.unwrap_or("") {
                "CN" => "哈罗！",
                _ => "哈囉！",
            }
        }
        "jp" => "ハロー",
        _ => "Hello!",
    }
}

fn main() {
    let support_languages = SupportLanguages {
        language_identifiers: language_region_pairs!["zh-TW", "zh_CN", "jp", "en"],
    };

    rocket::ignite().manage(support_languages).mount("/", routes![hello]).launch();
}
