pub fn egg_count(display_value: u32) -> usize {
    let mut temp = display_value;
    let mut count = 0;
    for nth_power in (0..32).rev() {
        let divisor = (2 as u32).pow(nth_power);
        if temp / divisor == 1 {
            temp = temp - divisor;
            count += 1;
        }
    }
    count
}
