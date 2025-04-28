use crate::traits::{Valid, Normalize};

pub struct ISSN {
    pub identifier: String,
}

impl ISSN {
    pub fn new(identifier: impl Into<String>) -> ISSN {
        ISSN {
            identifier: identifier.into(),
        }
    }
    /// ```
    /// use library_stdnums::ISSN;
    /// assert_eq!(ISSN::new("0378-5955").checkdigit(), '5');
    /// ```
    pub fn checkdigit(&self) -> char {
        let clean_string = &self.identifier.replace("-", "");
        let first_seven = clean_string.chars().take(7);
        let first_seven_digits = first_seven.filter_map(|x| x.to_digit(10));
        let multiplied = first_seven_digits.enumerate().map(|(index, digit)| digit * (8 - index as u32));

        let summed: u32 = multiplied.sum();
        let modulus: u32 = summed % 11;
        from_digit_to_checkdigit(modulus)
    }
}

impl Valid for ISSN {
    ///```
    /// use library_stdnums::ISSN;
    /// use library_stdnums::Valid;
    /// 
    /// assert_eq!(ISSN::new("0378-5955").valid(), true);
    /// assert_eq!(ISSN::new("0378-5951").valid(), false);
    /// ```
    fn valid(&self) -> bool {
        let basic_issn = reduce_to_basics(&self.identifier);
        let last_digit = match basic_issn {
            None => return false,
            Some(num) => num.chars().next_back().unwrap()
        };
        last_digit == self.checkdigit()
    }
}

impl Normalize for ISSN {
    ///```
    /// use library_stdnums::ISSN;
    /// use library_stdnums::Normalize;
    /// 
    /// assert_eq!(ISSN::new("0378-5955").normalize().unwrap(), "03785955".to_string());
    /// assert!(ISSN::new("abcdefg").normalize().is_none());
    /// ```
    fn normalize(&self) -> Option<String> {
        let basic_issn = reduce_to_basics(&self.identifier);
        if ISSN::new(basic_issn.clone()?).valid() {
            Some(basic_issn?)
        } else {
            None
        }
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
        assert_eq!(ISSN::new("0193-4511").checkdigit(), '1');
        assert_eq!(ISSN::new("1043-383X").checkdigit(), 'X');
        assert_eq!(ISSN::new("1561-4670").checkdigit(), '0');
    }
    #[test]
    fn it_calculates_validity() {
        assert!(ISSN::new("0193-4511").valid());
        assert!(ISSN::new("1043-383x").valid());
        assert!(!ISSN::new("0193-451X").valid());
    }

    #[test]
    fn it_reduces_to_basic() {
        assert!(reduce_to_basics("019X-4511").is_none());
    }

    #[test]
    fn it_normalizes() {
        assert_eq!(ISSN::new("0378-5955").normalize().unwrap(), "03785955".to_string());
        assert_eq!(ISSN::new("1043-383x").normalize().unwrap(), "1043383X".to_string());
    }

    #[test]
    fn it_returns_none_for_invalid_issns() {
        assert!(ISSN::new(String::from("abcdefg")).normalize().is_none());
        assert!(ISSN::new(String::from("XXXX-XXXX")).normalize().is_none());
    }
}
