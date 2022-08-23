use std::cmp::Ordering;

fn main() {
    unsafe trait UnsafeOrd {
        fn cmp(&self, other: &Self) -> Ordering;
    }
}
