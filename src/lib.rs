//! # `accept-language` Request Guard for Rocket Framework
//! This crate provides a request guard used for getting `accept-language` header.

extern crate accept_language;
extern crate rocket;
extern crate fluent_locale;

use fluent_locale::Locale;

use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

/// The request guard used for getting `accept-language` header.
pub struct AcceptLanguage {
    pub accept_language: Vec<Locale>,
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("accept-language").collect();

        if keys.len() < 1 {
            return Outcome::Success(AcceptLanguage {
                accept_language: Vec::new()
            });
        }

        let key = keys[0];

        let mut accept_language = Vec::new();

        for al in accept_language::parse(key) {
            if let Ok(locale) = Locale::new(&al, None) {
                accept_language.push(locale);
            };
        }

        Outcome::Success(AcceptLanguage {
            accept_language
        })
    }
}