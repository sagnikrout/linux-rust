//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_skb_ctx.c
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
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn process(skb: *mut __sk_buff) -> c_int {
    int process(struct __sk_buff *skb)
    {
    __pragma_loop_unroll_full
    for (int i = 0; i < 5; i++) {
    if (skb.cb[i] != i + 1)
    return 1;
    skb.cb[i]++;
    }
    skb.priority++;
    skb.tstamp++;
    skb.mark++;
    if (skb.wire_len != 100)
    return 1;
    if (skb.gso_segs != 8)
    return 1;
    if (skb.gso_size != 10)
    return 1;
    if (skb.ingress_ifindex != 11)
    return 1;
    if (skb.ifindex != 1)
    return 1;
    if (skb.hwtstamp != 11)
    return 1;
    return 0;
    }
