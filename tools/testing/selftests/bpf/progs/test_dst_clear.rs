//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_dst_clear.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const UDP_TEST_PORT: c_int = 7777;
    void *bpf_cast_to_kern_ctx(void *) __ksym;
    let mut had_dst: bool = false;
    let mut dst_cleared: bool = false;
    SEC("tc/egress")
#[no_mangle]
pub unsafe extern "C" fn dst_clear(skb: *mut __sk_buff) -> c_int {
    int dst_clear(struct __sk_buff *skb)
    {
    struct sk_buff *kskb;
    struct iphdr iph;
    struct udphdr udph;
    int err;
    if (skb.protocol != __bpf_constant_htons(ETH_P_IP))
    return TC_ACT_OK;
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &iph, sizeof(iph)))
    return TC_ACT_OK;
    if (iph.protocol != IPPROTO_UDP)
    return TC_ACT_OK;
    if (bpf_skb_load_bytes(skb, ETH_HLEN + sizeof(iph), &udph, sizeof(udph)))
    return TC_ACT_OK;
    if (udph.dest != __bpf_constant_htons(UDP_TEST_PORT))
    return TC_ACT_OK;
    kskb = bpf_cast_to_kern_ctx(skb);
    had_dst = (kskb._skb_refdst != 0);
// Same-protocol encap (IPIP): protocol stays IPv4, but the dst
// from the original routing is no longer valid for the outer hdr.
//
    err = bpf_skb_adjust_room(skb, (s32)sizeof(struct iphdr),
    BPF_ADJ_ROOM_MAC,
    BPF_F_ADJ_ROOM_FIXED_GSO |
    BPF_F_ADJ_ROOM_ENCAP_L3_IPV4);
    if (err)
    return TC_ACT_SHOT;
    dst_cleared = (kskb._skb_refdst == 0);
    return TC_ACT_SHOT;
    }
    char __license[] SEC("license") = "GPL";
