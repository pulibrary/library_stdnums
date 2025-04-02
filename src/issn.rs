use crate::traits::{Valid, Normalize};

pub struct ISSN {
    pub identifier: String,
}

pub fn build_issn(identifier: String) -> ISSN {
    ISSN {
        identifier,
    }
}

impl Valid for ISSN {
    ///```
    /// use library_stdnums::issn::build_issn;
    /// use library_stdnums::issn::ISSN;
    /// use crate::library_stdnums::traits::Valid;
    /// 
    /// let valid_issn: ISSN = build_issn(String::from("0378-5955"));
    /// let invalid_issn: ISSN = build_issn(String::from("0378-5951"));
    /// 
    /// assert_eq!(valid_issn.valid(), true);
    /// assert_eq!(invalid_issn.valid(), false);
    /// ```
    fn valid(&self) -> bool {
        let basic_issn = reduce_to_basics(&self.identifier);
        let last_digit = match basic_issn {
            None => return false,
            Some(num) => num.chars().next_back().unwrap()
        };
        last_digit == checkdigit(&self.identifier)
    }
}

impl Normalize for ISSN {
    ///```
    /// use library_stdnums::issn::build_issn;
    /// use library_stdnums::issn::ISSN;
    /// use crate::library_stdnums::traits::Normalize;
    /// 
    /// let valid_issn = build_issn(String::from("0378-5955"));
    /// let invalid_issn = build_issn(String::from("abcdefg"));
    /// 
    /// assert_eq!(valid_issn.normalize().unwrap(), "03785955".to_string());
    /// assert!(invalid_issn.normalize().is_none());
    /// ```
    fn normalize(&self) -> Option<String> {
        let basic_issn = reduce_to_basics(&self.identifier);
        if build_issn(basic_issn.clone()?).valid() {
            Some(basic_issn?)
        } else {
            None
        }
    }
}

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
        assert_eq!(build_issn(String::from("0193-4511")).valid(), true);
        assert_eq!(build_issn(String::from("1043-383x")).valid(), true);
        assert_eq!(build_issn(String::from("0193-451X")).valid(), false);
    }

    #[test]
    fn it_normalizes() {
        assert_eq!(build_issn(String::from("0378-5955")).normalize().unwrap(), "03785955".to_string());
        assert_eq!(build_issn(String::from("1043-383x")).normalize().unwrap(), "1043383X".to_string());
    }

    #[test]
    fn it_returns_none_for_invalid_issns() {
        assert!(build_issn(String::from("abcdefg")).normalize().is_none());
        assert!(build_issn(String::from("XXXX-XXXX")).normalize().is_none());
    }
}
