/// ```
/// use library_stdnums::issn::checkdigit;
/// assert_eq!(checkdigit("0378-5955"), 5);
/// ```
pub fn checkdigit(issn: &str) -> i8 {
    let clean_issn = issn.replace(char::is_ascii_punctuation, "")[0..6];
    let sum = clean_issn.chars()
        .enumerate()
        .map(|(i, digit)| digit.to_digit(10) * (8 - i))
        .collect<i8>();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_calculates_the_checkdigit() {
        assert_eq!(checkdigit("0193-4511"), 1);
    }
}
