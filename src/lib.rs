/*!
# `accept-language` Request Guard for Rocket Framework

This crate provides a request guard used for getting `accept-language` header.

See `examples`.
*/

pub extern crate rocket;
extern crate accept_language;
extern crate fluent_locale;

pub use fluent_locale::Locale;

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

/// The request guard used for getting `accept-language` header.
#[derive(Debug, Clone)]
pub struct AcceptLanguage {
    pub accept_language: Vec<Locale>,
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let raw_accept_language: Option<&str> = request.headers().get("accept-language").next(); // Only fetch the first one.

        match raw_accept_language {
            Some(raw_accept_language) => {
                let accept_language_split = accept_language::parse(raw_accept_language);

                let accept_language = accept_language_split.iter().filter_map(|al| Locale::new(al, None).ok()).collect();

                Outcome::Success(AcceptLanguage {
                    accept_language
                })
            }
            None => Outcome::Success(AcceptLanguage {
                accept_language: Vec::new()
            })
        }
    }
}

impl AcceptLanguage {
    /// Get the first region. For example, a region can be `"US"`, `"TW"` or `"GB"`.
    pub fn get_first_region(&self) -> Option<&str> {
        for locale in &self.accept_language {
            let region = locale.get_region();

            if !region.is_empty() {
                return Some(region);
            }
        }

        None
    }

    /// Get the first language. For example, a language can be `"en"`, `"zh"` or `"jp"`.
    pub fn get_first_language(&self) -> Option<&str> {
        for locale in &self.accept_language {
            let language = locale.get_language();

            if !language.is_empty() {
                return Some(language);
            }
        }

        None
    }

    /// Get the first language-region pair. The region might not exist. For example, a language-region pair can be `("en", Some("US"))`, `("en", Some("GB"))`, `("zh", Some("TW"))` or `("zh", None)`.
    pub fn get_first_language_region(&self) -> Option<(&str, Option<&str>)> {
        for locale in &self.accept_language {
            let language = locale.get_language();

            if !language.is_empty() {
                let region = locale.get_region();

                if region.is_empty() {
                    return Some((language, None));
                } else {
                    return Some((language, Some(region)));
                }
            }
        }

        None
    }
}