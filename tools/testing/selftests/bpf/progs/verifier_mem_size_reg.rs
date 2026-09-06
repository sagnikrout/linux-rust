//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_mem_size_reg.c
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
//
// The __szk size of a kfunc memory/size pair must be marked precise even when
// the nullable buffer is passed as NULL.
//
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("mark_precise: frame0: regs=r4 stack= before")
#[no_mangle]
pub unsafe extern "C" fn dynptr_slice_null_buf_size_precise(skb: *mut __sk_buff) -> c_int {
    int dynptr_slice_null_buf_size_precise(struct __sk_buff *skb)
    {
    struct bpf_dynptr dptr;
    char *p;
    bpf_dynptr_from_skb(skb, 0, &dptr);
    p = bpf_dynptr_slice(&dptr, 0, core::ptr::null_mut(), 8);
    if (p)
    return p[0];
    return 0;
    }
