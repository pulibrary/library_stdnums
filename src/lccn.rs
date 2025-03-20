pub fn valid(lccn: &str) -> bool {
    let normalized_version = normalized_version(lccn);
    println!("normalized version from valid is: {}", normalized_version);
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

/// Normalize an LCCN string based on the
/// [Library of Congress criteria](https://www.loc.gov/marc/lccn-namespace.html#syntax)
///
/// If the LCCN is valid, it will return it in a `Some`
/// 
/// ```
/// use library_stdnums::lccn::normalize;
/// assert_eq!(normalize("n78-890351"), Some("n78890351".to_string()));
/// assert_eq!(normalize("n78-890351").unwrap(), "n78890351");
/// ```
/// 
/// Returns None if the lccn is not valid
/// 
/// ```
/// use library_stdnums::lccn::normalize;
/// assert!(normalize("Bad LCCN").is_none());
/// ```
pub fn normalize(lccn: &str) -> Option<String> {
    let normalized_version = normalized_version(lccn);
    
    println!("normalized version is: {}", normalized_version);
    if valid(&normalized_version) {
        return Some(normalized_version);
    }
    None
}

fn normalized_version(lccn: &str) -> String {
    let basic_version = reduce_to_basic(lccn);
    if !basic_version.contains('-'){
        return basic_version;
    }
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
        assert_eq!(valid("78-890351"), true);
        assert_eq!(valid("n78-890351"), true);
        assert_eq!(valid("2001-890351"), true);
        assert_eq!(valid("nb78-890351"), true);
        assert_eq!(valid("agr78-890351"), true);
        assert_eq!(valid("n2001-890351"), true);
        assert_eq!(valid("nb2001-890351"), true);
        assert_eq!(valid("n78-89035100444"), false, "Too long");
        assert_eq!(valid("n78"), false, "Too short");
        assert_eq!(
            valid("378-890351"),
            false,
            "378-890351 should start with a letter"
        );
        assert_eq!(
            valid("naa078-890351"),
            false,
            "naa78-890351 should start with two letters"
        );
        assert_eq!(
            valid("122001-890351"),
            false,
            "122001-890351 should start with two letters"
        );
        assert_eq!(
            valid("n078-890351"),
            false,
            "n078-890351 should start with two letters or two digits"
        );
        assert_eq!(
            valid("na078-890351"),
            false,
            "na078-890351 should start with three letters or digits"
        );
        assert_eq!(
            valid("0an78-890351"),
            false,
            "0an78-890351 should start with three letters or digits"
        );
        assert_eq!(
            valid("n78-89c0351"),
            false,
            "n78-89c0351 has a letter after the dash"
        );
        assert_eq!(
            valid("n  78890351 "),
            true,
            "LCCNs with extra spaces are still considered valid if preprocessed=false"
        );
        assert_eq!(
            valid("   94014580 /AC/r95"),
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
    fn it_normalizes_all_the_ancient_perl_examples() {
        let test_examples = [["n78-890351", "n78890351"],
        ["n 78890351 ", "n78890351"],
        [" 85000002 ", "85000002"],
        ["85-2 ", "85000002"],
        ["2001-000002", "2001000002"],
        ["75-425165//r75", "75425165"],
        [" 79139101 /AC/r932", "79139101"],
        ["89-4", "89000004"],
        ["89-45", "89000045"],
        ["89-456", "89000456"],
        ["89-1234", "89001234"],
        ["89-001234", "89001234"],
        ["89001234", "89001234"],
        ["2002-1234", "2002001234"],
        ["2002-001234", "2002001234"],
        ["2002001234", "2002001234"],
        ["   89001234 ", "89001234"],
        ["  2002001234", "2002001234"],
        ["a89-1234", "a89001234"],
        ["a89-001234", "a89001234"],
        ["a89001234", "a89001234"],
        ["a2002-1234", "a2002001234"],
        ["a2002-001234", "a2002001234"],
        ["a2002001234", "a2002001234"],
        ["a 89001234 ", "a89001234"],
        ["a 89-001234 ", "a89001234"],
        ["a 2002001234", "a2002001234"],
        ["ab89-1234", "ab89001234"],
        ["ab89-001234", "ab89001234"],
        ["ab89001234", "ab89001234"],
        ["ab2002-1234", "ab2002001234"],
        ["ab2002-001234", "ab2002001234"],
        ["ab2002001234", "ab2002001234"],
        ["ab 89001234 ", "ab89001234"],
        ["ab 2002001234", "ab2002001234"],
        ["ab 89-1234", "ab89001234"],
        ["abc89-1234", "abc89001234"],
        ["abc89-001234", "abc89001234"],
        ["abc89001234", "abc89001234"],
        ["abc89001234 ", "abc89001234"],
        ["http://lccn.loc.gov/89001234", "89001234"],
        ["http://lccn.loc.gov/a89001234", "a89001234"],
        ["http://lccn.loc.gov/ab89001234", "ab89001234"],
        ["http://lccn.loc.gov/abc89001234", "abc89001234"],
        ["http://lccn.loc.gov/2002001234", "2002001234"],
        ["http://lccn.loc.gov/a2002001234", "a2002001234"],
        ["http://lccn.loc.gov/ab2002001234", "ab2002001234"],
        ["00-21595", "00021595"],
        ["2001001599", "2001001599"],
        ["99-18233", "99018233"],
        ["98000595", "98000595"],
        ["99005074", "99005074"],
        ["00003373", "00003373"],
        ["01001599", "01001599"],
        ["   95156543 ", "95156543"],
        ["   94014580 /AC/r95", "94014580"],
        ["   79310919 //r86", "79310919"],
        ["gm 71005810  ", "gm71005810"],
        ["sn2006058112  ", "sn2006058112"],
        ["gm 71-2450", "gm71002450"],
        ["2001-1114", "2001001114"]];

        for test in test_examples {
            assert_eq!(
                normalize(test[0]).unwrap(),
                test[1]
            );
        }
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
