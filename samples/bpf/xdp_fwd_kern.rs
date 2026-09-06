//! Automatically rewritten from C to Rust
//! Source: samples/bpf/xdp_fwd_kern.c
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
// Copyright (c) 2017-18 David Ahern <dsahern@gmail.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//

    struct {
    __uint(type, BPF_MAP_TYPE_DEVMAP);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
    __uint(max_entries, 64);
    } xdp_tx_ports SEC(".maps");
// from include/net/ip.h
#[no_mangle]
unsafe extern "C" fn ip_decrease_ttl(iph: *mut iphdr) -> __always_inline int {
    static __always_inline int ip_decrease_ttl(struct iphdr *iph)
    {
    let mut check: u32 = ( u32)iph.check;
    check += ( u32)htons(0x0100);
    iph.check = ( __sum16)(check + (check >= 0xFFFF));
    return --iph.ttl;
    }
#[no_mangle]
unsafe extern "C" fn xdp_fwd_flags(ctx: *mut xdp_md, flags: u32) -> __always_inline int {
    static __always_inline int xdp_fwd_flags(struct xdp_md *ctx, u32 flags)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    struct bpf_fib_lookup fib_params;
    struct ethhdr *eth = data;
    struct ipv6hdr *ip6h;
    struct iphdr *iph;
    u16 h_proto;
    u64 nh_off;
    int rc;
    nh_off = sizeof(*eth);
    if (data + nh_off > data_end)
    return XDP_DROP;
    __builtin_memset(&fib_params, 0, sizeof(fib_params));
    h_proto = eth.h_proto;
    if (h_proto == htons(ETH_P_IP)) {
    iph = data + nh_off;
    if (iph + 1 > data_end)
    return XDP_DROP;
    if (iph.ttl <= 1)
    return XDP_PASS;
    fib_params.family	= AF_INET;
    fib_params.tos		= iph.tos;
    fib_params.l4_protocol	= iph.protocol;
    fib_params.sport	= 0;
    fib_params.dport	= 0;
    fib_params.tot_len	= ntohs(iph.tot_len);
    fib_params.ipv4_src	= iph.saddr;
    fib_params.ipv4_dst	= iph.daddr;
    } else if (h_proto == htons(ETH_P_IPV6)) {
    struct in6_addr *src = (struct in6_addr *) fib_params.ipv6_src;
    struct in6_addr *dst = (struct in6_addr *) fib_params.ipv6_dst;
    ip6h = data + nh_off;
    if (ip6h + 1 > data_end)
    return XDP_DROP;
    if (ip6h.hop_limit <= 1)
    return XDP_PASS;
    fib_params.family	= AF_INET6;
    fib_params.flowinfo	= *(__be32 *)ip6h & IPV6_FLOWINFO_MASK;
    fib_params.l4_protocol	= ip6h.nexthdr;
    fib_params.sport	= 0;
    fib_params.dport	= 0;
    fib_params.tot_len	= ntohs(ip6h.payload_len);
// src			= ip6h->saddr;
// dst			= ip6h->daddr;
    } else {
    return XDP_PASS;
    }
    fib_params.ifindex = ctx.ingress_ifindex;
    rc = bpf_fib_lookup(ctx, &fib_params, sizeof(fib_params), flags);
//
// Some rc (return codes) from bpf_fib_lookup() are important,
// to understand how this XDP-prog interacts with network stack.
//
// BPF_FIB_LKUP_RET_NO_NEIGH:
// Even if route lookup was a success, then the MAC-addresses are also
// needed.  This is obtained from arp/neighbour table, but if table is
// (still) empty then BPF_FIB_LKUP_RET_NO_NEIGH is returned.  To avoid
// doing ARP lookup directly from XDP, then send packet to normal
// network stack via XDP_PASS and expect it will do ARP resolution.
//
// BPF_FIB_LKUP_RET_FWD_DISABLED:
// The bpf_fib_lookup respect sysctl net.ipv{4,6}.conf.all.forwarding
// setting, and will return BPF_FIB_LKUP_RET_FWD_DISABLED if not
// enabled this on ingress device.
//
    if (rc == BPF_FIB_LKUP_RET_SUCCESS) {
// Verify egress index has been configured as TX-port.
// (Note: User can still have inserted an egress ifindex that
// doesn't support XDP xmit, which will result in packet drops).
//
// Note: lookup in devmap supported since 0cdbb4b09a0.
// If not supported will fail with:
// cannot pass map_type 14 into func bpf_map_lookup_elem#1:
//
    if (!bpf_map_lookup_elem(&xdp_tx_ports, &fib_params.ifindex))
    return XDP_PASS;
    if (h_proto == htons(ETH_P_IP))
    ip_decrease_ttl(iph);
#[no_mangle]
pub unsafe extern "C" fn if(htons(ETH_P_IPV6): h_proto ==) -> else {
    else if (h_proto == htons(ETH_P_IPV6))
    ip6h.hop_limit--;
    memcpy(eth.h_dest, fib_params.dmac, ETH_ALEN);
    memcpy(eth.h_source, fib_params.smac, ETH_ALEN);
    return bpf_redirect_map(&xdp_tx_ports, fib_params.ifindex, 0);
    }
    return XDP_PASS;
    }
    SEC("xdp_fwd")
#[no_mangle]
pub unsafe extern "C" fn xdp_fwd_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_fwd_prog(struct xdp_md *ctx)
    {
    return xdp_fwd_flags(ctx, 0);
    }
    SEC("xdp_fwd_direct")
#[no_mangle]
pub unsafe extern "C" fn xdp_fwd_direct_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_fwd_direct_prog(struct xdp_md *ctx)
    {
    return xdp_fwd_flags(ctx, BPF_FIB_LOOKUP_DIRECT);
    }
    char _license[] SEC("license") = "GPL";
