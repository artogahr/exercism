#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    let decimal_number = base_to_decimal(number, from_base)?;
    decimal_to_base(decimal_number, to_base)
}

fn base_to_decimal(number: &[u32], base: u32) -> Result<u32, Error> {
    number.iter().try_fold(0u32, |acc, &digit| {
        if digit >= base {
            return Err(Error::InvalidDigit(digit));
        }
        Ok(acc * base + digit)
    })
}
fn decimal_to_base(number: u32, base: u32) -> Result<Vec<u32>, Error> {
    let mut number = number;
    if number == 0 {
        return Ok(vec![0]);
    }
    let mut result = Vec::new();
    while number > 0 {
        result.push(number % base);
        number /= base;
    }
    Ok(result.into_iter().rev().collect())
}
