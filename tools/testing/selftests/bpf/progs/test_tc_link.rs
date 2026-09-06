//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_link.c
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
// Copyright (c) 2023 Isovalent

    char LICENSE[] SEC("license") = "GPL";
    bool seen_tc1;
    bool seen_tc2;
    bool seen_tc3;
    bool seen_tc4;
    bool seen_tc5;
    bool seen_tc6;
    bool seen_tc7;
    bool seen_tc8;
    bool set_type;
    bool seen_eth;
    bool seen_host;
    bool seen_mcast;
    int mark, prio;
    unsigned short headroom, tailroom;
    SEC("tc/ingress")
#[no_mangle]
pub unsafe extern "C" fn tc1(skb: *mut __sk_buff) -> c_int {
    int tc1(struct __sk_buff *skb)
    {
    let mut eth: ethhdr = {};
    if (skb.protocol != __bpf_constant_htons(ETH_P_IP))
    goto out;
    if (bpf_skb_load_bytes(skb, 0, &eth, sizeof(eth)))
    goto out;
    seen_eth = eth.h_proto == bpf_htons(ETH_P_IP);
    seen_host = skb.pkt_type == PACKET_HOST;
    if (seen_host && set_type) {
    eth.h_dest[0] = 4;
    if (bpf_skb_store_bytes(skb, 0, &eth, sizeof(eth), 0))
    goto fail;
    bpf_skb_change_type(skb, PACKET_MULTICAST);
    }
    out:
    seen_tc1 = true;
    fail:
    return TCX_NEXT;
    }
    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn tc2(skb: *mut __sk_buff) -> c_int {
    int tc2(struct __sk_buff *skb)
    {
    seen_tc2 = true;
    return TCX_NEXT;
    }
    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn tc3(skb: *mut __sk_buff) -> c_int {
    int tc3(struct __sk_buff *skb)
    {
    seen_tc3 = true;
    return TCX_NEXT;
    }
    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn tc4(skb: *mut __sk_buff) -> c_int {
    int tc4(struct __sk_buff *skb)
    {
    seen_tc4 = true;
    return TCX_NEXT;
    }
    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn tc5(skb: *mut __sk_buff) -> c_int {
    int tc5(struct __sk_buff *skb)
    {
    seen_tc5 = true;
    return TCX_PASS;
    }
    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn tc6(skb: *mut __sk_buff) -> c_int {
    int tc6(struct __sk_buff *skb)
    {
    seen_tc6 = true;
    return TCX_PASS;
    }
    SEC("tc/ingress")
#[no_mangle]
pub unsafe extern "C" fn tc7(skb: *mut __sk_buff) -> c_int {
    int tc7(struct __sk_buff *skb)
    {
    let mut eth: ethhdr = {};
    if (skb.protocol != __bpf_constant_htons(ETH_P_IP))
    goto out;
    if (bpf_skb_load_bytes(skb, 0, &eth, sizeof(eth)))
    goto out;
    if (eth.h_dest[0] == 4 && set_type) {
    seen_mcast = skb.pkt_type == PACKET_MULTICAST;
    bpf_skb_change_type(skb, PACKET_HOST);
    }
    out:
    seen_tc7 = true;
    return TCX_PASS;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
    pub needed_headroom: c_ushort,
    pub needed_tailroom: c_ushort,
}

    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn tc8(skb: *mut __sk_buff) -> c_int {
    int tc8(struct __sk_buff *skb)
    {
    struct net_device *dev = BPF_CORE_READ((struct sk_buff *)skb, dev);
    seen_tc8 = true;
    mark = skb.mark;
    prio = skb.priority;
    headroom = BPF_CORE_READ(dev, needed_headroom);
    tailroom = BPF_CORE_READ(dev, needed_tailroom);
    return TCX_PASS;
    }
