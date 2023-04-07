pub fn square(s: u32) -> u64 {
    match s {
        s if s > 0 && s < 65 => u64::pow(2, s-1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for i in 1..65 {
        sum += square(i);
    }
    sum
}
