//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/get_func_ip_fsession_test.c
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

    char _license[] SEC("license") = "GPL";
    let mut test1_entry_result: __u64 = 0;
    let mut test1_exit_result: __u64 = 0;
    SEC("fsession/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1, a: c_int) -> c_int {
    int BPF_PROG(test1, int a)
    {
    let mut addr: __u64 = bpf_get_func_ip(ctx);
    if (bpf_session_is_return(ctx))
    test1_exit_result = (const void *) addr == &bpf_fentry_test1;
    else
    test1_entry_result = (const void *) addr == &bpf_fentry_test1;
    return 0;
    }
