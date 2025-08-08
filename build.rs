fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_err() {
        panic!("this is a Windows-only crate");
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap().eq("msvc") {
        println!("cargo:rustc-cdylib-link-arg=ucrt.lib");
        println!("cargo:rustc-cdylib-link-arg=libvcruntime.lib");
        println!("cargo:rustc-cdylib-link-arg=/ENTRY:DllMain");
        println!("cargo:rustc-cdylib-link-arg=/MERGE:.tls=.rdata");
        println!("cargo:rustc-link-arg-bins=poc.dll.lib");
    } else {
        println!("cargo:rustc-cdylib-link-arg=-nostdlib");
        println!("cargo:rustc-cdylib-link-arg=-Wl,--entry=DllMain");
        println!("cargo:rustc-link-arg-bins=-lpoc");
    }
    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        let mut build_dir = std::path::PathBuf::from(out_dir);
        build_dir.pop();
        build_dir.pop();
        build_dir.pop();
        println!("cargo:rustc-link-search=native={}", build_dir.display());
    }
}
