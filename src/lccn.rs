use regex::Regex;

pub fn valid_regex(lccn: &str, preprocessed: bool) -> bool {
    let clean = str::replace(lccn, '-', "");
    let last_eight_re: Regex = Regex::new(r".*\d{8}$").unwrap();
    let suffix = last_eight_re.is_match(&clean);
    if !suffix {
        return false;
    }

    match clean.len() {
        8 => true,
        9 => clean.chars().next().unwrap().is_alphabetic(),
        10 => Regex::new(r"^(\d{2}|[A-Za-z]{2})").unwrap().is_match(&clean),
        11 => Regex::new(r"^[A-Za-z](\d{2}|[A-Za-z]{2})").unwrap().is_match(&clean),
        12 => Regex::new(r"^[A-Za-z]{2}\d{2}").unwrap().is_match(&clean),
        _ => false,
    }

}

pub fn valid_char(lccn: &str, preprocessed: bool) -> bool {
    let clean = str::replace(lccn, '-', "");
    let suffix = clean.chars().rev().take(8).all(char::is_numeric);
    if !suffix {
        return false;
    }

    match clean.len() {
        8 => true,
        9 => clean.chars().next().unwrap().is_alphabetic(),
        10 => clean[..2].chars().all(char::is_numeric) || clean[..2].chars().all(char::is_alphabetic),
        11 => {
            let first_char = clean.chars().next().unwrap().is_alphabetic();
            let next_2_numeric = clean[1..3].chars().all(char::is_numeric);
            let next_2_alpha = clean[1..3].chars().all(char::is_alphabetic);
            first_char && (next_2_numeric || next_2_alpha)
        },
        12 => {
            let first_2_alpha = clean[1..2].chars().all(char::is_alphabetic);
            let next_2_numeric = clean[2..4].chars().all(char::is_numeric);
            first_2_alpha && next_2_numeric
        },
        _ => false,
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_correctly() {
        // assert_eq!(valid("78-890351", false), true);
        // assert_eq!(valid("n78-890351", false), true);
        // assert_eq!(valid("2001-890351", false), true);
        // assert_eq!(valid("nb78-890351", false), true);
        // assert_eq!(valid("agr78-890351", false), true);
        // assert_eq!(valid("n2001-890351", false), true);
        // assert_eq!(valid("nb2001-890351", false), true);
        // assert_eq!(valid("n78-89035100444", false), false, "Too long");
        // assert_eq!(valid("n78", false), false, "Too short");
        // assert_eq!(valid("na078-890351", false), false, "naa78-890351 should start with three letters or digits");
        // assert_eq!(valid("n078-890351", false), false, "n078-890351 should start with two letters or two digits");
        // assert_eq!(valid("na078-890351", false), false, "na078-890351 should start with three letters or digits");
        // assert_eq!(valid("0an78-890351", false), false, "0an78-890351 should start with three letters or digits");
        // assert_eq!(valid("n78-89c0351", false), false, "n78-89c0351 has a letter after the dash");

        assert_eq!(valid_regex("78-890351", false), true);
        assert_eq!(valid_regex("n78-890351", false), true);
        assert_eq!(valid_regex("2001-890351", false), true);
        assert_eq!(valid_regex("nb78-890351", false), true);
        assert_eq!(valid_regex("agr78-890351", false), true);
        assert_eq!(valid_regex("n2001-890351", false), true);
        assert_eq!(valid_regex("nb2001-890351", false), true);
        assert_eq!(valid_regex("n78-89035100444", false), false, "Too long");
        assert_eq!(valid_regex("n78", false), false, "Too short");
        assert_eq!(valid_regex("na078-890351", false), false, "naa78-890351 should start with three letters or digits");
        assert_eq!(valid_regex("n078-890351", false), false, "n078-890351 should start with two letters or two digits");
        assert_eq!(valid_regex("na078-890351", false), false, "na078-890351 should start with three letters or digits");
        assert_eq!(valid_regex("0an78-890351", false), false, "0an78-890351 should start with three letters or digits");
        assert_eq!(valid_regex("n78-89c0351", false), false, "n78-89c0351 has a letter after the dash");

        assert_eq!(valid_char("78-890351", false), true);
        assert_eq!(valid_char("n78-890351", false), true);
        assert_eq!(valid_char("2001-890351", false), true);
        assert_eq!(valid_char("nb78-890351", false), true);
        assert_eq!(valid_char("agr78-890351", false), true);
        assert_eq!(valid_char("n2001-890351", false), true);
        assert_eq!(valid_char("nb2001-890351", false), true);
        assert_eq!(valid_char("n78-89035100444", false), false, "Too long");
        assert_eq!(valid_char("n78", false), false, "Too short");
        assert_eq!(valid_char("na078-890351", false), false, "naa78-890351 should start with three letters or digits");
        assert_eq!(valid_char("n078-890351", false), false, "n078-890351 should start with two letters or two digits");
        assert_eq!(valid_char("na078-890351", false), false, "na078-890351 should start with three letters or digits");
        assert_eq!(valid_char("0an78-890351", false), false, "0an78-890351 should start with three letters or digits");
        assert_eq!(valid_char("n78-89c0351", false), false, "n78-89c0351 has a letter after the dash");
    }
}
