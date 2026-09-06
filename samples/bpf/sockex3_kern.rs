//! Automatically rewritten from C to Rust
//! Source: samples/bpf/sockex3_kern.c
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


// Copyright (c) 2015 PLUMgrid, http://plumgrid.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const IP_MF: c_uint = 0x2000;
pub const IP_OFFSET: c_uint = 0x1FFF;
pub const PARSE_VLAN: c_int = 1;
pub const PARSE_MPLS: c_int = 2;
pub const PARSE_IP: c_int = 3;
pub const PARSE_IPV6: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_hdr {
    pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_key_record {
    pub src: __be32,
    pub dst: __be32,
    union {
    pub ports: __be32,
    pub port16: [__be16; 2],
}

    __u32 ip_proto;
    };
    static inline void parse_eth_proto(struct __sk_buff *skb, u32 proto);
#[no_mangle]
pub unsafe extern "C" fn ip_is_fragment(ctx: *mut __sk_buff, nhoff: __u64) -> c_int {
    static inline int ip_is_fragment(struct __sk_buff *ctx, __u64 nhoff)
    {
#[no_mangle]
pub unsafe extern "C" fn load_half(_arg: ctx, iphdr: nhoff + offsetof(struct, _arg: frag_off)) -> return {
    return load_half(ctx, nhoff + offsetof(struct iphdr, frag_off))
    & (IP_MF | IP_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_hash(ctx: *mut __sk_buff, off: __u64) -> __u32 {
    static inline __u32 ipv6_addr_hash(struct __sk_buff *ctx, __u64 off)
    {
    let mut w0: __u64 = load_word(ctx, off);
    let mut w1: __u64 = load_word(ctx, off + 4);
    let mut w2: __u64 = load_word(ctx, off + 8);
    let mut w3: __u64 = load_word(ctx, off + 12);
    return (__u32)(w0 ^ w1 ^ w2 ^ w3);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct globals {
    pub flow: flow_key_record,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, struct globals);
    __uint(max_entries, 32);
    } percpu_map SEC(".maps");
// user poor man's per_cpu until native support is ready
    static struct globals *this_cpu_globals(void)
    {
    let mut key: u32 = bpf_get_smp_processor_id();
    return bpf_map_lookup_elem(&percpu_map, &key);
    }
// some simple stats for user space consumption
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub packets: __u64,
    pub bytes: __u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, struct flow_key_record);
    __type(value, struct pair);
    __uint(max_entries, 1024);
    } hash_map SEC(".maps");
#[no_mangle]
unsafe extern "C" fn update_stats(skb: *mut __sk_buff, g: *mut globals) {
    static void update_stats(struct __sk_buff *skb, struct globals *g)
    {
    let mut key: flow_key_record = g.flow;
    struct pair *value;
    value = bpf_map_lookup_elem(&hash_map, &key);
    if (value) {
    __sync_fetch_and_add(&value.packets, 1);
    __sync_fetch_and_add(&value.bytes, skb.len);
    } else {
    let mut val: pair = {1, skb.len};
    bpf_map_update_elem(&hash_map, &key, &val, BPF_ANY);
    }
    }
    static __always_inline void parse_ip_proto(struct __sk_buff *skb,
    struct globals *g, __u32 ip_proto)
    {
    let mut nhoff: __u32 = skb.cb[0];
    int poff;
    switch (ip_proto) {
    case IPPROTO_GRE: {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_hdr {
    pub flags: __be16,
    pub proto: __be16,
}

    __u32 gre_flags = load_half(skb,
    nhoff + offsetof(struct gre_hdr, flags));
    __u32 gre_proto = load_half(skb,
    nhoff + offsetof(struct gre_hdr, proto));
    if (gre_flags & (GRE_VERSION|GRE_ROUTING))
    break;
    nhoff += 4;
    if (gre_flags & GRE_CSUM)
    nhoff += 4;
    if (gre_flags & GRE_KEY)
    nhoff += 4;
    if (gre_flags & GRE_SEQ)
    nhoff += 4;
    skb.cb[0] = nhoff;
    parse_eth_proto(skb, gre_proto);
    break;
    }
    case IPPROTO_IPIP:
    parse_eth_proto(skb, ETH_P_IP);
    break;
    case IPPROTO_IPV6:
    parse_eth_proto(skb, ETH_P_IPV6);
    break;
    case IPPROTO_TCP:
    case IPPROTO_UDP:
    g.flow.ports = load_word(skb, nhoff);
    case IPPROTO_ICMP:
    g.flow.ip_proto = ip_proto;
    update_stats(skb, g);
    break;
    default:
    break;
    }
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn bpf_func_ip(skb: *mut __sk_buff) -> c_int {
    int bpf_func_ip(struct __sk_buff *skb)
    {
    struct globals *g = this_cpu_globals();
    __u32 nhoff, verlen, ip_proto;
    if (!g)
    return 0;
    nhoff = skb.cb[0];
    if (unlikely(ip_is_fragment(skb, nhoff)))
    return 0;
    ip_proto = load_byte(skb, nhoff + offsetof(struct iphdr, protocol));
    if (ip_proto != IPPROTO_GRE) {
    g.flow.src = load_word(skb, nhoff + offsetof(struct iphdr, saddr));
    g.flow.dst = load_word(skb, nhoff + offsetof(struct iphdr, daddr));
    }
    verlen = load_byte(skb, nhoff + 0/*offsetof(struct iphdr, ihl)*/);
    nhoff += (verlen & 0xF) << 2;
    skb.cb[0] = nhoff;
    parse_ip_proto(skb, g, ip_proto);
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn bpf_func_ipv6(skb: *mut __sk_buff) -> c_int {
    int bpf_func_ipv6(struct __sk_buff *skb)
    {
    struct globals *g = this_cpu_globals();
    __u32 nhoff, ip_proto;
    if (!g)
    return 0;
    nhoff = skb.cb[0];
    ip_proto = load_byte(skb,
    nhoff + offsetof(struct ipv6hdr, nexthdr));
    g.flow.src = ipv6_addr_hash(skb,
    nhoff + offsetof(struct ipv6hdr, saddr));
    g.flow.dst = ipv6_addr_hash(skb,
    nhoff + offsetof(struct ipv6hdr, daddr));
    nhoff += sizeof(struct ipv6hdr);
    skb.cb[0] = nhoff;
    parse_ip_proto(skb, g, ip_proto);
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn bpf_func_vlan(skb: *mut __sk_buff) -> c_int {
    int bpf_func_vlan(struct __sk_buff *skb)
    {
    __u32 nhoff, proto;
    nhoff = skb.cb[0];
    proto = load_half(skb, nhoff + offsetof(struct vlan_hdr,
    h_vlan_encapsulated_proto));
    nhoff += sizeof(struct vlan_hdr);
    skb.cb[0] = nhoff;
    parse_eth_proto(skb, proto);
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn bpf_func_mpls(skb: *mut __sk_buff) -> c_int {
    int bpf_func_mpls(struct __sk_buff *skb)
    {
    __u32 nhoff, label;
    nhoff = skb.cb[0];
    label = load_word(skb, nhoff);
    nhoff += sizeof(struct mpls_label);
    skb.cb[0] = nhoff;
    if (label & MPLS_LS_S_MASK) {
    let mut verlen: __u8 = load_byte(skb, nhoff);
    if ((verlen & 0xF0) == 4)
    parse_eth_proto(skb, ETH_P_IP);
    else
    parse_eth_proto(skb, ETH_P_IPV6);
    } else {
    parse_eth_proto(skb, ETH_P_MPLS_UC);
    }
    return 0;
    }
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(max_entries, 8);
    __array(values, u32 (void *));
    } prog_array_init SEC(".maps") = {
    .values = {
    [PARSE_VLAN] = (void *)&bpf_func_vlan,
    [PARSE_IP]   = (void *)&bpf_func_ip,
    [PARSE_IPV6] = (void *)&bpf_func_ipv6,
    [PARSE_MPLS] = (void *)&bpf_func_mpls,
    },
    };
// Protocol dispatch routine. It tail-calls next BPF program depending
// on eth proto. Note, we could have used ...
//
// bpf_tail_call(skb, &prog_array_init, proto);
//
// ... but it would need large prog_array and cannot be optimised given
// the map key is not static.
//
#[no_mangle]
pub unsafe extern "C" fn parse_eth_proto(skb: *mut __sk_buff, proto: u32) {
    static inline void parse_eth_proto(struct __sk_buff *skb, u32 proto)
    {
    switch (proto) {
    case ETH_P_8021Q:
    case ETH_P_8021AD:
    bpf_tail_call(skb, &prog_array_init, PARSE_VLAN);
    break;
    case ETH_P_MPLS_UC:
    case ETH_P_MPLS_MC:
    bpf_tail_call(skb, &prog_array_init, PARSE_MPLS);
    break;
    case ETH_P_IP:
    bpf_tail_call(skb, &prog_array_init, PARSE_IP);
    break;
    case ETH_P_IPV6:
    bpf_tail_call(skb, &prog_array_init, PARSE_IPV6);
    break;
    }
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn main_prog(skb: *mut __sk_buff) -> c_int {
    int main_prog(struct __sk_buff *skb)
    {
    let mut nhoff: __u32 = ETH_HLEN;
    let mut proto: __u32 = load_half(skb, 12);
    skb.cb[0] = nhoff;
    parse_eth_proto(skb, proto);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
