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
    let basic_issn = reduce_to_basics(issn);
    let last_digit = match basic_issn {
        None => return false,
        Some(num) => num.chars().next_back().unwrap()
    };
    last_digit == checkdigit(issn)
}

///```
/// use library_stdnums::issn::normalize;
/// assert_eq!(normalize("0378-5955").unwrap(), "03785955".to_string());
/// assert!(normalize("abcdefg").is_none());
/// ```
pub fn normalize(issn: &str) -> Option<String> {
    let basic_issn = reduce_to_basics(issn);
    if valid(&basic_issn.clone()?) {
        Some(basic_issn?)
    } else {
        None
    }
}

fn from_digit_to_checkdigit(num: u32) -> char {
    let orig_num = char::from_digit((11_u32 - num) % 11, 11).unwrap();
    if orig_num == 'a' {
        'X'
    } else {
        orig_num
    }
}

fn reduce_to_basics(issn: &str) -> Option<String> {
    let clean_string = issn
        .replace("-", "")
        .replace('x', "X");
    if clean_string.chars()
        .rev()
        .enumerate()
        .all(|(index, c)| c.is_ascii_digit() || (index == 0 && c == 'X' )) {
        Some(clean_string)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_calculates_the_checkdigit() {
        assert_eq!(checkdigit("0193-4511"), '1');
        assert_eq!(checkdigit("1043-383X"), 'X');
        assert_eq!(checkdigit("1561-4670"), '0');
    }
    #[test]
    fn it_calculates_validity() {
        assert_eq!(valid("0193-4511"), true);
        assert_eq!(valid("1043-383x"), true);
        assert_eq!(valid("0193-451X"), false);
    }

    #[test]
    fn it_normalizes() {
        assert_eq!(normalize("0378-5955").unwrap(), "03785955".to_string());
        assert_eq!(normalize("1043-383x").unwrap(), "1043383X".to_string());
    }

    #[test]
    fn it_returns_none_for_invalid_issns() {
        assert!(normalize("abcdefg").is_none());
        assert!(normalize("XXXX-XXXX").is_none());
    }
}
