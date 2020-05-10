fn main() {
    println!("Hello, world!");
    //panic!("crash and burn");
    let v = vec![1, 2, 3];
    println!("Hello, world! {:?}" ,v[99]);

}

// 実行結果
// $ RUST_BACKTRACE=1 cargo run
// Finished dev [unoptimized + debuginfo] target(s) in 0.00s
// Running `target/debug/panic_error`
// Hello, world!
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796:10
// stack backtrace:
// 0: backtrace::backtrace::libunwind::trace
//         at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
// 1: backtrace::backtrace::trace_unsynchronized
//         at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
// 2: std::sys_common::backtrace::_print_fmt
//         at src/libstd/sys_common/backtrace.rs:77
// 3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
//         at src/libstd/sys_common/backtrace.rs:61
// 4: core::fmt::write
//         at src/libcore/fmt/mod.rs:1028
// 5: std::io::Write::write_fmt
//         at src/libstd/io/mod.rs:1412
// 6: std::sys_common::backtrace::_print
//         at src/libstd/sys_common/backtrace.rs:65
// 7: std::sys_common::backtrace::print
//         at src/libstd/sys_common/backtrace.rs:50
// 8: std::panicking::default_hook::{{closure}}
//         at src/libstd/panicking.rs:188
// 9: std::panicking::default_hook
//         at src/libstd/panicking.rs:205
// 10: std::panicking::rust_panic_with_hook
//         at src/libstd/panicking.rs:464
// 11: std::panicking::continue_panic_fmt
//         at src/libstd/panicking.rs:373
// 12: rust_begin_unwind
//         at src/libstd/panicking.rs:302
// 13: core::panicking::panic_fmt
//         at src/libcore/panicking.rs:139
// 14: core::panicking::panic_bounds_check
//         at src/libcore/panicking.rs:96
// 15: <usize as core::slice::SliceIndex<[T]>>::index
//         at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2796
// 16: core::slice::<impl core::ops::index::Index<I> for [T]>::index
//         at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libcore/slice/mod.rs:2647
// 17: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
//         at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/liballoc/vec.rs:1861
// 18: panic_error::main
//         at src/main.rs:5    // <- ここがエラー箇所 *******
// 19: std::rt::lang_start::{{closure}}
//         at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libstd/rt.rs:61
// 20: std::rt::lang_start_internal::{{closure}}
//         at src/libstd/rt.rs:48
// 21: std::panicking::try::do_call
//         at src/libstd/panicking.rs:287
// 22: __rust_maybe_catch_panic
//         at src/libpanic_unwind/lib.rs:78
// 23: std::panicking::try
//         at src/libstd/panicking.rs:265
// 24: std::panic::catch_unwind
//         at src/libstd/panic.rs:396
// 25: std::rt::lang_start_internal
//         at src/libstd/rt.rs:47
// 26: std::rt::lang_start
//         at /rustc/73528e339aae0f17a15ffa49a8ac608f50c6cf14/src/libstd/rt.rs:61
// 27: main
// 28: __libc_start_main
// 29: _start
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace