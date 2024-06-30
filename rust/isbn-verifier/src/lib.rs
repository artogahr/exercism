/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn
        .to_string()
        .chars()
        .filter(|c| c.is_numeric() || *c == 'X')
        .collect();
    if isbn.len() != 10 {
        return false;
    }
    if isbn.chars().take(9).any(|num| num == 'X') {
        return false;
    }
    let sum: u32 = isbn
        .chars()
        .enumerate()
        .map(|(i, item)| {
            if item == 'X' {
                10 * (10 - i) as u32
            } else {
                item.to_digit(10).unwrap() * (10 - i) as u32
            }
        })
        .sum();
    sum % 11 == 0
}
