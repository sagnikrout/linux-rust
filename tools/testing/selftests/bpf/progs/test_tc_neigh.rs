//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_tc_neigh.c
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

pub const ip4_src: c_uint = 0xac100164 /* 172.16.1.100 */;
pub const ip4_dst: c_uint = 0xac100264 /* 172.16.2.100 */;

    0x00, 0x01, 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe }

    0x00, 0x02, 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe }

    a.s6_addr32[1] == b.s6_addr32[1] && \
    a.s6_addr32[2] == b.s6_addr32[2] && \
    a.s6_addr32[3] == b.s6_addr32[3])

    volatile const __u32 IFINDEX_SRC;
    volatile const __u32 IFINDEX_DST;
    static __always_inline bool is_remote_ep_v4(struct __sk_buff *skb,
    __be32 addr)
    {
    void *data_end = ctx_ptr(skb.data_end);
    void *data = ctx_ptr(skb.data);
    struct iphdr *ip4h;
    if (data + sizeof(struct ethhdr) > data_end)
    return false;
    ip4h = (struct iphdr *)(data + sizeof(struct ethhdr));
    if ((void *)(ip4h + 1) > data_end)
    return false;
    return ip4h.daddr == addr;
    }
    static __always_inline bool is_remote_ep_v6(struct __sk_buff *skb,
    struct in6_addr addr)
    {
    void *data_end = ctx_ptr(skb.data_end);
    void *data = ctx_ptr(skb.data);
    struct ipv6hdr *ip6h;
    if (data + sizeof(struct ethhdr) > data_end)
    return false;
    ip6h = (struct ipv6hdr *)(data + sizeof(struct ethhdr));
    if ((void *)(ip6h + 1) > data_end)
    return false;
    return v6_equal(ip6h.daddr, addr);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_chk(skb: *mut __sk_buff) -> c_int {
    int tc_chk(struct __sk_buff *skb)
    {
    void *data_end = ctx_ptr(skb.data_end);
    void *data = ctx_ptr(skb.data);
    __u32 *raw = data;
    if (data + sizeof(struct ethhdr) > data_end)
    return TC_ACT_SHOT;
    return !raw[0] && !raw[1] && !raw[2] ? TC_ACT_SHOT : TC_ACT_OK;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_dst(skb: *mut __sk_buff) -> c_int {
    int tc_dst(struct __sk_buff *skb)
    {
    __u8 zero[ETH_ALEN * 2];
    let mut redirect: bool = false;
    switch (skb.protocol) {
    case __bpf_constant_htons(ETH_P_IP):
    redirect = is_remote_ep_v4(skb, __bpf_constant_htonl(ip4_src));
    break;
    case __bpf_constant_htons(ETH_P_IPV6):
    redirect = is_remote_ep_v6(skb, (struct in6_addr){{ip6_src}});
    break;
    }
    if (!redirect)
    return TC_ACT_OK;
    __builtin_memset(&zero, 0, sizeof(zero));
    if (bpf_skb_store_bytes(skb, 0, &zero, sizeof(zero), 0) < 0)
    return TC_ACT_SHOT;
    return bpf_redirect_neigh(IFINDEX_SRC, core::ptr::null_mut(), 0, 0);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_src(skb: *mut __sk_buff) -> c_int {
    int tc_src(struct __sk_buff *skb)
    {
    __u8 zero[ETH_ALEN * 2];
    let mut redirect: bool = false;
    switch (skb.protocol) {
    case __bpf_constant_htons(ETH_P_IP):
    redirect = is_remote_ep_v4(skb, __bpf_constant_htonl(ip4_dst));
    break;
    case __bpf_constant_htons(ETH_P_IPV6):
    redirect = is_remote_ep_v6(skb, (struct in6_addr){{ip6_dst}});
    break;
    }
    if (!redirect)
    return TC_ACT_OK;
    __builtin_memset(&zero, 0, sizeof(zero));
    if (bpf_skb_store_bytes(skb, 0, &zero, sizeof(zero), 0) < 0)
    return TC_ACT_SHOT;
    return bpf_redirect_neigh(IFINDEX_DST, core::ptr::null_mut(), 0, 0);
    }
    char __license[] SEC("license") = "GPL";
