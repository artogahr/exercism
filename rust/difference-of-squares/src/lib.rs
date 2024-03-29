pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n+1 {
        sum += i.pow(2);
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
