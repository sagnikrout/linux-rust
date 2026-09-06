//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tcp_clamp_kern.c
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
// Sample BPF program to set send and receive buffers to 150KB, sndcwnd clamp
// to 100 packets and SYN and SYN_ACK RTOs to 10ms when both hosts are within
// the same datacenter. For his example, we assume they are within the same
// datacenter when the first 5.5 bytes of their IPv6 addresses are the same.
//
// Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
//

pub const DEBUG: c_int = 1;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_clamp(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_clamp(struct bpf_sock_ops *skops)
    {
    let mut bufsize: c_int = 150000;
    let mut to_init: c_int = 10;
    let mut clamp: c_int = 100;
    let mut rv: c_int = 0;
    int op;
// For testing purposes, only execute rest of BPF program
// if neither port numberis 55601
//
    if (bpf_ntohl(skops.remote_port) != 55601 && skops.local_port != 55601) {
    skops.reply = -1;
    return 0;
    }
    op = (int) skops.op;

    bpf_printk("BPF command: %d\n", op);

// Check that both hosts are within same datacenter. For this example
// it is the case when the first 5.5 bytes of their IPv6 addresses are
// the same.
//
    if (skops.family == AF_INET6 &&
    skops.local_ip6[0] == skops.remote_ip6[0] &&
    (bpf_ntohl(skops.local_ip6[1]) & 0xfff00000) ==
    (bpf_ntohl(skops.remote_ip6[1]) & 0xfff00000)) {
    switch (op) {
    case BPF_SOCK_OPS_TIMEOUT_INIT:
    rv = to_init;
    break;
    case BPF_SOCK_OPS_TCP_CONNECT_CB:
// Set sndbuf and rcvbuf of active connections
    rv = bpf_setsockopt(skops, SOL_SOCKET, SO_SNDBUF,
    &bufsize, sizeof(bufsize));
    rv += bpf_setsockopt(skops, SOL_SOCKET,
    SO_RCVBUF, &bufsize,
    sizeof(bufsize));
    break;
    case BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:
    rv = bpf_setsockopt(skops, SOL_TCP,
    TCP_BPF_SNDCWND_CLAMP,
    &clamp, sizeof(clamp));
    break;
    case BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB:
// Set sndbuf and rcvbuf of passive connections
    rv = bpf_setsockopt(skops, SOL_TCP,
    TCP_BPF_SNDCWND_CLAMP,
    &clamp, sizeof(clamp));
    rv += bpf_setsockopt(skops, SOL_SOCKET,
    SO_SNDBUF, &bufsize,
    sizeof(bufsize));
    rv += bpf_setsockopt(skops, SOL_SOCKET,
    SO_RCVBUF, &bufsize,
    sizeof(bufsize));
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
