use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::ptr;
use libc::proc_kmsgbuf;

#[no_mangle]
pub extern "C" fn say_hello() {
    println!("Hello from rust!")
}

#[repr(C)]
pub struct Point {
    x: i32,
    y: i32
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.x)
    }
}

#[no_mangle]
pub extern "C" fn point_new(x: i32, y: i32) -> *mut Point {
    let b = Box::new(Point {
        x, y
    });
    println!("Point is created, returning... {}", &b);
    Box::into_raw(b)
}

#[no_mangle]
pub extern "C" fn point_free(point: *mut Point) {
    if !point.is_null() {
        let b = unsafe {
            Box::from_raw(point)
        };
        println!("Freeing point {}", &b);
    }
}

#[no_mangle]
pub extern "C" fn point_print(point: *mut Point) -> *mut Point {
    if !point.is_null() {
        let b = unsafe {
            Box::from_raw(point)
        };
        println!("{}", &b);
        Box::into_raw(b)
    }
    ptr::null_mut()
}

#[cfg(test)]
mod test {
    use crate::say_hello;

    #[test]
    fn it_works() {
        say_hello();
    }
}
