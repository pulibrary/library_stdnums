pub fn valid(lccn: &str, preprocessed: bool) -> bool {
    // lccn = normalize(lccn) unless preprocessed
    let clean = str::replace(lccn, '-', "");
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
    }
}
