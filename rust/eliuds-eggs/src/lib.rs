pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .rev()
        .filter(|&nth_power| {
            let divisor = 2u32.pow(nth_power);
            display_value / divisor % 2 == 1
        })
        .count()
}
