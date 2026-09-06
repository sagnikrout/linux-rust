//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sk_assign.c
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
// Copyright (c) 2019 Cloudflare Ltd.
// Copyright (c) 2020 Isovalent, Inc.

// Use a new-style map definition.
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __type(key, int);
    __type(value, __u64);
    __uint(pinning, LIBBPF_PIN_BY_NAME);
    __uint(max_entries, 1);
    } server_map SEC(".maps");

// Pin map under /sys/fs/bpf/tc/globals/<map name>
pub const PIN_GLOBAL_NS: c_int = 2;
// Must match struct bpf_elf_map layout from iproute2
    struct {
    __u32 type;
    __u32 size_key;
    __u32 size_value;
    __u32 max_elem;
    __u32 flags;
    __u32 id;
    __u32 pinning;
    } server_map SEC("maps") = {
    .type = BPF_MAP_TYPE_SOCKMAP,
    .size_key = sizeof(int),
    .size_value  = sizeof(__u64),
    .max_elem = 1,
    .pinning = PIN_GLOBAL_NS,
    };

    char _license[] SEC("license") = "GPL";
// Fill 'tuple' with L3 info, and attempt to find L4. On fail, return NULL.
    static inline struct bpf_sock_tuple *
    get_tuple(struct __sk_buff *skb, bool *ipv4, bool *tcp)
    {
    void *data_end = (void *)(long)skb.data_end;
    void *data = (void *)(long)skb.data;
    struct bpf_sock_tuple *result;
    struct ethhdr *eth;
    let mut proto: __u8 = 0;
    __u64 ihl_len;
    eth = (struct ethhdr *)(data);
    if (eth + 1 > data_end)
    return core::ptr::null_mut();
    if (eth.h_proto == bpf_htons(ETH_P_IP)) {
    struct iphdr *iph = (struct iphdr *)(data + sizeof(*eth));
    if (iph + 1 > data_end)
    return core::ptr::null_mut();
    if (iph.ihl != 5)
// Options are not supported
    return core::ptr::null_mut();
    ihl_len = iph.ihl * 4;
    proto = iph.protocol;
// ipv4 = true;
    result = (struct bpf_sock_tuple *)&iph.saddr;
    } else if (eth.h_proto == bpf_htons(ETH_P_IPV6)) {
    struct ipv6hdr *ip6h = (struct ipv6hdr *)(data + sizeof(*eth));
    if (ip6h + 1 > data_end)
    return core::ptr::null_mut();
    ihl_len = sizeof(*ip6h);
    proto = ip6h.nexthdr;
// ipv4 = false;
    result = (struct bpf_sock_tuple *)&ip6h.saddr;
    } else {
    return (struct bpf_sock_tuple *)data;
    }
    if (proto != IPPROTO_TCP && proto != IPPROTO_UDP)
    return core::ptr::null_mut();
// tcp = (proto == IPPROTO_TCP);
    __sink(ihl_len);
    return result;
    }
    static inline int
    handle_udp(struct __sk_buff *skb, struct bpf_sock_tuple *tuple, bool ipv4)
    {
    struct bpf_sock *sk;
    let mut zero: c_int = 0;
    size_t tuple_len;
    __be16 dport;
    int ret;
    tuple_len = ipv4 ? sizeof(tuple.ipv4) : sizeof(tuple.ipv6);
    if ((void *)tuple + tuple_len > (void *)(long)skb.data_end)
    return TC_ACT_SHOT;
    sk = bpf_sk_lookup_udp(skb, tuple, tuple_len, BPF_F_CURRENT_NETNS, 0);
    if (sk)
    goto assign;
    dport = ipv4 ? tuple.ipv4.dport : tuple.ipv6.dport;
    if (dport != bpf_htons(4321))
    return TC_ACT_OK;
    sk = bpf_map_lookup_elem(&server_map, &zero);
    if (!sk)
    return TC_ACT_SHOT;
    assign:
    ret = bpf_sk_assign(skb, sk, 0);
    bpf_sk_release(sk);
    return ret;
    }
    static inline int
    handle_tcp(struct __sk_buff *skb, struct bpf_sock_tuple *tuple, bool ipv4)
    {
    struct bpf_sock *sk;
    let mut zero: c_int = 0;
    size_t tuple_len;
    __be16 dport;
    int ret;
    tuple_len = ipv4 ? sizeof(tuple.ipv4) : sizeof(tuple.ipv6);
    if ((void *)tuple + tuple_len > (void *)(long)skb.data_end)
    return TC_ACT_SHOT;
    sk = bpf_skc_lookup_tcp(skb, tuple, tuple_len, BPF_F_CURRENT_NETNS, 0);
    if (sk) {
    if (sk.state != BPF_TCP_LISTEN)
    goto assign;
    bpf_sk_release(sk);
    }
    dport = ipv4 ? tuple.ipv4.dport : tuple.ipv6.dport;
    if (dport != bpf_htons(4321))
    return TC_ACT_OK;
    sk = bpf_map_lookup_elem(&server_map, &zero);
    if (!sk)
    return TC_ACT_SHOT;
    if (sk.state != BPF_TCP_LISTEN) {
    bpf_sk_release(sk);
    return TC_ACT_SHOT;
    }
    assign:
    ret = bpf_sk_assign(skb, sk, 0);
    bpf_sk_release(sk);
    return ret;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn bpf_sk_assign_test(skb: *mut __sk_buff) -> c_int {
    int bpf_sk_assign_test(struct __sk_buff *skb)
    {
    struct bpf_sock_tuple *tuple;
    let mut ipv4: bool = false;
    let mut tcp: bool = false;
    let mut ret: c_int = 0;
    tuple = get_tuple(skb, &ipv4, &tcp);
    if (!tuple)
    return TC_ACT_SHOT;
// Note that the verifier socket return type for bpf_skc_lookup_tcp()
// differs from bpf_sk_lookup_udp(), so even though the C-level type is
// the same here, if we try to share the implementations they will
// fail to verify because we're crossing pointer types.
//
    if (tcp)
    ret = handle_tcp(skb, tuple, ipv4);
    else
    ret = handle_udp(skb, tuple, ipv4);
    let mut ret: return = = 0 ? TC_ACT_OK : TC_ACT_SHOT;
    }
