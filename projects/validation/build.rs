use std::env;

fn main() {
    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").expect("RUSTUP_TOOLCHAIN not found");
    if rust_toolchain.starts_with("stable") {
    }
    else if rust_toolchain.starts_with("nightly") {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }
    else {
        panic!("Unexpected value for rustc toolchain")
    }
}
