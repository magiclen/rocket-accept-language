`accept-language` Request Guard for Rocket Framework
====================

[![CI](https://github.com/magiclen/rocket-accept-language/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/rocket-accept-language/actions/workflows/ci.yml)

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

## Crates.io

https://crates.io/crates/rocket-accept-language

## Documentation

https://docs.rs/rocket-accept-language

## License

[MIT](LICENSE)