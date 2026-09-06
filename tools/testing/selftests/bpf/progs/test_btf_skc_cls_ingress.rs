//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_btf_skc_cls_ingress.c
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

pub const ENOENT: c_int = 2;

    let mut srv_sa6: sockaddr_in6 = {};
    let mut srv_sa4: sockaddr_in = {};
    let mut listen_tp_sport: __u16 = 0;
    let mut req_sk_sport: __u16 = 0;
    let mut recv_cookie: __u32 = 0;
    let mut gen_cookie: __u32 = 0;
    let mut mss: __u32 = 0;
    let mut linum: __u32 = 0;

    static void test_syncookie_helper(void *iphdr, int iphdr_size,
    struct tcphdr *th, struct tcp_sock *tp,
    struct __sk_buff *skb)
    {
    if (th.syn) {
    __s64 mss_cookie;
    void *data_end;
    data_end = (void *)(long)(skb.data_end);
    if (th.doff * 4 != 40) {
    LOG();
    return;
    }
    if ((void *)th + 40 > data_end) {
    LOG();
    return;
    }
    mss_cookie = bpf_tcp_gen_syncookie(tp, iphdr, iphdr_size,
    th, 40);
    if (mss_cookie < 0) {
    if (mss_cookie != -ENOENT)
    LOG();
    } else {
    gen_cookie = (__u32)mss_cookie;
    mss = mss_cookie >> 32;
    }
    } else if (gen_cookie) {
// It was in cookie mode
    int ret = bpf_tcp_check_syncookie(tp, iphdr, iphdr_size,
    th, sizeof(*th));
    if (ret < 0) {
    if (ret != -ENOENT)
    LOG();
    } else {
    recv_cookie = bpf_ntohl(th.ack_seq) - 1;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_ip_tcp(eth: *mut ethhdr, skb: *mut __sk_buff) -> c_int {
    static int handle_ip_tcp(struct ethhdr *eth, struct __sk_buff *skb)
    {
    struct bpf_sock_tuple *tuple = core::ptr::null_mut();
    let mut tuple_len: c_uint = 0;
    struct bpf_sock *bpf_skc;
    void *data_end, *iphdr;
    struct ipv6hdr *ip6h;
    struct iphdr *ip4h;
    struct tcphdr *th;
    int iphdr_size;
    data_end = (void *)(long)(skb.data_end);
    switch (eth.h_proto) {
    case bpf_htons(ETH_P_IP):
    ip4h = (struct iphdr *)(eth + 1);
    if (ip4h + 1 > data_end)
    return TC_ACT_OK;
    if (ip4h.protocol != IPPROTO_TCP)
    return TC_ACT_OK;
    th = (struct tcphdr *)(ip4h + 1);
    if (th + 1 > data_end)
    return TC_ACT_OK;
// Is it the testing traffic?
    if (th.dest != srv_sa4.sin_port)
    return TC_ACT_OK;
    tuple_len = sizeof(tuple.ipv4);
    tuple = (struct bpf_sock_tuple *)&ip4h.saddr;
    iphdr = ip4h;
    iphdr_size = sizeof(*ip4h);
    break;
    case bpf_htons(ETH_P_IPV6):
    ip6h = (struct ipv6hdr *)(eth + 1);
    if (ip6h + 1 > data_end)
    return TC_ACT_OK;
    if (ip6h.nexthdr != IPPROTO_TCP)
    return TC_ACT_OK;
    th = (struct tcphdr *)(ip6h + 1);
    if (th + 1 > data_end)
    return TC_ACT_OK;
// Is it the testing traffic?
    if (th.dest != srv_sa6.sin6_port)
    return TC_ACT_OK;
    tuple_len = sizeof(tuple.ipv6);
    tuple = (struct bpf_sock_tuple *)&ip6h.saddr;
    iphdr = ip6h;
    iphdr_size = sizeof(*ip6h);
    break;
    default:
    return TC_ACT_OK;
    }
    if ((void *)tuple + tuple_len > data_end) {
    LOG();
    return TC_ACT_OK;
    }
    bpf_skc = bpf_skc_lookup_tcp(skb, tuple, tuple_len,
    BPF_F_CURRENT_NETNS, 0);
    if (!bpf_skc) {
    LOG();
    return TC_ACT_OK;
    }
    if (bpf_skc.state == BPF_TCP_NEW_SYN_RECV) {
    struct request_sock *req_sk;
    req_sk = (struct request_sock *)bpf_skc_to_tcp_request_sock(bpf_skc);
    if (!req_sk) {
    LOG();
    goto release;
    }
    if (bpf_sk_assign(skb, req_sk, 0)) {
    LOG();
    goto release;
    }
    req_sk_sport = req_sk.__req_common.skc_num;
    bpf_sk_release(req_sk);
    return TC_ACT_OK;
    } else if (bpf_skc.state == BPF_TCP_LISTEN) {
    struct tcp_sock *tp;
    tp = bpf_skc_to_tcp_sock(bpf_skc);
    if (!tp) {
    LOG();
    goto release;
    }
    if (bpf_sk_assign(skb, tp, 0)) {
    LOG();
    goto release;
    }
    listen_tp_sport = tp.inet_conn.icsk_inet.sk.__sk_common.skc_num;
    test_syncookie_helper(iphdr, iphdr_size, th, tp, skb);
    bpf_sk_release(tp);
    return TC_ACT_OK;
    }
    if (bpf_sk_assign(skb, bpf_skc, 0))
    LOG();
    release:
    bpf_sk_release(bpf_skc);
    return TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn cls_ingress(skb: *mut __sk_buff) -> c_int {
    int cls_ingress(struct __sk_buff *skb)
    {
    struct ethhdr *eth;
    void *data_end;
    data_end = (void *)(long)(skb.data_end);
    eth = (struct ethhdr *)(long)(skb.data);
    if (eth + 1 > data_end)
    return TC_ACT_OK;
    if (eth.h_proto != bpf_htons(ETH_P_IP) &&
    eth.h_proto != bpf_htons(ETH_P_IPV6))
    return TC_ACT_OK;
    return handle_ip_tcp(eth, skb);
    }
    char _license[] SEC("license") = "GPL";
