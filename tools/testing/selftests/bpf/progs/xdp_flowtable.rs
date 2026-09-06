//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/xdp_flowtable.c
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
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

pub const ETH_P_IP: c_uint = 0x0800;
pub const ETH_P_IPV6: c_uint = 0x86dd;
pub const IP_MF: c_uint = 0x2000	/* "More Fragments" */;
pub const IP_OFFSET: c_uint = 0x1fff	/* "Fragment Offset" */;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_flowtable_opts___local {
    pub error: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_offload_tuple_rhash___local {
}

    struct flow_offload_tuple_rhash___local *
    bpf_xdp_flow_lookup(struct xdp_md *, struct bpf_fib_lookup *,
    struct bpf_flowtable_opts___local *, u32) __ksym;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 1);
    } stats SEC(".maps");
#[no_mangle]
unsafe extern "C" fn xdp_flowtable_offload_check_iphdr(iph: *mut iphdr) -> bool {
    static bool xdp_flowtable_offload_check_iphdr(struct iphdr *iph)
    {
// ip fragmented traffic
    if (iph.frag_off & bpf_htons(IP_MF | IP_OFFSET))
    return false;
// ip options
    if (iph.ihl * 4 != sizeof(*iph))
    return false;
    if (iph.ttl <= 1)
    return false;
    return true;
    }
    static bool xdp_flowtable_offload_check_tcp_state(void *ports, void *data_end,
    u8 proto)
    {
    if (proto == IPPROTO_TCP) {
    struct tcphdr *tcph = ports;
    if (tcph + 1 > data_end)
    return false;
    if (tcph.fin || tcph.rst)
    return false;
    }
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_ports___local {
    pub dest: __be16 source,,
    pub __attribute__((preserve_access_index)): },
    SEC("xdp.frags")
#[no_mangle]
pub unsafe extern "C" fn xdp_flowtable_do_lookup(ctx: *mut xdp_md) -> c_int {
    int xdp_flowtable_do_lookup(struct xdp_md *ctx)
    {
    pub )(long)ctx->data_end: *mut *mut void data_end = (void,
    pub {}: bpf_flowtable_opts___local opts =,
    pub tuplehash: *mut flow_offload_tuple_rhash___local,
    struct bpf_fib_lookup tuple = {
    .ifindex = ctx.ingress_ifindex,
}

    void *data = (void *)(long)ctx.data;
    struct ethhdr *eth = data;
    struct flow_ports___local *ports;
    __u32 *val, key = 0;
    if (eth + 1 > data_end)
    return XDP_DROP;
    switch (eth.h_proto) {
    case bpf_htons(ETH_P_IP): {
    struct iphdr *iph = data + sizeof(*eth);
    ports = (struct flow_ports___local *)(iph + 1);
    if (ports + 1 > data_end)
    return XDP_PASS;
// sanity check on ip header
    if (!xdp_flowtable_offload_check_iphdr(iph))
    return XDP_PASS;
    if (!xdp_flowtable_offload_check_tcp_state(ports, data_end,
    iph.protocol))
    return XDP_PASS;
    tuple.family		= AF_INET;
    tuple.tos		= iph.tos;
    tuple.l4_protocol	= iph.protocol;
    tuple.tot_len		= bpf_ntohs(iph.tot_len);
    tuple.ipv4_src		= iph.saddr;
    tuple.ipv4_dst		= iph.daddr;
    tuple.sport		= ports.source;
    tuple.dport		= ports.dest;
    break;
    }
    case bpf_htons(ETH_P_IPV6): {
    struct in6_addr *src = (struct in6_addr *)tuple.ipv6_src;
    struct in6_addr *dst = (struct in6_addr *)tuple.ipv6_dst;
    struct ipv6hdr *ip6h = data + sizeof(*eth);
    ports = (struct flow_ports___local *)(ip6h + 1);
    if (ports + 1 > data_end)
    return XDP_PASS;
    if (ip6h.hop_limit <= 1)
    return XDP_PASS;
    if (!xdp_flowtable_offload_check_tcp_state(ports, data_end,
    ip6h.nexthdr))
    return XDP_PASS;
    tuple.family		= AF_INET6;
    tuple.l4_protocol	= ip6h.nexthdr;
    tuple.tot_len		= bpf_ntohs(ip6h.payload_len);
// src			= ip6h->saddr;
// dst			= ip6h->daddr;
    tuple.sport		= ports.source;
    tuple.dport		= ports.dest;
    break;
    }
    default:
    return XDP_PASS;
    }
    tuplehash = bpf_xdp_flow_lookup(ctx, &tuple, &opts, sizeof(opts));
    if (!tuplehash)
    return XDP_PASS;
    val = bpf_map_lookup_elem(&stats, &key);
    if (val)
    __sync_add_and_fetch(val, 1);
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
