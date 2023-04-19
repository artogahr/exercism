pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let mut plants: Vec<&'static str> = Vec::new();
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let lines = _diagram.split('\n');

    let index = students.iter().position(|&r| r == _student).unwrap();
    let mut cut_str: String = String::new();

    for line in lines {
        cut_str = cut_str
            + &line
                .to_string()
                .chars()
                .nth(index * 2)
                .unwrap_or('.')
                .to_string()
            + &line
                .to_string()
                .chars()
                .nth(index * 2 + 1)
                .unwrap_or('.')
                .to_string()
    }

    for letter in cut_str.chars() {
        match letter {
            'V' => plants.push("violets"),
            'R' => plants.push("radishes"),
            'C' => plants.push("clover"),
            'G' => plants.push("grass"),
            _ => {
                plants.push("wrong");
            }
        }
    }
    plants
}
