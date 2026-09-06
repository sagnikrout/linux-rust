//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/traploop.c
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

#[no_mangle]
unsafe extern "C" fn trap_bench() {
    static void trap_bench(void)
    {
    unsigned long val;
    asm("mrs %0, ID_AA64ISAR0_EL1" : "=r" (val));   /* TRAP + ERET */
    }

    static void trap_bench(void) { }

#[no_mangle]
unsafe extern "C" fn traploop(argc: c_int, argv: *const c_char) -> c_int {
    static int traploop(int argc, const char **argv)
    {
    let mut num_loops: c_int = BENCH_RUNS;
    if (argc > 0)
    num_loops = atoi(argv[0]);
    for (int i = 0; i < num_loops; i++)
    trap_bench();
    return 0;
    }
    DEFINE_WORKLOAD(traploop);
