//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tcp_tos_reflect_kern.c
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
//
// Copyright (c) 2018 Facebook
//
// BPF program to automatically reflect TOS option from received syn packet
//
// Use "bpftool cgroup attach $cg sock_ops $prog" to load this BPF program.
//

pub const DEBUG: c_int = 1;
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_basertt(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_basertt(struct bpf_sock_ops *skops)
    {
    char header[sizeof(struct ipv6hdr)];
    struct ipv6hdr *hdr6;
    struct iphdr *hdr;
    let mut hdr_size: c_int = 0;
    let mut save_syn: c_int = 1;
    let mut tos: c_int = 0;
    let mut rv: c_int = 0;
    int op;
    op = (int) skops.op;

    bpf_printk("BPF command: %d\n", op);

    switch (op) {
    case BPF_SOCK_OPS_TCP_LISTEN_CB:
    rv = bpf_setsockopt(skops, SOL_TCP, TCP_SAVE_SYN,
    &save_syn, sizeof(save_syn));
    break;
    case BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB:
    if (skops.family == AF_INET)
    hdr_size = sizeof(struct iphdr);
    else
    hdr_size = sizeof(struct ipv6hdr);
    rv = bpf_getsockopt(skops, SOL_TCP, TCP_SAVED_SYN,
    header, hdr_size);
    if (!rv) {
    if (skops.family == AF_INET) {
    hdr = (struct iphdr *) header;
    tos = hdr.tos;
    if (tos != 0)
    bpf_setsockopt(skops, SOL_IP, IP_TOS,
    &tos, sizeof(tos));
    } else {
    hdr6 = (struct ipv6hdr *) header;
    tos = ((hdr6.priority) << 4 |
    (hdr6.flow_lbl[0]) >>  4);
    if (tos)
    bpf_setsockopt(skops, SOL_IPV6,
    IPV6_TCLASS,
    &tos, sizeof(tos));
    }
    rv = 0;
    }
    break;
    default:
    rv = -1;
    }

    bpf_printk("Returning %d\n", rv);

    skops.reply = rv;
    return 1;
    }
    char _license[] SEC("license") = "GPL";
