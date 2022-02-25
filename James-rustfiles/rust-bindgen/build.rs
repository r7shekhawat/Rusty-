extern crate bindgen;

use std::env;
use std::path::PathBuf;
fn main(){

    let bindings = bindgen::Builder::default()
        .header("/home/rootbutcher2/PROJECTS/Halide/include/HalideRuntime.h")
        //.clang_arg("-I/home/rootbutcher2/PROJECTS/Halide/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("error1");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("binding1.rs"))
        .expect("couldn't write bindings!");
}