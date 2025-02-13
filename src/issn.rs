/// ```
/// use library_stdnums::issn::checkdigit;
/// assert_eq!(checkdigit("0378-5955"), 5);
/// ```
pub fn checkdigit(issn: &str) -> u32 {
    let clean_string = issn.replace("-", "");
    let first_seven = clean_string.chars().take(7);
    let first_seven_digits = first_seven.filter_map(|x| x.to_digit(10));
    let multiplied = first_seven_digits.enumerate().map(|(index, digit)| digit * (8 - index as u32));

    let summed: u32 = multiplied.sum();
    let modulus: u32 = summed % 11;
    return 11 as u32 - modulus;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_calculates_the_checkdigit() {
        assert_eq!(checkdigit("0193-4511"), 1);
    }
}
