//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/ip_check_defrag.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub const NF_DROP: c_int = 0;
pub const NF_ACCEPT: c_int = 1;
pub const ETH_P_IP: c_uint = 0x0800;
pub const ETH_P_IPV6: c_uint = 0x86DD;
pub const IP_MF: c_uint = 0x2000;
pub const IP_OFFSET: c_uint = 0x1FFF;
pub const NEXTHDR_FRAGMENT: c_int = 44;
    let mut shootdowns: volatile int = 0;
#[no_mangle]
unsafe extern "C" fn is_frag_v4(iph: *mut iphdr) -> bool {
    static bool is_frag_v4(struct iphdr *iph)
    {
    int offset;
    int flags;
    offset = bpf_ntohs(iph.frag_off);
    flags = offset & ~IP_OFFSET;
    offset &= IP_OFFSET;
    offset <<= 3;
    return (flags & IP_MF) || offset;
    }
#[no_mangle]
unsafe extern "C" fn is_frag_v6(ip6h: *mut ipv6hdr) -> bool {
    static bool is_frag_v6(struct ipv6hdr *ip6h)
    {
// Simplifying assumption that there are no extension headers
// between fixed header and fragmentation header. This assumption
// is only valid in this test case. It saves us the hassle of
// searching all potential extension headers.
//
    return ip6h.nexthdr == NEXTHDR_FRAGMENT;
    }
#[no_mangle]
unsafe extern "C" fn handle_v4(skb: *mut __sk_buff) -> c_int {
    static int handle_v4(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    u8 iph_buf[20] = {};
    struct iphdr *iph;
    if (bpf_dynptr_from_skb(skb, 0, &ptr))
    return NF_DROP;
    iph = bpf_dynptr_slice(&ptr, 0, iph_buf, sizeof(iph_buf));
    if (!iph)
    return NF_DROP;
// Shootdown any frags
    if (is_frag_v4(iph)) {
    shootdowns++;
    return NF_DROP;
    }
    return NF_ACCEPT;
    }
#[no_mangle]
unsafe extern "C" fn handle_v6(skb: *mut __sk_buff) -> c_int {
    static int handle_v6(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    struct ipv6hdr *ip6h;
    u8 ip6h_buf[40] = {};
    if (bpf_dynptr_from_skb(skb, 0, &ptr))
    return NF_DROP;
    ip6h = bpf_dynptr_slice(&ptr, 0, ip6h_buf, sizeof(ip6h_buf));
    if (!ip6h)
    return NF_DROP;
// Shootdown any frags
    if (is_frag_v6(ip6h)) {
    shootdowns++;
    return NF_DROP;
    }
    return NF_ACCEPT;
    }
    SEC("netfilter")
#[no_mangle]
pub unsafe extern "C" fn defrag(ctx: *mut bpf_nf_ctx) -> c_int {
    int defrag(struct bpf_nf_ctx *ctx)
    {
    struct __sk_buff *skb = (struct __sk_buff *)ctx.skb;
    switch (bpf_ntohs(ctx.skb.protocol)) {
    case ETH_P_IP:
    return handle_v4(skb);
    case ETH_P_IPV6:
    return handle_v6(skb);
    default:
    return NF_ACCEPT;
    }
    }
    char _license[] SEC("license") = "GPL";
