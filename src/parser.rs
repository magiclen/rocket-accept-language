use unic_langid::LanguageIdentifier;

/// Parse an `accept-language` header value into language identifiers, ordered by descending quality.
pub(crate) fn parse_accept_language(raw_accept_language: &str) -> Vec<LanguageIdentifier> {
    let mut candidates: Vec<(LanguageIdentifier, f32)> = Vec::new();

    for element in raw_accept_language.split(',') {
        let mut sections = element.split(';');

        let Some(language_identifier) = sections
            .next()
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .and_then(|tag| LanguageIdentifier::from_bytes(tag.as_bytes()).ok())
        else {
            continue;
        };

        let mut quality = 1f32;

        for section in sections {
            let Some((key, value)) = section.split_once('=') else {
                continue;
            };

            if key.trim().eq_ignore_ascii_case("q") {
                // An unparsable or non-finite quality value is treated as 0 so that this element gets dropped.
                quality = match value.trim().parse::<f32>() {
                    Ok(q) if q.is_finite() => q.clamp(0f32, 1f32),
                    _ => 0f32,
                };

                break;
            }
        }

        // A quality value of 0 means "not acceptable".
        if quality > 0f32 {
            candidates.push((language_identifier, quality));
        }
    }

    // `sort_by` is stable, so elements sharing a quality value keep their original order.
    candidates.sort_by(|a, b| b.1.total_cmp(&a.1));

    candidates.into_iter().map(|(language_identifier, _)| language_identifier).collect()
}

#[cfg(test)]
mod tests {
    use unic_langid::langid;

    use super::parse_accept_language;

    #[test]
    fn keep_the_header_order_when_no_quality_is_given() {
        assert_eq!(
            vec![langid!("en-US"), langid!("en"), langid!("zh-TW")],
            parse_accept_language("en-US,en,zh-TW")
        );
    }

    #[test]
    fn sort_by_descending_quality() {
        assert_eq!(vec![langid!("de"), langid!("en")], parse_accept_language("en;q=0.5,de;q=0.9"));
    }

    #[test]
    fn an_omitted_quality_defaults_to_one() {
        assert_eq!(vec![langid!("de"), langid!("en")], parse_accept_language("en;q=0.5,de"));
    }

    #[test]
    fn keep_the_header_order_within_the_same_quality() {
        assert_eq!(
            vec![langid!("en"), langid!("de"), langid!("ja")],
            parse_accept_language("en;q=0.8,de;q=0.8,ja;q=0.5")
        );
    }

    #[test]
    fn drop_elements_which_are_not_acceptable() {
        assert_eq!(vec![langid!("zh-TW")], parse_accept_language("en;q=0,zh-TW"));
    }

    #[test]
    fn allow_spaces_and_tabs_between_elements() {
        assert_eq!(
            vec![langid!("en-US"), langid!("zh-TW")],
            parse_accept_language("en-US, \tzh-TW")
        );
    }

    #[test]
    fn parse_a_header_sent_by_a_real_browser() {
        assert_eq!(
            vec![langid!("zh-TW"), langid!("zh"), langid!("en-US"), langid!("en")],
            parse_accept_language("zh-TW,zh;q=0.9,en-US;q=0.8,en;q=0.7")
        );
    }
}
