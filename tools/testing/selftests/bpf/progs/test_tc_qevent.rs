//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_qevent.c
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

    let mut redirect_ifindex: c_int = 1;
    let mut verdict_calls: __u64 = 0;
    let mut helper_calls: __u64 = 0;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn qevent_redirect_verdict(skb: *mut __sk_buff) -> c_int {
    int qevent_redirect_verdict(struct __sk_buff *skb)
    {
    __sync_fetch_and_add(&verdict_calls, 1);
    return TCX_REDIRECT;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn qevent_redirect_helper(skb: *mut __sk_buff) -> c_int {
    int qevent_redirect_helper(struct __sk_buff *skb)
    {
    __sync_fetch_and_add(&helper_calls, 1);
    return bpf_redirect(redirect_ifindex, 0);
    }
    char _license[] SEC("license") = "GPL";
