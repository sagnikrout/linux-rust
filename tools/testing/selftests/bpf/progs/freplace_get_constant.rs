//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/freplace_get_constant.c
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

    let mut test_get_constant: volatile __u64 = 0;
    SEC("freplace/get_constant")
#[no_mangle]
pub unsafe extern "C" fn security_new_get_constant(val: c_long) -> c_int {
    int security_new_get_constant(long val)
    {
    if (val != 123)
    return 0;
    test_get_constant = 1;
    return test_get_constant; /* original get_constant() returns val - 122 */
    }
    char _license[] SEC("license") = "GPL";
