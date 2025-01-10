pub fn valid(lccn: &str, preprocessed: bool) -> bool {
    // lccn = normalize(lccn) unless preprocessed
    let normalized_version = normalized_version(lccn);
    let clean = str::replace(&normalized_version, '-', "");
    let suffix = clean.chars().rev().take(8).all(char::is_numeric);
    if !suffix {
        return false;
    }

    match clean.len() {
        8 => true,
        9 => clean.chars().next().unwrap().is_alphabetic(),
        10 => {
            clean[..2].chars().all(char::is_numeric) || clean[..2].chars().all(char::is_alphabetic)
        }
        11 => {
            let first_char = clean.chars().next().unwrap().is_alphabetic();
            let next_2_numeric = clean[1..3].chars().all(char::is_numeric);
            let next_2_alpha = clean[1..3].chars().all(char::is_alphabetic);
            first_char && (next_2_numeric || next_2_alpha)
        }
        12 => {
            let first_2_alpha = clean[1..2].chars().all(char::is_alphabetic);
            let next_2_numeric = clean[2..4].chars().all(char::is_numeric);
            first_2_alpha && next_2_numeric
        }
        _ => false,
    }
}

/// Normalize an LCCN string based on the criteria at
/// https://www.loc.gov/marc/lccn-namespace.html#syntax
///
/// Returns None if the lccn is not valid
pub fn normalize(lccn: &str) -> Option<String> {
    let normalized_version = normalized_version(lccn);

    if valid(&normalized_version, true) {
        return Some(normalized_version);
    }
    None
}

fn normalized_version(lccn: &str) -> String {
    let basic_version = reduce_to_basic(lccn);
    let mut lccn_segments = basic_version.split('-');
    let first_segment = lccn_segments.next().unwrap_or_default();
    let second_segment = lccn_segments.next().unwrap_or_default();
    format!("{}{:0>6}", first_segment, second_segment)
}

fn reduce_to_basic(lccn: &str) -> String {
    lccn.replace(char::is_whitespace, "")
        .replace("http://lccn.loc.gov/", "")
        .chars()
        .take_while(|&ch| ch != '/')
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_correctly() {
        assert_eq!(valid("78-890351", false), true);
        assert_eq!(valid("n78-890351", false), true);
        assert_eq!(valid("2001-890351", false), true);
        assert_eq!(valid("nb78-890351", false), true);
        assert_eq!(valid("agr78-890351", false), true);
        assert_eq!(valid("n2001-890351", false), true);
        assert_eq!(valid("nb2001-890351", false), true);
        assert_eq!(valid("n78-89035100444", false), false, "Too long");
        assert_eq!(valid("n78", false), false, "Too short");
        assert_eq!(
            valid("378-890351", false),
            false,
            "378-890351 should start with a letter"
        );
        assert_eq!(
            valid("naa078-890351", false),
            false,
            "naa78-890351 should start with two letters"
        );
        assert_eq!(
            valid("122001-890351", false),
            false,
            "122001-890351 should start with two letters"
        );
        assert_eq!(
            valid("n078-890351", false),
            false,
            "n078-890351 should start with two letters or two digits"
        );
        assert_eq!(
            valid("na078-890351", false),
            false,
            "na078-890351 should start with three letters or digits"
        );
        assert_eq!(
            valid("0an78-890351", false),
            false,
            "0an78-890351 should start with three letters or digits"
        );
        assert_eq!(
            valid("n78-89c0351", false),
            false,
            "n78-89c0351 has a letter after the dash"
        );
        assert_eq!(
            valid("n  78890351 ", false),
            true,
            "LCCNs with extra spaces are still considered valid if preprocessed=false"
        );
        assert_eq!(
            valid("   94014580 /AC/r95", false),
            true,
            "LCCNs with a suffix are considered valid if preprocessed=false"
        );
    }


    #[test]
    fn it_reduces_to_basic_form() {
        assert_eq!(
            reduce_to_basic("n  78890351 "),
            "n78890351",
            "It removes spaces"
        );
        assert_eq!(
            reduce_to_basic("http://lccn.loc.gov/89001234"),
            "89001234",
            "It removes the URI"
        );
        assert_eq!(
            reduce_to_basic("   94014580 /AC/r95"),
            "94014580",
            "It removes everything after the first /"
        );
    }

    #[test]
    fn it_normalizes() {
        assert_eq!(
            normalize("2001-000002").unwrap(),
            "2001000002",
            "It removes hyphens"
        );
        assert_eq!(
            normalize("85-2").unwrap(),
            "85000002",
            "It left-fills the substring with zeros until the length is six"
        );
        assert_eq!(
            normalize("n78-890351").unwrap(),
            "n78890351",
            "It retains prefixes"
        );
        assert!(normalize("n78-89035100444").is_none());
    }
    #[test]
    fn it_normalizes_with_suffixes() {
        assert_eq!(
            normalize("75-425165//r75").unwrap(),
            "75425165",
            "It removes suffixes which are not officially part of the lccn"
        );
    }
}
