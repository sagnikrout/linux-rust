//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/cgroup_skb_sk_lookup_kern.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
    let mut g_serv_port: __u16 = 0;
#[no_mangle]
pub unsafe extern "C" fn set_ip(dst: *mut __u32, src: *const in6_addr) {
    static inline void set_ip(__u32 *dst, const struct in6_addr *src)
    {
    dst[0] = src.in6_u.u6_addr32[0];
    dst[1] = src.in6_u.u6_addr32[1];
    dst[2] = src.in6_u.u6_addr32[2];
    dst[3] = src.in6_u.u6_addr32[3];
    }
    static inline void set_tuple(struct bpf_sock_tuple *tuple,
    const struct ipv6hdr *ip6h,
    const struct tcphdr *tcph)
    {
    set_ip(tuple.ipv6.saddr, &ip6h.daddr);
    set_ip(tuple.ipv6.daddr, &ip6h.saddr);
    tuple.ipv6.sport = tcph.dest;
    tuple.ipv6.dport = tcph.source;
    }
    static inline int is_allowed_peer_cg(struct __sk_buff *skb,
    const struct ipv6hdr *ip6h,
    const struct tcphdr *tcph)
    {
    __u64 cgid, acgid, peer_cgid, peer_acgid;
    struct bpf_sock_tuple tuple;
    let mut tuple_len: usize = sizeof(tuple.ipv6);
    struct bpf_sock *peer_sk;
    set_tuple(&tuple, ip6h, tcph);
    peer_sk = bpf_sk_lookup_tcp(skb, &tuple, tuple_len,
    BPF_F_CURRENT_NETNS, 0);
    if (!peer_sk)
    return 0;
    cgid = bpf_skb_cgroup_id(skb);
    peer_cgid = bpf_sk_cgroup_id(peer_sk);
    acgid = bpf_skb_ancestor_cgroup_id(skb, 2);
    peer_acgid = bpf_sk_ancestor_cgroup_id(peer_sk, 2);
    bpf_sk_release(peer_sk);
    return cgid && cgid == peer_cgid && acgid && acgid == peer_acgid;
    }
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn ingress_lookup(skb: *mut __sk_buff) -> c_int {
    int ingress_lookup(struct __sk_buff *skb)
    {
    struct ipv6hdr ip6h;
    struct tcphdr tcph;
    if (skb.protocol != bpf_htons(ETH_P_IPV6))
    return 1;
// For SYN packets coming to listening socket skb->remote_port will be
// zero, so IPv6/TCP headers are loaded to identify remote peer
// instead.
//
    if (bpf_skb_load_bytes(skb, 0, &ip6h, sizeof(ip6h)))
    return 1;
    if (ip6h.nexthdr != IPPROTO_TCP)
    return 1;
    if (bpf_skb_load_bytes(skb, sizeof(ip6h), &tcph, sizeof(tcph)))
    return 1;
    if (!g_serv_port)
    return 0;
    if (tcph.dest != g_serv_port)
    return 1;
    return is_allowed_peer_cg(skb, &ip6h, &tcph);
    }
