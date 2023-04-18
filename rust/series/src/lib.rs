pub fn series(digits: &str, len: usize) -> Vec<String> {
    let leni32: i32 = len as i32;
    let mut strings: Vec<String> = Vec::new();
    for i in 0..(digits.len() as i32) - leni32.checked_sub(1).unwrap_or(-1) {
        strings.push(digits[(i as usize)..(i as usize + len)].to_string());
    }
    strings
}
