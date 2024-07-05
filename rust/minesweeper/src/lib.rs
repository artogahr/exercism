pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut m_vec: Vec<Vec<u8>> = minefield.iter().map(|&s| s.as_bytes().to_vec()).collect();
    for i in 0..minefield.len() {
        for j in 0..minefield[i].len() {
            for x in 0..=2 {
                for y in 0..=2 {
                    if (i == 0 && x == 0)
                        || (j == 0 && y == 0)
                        || (x + i > minefield.len())
                        || (y + j > minefield[i].len())
                    {
                        continue;
                    }
                    if minefield[i + x - 1].as_bytes()[j + y - 1] == 42 {
                        m_vec[i][j] = increase_by_one(m_vec[i][j]);
                    }
                }
            }
        }
    }
    m_vec
        .iter()
        .map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
        .collect()
}

fn increase_by_one(byte: u8) -> u8 {
    // 49 -> '0' as byte
    // 32 -> ' ' as byte
    // 42 -> '*' as byte
    match byte {
        49..=59 => byte + 1,
        32 => 49,
        _ => byte,
    }
}
