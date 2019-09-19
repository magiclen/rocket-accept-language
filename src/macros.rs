#[doc(hidden)]
#[macro_export]
macro_rules! language_identifiers_counter {
    () => (0usize);
    ( $language:expr, $($languages:expr),* $(,)* ) => (1usize + $crate::language_identifiers_counter!($($languages,)*));
}

#[doc(hidden)]
#[macro_export]
macro_rules! create_language_identifier {
    ($language:ident $(,)*) => {{
        use $crate::tinystr::TinyStr8;

        unsafe {
            $crate::LanguageIdentifier::from_raw_parts_unchecked(
                Some(stringify!($language).parse().unwrap()),
                None,
                None,
                None,
            )
        }
    }};
    ($language:ident, $region:ident $(,)*) => {{
        use $crate::tinystr::TinyStr8;

        unsafe {
            $crate::LanguageIdentifier::from_raw_parts_unchecked(
                Some(stringify!($language).parse().unwrap()),
                None,
                Some(stringify!($region).parse().unwrap()),
                None,
            )
        }
    }};
}

/// This macro can be used to create a `Vec<LanguageIdentifier>` instance quickly by providing multiple `<language>[-<region>]` tokens separated by commas.
#[macro_export]
macro_rules! unchecked_language_region_pairs {
    ($($language:ident $(- $region:ident)?),* $(,)*) => {
        {
            let mut v = Vec::with_capacity($crate::language_identifiers_counter!($($language,)*));

            $(
                v.push($crate::create_language_identifier!($language $(, $region)?));
            )*

            v
        }
    };
}
