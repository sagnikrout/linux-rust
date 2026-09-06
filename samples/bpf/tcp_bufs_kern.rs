//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tcp_bufs_kern.c
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
// BPF program to set initial receive window to 40 packets and send
// and receive buffers to 1.5MB. This would usually be done after
// doing appropriate checks that indicate the hosts are far enough
// away (i.e. large RTT).
//
// Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
//

pub const DEBUG: c_int = 1;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_bufs(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_bufs(struct bpf_sock_ops *skops)
    {
    let mut bufsize: c_int = 1500000;
    let mut rwnd_init: c_int = 40;
    let mut rv: c_int = 0;
    int op;
// For testing purposes, only execute rest of BPF program
// if neither port numberis 55601
//
    if (bpf_ntohl(skops.remote_port) != 55601 &&
    skops.local_port != 55601) {
    skops.reply = -1;
    return 1;
    }
    op = (int) skops.op;

    bpf_printk("Returning %d\n", rv);

// Usually there would be a check to insure the hosts are far
// from each other so it makes sense to increase buffer sizes
//
    switch (op) {
    case BPF_SOCK_OPS_RWND_INIT:
    rv = rwnd_init;
    break;
    case BPF_SOCK_OPS_TCP_CONNECT_CB:
// Set sndbuf and rcvbuf of active connections
    rv = bpf_setsockopt(skops, SOL_SOCKET, SO_SNDBUF, &bufsize,
    sizeof(bufsize));
    rv += bpf_setsockopt(skops, SOL_SOCKET, SO_RCVBUF,
    &bufsize, sizeof(bufsize));
    break;
    case BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB:
// Nothing to do
    break;
    case BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB:
// Set sndbuf and rcvbuf of passive connections
    rv = bpf_setsockopt(skops, SOL_SOCKET, SO_SNDBUF, &bufsize,
    sizeof(bufsize));
    rv += bpf_setsockopt(skops, SOL_SOCKET, SO_RCVBUF,
    &bufsize, sizeof(bufsize));
    break;
    default:
    rv = -1;
    }

    bpf_printk("Returning %d\n", rv);

    skops.reply = rv;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
