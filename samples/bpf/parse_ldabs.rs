//! Automatically rewritten from C to Rust
//! Source: samples/bpf/parse_ldabs.c
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


// Copyright (c) 2016 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const DEFAULT_PKTGEN_UDP_PORT: c_int = 9;
pub const IP_MF: c_uint = 0x2000;
pub const IP_OFFSET: c_uint = 0x1FFF;
#[no_mangle]
pub unsafe extern "C" fn ip_is_fragment(ctx: *mut __sk_buff, nhoff: __u64) -> c_int {
    static inline int ip_is_fragment(struct __sk_buff *ctx, __u64 nhoff)
    {
#[no_mangle]
pub unsafe extern "C" fn load_half(_arg: ctx, iphdr: nhoff + offsetof(struct, _arg: frag_off)) -> return {
    return load_half(ctx, nhoff + offsetof(struct iphdr, frag_off))
    & (IP_MF | IP_OFFSET);
    }
    SEC("ldabs")
#[no_mangle]
pub unsafe extern "C" fn handle_ingress(skb: *mut __sk_buff) -> c_int {
    int handle_ingress(struct __sk_buff *skb)
    {
    let mut troff: __u64 = ETH_HLEN + sizeof(struct iphdr);
    if (load_half(skb, offsetof(struct ethhdr, h_proto)) != ETH_P_IP)
    return 0;
    if (load_byte(skb, ETH_HLEN + offsetof(struct iphdr, protocol)) != IPPROTO_UDP ||
    load_byte(skb, ETH_HLEN) != 0x45)
    return 0;
    if (ip_is_fragment(skb, ETH_HLEN))
    return 0;
    if (load_half(skb, troff + offsetof(struct udphdr, dest)) == DEFAULT_PKTGEN_UDP_PORT)
    return TC_ACT_SHOT;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
