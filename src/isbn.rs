pub fn checkdigit(isbn: &str) -> Option<char> {
  let clean_string = isbn.replace("-", "");
  match clean_string.len() {
      10 => Some(checkdigit_ten(isbn)),
      13 => Some(checkdigit_thirteen(isbn)),
      _ => None
  }
}

fn checkdigit_ten(isbn: &str) -> char {
  'a'
}

fn checkdigit_thirteen(isbn: &str) -> char {
  'a'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_calculates_the_checkdigit() {
    assert_eq!(checkdigit("0139381430").unwrap(), '0');
  }
}
