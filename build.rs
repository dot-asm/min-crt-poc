fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_err()
        || std::env::var("CARGO_CFG_TARGET_ENV").unwrap().eq("msvc")
    {
        panic!("this is a MinGW-only crate");
    }
    println!("cargo:rustc-cdylib-link-arg=-nostdlib");
    println!("cargo:rustc-cdylib-link-arg=-Wl,--entry=DllMain");
    println!("cargo:rustc-link-arg-bins=-lpoc");
}
