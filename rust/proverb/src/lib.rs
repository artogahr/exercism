pub fn build_proverb(list: &[&str]) -> String {
    let mut temp: String = String::new();
    if !list.is_empty() {
        for i in 0..list.len() - 1 {
            temp += &(format!(
                "For want of a {0} the {1} was lost.\n",
                list[i],
                list[i + 1]
            ));
        }
        temp += &(format!("And all for the want of a {0}.", list[0]));
    }
    temp
}
