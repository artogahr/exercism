use std::cmp::Ordering;
pub fn find<R: AsRef<[T]>, T: Ord>(mut array: R, key: T) -> Option<usize> {
    let mut cut_elements = 0;
    let mut array = array.as_ref();
    loop {
        if array.len() == 0 {
            return None;
        }
        let index = array.len() / 2;
        match array[index].cmp(&key) {
            Ordering::Equal => {
                return Some(index + cut_elements);
            }
            Ordering::Greater => {
                (array, _) = array.split_at(index);
            }
            Ordering::Less => {
                (_, array) = array.split_at(index + 1);
                cut_elements += index + 1;
            }
        }
    }
}
