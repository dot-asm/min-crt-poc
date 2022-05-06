The key component is [`dllmain.rs`](src/dllmain.rs), a trimmed-down MinGW CRT initialization module, that should allow linking `std-<release-id>.dll` without reference to a specific run-time, legacy msvcrt.dll or Universal CRT.

While compiling with `-C prefer-dynamic` might not be that popular, the fact that the `std-<release-id>.dll` can be linked without reference to a specific run-time strongly suggests that the static `libstd-<release-id>.rlib` and Rust-generated code in general don't actually have the dependency either. Or rather it's not strong enough(\*) to prevent the choice of the run-time to be postponed till the application link time. This is the main proposition.

If adopted, the approach would allow to provide run-time-neutral MinGW GCC and (recently proposed) LLVM targets and thus to fold msvcrt and ucrt flavours. Or in more practical terms, after executing `rustup target add <arch>-pc-windows-<non-msvc>` users will be free to choose between msvcrt and ucrt by simply adjusting their `PATH` environment variable(\*\*).

Just in case for reference. While both MinGW GCC and LLVM toolchains use the same C run-times, they are using different exception handling mechanisms. This is what makes it impossible to produce a "universal" non-MSVC Windows target that would work with either toolchain.

The rest of the crate is a test-bed PoC around the `dllmain.rs`. You're more than likely going to have to run `cargo run ...` twice to test. This is because application code gets linked before the library is available. If you know how to arrange the link invocations in specific order, do tell:-) Either way, once confirmed to execute, double-check that `poc.dll` doesn't have imports from your CRT.

Even though the primary target is MinGW, the dllmain.rs and PoC work even with MSVC.

(\*) Rust compiler depends on memcpy/move/set/cmp and strlen being externally available. These are interchangeable between all known run-times.

(\*\*) In the context one can make a case for a cross-check for a matching compiler found on the `PATH`. Most notably MinGW GCC target driver could verify that the library search path in `x86_64-w64-mingw32-gcc -print-search-dirs` output has corresponding exception handling library, `libgcc_eh.a`.
