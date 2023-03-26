pub fn nth(mut n: u32) -> u32 {
    if n == 0 { return 2 }
    let mut i = 1;
    while n > 0 {
        i += 2;
        if is_prime(i) { n -= 1}
    }
    i
}

pub fn is_prime(n: u32) -> bool {
    for i in 2..(n as f32).sqrt() as u32 + 1 {
        if n % i == 0 { return false } 
    }
    true
}
