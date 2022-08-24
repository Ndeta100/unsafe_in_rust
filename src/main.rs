use std::cmp::Ordering;

fn main() {
    unsafe trait UnsafeOrd {
        fn cmp(&self, other: &Self) -> Ordering;
    }
    let array: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let idx = 7;
    let results = index(idx, &array);
    println!("We have {:?} at index {}", results, idx);
}
fn index(index: usize, array: &[u8]) -> Option<u8> {
    if index <= array.len() {
        unsafe { Some(*array.get_unchecked(index)) }
    } else {
        None
    }
}
