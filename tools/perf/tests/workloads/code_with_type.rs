//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/code_with_type.c
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

    extern void test_rs(uint count);
    static volatile sig_atomic_t done;
#[no_mangle]
unsafe extern "C" fn sighandler(__maybe_unused: int sig) {
    static void sighandler(int sig __maybe_unused)
    {
    done = 1;
    }
#[no_mangle]
unsafe extern "C" fn code_with_type(argc: c_int, argv: *const c_char) -> c_int {
    static int code_with_type(int argc, const char **argv)
    {
    let mut sec: c_int = 1, num_loops = 100;
    pthread_setname_np(pthread_self(), "perf-code-with-type");
    if (argc > 0)
    sec = atoi(argv[0]);
    if (argc > 1)
    num_loops = atoi(argv[1]);
    signal(SIGINT, sighandler);
    signal(SIGALRM, sighandler);
    alarm(sec);
//
// Rust doesn't have signal management in the standard library. To
// not deal with any external crates, offload signal handling to the
// outside code.
//
    while (!done) {
    test_rs(num_loops);
    continue;
    }
    return 0;
    }
    DEFINE_WORKLOAD(code_with_type);
