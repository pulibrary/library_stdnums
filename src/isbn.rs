pub fn checkdigit(isbn: &str) -> Option<char> {
  let clean_string = isbn.replace("-", "");
  match clean_string.len() {
      10 => Some(checkdigit_ten(isbn)),
      13 => Some(checkdigit_thirteen(isbn)),
      _ => None
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
}
