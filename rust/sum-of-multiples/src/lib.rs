pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_factors: Vec<u32> = Vec::new();
    for &factor in factors {
        for i in 1.. {
            if factor * i < limit && factor > 0 {
                if !unique_factors.contains(&(factor*i)) {
                    unique_factors.push(factor*i);
                }
            } else {
                break;
            }
        }
    }
    unique_factors.iter().sum()
}
