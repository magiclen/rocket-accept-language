/*!
# `accept-language` Request Guard for Rocket Framework

This crate provides a request guard used for getting `accept-language` header.

See `examples`.
*/

extern crate accept_language;
pub extern crate rocket;
pub extern crate tinystr;
pub extern crate unic_langid;
pub extern crate unic_langid_macros;

mod macros;

use unic_langid::parser::parse_language_identifier;
use unic_langid::subtags::{Language, Region};
pub use unic_langid::LanguageIdentifier;

use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

/// The request guard used for getting `accept-language` header.
#[derive(Debug, Clone)]
pub struct AcceptLanguage {
    pub accept_language: Vec<LanguageIdentifier>,
}

macro_rules! impl_request_guard {
    ($request:ident) => {
        {
            let raw_accept_language: Option<&str> = $request.headers().get("accept-language").next(); // Only fetch the first one.

            match raw_accept_language {
                Some(raw_accept_language) => {
                    let accept_language_split = accept_language::parse(raw_accept_language);

                    let accept_language = accept_language_split.iter().filter_map(|al| parse_language_identifier(al.as_bytes()).ok()).collect();

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AcceptLanguage {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(impl_request_guard!(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r AcceptLanguage {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(request.local_cache(|| impl_request_guard!(request)))
    }
}

impl AcceptLanguage {
    /// Get the first region. For example, a region can be `"US"`, `"TW"` or `"GB"`.
    pub fn get_first_region(&self) -> Option<Region> {
        for locale in &self.accept_language {
            let region = locale.region;

            if region.is_some() {
                return region;
            }
        }

        None
    }

    /// Get the first language. For example, a language can be `"en"`, `"zh"` or `"jp"`.
    pub fn get_first_language(&self) -> Option<Language> {
        self.accept_language.get(0).map(|locale| locale.language)
    }

    /// Get the first language-region pair. The region might not exist. For example, a language-region pair can be `("en", Some("US"))`, `("en", Some("GB"))`, `("zh", Some("TW"))` or `("zh", None)`.
    pub fn get_first_language_region(&self) -> Option<(Language, Option<Region>)> {
        if let Some(locale) = self.accept_language.get(0) {
            let language = locale.language;

            let region = locale.region;

            Some((language, region))
        } else {
            None
        }
    }

    /// Get the appropriate language-region pair. If the region can not be matched, and there is no matched language-region pairs, returns the first matched language.
    pub fn get_appropriate_language_region(
        &self,
        locales: &[LanguageIdentifier],
    ) -> Option<(Language, Option<Region>)> {
        let mut filtered_language = None;

        for locale in &self.accept_language {
            let language = locale.language;

            for t_locale in locales {
                let t_language = t_locale.language;

                if language == t_language {
                    let region = locale.region;
                    let t_region = t_locale.region;

                    if region == t_region {
                        return Some((language, region));
                    } else if filtered_language.is_none() {
                        filtered_language = Some((language, None));
                    }
                }
            }
        }

        filtered_language
    }
}
