//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fmod_ret_freplace.c
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

    let mut test_fmod_ret: volatile __u64 = 0;
    SEC("fmod_ret/security_new_get_constant")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fmod_ret_test, val: c_long, ret: c_int) -> c_int {
    int BPF_PROG(fmod_ret_test, long val, int ret)
    {
    test_fmod_ret = 1;
    return 120;
    }
    char _license[] SEC("license") = "GPL";
