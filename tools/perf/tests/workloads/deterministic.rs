//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/deterministic.c
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

    let mut dt_work: c_int = 1234;
#[no_mangle]
unsafe extern "C" fn function1() {
    static void function1(void)
    {
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;
    }
#[no_mangle]
unsafe extern "C" fn function2() {
    static void function2(void)
    {
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;
    }
    static int deterministic(int argc __maybe_unused,
    const char **argv __maybe_unused)
    {
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;
    function1();
    dt_work += 7;
    dt_work += 7;
    dt_work += 7;
    function2();
    return 0;
    }
    DEFINE_WORKLOAD(deterministic);
