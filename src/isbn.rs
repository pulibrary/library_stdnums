use crate::traits::{Normalize, Valid};

pub struct ISBN {
  pub identifier: String,
}

impl ISBN {
  pub fn new(identifier: impl Into<String>) -> ISBN {
    ISBN {identifier: identifier.into()}
  }
  /// Calculate the checkdigit for a given ISBN
  ///
  /// Returns an Option<char> if the ISBN is a valid length
  ///
  /// ```
  /// use library_stdnums::isbn::ISBN;
  /// assert_eq!(ISBN::new("0139381430").checkdigit().unwrap(), '0');
  /// ```
  ///
  /// Returns None if the ISBN content is not valid length
  ///
  /// ```
  /// use library_stdnums::isbn::ISBN;
  /// assert_eq!(ISBN::new("Bad ISBN").checkdigit(), None);
  /// ```
  pub fn checkdigit(&self) -> Option<char> {
    let basic_string = reduce_to_basic(&self.identifier);
    match basic_string.len() {
        10 => Some(checkdigit_ten(&self.identifier)),
        13 => Some(checkdigit_thirteen(&self.identifier)),
        _ => None
    }
  }
}

impl Valid for ISBN {
  /// Assert if the ISBN is valid by verifying the checkdigit
  ///
  /// Returns true if the ISBN is valid
  /// 
  /// ```
  /// use library_stdnums::isbn::ISBN;
  /// use library_stdnums::traits::Valid;
  /// assert!(ISBN::new("0139381430").valid());
  /// ```
  ///
  /// Returns false if the ISBN is invalid
  ///
  /// ```
  /// use library_stdnums::isbn::ISBN;
  /// use library_stdnums::traits::Valid;
  /// assert_eq!(ISBN::new("0139381432").valid(), false);
  /// ```
  fn valid(&self) -> bool {
    let basic_string = reduce_to_basic(&self.identifier);
    match basic_string.len() {
      10 => checkdigit_ten(&basic_string) == basic_string.chars().next_back().unwrap(),
      13 => checkdigit_thirteen(&basic_string) == basic_string.chars().next_back().unwrap(),
      _ => false
    }
  }
}

impl Normalize for ISBN {
  /// Converts an ISBN to a normalized ISBN13
  ///
  /// Returns an Option<String> if the ISBN is valid
  /// 
  /// ```
  /// use library_stdnums::isbn::ISBN;
  /// use library_stdnums::traits::Normalize;
  /// assert_eq!(ISBN::new("ISBN: 978-0-306-40615-7").normalize().unwrap(), "9780306406157");
  /// assert_eq!(ISBN::new("0-306-40615-2").normalize().unwrap(), "9780306406157");
  /// ```
  ///
  /// Returns None if the ISBN is invalid
  ///
  /// ```
  /// use library_stdnums::isbn::ISBN;
  /// use library_stdnums::traits::Normalize;
  /// assert_eq!(ISBN::new("013938143").normalize(), None);
  /// ```
  fn normalize(&self) -> Option<String> {
    convert_to_13(&self.identifier)
  }
}

/// Converts an ISBN to its corresponding ISBN13
///
/// Returns an Option<String> if the ISBN is valid
/// 
/// ```
/// use library_stdnums::isbn::convert_to_13;
/// assert_eq!(convert_to_13("0-306-40615-2").unwrap(), "9780306406157");
/// ```
/// 
/// /// Returns an Option<String> if the ISBN is valid and already an ISBN13
/// 
/// ```
/// use library_stdnums::isbn::convert_to_13;
/// assert_eq!(convert_to_13("978-1-449-37332-0").unwrap(), "9781449373320");
/// ```
///
/// Returns None if the ISBN is invalid
///
/// ```
/// use library_stdnums::isbn::convert_to_13;
/// assert_eq!(convert_to_13("013938143"), None);
/// ```
pub fn convert_to_13(isbn: &str) -> Option<String> {
  if !ISBN::new(isbn).valid() {return None};
  let basic_string = reduce_to_basic(isbn);
  let prepended_string = format!("{}{}", "978", &basic_string[..9]);
  match basic_string.len() {
    10 => Some(format!("{}{}", prepended_string, checkdigit_thirteen(&prepended_string)).to_string()),
    13 => Some(basic_string.to_string()),
    _ => None,
  }
  
}

/// Converts an ISBN to its corresponding ISBN10
///
/// Returns an Option<String> if the ISBN is valid
/// 
/// ```
/// use library_stdnums::isbn::convert_to_10;
/// assert_eq!(convert_to_10("9780306406157").unwrap(), "0306406152");
/// ```
///
/// Returns None if the ISBN is invalid
///
/// ```
/// use library_stdnums::isbn::convert_to_10;
/// assert_eq!(convert_to_10("013938143"), None);
/// ```
/// 
/// Returns None if an ISBN13 begins with '979'
/// 
/// ```
/// use library_stdnums::isbn::convert_to_10;
/// assert_eq!(convert_to_10("9798531132178"), None);
/// ```
pub fn convert_to_10(isbn: &str) -> Option<String> {
  if !ISBN::new(isbn).valid() {return None};
  let basic_string = reduce_to_basic(isbn);
  if basic_string.starts_with("979") {
    return None;
  }
  match basic_string.len() {
    10 => Some(basic_string),
    13 => Some(format!("{}{}", &basic_string[3..12], checkdigit_ten(&basic_string[3..]))),
    _ => None,
  }
}

fn reduce_to_basic(isbn: &str) -> String {
  let clean_string = isbn.replace("-", "");
  scrub_alpha_prefix(&clean_string)
}

fn scrub_alpha_prefix(string_to_scrub: &str) -> String {
  string_to_scrub.chars()
    .skip_while(|c| !c.is_ascii_digit())
    .take_while(|c| c.is_ascii_digit() || c == &'X')
    .collect::<String>()
}

fn checkdigit_ten(isbn: &str) -> char {
  let basic_string = reduce_to_basic(isbn);
  let first_nine = basic_string.chars().take(9);
  let first_nine_digits = first_nine.filter_map(|x| x.to_digit(10));
  let multiplied = first_nine_digits.enumerate().map(|(index, digit)| digit * (10 - index as u32));

  let summed: u32 = multiplied.sum();
  let modulus: u32 = summed % 11;
  from_digit_to_checkdigit(modulus)
}

fn checkdigit_thirteen(isbn: &str) -> char {
  let basic_string = reduce_to_basic(isbn);
  let first_twelve = basic_string.chars().take(12);
  let first_twelve_digits = first_twelve.filter_map(|x| x.to_digit(10));
  let multiplied = first_twelve_digits.enumerate().map(|(index, digit)| digit * (1 + (index as u32 % 2) * 2 ));

  let summed: u32 = multiplied.sum();
  let modulus = summed % 10;
  let finished = (10 - modulus) % 10;
  char::from_digit(finished, 10).unwrap()
}

fn from_digit_to_checkdigit(num: u32) -> char {
  let orig_num = char::from_digit((11_u32 - num) % 11, 11).unwrap();
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
  fn it_can_create_an_isbn() {
    let isbn = ISBN::new("0139381430");
    assert_eq!(isbn.identifier, "0139381430");
  }

  #[test]
  fn it_calculates_the_checkdigit() {
    assert_eq!(ISBN::new("0139381430").checkdigit().unwrap(), '0');
    assert_eq!(ISBN::new("0-8044-2957-X").checkdigit().unwrap(), 'X');
    assert_eq!(ISBN::new("9781449373320").checkdigit().unwrap(), '0');
    assert_eq!(ISBN::new("9780306406152").checkdigit().unwrap(), '7')
  }

  #[test]
  fn it_checks_the_validity() {
    assert!(ISBN::new("0139381430").valid());
    assert!(ISBN::new("9781449373320").valid());
    assert!(ISBN::new("0-8044-2957-X").valid());
    assert!(ISBN::new("ABC0139381430").valid());
  }

  #[test]
  fn it_catches_invalid() {
    assert!(!ISBN::new("01393814300").valid());
    assert!(!ISBN::new("0139381432").valid());
    assert!(!ISBN::new("9781449373322").valid());
  }

  #[test]
  fn it_reduces_to_basic() {
    assert_eq!(reduce_to_basic("0-8044-2957-X"), "080442957X");
    assert_eq!(reduce_to_basic("ABC0139381430"), "0139381430");
  }

  #[test]
  fn it_scrubs_alpha_prefix() {
    assert_eq!(scrub_alpha_prefix("A1"), "1");
    assert_eq!(scrub_alpha_prefix("A123"), "123");
    assert_eq!(scrub_alpha_prefix("ABC0139381430"), "0139381430");
    assert_eq!(scrub_alpha_prefix("ABC080442957X"), "080442957X");
    assert_eq!(scrub_alpha_prefix("ABC080442957Y"), "080442957");
  }

  #[test]
  fn it_converts_isbn_10_to_13() {
    assert_eq!(convert_to_13("9781449373320").unwrap(), "9781449373320");
    assert_eq!(convert_to_13("0-306-40615-2").unwrap(), "9780306406157");
  }

  #[test]
  fn it_converts_isbn_13_to_10() {
    assert_eq!(convert_to_10("9780306406157").unwrap(), "0306406152");
    assert_eq!(convert_to_10("0306406152").unwrap(), "0306406152");
    assert_eq!(convert_to_10("9798531132178"), None);
    assert_eq!(convert_to_10("1"), None);
    assert_eq!(convert_to_10("9780306406157978030640615797803064061579780306406157"), None);
  }

  #[test]
  fn it_normalizes() {
    assert_eq!(ISBN::new("0-306-40615-2").normalize().unwrap(), "9780306406157");
    assert_eq!(ISBN::new("0-306-40615-X").normalize(), None);
    assert_eq!(ISBN::new("ISBN: 978-0-306-40615-7").normalize().unwrap(), "9780306406157");
    assert_eq!(ISBN::new("ISBN: 978-0-306-40615-3").normalize(), None);
  }
}
