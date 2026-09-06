//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fib_lookup.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    let mut fib_params: bpf_fib_lookup = {};
    let mut fib_lookup_ret: c_int = 0;
    let mut lookup_flags: c_int = 0;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn fib_lookup(skb: *mut __sk_buff) -> c_int {
    int fib_lookup(struct __sk_buff *skb)
    {
    fib_lookup_ret = bpf_fib_lookup(skb, &fib_params, sizeof(fib_params),
    lookup_flags);
    return TC_ACT_SHOT;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn fib_lookup_xdp(ctx: *mut xdp_md) -> c_int {
    int fib_lookup_xdp(struct xdp_md *ctx)
    {
    fib_lookup_ret = bpf_fib_lookup(ctx, &fib_params, sizeof(fib_params),
    lookup_flags);
    return XDP_DROP;
    }
    let mut redirected: c_int = 0;
    let mut passed: c_int = 0;
    let mut delivered: c_int = 0;
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn fib_lookup_redirect(ctx: *mut xdp_md) -> c_int {
    int fib_lookup_redirect(struct xdp_md *ctx)
    {
    let mut params: bpf_fib_lookup = fib_params;
    long ret;
    ret = bpf_fib_lookup(ctx, &params, sizeof(params), lookup_flags);
    if (ret == BPF_FIB_LKUP_RET_SUCCESS) {
    redirected++;
    return bpf_redirect(params.ifindex, 0);
    }
    passed++;
    return XDP_PASS;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_count(ctx: *mut xdp_md) -> c_int {
    int xdp_count(struct xdp_md *ctx)
    {
    void *data = (void *)(long)ctx.data;
    void *data_end = (void *)(long)ctx.data_end;
    struct ethhdr *eth = data;
    struct iphdr *iph;
//
// count only the test's TCP frames: the netns has live
// link-local traffic (DAD, MLD) that would satisfy a bare
// counter
//
    if ((void *)(eth + 1) > data_end ||
    eth.h_proto != bpf_htons(ETH_P_IP))
    return XDP_DROP;
    iph = (void *)(eth + 1);
    if ((void *)(iph + 1) > data_end || iph.protocol != IPPROTO_TCP)
    return XDP_DROP;
    delivered++;
    return XDP_DROP;
    }
    char _license[] SEC("license") = "GPL";
