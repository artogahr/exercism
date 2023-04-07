pub fn factors(mut n: u64) -> Vec<u64> {
    let mut veccy: Vec<u64> = vec![];
    loop {
        for i in 2..n+1 {
            if n%i == 0 {
                veccy.push(i);
                n /= i;
                break;
            }
        }
        if n == 1 { break; }
    }
    veccy
}
