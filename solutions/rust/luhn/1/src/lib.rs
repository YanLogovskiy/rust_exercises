pub fn is_valid(code: &str) -> bool {
    let stripped: String = code.replace(" ", "");
    if stripped.len() <= 1 {
        return false;
    }
    if !stripped.chars().all(|c: char| c.is_ascii_digit()) {
        return false;
    }
    let digits: Vec<u32> = stripped.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut sum: u32 = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        if i % 2 == 1 {
            let doubled: u32 = *digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += *digit;
        }
    }
    sum.is_multiple_of(10)
}