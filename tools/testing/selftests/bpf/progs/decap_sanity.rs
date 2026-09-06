//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/decap_sanity.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

pub const UDP_TEST_PORT: c_int = 7777;
    void *bpf_cast_to_kern_ctx(void *) __ksym;
    let mut init_csum_partial: bool = false;
    let mut final_csum_none: bool = false;
    let mut broken_csum_start: bool = false;
#[no_mangle]
unsafe extern "C" fn skb_headlen(skb: *const sk_buff) -> c_uint {
    static unsigned int skb_headlen(const struct sk_buff *skb)
    {
    return skb.len - skb.data_len;
    }
#[no_mangle]
unsafe extern "C" fn skb_headroom(skb: *const sk_buff) -> c_uint {
    static unsigned int skb_headroom(const struct sk_buff *skb)
    {
    return skb.data - skb.head;
    }
#[no_mangle]
unsafe extern "C" fn skb_checksum_start_offset(skb: *const sk_buff) -> c_int {
    static int skb_checksum_start_offset(const struct sk_buff *skb)
    {
    return skb.csum_start - skb_headroom(skb);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn decap_sanity(skb: *mut __sk_buff) -> c_int {
    int decap_sanity(struct __sk_buff *skb)
    {
    struct sk_buff *kskb;
    struct ipv6hdr ip6h;
    struct udphdr udph;
    int err;
    if (skb.protocol != __bpf_constant_htons(ETH_P_IPV6))
    return TC_ACT_SHOT;
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &ip6h, sizeof(ip6h)))
    return TC_ACT_SHOT;
    if (ip6h.nexthdr != IPPROTO_UDP)
    return TC_ACT_SHOT;
    if (bpf_skb_load_bytes(skb, ETH_HLEN + sizeof(ip6h), &udph, sizeof(udph)))
    return TC_ACT_SHOT;
    if (udph.dest != __bpf_constant_htons(UDP_TEST_PORT))
    return TC_ACT_SHOT;
    kskb = bpf_cast_to_kern_ctx(skb);
    init_csum_partial = (kskb.ip_summed == CHECKSUM_PARTIAL);
    err = bpf_skb_adjust_room(skb, -(s32)(ETH_HLEN + sizeof(ip6h) + sizeof(udph)),
    1, BPF_F_ADJ_ROOM_FIXED_GSO);
    if (err)
    return TC_ACT_SHOT;
    final_csum_none = (kskb.ip_summed == CHECKSUM_NONE);
    if (kskb.ip_summed == CHECKSUM_PARTIAL &&
    (unsigned int)skb_checksum_start_offset(kskb) >= skb_headlen(kskb))
    broken_csum_start = true;
    return TC_ACT_SHOT;
    }
    char __license[] SEC("license") = "GPL";
