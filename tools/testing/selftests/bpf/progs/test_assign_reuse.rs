//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_assign_reuse.c
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
    __u64 sk_cookie_seen;
    __u64 reuseport_executed;
    union {
    struct tcphdr tcp;
    struct udphdr udp;
    } headers;
    const volatile __u16 dest_port;
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } sk_map SEC(".maps");
    SEC("sk_reuseport")
#[no_mangle]
pub unsafe extern "C" fn reuse_accept(ctx: *mut sk_reuseport_md) -> c_int {
    int reuse_accept(struct sk_reuseport_md *ctx)
    {
    reuseport_executed++;
    if (ctx.ip_protocol == IPPROTO_TCP) {
    if (ctx.data + sizeof(headers.tcp) > ctx.data_end)
    return SK_DROP;
    if (__builtin_memcmp(&headers.tcp, ctx.data, sizeof(headers.tcp)) != 0)
    return SK_DROP;
    } else if (ctx.ip_protocol == IPPROTO_UDP) {
    if (ctx.data + sizeof(headers.udp) > ctx.data_end)
    return SK_DROP;
    if (__builtin_memcmp(&headers.udp, ctx.data, sizeof(headers.udp)) != 0)
    return SK_DROP;
    } else {
    return SK_DROP;
    }
    sk_cookie_seen = bpf_get_socket_cookie(ctx.sk);
    return SK_PASS;
    }
    SEC("sk_reuseport")
#[no_mangle]
pub unsafe extern "C" fn reuse_drop(ctx: *mut sk_reuseport_md) -> c_int {
    int reuse_drop(struct sk_reuseport_md *ctx)
    {
    reuseport_executed++;
    sk_cookie_seen = 0;
    return SK_DROP;
    }
    static int
    assign_sk(struct __sk_buff *skb)
    {
    let mut zero: c_int = 0, ret = 0;
    struct bpf_sock *sk;
    sk = bpf_map_lookup_elem(&sk_map, &zero);
    if (!sk)
    return TC_ACT_SHOT;
    ret = bpf_sk_assign(skb, sk, 0);
    bpf_sk_release(sk);
    return ret ? TC_ACT_SHOT : TC_ACT_OK;
    }
    static bool
    maybe_assign_tcp(struct __sk_buff *skb, struct tcphdr *th)
    {
    if (th + 1 > (void *)(long)(skb.data_end))
    return TC_ACT_SHOT;
    if (!th.syn || th.ack || th.dest != bpf_htons(dest_port))
    return TC_ACT_OK;
    __builtin_memcpy(&headers.tcp, th, sizeof(headers.tcp));
    return assign_sk(skb);
    }
    static bool
    maybe_assign_udp(struct __sk_buff *skb, struct udphdr *uh)
    {
    if (uh + 1 > (void *)(long)(skb.data_end))
    return TC_ACT_SHOT;
    if (uh.dest != bpf_htons(dest_port))
    return TC_ACT_OK;
    __builtin_memcpy(&headers.udp, uh, sizeof(headers.udp));
    return assign_sk(skb);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_main(skb: *mut __sk_buff) -> c_int {
    int tc_main(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct ethhdr *eth;
    eth = (struct ethhdr *)(data);
    if (eth + 1 > data_end)
    return TC_ACT_SHOT;
    if (eth.h_proto == bpf_htons(ETH_P_IP)) {
    struct iphdr *iph = (struct iphdr *)(data + sizeof(*eth));
    if (iph + 1 > data_end)
    return TC_ACT_SHOT;
    if (iph.protocol == IPPROTO_TCP)
    return maybe_assign_tcp(skb, (struct tcphdr *)(iph + 1));
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: iph->protocol ==) -> else {
    else if (iph.protocol == IPPROTO_UDP)
    return maybe_assign_udp(skb, (struct udphdr *)(iph + 1));
    else
    return TC_ACT_SHOT;
    } else {
    struct ipv6hdr *ip6h = (struct ipv6hdr *)(data + sizeof(*eth));
    if (ip6h + 1 > data_end)
    return TC_ACT_SHOT;
    if (ip6h.nexthdr == IPPROTO_TCP)
    return maybe_assign_tcp(skb, (struct tcphdr *)(ip6h + 1));
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: ip6h->nexthdr ==) -> else {
    else if (ip6h.nexthdr == IPPROTO_UDP)
    return maybe_assign_udp(skb, (struct udphdr *)(ip6h + 1));
    else
    return TC_ACT_SHOT;
    }
    }
