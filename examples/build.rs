//! Puts `memory.x` on the linker search path and selects the `cortex-m-rt`
//! linker script for every example in this package.

use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by cargo"));
    fs::copy("memory.x", out_dir.join("memory.x")).expect("copy memory.x to OUT_DIR");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-arg-examples=--nmagic");
    println!("cargo:rustc-link-arg-examples=-Tlink.x");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
