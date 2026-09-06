//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/atomic_bounds.c
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

    bool skip_tests __attribute((__section__(".data"))) = false;

    let mut skip_tests: bool = true;

    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sub, x: c_int) -> c_int {
    int BPF_PROG(sub, int x)
    {

    let mut a: c_int = 0;
    let mut b: c_int = __sync_fetch_and_add(&a, 1);
// b is certainly 0 here. Can the verifier tell?
    while (b)
    continue;

    return 0;
    }
