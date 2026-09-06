//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/tests/test-threadmap.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

    static int libperf_print(enum libperf_print_level level,
    const char *fmt, va_list ap)
    {
    return vfprintf(stderr, fmt, ap);
    }
#[no_mangle]
unsafe extern "C" fn test_threadmap_array(nr: c_int, array: *mut pid_t) -> c_int {
    static int test_threadmap_array(int nr, pid_t *array)
    {
    struct perf_thread_map *threads;
    int i;
    threads = perf_thread_map__new_array(nr, array);
    __T("Failed to allocate new thread map", threads);
    __T("Unexpected number of threads", perf_thread_map__nr(threads) == nr);
    for (i = 0; i < nr; i++) {
    __T("Unexpected initial value of thread",
    perf_thread_map__pid(threads, i) == (array ? array[i] : -1));
    }
    for (i = 1; i < nr; i++)
    perf_thread_map__set_pid(threads, i, i * 100);
    __T("Unexpected value of thread 0",
    perf_thread_map__pid(threads, 0) == (array ? array[0] : -1));
    for (i = 1; i < nr; i++) {
    __T("Unexpected thread value",
    perf_thread_map__pid(threads, i) == i * 100);
    }
    perf_thread_map__put(threads);
    return 0;
    }
pub const THREADS_NR: c_int = 10;
#[no_mangle]
pub unsafe extern "C" fn test_threadmap(argc: c_int, argv: *mut c_char) -> c_int {
    int test_threadmap(int argc, char **argv)
    {
    struct perf_thread_map *threads;
    pid_t thr_array[THREADS_NR];
    int i;
    __T_START;
    libperf_init(libperf_print);
    threads = perf_thread_map__new_dummy();
    if (!threads)
    return -1;
    perf_thread_map__get(threads);
    perf_thread_map__put(threads);
    perf_thread_map__put(threads);
    test_threadmap_array(THREADS_NR, core::ptr::null_mut());
    for (i = 0; i < THREADS_NR; i++)
    thr_array[i] = i + 100;
    test_threadmap_array(THREADS_NR, thr_array);
    __T_END;
    let mut tests_failed: return = = 0 ? 0 : -1;
    }
