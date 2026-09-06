//! Automatically rewritten from C to Rust
//! Source: samples/bpf/xdp_router_ipv4.bpf.c
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


// Copyright (C) 2017 Cavium, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of version 2 of the GNU General Public License
// as published by the Free Software Foundation.
//

pub const ETH_ALEN: c_int = 6;
pub const ETH_P_8021Q: c_uint = 0x8100;
pub const ETH_P_8021AD: c_uint = 0x88A8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trie_value {
    pub prefix: [__u8; 4],
    pub value: __be64,
    pub ifindex: c_int,
    pub metric: c_int,
    pub gw: __be32,
}

// Key for lpm_trie
    union key_4 {
    u32 b32[2];
    u8 b8[8];
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_entry {
    pub mac: __be64,
    pub dst: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct direct_map {
    pub arp: arp_entry,
    pub ifindex: c_int,
    pub mac: __be64,
}

// Map for trie implementation
    struct {
    __uint(type, BPF_MAP_TYPE_LPM_TRIE);
    __uint(key_size, 8);
    __uint(value_size, sizeof(struct trie_value));
    __uint(max_entries, 50);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    } lpm_map SEC(".maps");
// Map for ARP table
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __be32);
    __type(value, __be64);
    __uint(max_entries, 50);
    } arp_table SEC(".maps");
// Map to keep the exact match entries in the route table
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __be32);
    __type(value, struct direct_map);
    __uint(max_entries, 50);
    } exact_match SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_DEVMAP);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
    __uint(max_entries, 100);
    } tx_port SEC(".maps");
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_router_ipv4_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_router_ipv4_prog(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    struct ethhdr *eth = data;
    let mut nh_off: u64 = sizeof(*eth);
    struct datarec *rec;
    __be16 h_proto;
    let mut key: u32 = 0;
    rec = bpf_map_lookup_elem(&rx_cnt, &key);
    if (rec)
    NO_TEAR_INC(rec.processed);
    if (data + nh_off > data_end)
    goto drop;
    h_proto = eth.h_proto;
    if (h_proto == bpf_htons(ETH_P_8021Q) ||
    h_proto == bpf_htons(ETH_P_8021AD)) {
    struct vlan_hdr *vhdr;
    vhdr = data + nh_off;
    nh_off += sizeof(struct vlan_hdr);
    if (data + nh_off > data_end)
    goto drop;
    h_proto = vhdr.h_vlan_encapsulated_proto;
    }
    switch (bpf_ntohs(h_proto)) {
    case ETH_P_ARP:
    if (rec)
    NO_TEAR_INC(rec.xdp_pass);
    return XDP_PASS;
    case ETH_P_IP: {
    struct iphdr *iph = data + nh_off;
    struct direct_map *direct_entry;
    __be64 *dest_mac, *src_mac;
    int forward_to;
    if (iph + 1 > data_end)
    goto drop;
    direct_entry = bpf_map_lookup_elem(&exact_match, &iph.daddr);
// Check for exact match, this would give a faster lookup
    if (direct_entry && direct_entry.mac &&
    direct_entry.arp.mac) {
    src_mac = &direct_entry.mac;
    dest_mac = &direct_entry.arp.mac;
    forward_to = direct_entry.ifindex;
    } else {
    struct trie_value *prefix_value;
    union key_4 key4;
// Look up in the trie for lpm
    key4.b32[0] = 32;
    key4.b8[4] = iph.daddr & 0xff;
    key4.b8[5] = (iph.daddr >> 8) & 0xff;
    key4.b8[6] = (iph.daddr >> 16) & 0xff;
    key4.b8[7] = (iph.daddr >> 24) & 0xff;
    prefix_value = bpf_map_lookup_elem(&lpm_map, &key4);
    if (!prefix_value)
    goto drop;
    forward_to = prefix_value.ifindex;
    src_mac = &prefix_value.value;
    if (!src_mac)
    goto drop;
    dest_mac = bpf_map_lookup_elem(&arp_table, &iph.daddr);
    if (!dest_mac) {
    if (!prefix_value.gw)
    goto drop;
    dest_mac = bpf_map_lookup_elem(&arp_table,
    &prefix_value.gw);
    if (!dest_mac) {
// Forward the packet to the kernel in
// order to trigger ARP discovery for
// the default gw.
//
    if (rec)
    NO_TEAR_INC(rec.xdp_pass);
    return XDP_PASS;
    }
    }
    }
    if (src_mac && dest_mac) {
    int ret;
    __builtin_memcpy(eth.h_dest, dest_mac, ETH_ALEN);
    __builtin_memcpy(eth.h_source, src_mac, ETH_ALEN);
    ret = bpf_redirect_map(&tx_port, forward_to, 0);
    if (ret == XDP_REDIRECT) {
    if (rec)
    NO_TEAR_INC(rec.xdp_redirect);
    return ret;
    }
    }
    }
    default:
    break;
    }
    drop:
    if (rec)
    NO_TEAR_INC(rec.xdp_drop);
    return XDP_DROP;
    }
    char _license[] SEC("license") = "GPL";
