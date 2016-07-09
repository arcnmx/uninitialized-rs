use std::env::var_os;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if var_os("UNSAFE_UNINITIALIZED").is_some() {
        println!("cargo:rustc-cfg=feature=\"uninitialized\"");
    }
}
