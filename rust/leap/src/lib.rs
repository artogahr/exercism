pub fn is_leap_year(year: u64) -> bool {
    if is_div_4(year) {
        if is_div_100(year) {
            if is_div_400(year) {
                return true
            }
            return false
        }
        return true
    }
    false
}

pub fn is_div_100(num: u64) -> bool
{
    num % 100 == 0 
}

pub fn is_div_4(num: u64) -> bool
{
    num % 4 == 0 
}

pub fn is_div_400(num: u64) -> bool
{
    num % 400 == 0 
}

