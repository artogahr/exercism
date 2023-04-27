/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code: String = code.chars().rev().collect();
    code.retain(|c| !c.is_whitespace());
    // Now code contains stripped out number
    if code.len() <= 1 || code.chars().any(|c| !c.is_ascii_digit()){
        return false;
    }
    let numbers: Vec<u32> = code
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 != 0 {
                if c.to_digit(10).unwrap() < 5 {
                    (c.to_digit(10).unwrap() * 2) as u32
                } else {
                    ((c.to_digit(10).unwrap() * 2) - 9) as u32
                }
            } else {
                c.to_digit(10).unwrap() as u32
            }
        })
        .collect();

    if numbers.iter().sum::<u32>() % 10 == 0 {
        true
    } else {
        false
    }
}
