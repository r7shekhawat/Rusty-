#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "/binding1.rs"));

fn main(){
    println!("hey");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn memalloc(){
        use crate::halide_malloc;
        let raw_ptr: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        unsafe {
            halide_malloc(raw_ptr, 64);
        }
    }
}
