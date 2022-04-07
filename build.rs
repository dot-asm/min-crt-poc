fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_err() {
        panic!("this is a Windows-only crate");
    }
    println!("cargo:rustc-cdylib-link-arg=-nostdlib");
    println!("cargo:rustc-cdylib-link-arg=-Wl,--entry=DllMain");
    println!("cargo:rustc-link-arg-bins=-lpoc");
}
