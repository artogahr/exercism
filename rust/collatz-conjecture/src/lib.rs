pub fn collatz(mut n: u64) -> Option<u64> {
    let mut count: u64 = 0;
    if n == 0 { return None }
    while n > 1 {
        count += 1;
        if let 0=n%2 { 
            n /= 2;
            continue;
        }
        else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
    }
    Some(count)
}
