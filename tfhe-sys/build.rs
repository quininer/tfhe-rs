extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header("tfhe/src/include/tfhe.h")
        .ctypes_prefix("::libc")
        .generate().unwrap()
        .write_to_file(outdir.join("ffi.rs")).unwrap();

    let dst = cmake::build("tfhe/src");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=tfhe-nayuki-avx");
    println!("cargo:rustc-link-lib=stdc++");
}
