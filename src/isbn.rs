pub fn checkdigit(isbn: &str) -> Option<char> {
  let clean_string = isbn.replace("-", "");
  match clean_string.len() {
      10 => Some(checkdigit_ten(isbn)),
      13 => Some(checkdigit_thirteen(isbn)),
      _ => None
  }
}

pub fn valid(isbn: &str) -> bool {
  let clean_string = isbn.replace("-", "");
  let scrubbed_string = scrub_alpha_prefix(&clean_string);

  match scrubbed_string.len() {
    10 => checkdigit_ten(&scrubbed_string) == scrubbed_string.chars().next_back().unwrap(),
    13 => checkdigit_thirteen(&scrubbed_string) == scrubbed_string.chars().next_back().unwrap(),
    _ => false
  }
}

pub fn convert_to_13(isbn: &str) -> Option<String> {
  if !valid(isbn) {return None};
  let clean_string = isbn.replace("-", "");
  let scrubbed_string = scrub_alpha_prefix(&clean_string);
  let prepended_string = format!("{}{}", "978", &scrubbed_string[..9]);
  match scrubbed_string.len() {
    10 => Some(format!("{}{}", prepended_string, checkdigit_thirteen(&prepended_string)).to_string()),
    13 => Some(scrubbed_string.to_string()),
    _ => None,
  }
  
}

pub fn convert_to_10(isbn: &str) -> Option<String> {
  if !valid(isbn) {return None};
  let clean_string = isbn.replace("-", "");
  let scrubbed_string = scrub_alpha_prefix(&clean_string);
  if scrubbed_string.starts_with("979") {
    return None;
  }
  match scrubbed_string.len() {
    10 => Some(scrubbed_string),
    13 => Some(format!("{}{}", &scrubbed_string[3..12], checkdigit_ten(&scrubbed_string[3..]))),
    _ => None,
  }
}

pub fn normalize(isbn: &str) -> Option<String> {
  convert_to_13(isbn)
}

fn scrub_alpha_prefix(string_to_scrub: &str) -> String {
  string_to_scrub.chars()
    .skip_while(|c| !c.is_ascii_digit())
    .take_while(|c| c.is_ascii_digit() || c == &'X')
    .collect::<String>()
}

fn checkdigit_ten(isbn: &str) -> char {
  let clean_string = isbn.replace("-", "");
  let first_nine = clean_string.chars().take(9);
  let first_nine_digits = first_nine.filter_map(|x| x.to_digit(10));
  let multiplied = first_nine_digits.enumerate().map(|(index, digit)| digit * (10 - index as u32));

  let summed: u32 = multiplied.sum();
  let modulus: u32 = summed % 11;
  from_digit_to_checkdigit(modulus)
}

fn checkdigit_thirteen(isbn: &str) -> char {
  let clean_string = isbn.replace("-", "");
  let first_twelve = clean_string.chars().take(12);
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
  fn it_calculates_the_checkdigit() {
    assert_eq!(checkdigit("0139381430").unwrap(), '0');
    assert_eq!(checkdigit("0-8044-2957-X").unwrap(), 'X');
    assert_eq!(checkdigit("9781449373320").unwrap(), '0');
    assert_eq!(checkdigit("9780306406152").unwrap(), '7')
  }

  #[test]
  fn it_checks_the_validity() {
    assert!(valid("0139381430"));
    assert!(valid("9781449373320"));
    assert!(valid("0-8044-2957-X"));
    assert!(valid("ABC0139381430"));
  }

  #[test]
  fn it_catches_invalid() {
    assert!(!valid("01393814300"));
    assert!(!valid("0139381432"));
    assert!(!valid("9781449373322"));
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
    assert_eq!(normalize("0-306-40615-2").unwrap(), "9780306406157");
    assert_eq!(normalize("0-306-40615-X"), None);
    assert_eq!(normalize("ISBN: 978-0-306-40615-7").unwrap(), "9780306406157");
    assert_eq!(normalize("ISBN: 978-0-306-40615-3"), None);
  }
}
