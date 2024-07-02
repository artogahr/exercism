use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut found_words: HashSet<&'a str> = HashSet::new();
    let word_lowercase: Vec<String> = word.chars().map(|c| c.to_lowercase().to_string()).collect();
    let mut word_sorted = word_lowercase.clone();
    word_sorted.sort();

    for candidate in possible_anagrams {
        let candidate_lowercase: Vec<String> = candidate
            .chars()
            .map(|c| c.to_lowercase().to_string())
            .collect();
        if candidate_lowercase == word_lowercase {
            continue;
        }
        let mut candidate_sorted = candidate_lowercase;
        candidate_sorted.sort();
        if word_sorted == candidate_sorted {
            found_words.insert(candidate);
            continue;
        }
    }
    found_words
}
