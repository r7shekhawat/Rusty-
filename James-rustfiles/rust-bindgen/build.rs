//extern crate bindgen;


//use std::env;
//use std::path::PathBuf;
fn main(){

    //let bindings = bindgen::Builder::default()
    //    .header("/home/rootbutcher2/PROJECTS/Halide/include/HalideRuntime.h")
    //    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //    .generate()
    //    .expect("error1");

    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    //bindings
    //    .write_to_file(out_path.join("HalideRuntime.rs"))
    //    .expect("couldn't write bindings!");

    println!("cargo:rustc-link-search=/home/rootbutcher2/PROJECTS/Halide/bin/host");
    //println!("cargo:rustc-link-lib=runtime");
}
//home/rootbutcher2/PROJECTS/Halide/bin/host/libruntime.a
//home/rootbutcher2/PROJECTS/Halide/bin/libHalide.so
//home/rootbutcher2/PROJECTS/Halide/lib/libHalide.a
//home/rootbutcher2/PROJECTS/Halide/bin/host/libruntime.a