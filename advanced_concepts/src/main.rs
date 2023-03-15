fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);
    unsafe {
        println!("r1 is: {:?}", *r1);
        println!("r2 is: {:?}", *r2);
        let addres = 0x012345usize;
        let r3 = addres as *const i32;
        // println!("r3 is: {:?}", *r3); SEGFAULT
    }
    unsafe {
        dangerous();
        println!("Abs of -3: {}", abs(-3));
    }
}

//unsafe functions

unsafe fn dangerous() {
    println!("DANGEROUS");
}

extern "C" {
    fn abs(input: i32) -> i32;
}
