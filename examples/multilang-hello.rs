#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_accept_language;

use rocket::State;

use rocket_accept_language::unic_langid::subtags::{Language, Region};
use rocket_accept_language::{AcceptLanguage, LanguageIdentifier};

const LANGUAGE_ZH: Language = language!("zh");
const LANGUAGE_EN: Language = language!("en");
const LANGUAGE_JP: Language = language!("jp");
const REGION_CN: Region = region!("cn");
const REGION_TW: Region = region!("tw");

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
    let (language, region) = accept_language
        .get_appropriate_language_region(support_languages.as_language_identifiers())
        .unwrap_or((LANGUAGE_EN, None));

    match language {
        LANGUAGE_ZH => {
            match region.unwrap_or(REGION_TW) {
                REGION_CN => "哈罗！",
                _ => "哈囉！",
            }
        }
        LANGUAGE_JP => "ハロー",
        _ => "Hello!",
    }
}

#[launch]
fn rocket() -> _ {
    let support_languages = SupportLanguages {
        language_identifiers: language_region_pairs!["zh-TW", "zh_CN", "jp", "en"],
    };

    rocket::build().manage(support_languages).mount("/", routes![hello])
}
