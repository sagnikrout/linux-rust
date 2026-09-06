//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/inlineloop.c
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

    static volatile int a;
    static volatile sig_atomic_t done;
#[no_mangle]
unsafe extern "C" fn sighandler(__maybe_unused: int sig) {
    static void sighandler(int sig __maybe_unused)
    {
    done = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(b: (always_inline)) leaf(int) {
    static inline void __attribute__((always_inline)) leaf(int b)
    {
    again:
    a += b;
    if (!done)
    goto again;
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(b: (always_inline)) middle(int) {
    static inline void __attribute__((always_inline)) middle(int b)
    {
    leaf(b);
    }
#[no_mangle]
unsafe extern "C" fn parent(b: c_int) -> noinline void {
    static noinline void parent(int b)
    {
    middle(b);
    }
#[no_mangle]
unsafe extern "C" fn inlineloop(argc: c_int, argv: *const c_char) -> c_int {
    static int inlineloop(int argc, const char **argv)
    {
    let mut sec: c_int = 1;
    pthread_setname_np(pthread_self(), "perf-inlineloop");
    if (argc > 0)
    sec = atoi(argv[0]);
    signal(SIGINT, sighandler);
    signal(SIGALRM, sighandler);
    alarm(sec);
    parent(sec);
    return 0;
    }
    DEFINE_WORKLOAD(inlineloop);
