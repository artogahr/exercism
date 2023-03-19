pub fn reply(message: &str) -> &str {
    match message {
        str if str.trim().is_empty() => "Fine. Be that way!",
        str if str.trim().ends_with('?') && str.to_uppercase() == str && str.chars().any(|c| c.is_ascii_alphabetic()) => "Calm down, I know what I'm doing!",
        str if str.trim().ends_with('?') => "Sure.",
        str if str.to_uppercase() == str && str.chars().any(|c| c.is_ascii_alphabetic()) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
