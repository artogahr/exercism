pub fn nth(mut n: u32) -> u32 {
    n += 1;
    let mut i = 1;
    while n > 0 {
        i += 1;
        if is_prime(i) { n -= 1}
    }
    i
}

pub fn is_prime(n: u32) -> bool {
    for i in 2..(n/2)+1 {
        if n % i == 0 { return false } 
    }
    true
}
