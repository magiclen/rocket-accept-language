/*!
# `accept-language` Request Guard for Rocket Framework

This crate provides a request guard used for getting `accept-language` header.

See `examples`.
*/

pub extern crate rocket;
pub extern crate fluent_locale;
extern crate accept_language;

pub use fluent_locale::Locale;

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

/// The request guard used for getting `accept-language` header.
#[derive(Debug, Clone)]
pub struct AcceptLanguage {
    pub accept_language: Vec<Locale>,
}

macro_rules! impl_request_guard {
    ($request:ident) => {
        {
            let raw_accept_language: Option<&str> = $request.headers().get("accept-language").next(); // Only fetch the first one.

            match raw_accept_language {
                Some(raw_accept_language) => {
                    let accept_language_split = accept_language::parse(raw_accept_language);

                    let accept_language = accept_language_split.iter().filter_map(|al| Locale::new(al, None).ok()).collect();

                    AcceptLanguage {
                        accept_language
                    }
                }
                None => AcceptLanguage {
                    accept_language: Vec::new()
                }
            }
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(impl_request_guard!(request))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &'a AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(request.local_cache(|| impl_request_guard!(request)))
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
        self.accept_language.iter().next().map(|locale| locale.get_language())
    }

    /// Get the first language-region pair. The region might not exist. For example, a language-region pair can be `("en", Some("US"))`, `("en", Some("GB"))`, `("zh", Some("TW"))` or `("zh", None)`.
    pub fn get_first_language_region(&self) -> Option<(&str, Option<&str>)> {
        for locale in &self.accept_language {
            let language = locale.get_language();

            let region = locale.get_region();

            if region.is_empty() {
                return Some((language, None));
            } else {
                return Some((language, Some(region)));
            }
        }

        None
    }

    /// Get the appropriate language-region pair. If the region can not be matched, and there is no matched language-region pairs, returns the first matched language.
    pub fn get_appropriate_language_region(&self, locales: &[Locale]) -> Option<(&str, Option<&str>)> {
        let mut filtered_language = None;

        for locale in &self.accept_language {
            let language = locale.get_language();

            for t_locale in locales {
                let t_language = t_locale.get_language();

                if language == t_language {
                    let region = locale.get_region();
                    let t_region = t_locale.get_region();

                    if region == t_region {
                        if region.is_empty() {
                            return Some((language, None));
                        } else {
                            return Some((language, Some(region)));
                        }
                    } else {
                        if filtered_language.is_none() {
                            filtered_language = Some((language, None));
                        }
                    }
                }
            }
        }

        filtered_language
    }
}