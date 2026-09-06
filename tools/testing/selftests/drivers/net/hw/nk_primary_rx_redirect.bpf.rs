//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/net/hw/nk_primary_rx_redirect.bpf.c
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

    volatile __u32 phys_ifindex;
    SEC("tc/ingress")
#[no_mangle]
pub unsafe extern "C" fn nk_primary_rx_redirect(skb: *mut __sk_buff) -> c_int {
    int nk_primary_rx_redirect(struct __sk_buff *skb)
    {
    void *data_end = ctx_ptr(skb.data_end);
    void *data = ctx_ptr(skb.data);
    struct ethhdr *eth;
    struct ipv6hdr *ip6h;
    eth = data;
    if ((void *)(eth + 1) > data_end)
    return TC_ACT_OK;
    if (eth.h_proto != bpf_htons(ETH_P_IPV6))
    return TC_ACT_OK;
    ip6h = data + sizeof(struct ethhdr);
    if ((void *)(ip6h + 1) > data_end)
    return TC_ACT_OK;
    if (ip6h.nexthdr == IPPROTO_ICMPV6)
    return TC_ACT_OK;
    return bpf_redirect_neigh(phys_ifindex, core::ptr::null_mut(), 0, 0);
    }
    char __license[] SEC("license") = "GPL";
