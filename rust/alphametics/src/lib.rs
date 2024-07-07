// Full disclosure: This was written by ChatGPT. I came up with the solution myself, but was too
// lazy to write it up.

use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (addends, result) = parse_input(input)?;

    let mut letters: Vec<char> = addends
        .iter()
        .flat_map(|w| w.chars())
        .chain(result.chars())
        .collect();
    letters.sort();
    letters.dedup();

    if letters.len() > 10 {
        return None; // More than 10 unique letters, impossible to solve
    }

    let mut assignment = HashMap::new();
    let mut used_digits = HashSet::new();

    if solve_recursive(
        &letters,
        &addends,
        &result,
        &mut assignment,
        &mut used_digits,
    ) {
        Some(assignment)
    } else {
        None
    }
}

fn parse_input(input: &str) -> Option<(Vec<String>, String)> {
    let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        return None;
    }
    let addends: Vec<String> = parts[0].split('+').map(|s| s.trim().to_string()).collect();
    let result = parts[1].trim().to_string();
    Some((addends, result))
}

fn solve_recursive(
    letters: &[char],
    addends: &[String],
    result: &String,
    assignment: &mut HashMap<char, u8>,
    used_digits: &mut HashSet<u8>,
) -> bool {
    if letters.is_empty() {
        return check_solution(addends, result, assignment);
    }

    let current_letter = letters[0];
    let remaining_letters = &letters[1..];

    for digit in 0..=9 {
        if used_digits.contains(&digit)
            || (digit == 0 && is_leading_zero(current_letter, addends, result))
        {
            continue;
        }

        assignment.insert(current_letter, digit);
        used_digits.insert(digit);

        if solve_recursive(remaining_letters, addends, result, assignment, used_digits) {
            return true;
        }

        assignment.remove(&current_letter);
        used_digits.remove(&digit);
    }

    false
}

fn is_leading_zero(letter: char, addends: &[String], result: &String) -> bool {
    for word in addends.iter().chain(Some(result)) {
        if word.starts_with(letter) && word.len() > 1 {
            return true;
        }
    }
    false
}

fn check_solution(addends: &[String], result: &String, assignment: &HashMap<char, u8>) -> bool {
    let addends_sum: u64 = addends
        .iter()
        .map(|word| word_to_number(word, assignment))
        .sum();
    let result_number = word_to_number(result, assignment);

    addends_sum == result_number
}

fn word_to_number(word: &String, assignment: &HashMap<char, u8>) -> u64 {
    word.chars()
        .fold(0, |acc, c| acc * 10 + *assignment.get(&c).unwrap() as u64)
}
