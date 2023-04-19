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
    let b = base as u128;
    let e = exponent as u128;
    let m = modulus as u128;
    let mut c: u128 = 1;
    let mut e1: u128 = 0;
    while e1 < e {
        e1 += 1;
        c = (b * c) % m;
    }
    c as u64
}