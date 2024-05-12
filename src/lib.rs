pub fn is_armstrong_number(mut number: u32) -> bool {
    let original_number = number;
    let mut sum: u64 = 0;
    let num_digits = (number as f64).log10() as u32 + 1;

    while number != 0 {
        let digit = number % 10;
        sum += u64::pow(digit as u64, num_digits);
        if sum > u64::from(original_number) {
            return false; // Overflow occurred
        }
        number /= 10;
    }

    sum == u64::from(original_number)
}