//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/net/hw/nk_forward.bpf.c
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

pub const TC_ACT_OK: c_int = 0;
pub const ETH_P_IPV6: c_uint = 0x86DD;

    a.s6_addr32[1] == b.s6_addr32[1])
    volatile __u32 netkit_ifindex;
    volatile __u8 ipv6_prefix[16];
    SEC("tc/ingress")
#[no_mangle]
pub unsafe extern "C" fn tc_redirect_peer(skb: *mut __sk_buff) -> c_int {
    int tc_redirect_peer(struct __sk_buff *skb)
    {
    void *data_end = ctx_ptr(skb.data_end);
    void *data = ctx_ptr(skb.data);
    struct in6_addr *peer_addr;
    struct ipv6hdr *ip6h;
    struct ethhdr *eth;
    peer_addr = (struct in6_addr *)ipv6_prefix;
    if (skb.protocol != bpf_htons(ETH_P_IPV6))
    return TC_ACT_OK;
    eth = data;
    if ((void *)(eth + 1) > data_end)
    return TC_ACT_OK;
    ip6h = data + sizeof(struct ethhdr);
    if ((void *)(ip6h + 1) > data_end)
    return TC_ACT_OK;
    if (!v6_p64_equal(ip6h.daddr, (*peer_addr)))
    return TC_ACT_OK;
    return bpf_redirect_peer(netkit_ifindex, 0);
    }
    char __license[] SEC("license") = "GPL";
