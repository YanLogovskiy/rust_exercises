pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let num_digits = num_string.len();
    let mut sum: u32 = 0;
    for digit in num_string.chars() {
        let digit_value = digit.to_digit(10).unwrap();
        let digit_power = digit_value.pow(num_digits as u32);
        sum += digit_power;
    }
    sum == num
}