//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_trace_ext_tracing.c
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

    let mut fentry_called: __u64 = 0;
    SEC("fentry/test_pkt_md_access_new")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry, skb: *mut sk_buff) -> c_int {
    int BPF_PROG(fentry, struct sk_buff *skb)
    {
    fentry_called = skb.len;
    return 0;
    }
    let mut fexit_called: __u64 = 0;
    SEC("fexit/test_pkt_md_access_new")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fexit, skb: *mut sk_buff) -> c_int {
    int BPF_PROG(fexit, struct sk_buff *skb)
    {
    fexit_called = skb.len;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
