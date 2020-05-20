/// This macro can be used to create a `Vec<LanguageIdentifier>` instance quickly by providing multiple `<language>[-<region>]` or `<language>[_<region>]` strings separated by commas.
#[macro_export]
macro_rules! language_region_pairs {
    ($($id:expr),* $(,)*) => {
        vec![$($crate::unic_langid::langid!($id),)*]
    };
}

/// This macro can be used to create a `Language` instance constantly .
#[macro_export]
macro_rules! language {
    ($id:expr) => {
        $crate::unic_langid_macros::lang!($id)
    };
}

/// This macro can be used to create a `Region` instance constantly .
#[macro_export]
macro_rules! region {
    ($id:expr) => {
        $crate::unic_langid_macros::region!($id)
    };
}
