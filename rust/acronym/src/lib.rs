pub fn abbreviate(phrase: &str) -> String {
    let filtered: String = phrase
        .chars()
        .filter(|ch| ch.is_alphabetic() || *ch == '-' || *ch == ' ')
        .collect();
    filtered
        .char_indices()
        .filter_map(|(i, ch)| {
            if i == 0 {
                Some(ch.to_ascii_uppercase())
            } else {
                let prev = filtered.chars().nth(i - 1).unwrap();
                match (prev, ch) {
                    (' ' | '-', ch) if ch.is_alphabetic() => Some(ch.to_ascii_uppercase()),
                    (pr, ch) if pr.is_lowercase() && ch.is_uppercase() => Some(ch),
                    _ => None,
                }
            }
        })
        .collect()
}
