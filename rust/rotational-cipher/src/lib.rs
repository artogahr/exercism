use std::u32;

pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() && c.is_alphabetic() {
                char::from_u32((c as u32 - 65 + key as u32) % 26 + 65).unwrap()
            } else if c.is_lowercase() && c.is_alphabetic() {
                char::from_u32((c as u32 + key as u32 - 97) % 26 + 97).unwrap()
            } else {
                c
            }
        })
        .collect()
}
