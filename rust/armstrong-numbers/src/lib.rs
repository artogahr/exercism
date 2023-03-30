pub fn is_armstrong_number(num: u64) -> bool {
    let mut sum = 0;
    let mut temp_num = num;
    let num_count: usize = num.to_string().len();
    loop {
        let rem = temp_num % 10;
        temp_num = temp_num / 10;
        sum += rem.pow(num_count as u32);
        if temp_num == 0 {break};
    }

    if sum == num {return true} else {return false}
}