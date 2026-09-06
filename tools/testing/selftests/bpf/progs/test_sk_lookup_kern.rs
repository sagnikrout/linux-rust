//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sk_lookup_kern.c
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
// Copyright (c) 2018 Covalent IO, Inc. http://covalent.io

    char _license[] SEC("license") = "GPL";
// Fill 'tuple' with L3 info, and attempt to find L4. On fail, return NULL.
    static struct bpf_sock_tuple *get_tuple(void *data, __u64 nh_off,
    void *data_end, __u16 eth_proto,
    bool *ipv4)
    {
    struct bpf_sock_tuple *result;
    let mut ihl_len: __u64 = 0;
    let mut proto: __u8 = 0;
    if (eth_proto == bpf_htons(ETH_P_IP)) {
    struct iphdr *iph = (struct iphdr *)(data + nh_off);
    if (iph + 1 > data_end)
    return core::ptr::null_mut();
    ihl_len = iph.ihl * 4;
    proto = iph.protocol;
// ipv4 = true;
    result = (struct bpf_sock_tuple *)&iph.saddr;
    } else if (eth_proto == bpf_htons(ETH_P_IPV6)) {
    struct ipv6hdr *ip6h = (struct ipv6hdr *)(data + nh_off);
    if (ip6h + 1 > data_end)
    return core::ptr::null_mut();
    ihl_len = sizeof(*ip6h);
    proto = ip6h.nexthdr;
// ipv4 = true;
    result = (struct bpf_sock_tuple *)&ip6h.saddr;
    }
    if (data + nh_off + ihl_len > data_end || proto != IPPROTO_TCP)
    return core::ptr::null_mut();
    return result;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn sk_lookup_success(skb: *mut __sk_buff) -> c_int {
    int sk_lookup_success(struct __sk_buff *skb)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct ethhdr *eth = (struct ethhdr *)(data);
    struct bpf_sock_tuple *tuple;
    struct bpf_sock *sk;
    size_t tuple_len;
    bool ipv4;
    if (eth + 1 > data_end)
    return TC_ACT_SHOT;
    tuple = get_tuple(data, sizeof(*eth), data_end, eth.h_proto, &ipv4);
    if (!tuple || tuple + sizeof *tuple > data_end)
    return TC_ACT_SHOT;
    tuple_len = ipv4 ? sizeof(tuple.ipv4) : sizeof(tuple.ipv6);
    sk = bpf_sk_lookup_tcp(skb, tuple, tuple_len, BPF_F_CURRENT_NETNS, 0);
    bpf_printk("sk=%d\n", sk ? 1 : 0);
    if (sk)
    bpf_sk_release(sk);
    return sk ? TC_ACT_OK : TC_ACT_UNSPEC;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn sk_lookup_success_simple(skb: *mut __sk_buff) -> c_int {
    int sk_lookup_success_simple(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
    sk = bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    if (sk)
    bpf_sk_release(sk);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_use_after_free(skb: *mut __sk_buff) -> c_int {
    int err_use_after_free(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
    let mut family: __u32 = 0;
    sk = bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    if (sk) {
    bpf_sk_release(sk);
    family = sk.family;
    }
    return family;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_modify_sk_pointer(skb: *mut __sk_buff) -> c_int {
    int err_modify_sk_pointer(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
    sk = bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    if (sk) {
    sk += 1;
    bpf_sk_release(sk);
    }
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_modify_sk_or_null_pointer(skb: *mut __sk_buff) -> c_int {
    int err_modify_sk_or_null_pointer(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
    sk = bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    sk += 1;
    if (sk)
    bpf_sk_release(sk);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_no_release(skb: *mut __sk_buff) -> c_int {
    int err_no_release(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_release_twice(skb: *mut __sk_buff) -> c_int {
    int err_release_twice(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
    sk = bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    bpf_sk_release(sk);
    bpf_sk_release(sk);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_release_unchecked(skb: *mut __sk_buff) -> c_int {
    int err_release_unchecked(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    struct bpf_sock *sk;
    sk = bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    bpf_sk_release(sk);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lookup_no_release(skb: *mut __sk_buff) {
    void lookup_no_release(struct __sk_buff *skb)
    {
    let mut tuple: bpf_sock_tuple = {};
    bpf_sk_lookup_tcp(skb, &tuple, sizeof(tuple), BPF_F_CURRENT_NETNS, 0);
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn err_no_release_subcall(skb: *mut __sk_buff) -> c_int {
    int err_no_release_subcall(struct __sk_buff *skb)
    {
    lookup_no_release(skb);
    return 0;
    }
