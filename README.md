The key component is [`dllmain.rs`](src/dllmain.rs), a trimmed-down MinGW CRT initialization module, that should allow linking `std-<release-id>.dll` without reference to a specific run-time, legacy msvcrt.dll or Universal CRT. So that the choice of the run-time can be left to any given application.

Even though the primary target is MinGW, it works even with MSVC.

The rest of the crate is a test-bed PoC around the `dllmain.rs`. You're more than likely going to have to run `cargo run ...` twice to test. This is because application code gets linked before the library is available. If you know how to arrange the link invocations in specific order, do tell:-) Either way, once confirmed to execute, double-check that `poc.dll` doesn't have imports from your CRT.
