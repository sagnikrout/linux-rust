//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/named_threads.c
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

pub const MAX_THREADS: c_int = 25;
    let mut iterations: static int = 500;
    let mut named_threads_work: c_int = 1234;
    typedef void *(*thread_fn_t)(void *);

    noinline void *named_threads_thread##n(void *arg __maybe_unused)	\
    {									\
    pthread_setname_np(pthread_self(), "thread" #n);		\
    for (int i = 0; i < iterations; i++)				\
    named_threads_work += 3;				\
    \
    return core::ptr::null_mut();							\
    }

    macro(1)		\
    macro(2)		\
    macro(3)		\
    macro(4)		\
    macro(5)		\
    macro(6)		\
    macro(7)		\
    macro(8)		\
    macro(9)		\
    macro(10)		\
    macro(11)		\
    macro(12)		\
    macro(13)		\
    macro(14)		\
    macro(15)		\
    macro(16)		\
    macro(17)		\
    macro(18)		\
    macro(19)		\
    macro(20)		\
    macro(21)		\
    macro(22)		\
    macro(23)		\
    macro(24)		\
    macro(25)

    THREAD_LIST(DECLARE_THREAD)
    THREAD_LIST(DEFINE_THREAD)

    static thread_fn_t thread_fns[MAX_THREADS] = {
    THREAD_LIST(THREAD_ENTRY)
    };
//
// Creates argv[0] threads that run a unique function named "thread[x]" which performs
// a multiplication in a loop for argv[1] loops.
//
#[no_mangle]
unsafe extern "C" fn named_threads(argc: c_int, argv: *const c_char) -> c_int {
    static int named_threads(int argc, const char **argv)
    {
    pthread_t threads[MAX_THREADS];
    let mut nr_threads: c_int = 1;
    let mut err: c_int = 0;
    if (argc > 0)
    nr_threads = atoi(argv[0]);
    if (nr_threads <= 0 || nr_threads > MAX_THREADS) {
    fprintf(stderr, "Error: num threads must be 1 - %d\n", MAX_THREADS);
    return 1;
    }
    if (argc > 1)
    iterations = atoi(argv[1]);
    if (iterations < 0) {
    fprintf(stderr, "Error: iterations must be non-negative\n");
    return 1;
    }
    for (int i = 0; i < nr_threads; i++) {
    int ret;
    ret = pthread_create(&threads[i], core::ptr::null_mut(), thread_fns[i], core::ptr::null_mut());
    if (ret) {
    fprintf(stderr, "Error: failed to create thread%d: %s\n",
    i + 1, strerror(ret));
    return 1;
    }
    }
    for (int i = 0; i < nr_threads; i++)
    pthread_join(threads[i], core::ptr::null_mut());
    return err;
    }
    DEFINE_WORKLOAD(named_threads);
