pub fn raindrops(n: u32) -> String {
    let mut flag: bool = false;
    let mut result: String = String::new();
    if n % 3 == 0 {
        result.push_str("Pling");
        flag = true;
    }
    if n % 5 == 0 {
        result.push_str("Plang");
        flag = true;
    }
    if n % 7 == 0 {
        result.push_str("Plong");
        flag = true;
    }
    if flag {
        result
    } else {
        n.to_string()
    }
}
