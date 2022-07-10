mod min_crt_init {
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]

    use core::ffi::c_void;
    use core::option::Option;

    type CallBack = Option<unsafe extern "system" fn(*const c_void, u32, *const c_void)>;
    type CrtInit = Option<unsafe extern "C" fn()>;

    #[repr(C)]
    struct IMAGE_TLS_DIRECTORY {
        StartAddressOfRawData: *const c_void,
        EndAddressOfRawData: *const c_void,
        AddressOfIndex: *const u32,
        AddressOfCallBacks: *const CallBack,
        SizeOfZeroFill: u32,
        Characteristics: u32,
    }
    unsafe impl Sync for IMAGE_TLS_DIRECTORY {}

    unsafe extern "system" fn nop(_: *const c_void, _: u32, _: *const c_void) {}

    #[used]
    #[link_section = ".CRT$XLA"]
    static __xl_a: CallBack = Some(nop);
    #[used]
    #[link_section = ".CRT$XLZ"]
    static __xl_z: CallBack = None;

    #[link_section = ".tls"]
    static _tls_start: u64 = 0;
    #[link_section = ".tls$ZZZ"]
    static _tls_end: u64 = 0;

    #[no_mangle]
    static mut _tls_index: u32 = 0;

    #[no_mangle]
    static _tls_used: IMAGE_TLS_DIRECTORY = IMAGE_TLS_DIRECTORY {
        StartAddressOfRawData: unsafe { core::mem::transmute(&_tls_start) },
        EndAddressOfRawData: unsafe { core::mem::transmute(&_tls_end) },
        AddressOfIndex: unsafe { &_tls_index },
        AddressOfCallBacks: &__xl_a,
        SizeOfZeroFill: 0,
        Characteristics: 0,
    };

    #[link_section = ".CRT$XCA"]
    static __xc_a: CrtInit = None;
    #[link_section = ".CRT$XCZ"]
    static __xc_z: CrtInit = None;

    #[no_mangle]
    unsafe extern "system" fn DllMain(h: *const c_void, reason: u32, _: *const c_void) -> i32 {
        const DLL_PROCESS_ATTACH: u32 = 1;
        extern "system" { fn DisableThreadLibraryCalls(h: *const c_void); }
        if reason == DLL_PROCESS_ATTACH {
            DisableThreadLibraryCalls(h);
            let mut ptr = &__xc_a as *const CrtInit;
            while ptr < &__xc_z {
                if let Some(f) = *ptr {
                    f();
                }
                ptr = ptr.add(1);
            }
        }
        1
    }

    #[no_mangle]
    extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        for i in 0..n {
            unsafe { *dst.add(i) = *src.add(i) };
        }
        dst
    }

    #[no_mangle]
    extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        if src >= unsafe { dst.add(n) } || unsafe { src.add(n) } <= dst {
            for i in 0..n {
                unsafe { *dst.add(i) = *src.add(i) };
            }
        } else if src > dst {
            for i in 0..n {
                unsafe { dst.add(i).write_volatile(src.add(i).read_volatile()) };
            }
        } else if src != dst {
            for i in 1..n + 1 {
                unsafe { dst.add(n - i).write_volatile(src.add(n - i).read_volatile()) };
            }
        }
        dst
    }

    #[no_mangle]
    extern "C" fn memset(dst: *mut u8, c: i32, n: usize) -> *mut u8 {
        for i in 0..n {
            unsafe { *dst.add(i) = c as u8 };
        }
        dst
    }

    #[no_mangle]
    extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
        for i in 0..n {
            let ret = unsafe { (*s1.add(i) as i32) - (*s2.add(i) as i32) };
            if ret != 0 {
                return ret;
            }
        }
        0
    }

    #[no_mangle]
    extern "C" fn strlen(s: *const u8) -> usize {
        let mut len = 0usize;
        while unsafe { *s.add(len) } != 0u8 {
            len += 1;
        }
        len
    }
}
