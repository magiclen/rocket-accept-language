#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

use rocket::State;
use rocket_accept_language::{
    AcceptLanguage, LanguageIdentifier,
    unic_langid::subtags::{Language, Region},
};

const LANGUAGE_ZH: Language = language!("zh");
const LANGUAGE_JA: Language = language!("ja");
const REGION_CN: Region = region!("cn");

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
    support_languages: &State<SupportLanguages>,
) -> &'static str {
    let locale =
        accept_language.get_appropriate_locale(support_languages.as_language_identifiers());

    match locale {
        Some(locale) if locale.language == LANGUAGE_ZH => match locale.region {
            Some(REGION_CN) => "哈罗！",
            _ => "哈囉！",
        },
        Some(locale) if locale.language == LANGUAGE_JA => "ハロー",
        _ => "Hello!",
    }
}

#[launch]
fn rocket() -> _ {
    let support_languages = SupportLanguages {
        language_identifiers: language_region_pairs!["zh-TW", "zh_CN", "ja", "en"],
    };

    rocket::build().manage(support_languages).mount("/", routes![hello])
}
