use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exp(b_pub, a, p)
}

pub fn modular_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1_u128;
    let mut e = exponent as u128;
    let mut b = base as u128;
    let m = modulus as u128;
    assert_ne!((m - 1).checked_mul(m - 1), None);
    b = b % m;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        e = e >> 1;
        b = (b * b) % m;
    }
    result as u64 % modulus
}
