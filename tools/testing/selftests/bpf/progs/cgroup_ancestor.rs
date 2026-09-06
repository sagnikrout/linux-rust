//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_ancestor.c
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
// Copyright (c) 2018 Facebook

pub const NUM_CGROUP_LEVELS: c_int = 4;
    __u64 cgroup_ids[NUM_CGROUP_LEVELS];
    __u16 dport;
#[no_mangle]
unsafe extern "C" fn log_nth_level(skb: *mut __sk_buff, level: __u32) -> __always_inline void {
    static __always_inline void log_nth_level(struct __sk_buff *skb, __u32 level)
    {
// [1] &level passed to external function that may change it, it's
// incompatible with loop unroll.
//
    cgroup_ids[level] = bpf_skb_ancestor_cgroup_id(skb, level);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn log_cgroup_id(skb: *mut __sk_buff) -> c_int {
    int log_cgroup_id(struct __sk_buff *skb)
    {
    struct sock *sk = (void *)skb.sk;
    if (!sk)
    return TC_ACT_OK;
    sk = bpf_core_cast(sk, struct sock);
    if (sk.sk_protocol == IPPROTO_UDP && sk.sk_dport == dport) {
    log_nth_level(skb, 0);
    log_nth_level(skb, 1);
    log_nth_level(skb, 2);
    log_nth_level(skb, 3);
    }
    return TC_ACT_OK;
    }
    char _license[] SEC("license") = "GPL";
