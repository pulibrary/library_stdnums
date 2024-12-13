use regex::Regex;

pub fn valid(lccn: &str, preprocessed: bool) -> bool {
    // clean = lccn.gsub(/\-/, '')
    let clean = str::replace(lccn, '-', "");
    //   suffix = clean[-8..-1] # "the rightmost eight characters are always digits"
    let re = Regex::new(r".*\d{8}$").unwrap();
    let suffix = re.is_match(&clean);
    //   return false unless suffix and suffix =~ /^\d+$/
    if !suffix {
        return false;
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
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_correctly() {
        assert_eq!(valid("n78-890351", false), true);
    }
}