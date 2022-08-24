use std::{cmp::Ordering, ptr};

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
//Note: This definition is naive. See the chapter on imnplementing Vec
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}
impl<T> Vec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            //Not important for this example
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        }
    }
    // pub fn new(mut self) -> Vec<T> {
    //     Vec {
    //         ptr: ,
    //         len: 0,
    //         cap: 0,
    //     }
    // }
    fn make_room(&mut self) {
        //grow the capacity
        self.cap += 1;
    }
}
