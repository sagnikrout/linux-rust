//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/brstack.c
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

pub const BENCH_RUNS: c_int = 999999;
    static volatile int cnt;
#[no_mangle]
unsafe extern "C" fn brstack_bar() {
    }				/* return */
#[no_mangle]
unsafe extern "C" fn brstack_foo() {
    brstack_bar();		/* call */
    }				/* return */
#[no_mangle]
unsafe extern "C" fn brstack_bench() {
    void (*brstack_foo_ind)(void) = brstack_foo;
    if ((cnt++) % 3)	/* branch (cond) */
    brstack_foo();	/* call */
    brstack_bar();		/* call */
    brstack_foo_ind();	/* call (ind) */
    }
#[no_mangle]
unsafe extern "C" fn brstack(argc: c_int, argv: *const c_char) -> c_int {
    static int brstack(int argc, const char **argv)
    {
    let mut num_loops: c_int = BENCH_RUNS;
    if (argc > 0)
    num_loops = atoi(argv[0]);
    while (1) {
    if ((cnt++) > num_loops)
    break;
    brstack_bench();/* call */
    }			/* branch (uncond) */
    return 0;
    }
    DEFINE_WORKLOAD(brstack);
