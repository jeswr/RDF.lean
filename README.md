# RustFFI.lean

```bash
$ lake clean; cargo clean; rm -rf ./.lake/; rm -rf build/; rm -rf target/; rm -rf lake-manifest.json; rm -rf Cargo.lock; lake exe ffi
     Removed 277 files, 198.2MiB total
info: stderr:
    Updating crates.io index
   Compiling autocfg v1.1.0
   Compiling libc v0.2.153
   Compiling parking_lot_core v0.9.9
   Compiling cfg-if v1.0.0
   Compiling smallvec v1.13.1
   Compiling scopeguard v1.2.0
   Compiling lean-sys v0.0.5
   Compiling static_assertions v1.1.0
   Compiling lock_api v0.4.11
   Compiling memoffset v0.9.0
   Compiling parking_lot v0.12.1
   Compiling some_rust_lib v0.1.0 (/home/jeswr/Documents/GitHub/lurk-lab/RustFFI.lean)
    Finished release [optimized] target(s) in 1.48s
info: [0/2] Linking libsome_rust_lib.so
info: [0/2] Compiling ffi.c
info: [0/2] Creating libffi.a
info: [0/2] Linking libffi.so
error: > /home/jeswr/.elan/toolchains/leanprover--lean4---stable/bin/leanc -shared -o ./.lake/build/libsome_rust_lib.so -Wl,--whole-archive ./.lake/build/libsome_rust_lib.a -Wl,--no-whole-archive
error: stderr:
ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::in_thread_finalization()) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::run_thread_finalizers_core(std::__1::vector<std::__1::pair<void (*)(void*), void*>, std::__1::allocator<std::__1::pair<void (*)(void*), void*>>>&)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::run_thread_finalizers()) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::run_post_thread_finalizers()) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::lthread::imp::_main(void*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::lthread::imp::_main(void*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::thread_finalizers_manager::finalize_thread(void*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_finalizing cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(thread.cpp.o)
>>> referenced by thread.cpp
>>>               thread.cpp.o:(lean::thread_finalizers_manager::finalize_thread(void*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_current_task_object cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(object.cpp.o)
>>> referenced by object.cpp
>>>               object.cpp.o:(lean::task_bind_fn1(lean_object*, lean_object*, lean_object*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_current_task_object cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(object.cpp.o)
>>> referenced by object.cpp
>>>               object.cpp.o:(lean_io_check_canceled_core) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_current_task_object cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(object.cpp.o)
>>> referenced by object.cpp
>>>               object.cpp.o:(lean::task_manager::run_task(std::__1::unique_lock<std::__1::mutex>&, lean_task*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_current_task_object cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(object.cpp.o)
>>> referenced by object.cpp
>>>               object.cpp.o:(lean::task_manager::run_task(std::__1::unique_lock<std::__1::mutex>&, lean_task*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_current_task_object cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(object.cpp.o)
>>> referenced by object.cpp
>>>               object.cpp.o:(lean::task_manager::run_task(std::__1::unique_lock<std::__1::mutex>&, lean_task*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_current_task_object cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(object.cpp.o)
>>> referenced by object.cpp
>>>               object.cpp.o:(lean::task_manager::run_task(std::__1::unique_lock<std::__1::mutex>&, lean_task*)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_heartbeat cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(interrupt.cpp.o)
>>> referenced by interrupt.cpp
>>>               interrupt.cpp.o:(lean::inc_heartbeat()) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_heartbeat cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(interrupt.cpp.o)
>>> referenced by interrupt.cpp
>>>               interrupt.cpp.o:(lean::reset_heartbeat()) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_max_heartbeat cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(interrupt.cpp.o)
>>> referenced by interrupt.cpp
>>>               interrupt.cpp.o:(lean::set_max_heartbeat(unsigned long)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_max_heartbeat cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(interrupt.cpp.o)
>>> referenced by interrupt.cpp
>>>               interrupt.cpp.o:(lean::get_max_heartbeat()) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_max_heartbeat cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(interrupt.cpp.o)
>>> referenced by interrupt.cpp
>>>               interrupt.cpp.o:(lean::set_max_heartbeat_thousands(unsigned int)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: relocation R_X86_64_TPOFF32 against lean::g_heartbeat cannot be used with -shared
>>> defined in ./.lake/build/libsome_rust_lib.a(interrupt.cpp.o)
>>> referenced by interrupt.cpp
>>>               interrupt.cpp.o:(lean::scope_heartbeat::scope_heartbeat(unsigned long)) in archive ./.lake/build/libsome_rust_lib.a

ld.lld: error: too many errors emitted, stopping now (use --error-limit=0 to see all errors)
clang: error: linker command failed with exit code 1 (use -v to see invocation)
error: external command `/home/jeswr/.elan/toolchains/leanprover--lean4---stable/bin/leanc` exited with code 1
```
