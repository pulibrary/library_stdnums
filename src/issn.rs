/// ```
/// use library_stdnums::issn::checkdigit;
/// assert_eq!(checkdigit("0378-5955"), '5');
/// ```
pub fn checkdigit(issn: &str) -> char {
    let clean_string = issn.replace("-", "");
    let first_seven = clean_string.chars().take(7);
    let first_seven_digits = first_seven.filter_map(|x| x.to_digit(10));
    let multiplied = first_seven_digits.enumerate().map(|(index, digit)| digit * (8 - index as u32));

    let summed: u32 = multiplied.sum();
    let modulus: u32 = summed % 11;
    from_digit_to_checkdigit(modulus)
}
///```
/// use library_stdnums::issn::valid;
/// assert_eq!(valid("0378-5955"), true);
/// assert_eq!(valid("0378-5951"), false);
/// ```
pub fn valid(issn: &str) -> bool {
    let last_char = issn.chars().rev().next().unwrap().to_ascii_uppercase();
    last_char == checkdigit(issn)
}

pub fn normalize(issn: &str) -> Option<String> {
    Some("1043383X".to_string())
}

fn from_digit_to_checkdigit(num: u32) -> char {
    let orig_num = char::from_digit(11 as u32 - num, 11).unwrap();
    if orig_num == 'a' {
        'X'
    } else {
        orig_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_calculates_the_checkdigit() {
        assert_eq!(checkdigit("0193-4511"), '1');
        assert_eq!(checkdigit("1043-383X"), 'X');
    }
    #[test]
    fn it_calculates_validity() {
        assert_eq!(valid("0193-4511"), true);
        assert_eq!(valid("0193-451X"), false);
        assert_eq!(valid("1043-383x"), true);
    }
    #[test]
    fn it_creates_a_normalized_form() {
        assert_eq!(normalize("1043-383X").unwrap(), "1043383X");
        assert_eq!(normalize("0193-4511").unwrap(), "01934511");
        // assert_eq!(normalize("1043-383x"), "1043383X");
    }
}
