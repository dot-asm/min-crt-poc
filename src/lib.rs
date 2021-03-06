include!("dllmain.rs");

mod selftest {
    #[used]
    #[link_section = ".CRT$XCU"]
    static INIT_REG: unsafe extern "C" fn() = init;

    static mut MAGIC: u32 = 0;
    unsafe extern "C" fn init() {
        MAGIC = if MAGIC == 0 { 42 } else { MAGIC / 2 };
    }

    #[no_mangle]
    pub extern "C" fn poc() -> u32 {
        rayon::scope(|s| s.spawn(|_| println!("hello from rayon::scope")));
        unsafe { MAGIC }
    }
}
