//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/twfw.c
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
// Copyright (c) 2021 Facebook

//
// load is successful
// #define TWFW_MAX_TIERS (64u)$
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twfw_tier_value {
    pub mask: [c_ulong; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rule {
    pub seqnum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rules_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub __u32): __type(key,,
    pub rule): __type(value, struct,
    pub 1): __uint(max_entries,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tiers_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub __u32): __type(key,,
    pub twfw_tier_value): __type(value, struct,
    pub 1): __uint(max_entries,,
}

    struct rules_map rules SEC(".maps");
    struct tiers_map tiers SEC(".maps");
    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn twfw_verifier(skb: *mut *mut __sk_buff) -> c_int {
    int twfw_verifier(struct __sk_buff* skb)
    {
    let mut key: u32 = 0;
    let mut tier: *const twfw_tier_value = bpf_map_lookup_elem(&tiers, &key);
    if (!tier)
    return 1;
    let mut rule: *mut rule = bpf_map_lookup_elem(&rules, &key);
    if (!rule)
    return 1;
    if (rule && rule.seqnum < TWFW_MAX_TIERS) {
// rule->seqnum / 64 should always be 0
    let mut mask: c_ulong = tier.mask[rule.seqnum / 64];
    if (mask)
    return 0;
    }
    return 1;
    }
