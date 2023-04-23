pub fn number(user_number: &str) -> Option<String> {
    let sanitized = sanitize_input(user_number);
    normalize_input(sanitized)
}

fn normalize_input(user_number: String) -> Option<String> {
    match user_number.len() {
        11 => {
            if user_number.chars().nth(0) != Some('1') {
                return None;
            } else {
                if user_number.chars().nth(1) == Some('1')
                    || user_number.chars().nth(4) == Some('1')
                    || user_number.chars().nth(1) == Some('0')
                    || user_number.chars().nth(4) == Some('0')
                {
                    return None;
                }
                return Some(user_number[1..].to_string());
            }
        }
        10 => {
                if user_number.chars().nth(0) == Some('1')
                    || user_number.chars().nth(3) == Some('1')
                    || user_number.chars().nth(0) == Some('0')
                    || user_number.chars().nth(3) == Some('0')
                {
                    return None;
                }
            return Some(user_number)
            
        },
        _ => return None,
    }
}

fn sanitize_input(user_number: &str) -> String {
    user_number
        .chars()
        .into_iter()
        .map(|ch| match ch {
            ch if ch.is_numeric() => ch,
            _ => '-',
        })
        .collect::<String>()
        .replace("-", "")
}
