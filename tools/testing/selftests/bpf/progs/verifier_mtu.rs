//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_mtu.c
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

    SEC("tc/ingress")
    __description("uninit/mtu: write rejected")
    __success
    __caps_unpriv(CAP_BPF|CAP_NET_ADMIN)
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(stack": "invalid read from) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn tc_uninit_mtu(ctx: *mut __sk_buff) -> c_int {
    int tc_uninit_mtu(struct __sk_buff *ctx)
    {
    __u32 mtu;
    bpf_check_mtu(ctx, 0, &mtu, 0, 0);
    return TCX_PASS;
    }
    char LICENSE[] SEC("license") = "GPL";
