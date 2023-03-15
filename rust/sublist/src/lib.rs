#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}
pub fn sublist<T: Eq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (lena, lenb) if lena > lenb => {
            if a.windows(b.len()).any(|slice| slice == b) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (lena, lenb) if lena < lenb => {
            if b.windows(a.len()).any(|slice| slice == a) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if a == b {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
