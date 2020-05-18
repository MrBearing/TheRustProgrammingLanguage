use std::slice;

pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // let address = 0x012345usize;
    // println!("try get address 0x012345 s value");
    // let r = address as *const i32;
    // println!("put in pointer");
    // unsafe {
    //     println!("r is: {}", *r); // segfault が発生
    // }

    unsafe{
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


}
