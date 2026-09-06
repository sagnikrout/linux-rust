//! Automatically rewritten from C to Rust
//! Source: samples/bpf/parse_varlen.c
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
pub const DEBUG: c_int = 0;
#[no_mangle]
unsafe extern "C" fn tcp(data: *mut c_void, tp_off: u64, data_end: *mut c_void) -> c_int {
    static int tcp(void *data, uint64_t tp_off, void *data_end)
    {
    struct tcphdr *tcp = data + tp_off;
    if (tcp + 1 > data_end)
    return 0;
    if (tcp.dest == htons(80) || tcp.source == htons(80))
    return TC_ACT_SHOT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn udp(data: *mut c_void, tp_off: u64, data_end: *mut c_void) -> c_int {
    static int udp(void *data, uint64_t tp_off, void *data_end)
    {
    struct udphdr *udp = data + tp_off;
    if (udp + 1 > data_end)
    return 0;
    if (udp.dest == htons(DEFAULT_PKTGEN_UDP_PORT) ||
    udp.source == htons(DEFAULT_PKTGEN_UDP_PORT)) {
    if (DEBUG) {
    char fmt[] = "udp port 9 indeed\n";
    bpf_trace_printk(fmt, sizeof(fmt));
    }
    return TC_ACT_SHOT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parse_ipv4(data: *mut c_void, nh_off: u64, data_end: *mut c_void) -> c_int {
    static int parse_ipv4(void *data, uint64_t nh_off, void *data_end)
    {
    struct iphdr *iph;
    uint64_t ihl_len;
    iph = data + nh_off;
    if (iph + 1 > data_end)
    return 0;
    if (ip_is_fragment(iph))
    return 0;
    ihl_len = iph.ihl * 4;
    if (iph.protocol == IPPROTO_IPIP) {
    iph = data + nh_off + ihl_len;
    if (iph + 1 > data_end)
    return 0;
    ihl_len += iph.ihl * 4;
    }
    if (iph.protocol == IPPROTO_TCP)
    return tcp(data, nh_off + ihl_len, data_end);
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: iph->protocol ==) -> else {
    else if (iph.protocol == IPPROTO_UDP)
    return udp(data, nh_off + ihl_len, data_end);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parse_ipv6(data: *mut c_void, nh_off: u64, data_end: *mut c_void) -> c_int {
    static int parse_ipv6(void *data, uint64_t nh_off, void *data_end)
    {
    struct ipv6hdr *ip6h;
    struct iphdr *iph;
    let mut ihl_len: u64 = sizeof(struct ipv6hdr);
    uint64_t nexthdr;
    ip6h = data + nh_off;
    if (ip6h + 1 > data_end)
    return 0;
    nexthdr = ip6h.nexthdr;
    if (nexthdr == IPPROTO_IPIP) {
    iph = data + nh_off + ihl_len;
    if (iph + 1 > data_end)
    return 0;
    ihl_len += iph.ihl * 4;
    nexthdr = iph.protocol;
    } else if (nexthdr == IPPROTO_IPV6) {
    ip6h = data + nh_off + ihl_len;
    if (ip6h + 1 > data_end)
    return 0;
    ihl_len += sizeof(struct ipv6hdr);
    nexthdr = ip6h.nexthdr;
    }
    if (nexthdr == IPPROTO_TCP)
    return tcp(data, nh_off + ihl_len, data_end);
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: nexthdr ==) -> else {
    else if (nexthdr == IPPROTO_UDP)
    return udp(data, nh_off + ihl_len, data_end);
    return 0;
    }
    SEC("varlen")
#[no_mangle]
pub unsafe extern "C" fn handle_ingress(skb: *mut __sk_buff) -> c_int {
    int handle_ingress(struct __sk_buff *skb)
    {
    void *data = (void *)(long)skb.data;
    struct ethhdr *eth = data;
    void *data_end = (void *)(long)skb.data_end;
    uint64_t h_proto, nh_off;
    nh_off = sizeof(*eth);
    if (data + nh_off > data_end)
    return 0;
    h_proto = eth.h_proto;
    if (h_proto == ETH_P_8021Q || h_proto == ETH_P_8021AD) {
    struct vlan_hdr *vhdr;
    vhdr = data + nh_off;
    nh_off += sizeof(struct vlan_hdr);
    if (data + nh_off > data_end)
    return 0;
    h_proto = vhdr.h_vlan_encapsulated_proto;
    }
    if (h_proto == ETH_P_8021Q || h_proto == ETH_P_8021AD) {
    struct vlan_hdr *vhdr;
    vhdr = data + nh_off;
    nh_off += sizeof(struct vlan_hdr);
    if (data + nh_off > data_end)
    return 0;
    h_proto = vhdr.h_vlan_encapsulated_proto;
    }
    if (h_proto == htons(ETH_P_IP))
    return parse_ipv4(data, nh_off, data_end);
#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): h_proto ==) -> else {
    else if (h_proto == htons(ETH_P_IPV6))
    return parse_ipv6(data, nh_off, data_end);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
