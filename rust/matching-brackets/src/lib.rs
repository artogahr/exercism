pub fn brackets_are_balanced(string: &str) -> bool {
    enum Encloser {
        Brackets,
        Braces,
        Parantheses,
    }
    let mut push_stack: Vec<Encloser> = Vec::new();

    for c in string.chars() {
        match c {
            '(' => {
                push_stack.push(Encloser::Parantheses);
            }
            '[' => {
                push_stack.push(Encloser::Brackets);
            }
            '{' => {
                push_stack.push(Encloser::Braces);
            }
            ')' => match push_stack.last() {
                Some(encloser) => {
                    if matches!(encloser, Encloser::Parantheses) {
                        push_stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            },
            ']' => match push_stack.last() {
                Some(encloser) => {
                    if matches!(encloser, Encloser::Brackets) {
                        push_stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            },
            '}' => match push_stack.last() {
                Some(encloser) => {
                    if matches!(encloser, Encloser::Braces) {
                        push_stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            },
            _ => continue,
        }
    }
    push_stack.is_empty()
}
