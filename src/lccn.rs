use regex::Regex;

pub fn valid(lccn: &str, preprocessed: bool) -> bool {
    // clean = lccn.gsub(/\-/, '')
    let clean = str::replace(lccn, '-', "");
    //   suffix = clean[-8..-1] # "the rightmost eight characters are always digits"
    let last_eight_re: Regex = Regex::new(r".*\d{8}$").unwrap();
    let suffix = last_eight_re.is_match(&clean);
    //   return false unless suffix and suffix =~ /^\d+$/
    if !suffix {
        return false;
    }

    match clean.len() {
        8 => true,
        9 => clean.chars().next().unwrap().is_alphabetic(),
        10 => clean[..1].chars().all(char::is_numeric) || clean[..1].chars().all(char::is_alphabetic),
        _ => false,
    }
    //   case clean.size # "...is a character string eight to twelve digits in length"
    //   when 8
    //     return true
    //   when 9
    //     return true if clean =~ /^[A-Za-z]/
    //   when 10
    //     return true if clean =~ /^\d{2}/ or clean =~ /^[A-Za-z]{2}/
    //   when 11
    //     return true if clean =~ /^[A-Za-z](\d{2}|[A-Za-z]{2})/
    //   when 12
    //     return true if clean =~ /^[A-Za-z]{2}\d{2}/
    //   else
    //     return false
    //   end

    //   return false
    // true
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
        assert_eq!(valid("na078-890351", false), false, "naa78-890351 should start with three letters or digits");
        assert_eq!(valid("n078-890351", false), false, "n078-890351 should start with two letters or two digits");
        assert_eq!(valid("na078-890351", false), false, "na078-890351 should start with three letters or digits");
        assert_eq!(valid("0an78-890351", false), false, "0an78-890351 should start with three letters or digits");
        assert_eq!(valid("n78-89c0351", false), false, "n78-89c0351 has a letter after the dash");

        // StdNum::LCCN.valid?("n78-890351").must_equal true
        // StdNum::LCCN.valid?("n78-89035100444").must_equal false, "Too long"
        // StdNum::LCCN.valid?("n78").must_equal false, "Too short"
        // StdNum::LCCN.valid?("na078-890351").must_equal false, "naa78-890351 should start with three letters or digits"
        // StdNum::LCCN.valid?("n078-890351").must_equal false, "n078-890351 should start with two letters or two digits"
        // StdNum::LCCN.valid?("na078-890351").must_equal false, "na078-890351 should start with three letters or digits"
        // StdNum::LCCN.valid?("0an78-890351").must_equal false, "0an78-890351 should start with three letters or digits"
        // StdNum::LCCN.valid?("n78-89c0351").must_equal false, "n78-89c0351 has a letter after the dash"
    }
}
