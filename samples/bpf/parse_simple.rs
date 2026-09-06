//! Automatically rewritten from C to Rust
//! Source: samples/bpf/parse_simple.c
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


// Copyright (c) 2016 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const DEFAULT_PKTGEN_UDP_PORT: c_int = 9;
// copy of 'struct ethhdr' without __packed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_hdr {
    pub h_dest: [c_uchar; ETH_ALEN],
    pub h_source: [c_uchar; ETH_ALEN],
    pub h_proto: c_ushort,
}

    SEC("simple")
#[no_mangle]
pub unsafe extern "C" fn handle_ingress(skb: *mut __sk_buff) -> c_int {
    int handle_ingress(struct __sk_buff *skb)
    {
    void *data = (void *)(long)skb.data;
    struct eth_hdr *eth = data;
    struct iphdr *iph = data + sizeof(*eth);
    struct udphdr *udp = data + sizeof(*eth) + sizeof(*iph);
    void *data_end = (void *)(long)skb.data_end;
// single length check
    if (data + sizeof(*eth) + sizeof(*iph) + sizeof(*udp) > data_end)
    return 0;
    if (eth.h_proto != htons(ETH_P_IP))
    return 0;
    if (iph.protocol != IPPROTO_UDP || iph.ihl != 5)
    return 0;
    if (ip_is_fragment(iph))
    return 0;
    if (udp.dest == htons(DEFAULT_PKTGEN_UDP_PORT))
    return TC_ACT_SHOT;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
