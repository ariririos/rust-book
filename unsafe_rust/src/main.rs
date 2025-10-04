use std::slice;

static mut HELLO_WORLD: &str = "Hello world";
static mut COUNTER: u32 = 0;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}


// SAFETY: only call from one thread at a time to prevent data races
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;
    unsafe {
        println!("*r1: {}", *r1);
        println!("*r2: {}", *r2);
    }

    // let address = 0x000000usize;
    // let r = address as *const i32;
    // unsafe {
    //     println!("*???: {}", *r);
    // }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // let address = 0x01234usize;
    // let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    // println!("values: {values:?}");

    unsafe extern "C" {
        fn absb(input: i32) -> i32;
    }

    unsafe {
        println!("abs(-34) according to C: {}", absb(-34));
    }

    unsafe {
        println!("HELLO_WORLD: {}", *(&raw const HELLO_WORLD));
    }

    unsafe {
        // SAFETY: non-thread safe function is only called from a single thread in `main`
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

