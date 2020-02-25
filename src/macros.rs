/// This macro can be used to create a `Vec<LanguageIdentifier>` instance quickly by providing multiple `<language>[-<region>]` or `<language>[_<region>]` strings separated by commas.
#[macro_export]
macro_rules! language_region_pairs {
    ($($id:expr),* $(,)*) => {
        {
            vec![$($crate::unic_langid::langid!($id),)*]
        }
    };
}
