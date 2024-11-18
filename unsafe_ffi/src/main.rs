use std::ops::Add;

#[no_mangle]
pub extern "C" fn call_form_c() {
    println!("rust form c");
}

static mut HELLO_WORD: &str = "shhh";

struct Millimeters(u32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, rhs: Millimeters) -> Millimeters {
        Millimeters(self.0 + rhs.0)
    }
}
 
fn main() {
    unsafe {
        HELLO_WORD = "2";
        println!("Hello, world!{}", HELLO_WORD);
        let m = Millimeters(1) + Millimeters(2);
        let s: &str = "hello";
    }
}
