//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tcp_basertt_kern.c
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


// Copyright (c) 2017 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// BPF program to set base_rtt to 80us when host is running TCP-NV and
// both hosts are in the same datacenter (as determined by IPv6 prefix).
//
// Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
//

pub const DEBUG: c_int = 1;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_basertt(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_basertt(struct bpf_sock_ops *skops)
    {
    char cong[20];
    char nv[] = "nv";
    let mut rv: c_int = 0, n;
    int op;
    op = (int) skops.op;

    bpf_printk("BPF command: %d\n", op);

// Check if both hosts are in the same datacenter. For this
// example they are if the 1st 5.5 bytes in the IPv6 address
// are the same.
//
    if (skops.family == AF_INET6 &&
    skops.local_ip6[0] == skops.remote_ip6[0] &&
    (bpf_ntohl(skops.local_ip6[1]) & 0xfff00000) ==
    (bpf_ntohl(skops.remote_ip6[1]) & 0xfff00000)) {
    switch (op) {
    case BPF_SOCK_OPS_BASE_RTT:
    n = bpf_getsockopt(skops, SOL_TCP, TCP_CONGESTION,
    cong, sizeof(cong));
    if (!n && !__builtin_memcmp(cong, nv, sizeof(nv))) {
// Set base_rtt to 80us
    rv = 80;
    } else if (n) {
    rv = n;
    } else {
    rv = -1;
    }
    break;
    default:
    rv = -1;
    }
    } else {
    rv = -1;
    }

    bpf_printk("Returning %d\n", rv);

    skops.reply = rv;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
