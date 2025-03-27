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
  match clean_string.len() {
    10 => checkdigit_ten(&clean_string) == clean_string.chars().rev().next().unwrap(),
    13 => checkdigit_thirteen(&clean_string) == clean_string.chars().rev().next().unwrap(),
    _ => false
  }
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

fn from_digit_to_checkdigit(num: u32) -> char {
    let orig_num = char::from_digit((11_u32 - num) % 11, 11).unwrap();
    if orig_num == 'a' {
        'X'
    } else {
        orig_num
    }
}

fn checkdigit_thirteen(isbn: &str) -> char {
  let clean_string = isbn.replace("-", "");
  let first_twelve = clean_string.chars().take(12);
  let first_twelve_digits = first_twelve.filter_map(|x| x.to_digit(10));
  let multiplied = first_twelve_digits.enumerate().map(|(index, digit)| digit * (1 + (index as u32 % 2) * 2 ));

  let summed: u32 = multiplied.sum();
  let modulus = summed % 10;
  char::from_digit(modulus, 10).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_calculates_the_checkdigit() {
    assert_eq!(checkdigit("0139381430").unwrap(), '0');
    assert_eq!(checkdigit("0-8044-2957-X").unwrap(), 'X');
    assert_eq!(checkdigit("9781449373320").unwrap(), '0');
  }

  #[test]
  fn it_checks_the_validity() {
    assert_eq!(valid("0139381430"), true);
    assert_eq!(valid("9781449373320"), true);
    assert_eq!(valid("0-8044-2957-X"), true);
  }

  #[test]
  fn it_catches_invalid() {
    assert_eq!(valid("01393814300"), false);
    assert_eq!(valid("0139381432"), false);
    assert_eq!(valid("9781449373322"), false);
    // assert_eq!(valid("A0139381430"), false);
  }
}
