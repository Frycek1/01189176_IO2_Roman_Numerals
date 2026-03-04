#[derive(Debug, PartialEq)]
enum RomanNumeralsErr {
    TooBig,
    TooSmall,
}

#[allow(dead_code)]
fn convert(mut num: i64) -> Result<String, RomanNumeralsErr>{
    if num >= 4000 {
        return Err(RomanNumeralsErr::TooBig);
    }
    else if num <= 0 {
        return Err(RomanNumeralsErr::TooSmall);
    }
    let numerals = [
        ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
        ["X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
        ["C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
        ["M", "MM", "MMM", "", "", "", "", "", ""],
    ];

    let mut result = String::new();

    let mut i = 0;
    while num > 0 {
        let digit = (num % 10) as usize;
        if digit > 0 {
            let roman_part = numerals[i][digit - 1];
            result.insert_str(0, roman_part);
        }
        num /= 10;
        i += 1;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::{convert, RomanNumeralsErr};
    #[test]
    fn should_success_simple() {
        assert_eq!(Ok(String::from("I")), convert(1));
        assert_eq!(Ok(String::from("X")), convert(10));
        assert_eq!(Ok(String::from("C")), convert(100));
        assert_eq!(Ok(String::from("M")), convert(1000));
    }
    #[test]
    fn should_success_mixed() {
        let test_cases = [
        (1, "I"),
        (7, "VII"),
        (10, "X"),
        (42, "XLII"),
        (100, "C"),
        (888, "DCCCLXXXVIII"),
        (1000, "M"),
        (3917, "MMMCMXVII"),
        ];
        for (num, expected) in test_cases {
            assert_eq!(Ok(String::from(expected)), convert(num),);
        }
    }
    
    

    #[test]
    fn should_fail_too_small() {
        assert_eq!(Err(RomanNumeralsErr::TooSmall), convert(0));
    }
    
    #[test]
    fn should_fail_too_big() {
        assert_eq!(Err(RomanNumeralsErr::TooBig), convert(4000));
    }
}