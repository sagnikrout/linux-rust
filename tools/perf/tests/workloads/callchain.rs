//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/callchain.c
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

//
// Mark as noinline to establish the call chain, and avoid the static
// annotation to prevent LTO from renaming the functions.
//
    noinline void callchain_do_syscall(void);
    noinline void callchain_foo(void);
    noinline int callchain(int argc, const char **argv);
#[no_mangle]
pub unsafe extern "C" fn callchain_do_syscall() -> noinline void {
    noinline void callchain_do_syscall(void)
    {
    syscall(SYS_gettid);
    }
#[no_mangle]
pub unsafe extern "C" fn callchain_foo() -> noinline void {
    noinline void callchain_foo(void)
    {
    callchain_do_syscall();
    }
    noinline int callchain(int argc __maybe_unused,
    const char **argv __maybe_unused)
    {
    callchain_foo();
    return 0;
    }
    DEFINE_WORKLOAD(callchain);
