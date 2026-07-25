/*!
# `accept-language` Request Guard for Rocket Framework

This crate provides a request guard used for getting `accept-language` header.

```rust
use rocket::get;
use rocket_accept_language::{
    language, language_region_pairs, unic_langid::subtags::Language, AcceptLanguage,
    LanguageIdentifier,
};

const LANGUAGE_ZH: Language = language!("zh");
const LANGUAGE_JA: Language = language!("ja");

#[get("/")]
fn hello(accept_language: &AcceptLanguage) -> &'static str {
    let supported_locales: Vec<LanguageIdentifier> = language_region_pairs!["en", "zh-Hant", "ja"];

    match accept_language.get_appropriate_locale(&supported_locales) {
        Some(locale) if locale.language == LANGUAGE_ZH => "哈囉！",
        Some(locale) if locale.language == LANGUAGE_JA => "ハロー",
        _ => "Hello!",
    }
}
```
*/

pub extern crate rocket;
pub extern crate unic_langid;

mod macros;
mod parser;

use rocket::{
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
pub use unic_langid::LanguageIdentifier;
use unic_langid::subtags::{Language, Region};

use crate::parser::parse_accept_language;

/// The request guard used for getting `accept-language` header.
#[derive(Debug, Clone)]
pub struct AcceptLanguage {
    /// The requested locales, ordered by descending quality.
    pub accept_language: Vec<LanguageIdentifier>,
}

#[inline]
fn from_request(request: &Request<'_>) -> AcceptLanguage {
    let raw_accept_language: Option<&str> = request.headers().get("accept-language").next(); // Only fetch the first one.

    let accept_language = raw_accept_language.map(parse_accept_language).unwrap_or_default();

    AcceptLanguage {
        accept_language,
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AcceptLanguage {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(from_request(request))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r AcceptLanguage {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(request.local_cache(|| from_request(request)))
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

    /// Get the first language. For example, a language can be `"en"`, `"zh"` or `"ja"`.
    pub fn get_first_language(&self) -> Option<Language> {
        self.accept_language.first().map(|locale| locale.language)
    }

    /// Get the first language-region pair. The region might not exist. For example, a language-region pair can be `("en", Some("US"))`, `("en", Some("GB"))`, `("zh", Some("TW"))` or `("zh", None)`.
    pub fn get_first_language_region(&self) -> Option<(Language, Option<Region>)> {
        if let Some(locale) = self.accept_language.first() {
            let language = locale.language;

            let region = locale.region;

            Some((language, region))
        } else {
            None
        }
    }

    /// Get the most appropriate locale out of `locales`.
    ///
    /// Matching is attempted in tiers, and each tier scans the requested locales in order of preference. A tier only accepts an exact match on the subtags it compares: first `language`-`script`-`region`, then `language`-`script`, then `language`-`region`, and finally `language` alone.
    pub fn get_appropriate_locale<'a>(
        &self,
        locales: &'a [LanguageIdentifier],
    ) -> Option<&'a LanguageIdentifier> {
        // The `script` subtag is more significant than the `region` one, so it is given up later.
        for (compare_script, compare_region) in
            [(true, true), (true, false), (false, true), (false, false)]
        {
            for locale in &self.accept_language {
                for t_locale in locales {
                    if locale.language != t_locale.language {
                        continue;
                    }

                    if compare_script && locale.script != t_locale.script {
                        continue;
                    }

                    if compare_region && locale.region != t_locale.region {
                        continue;
                    }

                    return Some(t_locale);
                }
            }
        }

        None
    }

    /// Get the appropriate language-region pair. If the region can not be matched, and there is no matched language-region pairs, returns the first matched language.
    #[deprecated(
        since = "0.9.0",
        note = "use `get_appropriate_locale` instead, which also takes the `script` subtag into \
                account"
    )]
    pub fn get_appropriate_language_region(
        &self,
        locales: &[LanguageIdentifier],
    ) -> Option<(Language, Option<Region>)> {
        self.get_appropriate_locale(locales).map(|locale| (locale.language, locale.region))
    }
}
