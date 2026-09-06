//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_check_mtu.c
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
// Copyright (c) 2020 Jesper Dangaard Brouer

    char _license[] SEC("license") = "GPL";
// Userspace will update with MTU it can see on device
    volatile const int GLOBAL_USER_MTU;
    volatile const __u32 GLOBAL_USER_IFINDEX;
// BPF-prog will update these with MTU values it can see
    let mut global_bpf_mtu_xdp: __u32 = 0;
    let mut global_bpf_mtu_tc: __u32 = 0;
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_use_helper_basic(ctx: *mut xdp_md) -> c_int {
    int xdp_use_helper_basic(struct xdp_md *ctx)
    {
    let mut mtu_len: __u32 = 0;
    if (bpf_check_mtu(ctx, 0, &mtu_len, 0, 0))
    return XDP_ABORTED;
    return XDP_PASS;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_use_helper(ctx: *mut xdp_md) -> c_int {
    int xdp_use_helper(struct xdp_md *ctx)
    {
    int retval = XDP_PASS; /* Expected retval on successful test */
    let mut mtu_len: __u32 = 0;
    let mut ifindex: __u32 = 0;
    let mut delta: c_int = 0;
// When ifindex is zero, save net_device lookup and use ctx netdev
    if (GLOBAL_USER_IFINDEX > 0)
    ifindex = GLOBAL_USER_IFINDEX;
    if (bpf_check_mtu(ctx, ifindex, &mtu_len, delta, 0)) {
// mtu_len is also valid when check fail
    retval = XDP_ABORTED;
    goto out;
    }
    if (mtu_len != GLOBAL_USER_MTU)
    retval = XDP_DROP;
    out:
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_exceed_mtu(ctx: *mut xdp_md) -> c_int {
    int xdp_exceed_mtu(struct xdp_md *ctx)
    {
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let mut data_len: __u32 = data_end - data;
    int retval = XDP_ABORTED; /* Fail */
    let mut mtu_len: __u32 = 0;
    int delta;
    int err;
// Exceed MTU with 1 via delta adjust
    delta = GLOBAL_USER_MTU - (data_len - ETH_HLEN) + 1;
    err = bpf_check_mtu(ctx, ifindex, &mtu_len, delta, 0);
    if (err) {
    retval = XDP_PASS; /* Success in exceeding MTU check */
    if (err != BPF_MTU_CHK_RET_FRAG_NEEDED)
    retval = XDP_DROP;
    }
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_minus_delta(ctx: *mut xdp_md) -> c_int {
    int xdp_minus_delta(struct xdp_md *ctx)
    {
    int retval = XDP_PASS; /* Expected retval on successful test */
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let mut data_len: __u32 = data_end - data;
    let mut mtu_len: __u32 = 0;
    int delta;
// Borderline test case: Minus delta exceeding packet length allowed
    delta = -((data_len - ETH_HLEN) + 1);
// Minus length (adjusted via delta) still pass MTU check, other helpers
// are responsible for catching this, when doing actual size adjust
//
    if (bpf_check_mtu(ctx, ifindex, &mtu_len, delta, 0))
    retval = XDP_ABORTED;
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_input_len(ctx: *mut xdp_md) -> c_int {
    int xdp_input_len(struct xdp_md *ctx)
    {
    int retval = XDP_PASS; /* Expected retval on successful test */
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let mut data_len: __u32 = data_end - data;
// API allow user give length to check as input via mtu_len param,
// resulting MTU value is still output in mtu_len param after call.
//
// Input len is L3, like MTU and iph->tot_len.
// Remember XDP data_len is L2.
//
    let mut mtu_len: __u32 = data_len - ETH_HLEN;
    if (bpf_check_mtu(ctx, ifindex, &mtu_len, 0, 0))
    retval = XDP_ABORTED;
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_input_len_exceed(ctx: *mut xdp_md) -> c_int {
    int xdp_input_len_exceed(struct xdp_md *ctx)
    {
    int retval = XDP_ABORTED; /* Fail */
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    int err;
// API allow user give length to check as input via mtu_len param,
// resulting MTU value is still output in mtu_len param after call.
//
// Input length value is L3 size like MTU.
//
    let mut mtu_len: __u32 = GLOBAL_USER_MTU;
    mtu_len += 1; /* Exceed with 1 */
    err = bpf_check_mtu(ctx, ifindex, &mtu_len, 0, 0);
    if (err == BPF_MTU_CHK_RET_FRAG_NEEDED)
    retval = XDP_PASS ; /* Success in exceeding MTU check */
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_use_helper(ctx: *mut __sk_buff) -> c_int {
    int tc_use_helper(struct __sk_buff *ctx)
    {
    int retval = BPF_OK; /* Expected retval on successful test */
    let mut mtu_len: __u32 = 0;
    let mut delta: c_int = 0;
    if (bpf_check_mtu(ctx, 0, &mtu_len, delta, 0)) {
    retval = BPF_DROP;
    goto out;
    }
    if (mtu_len != GLOBAL_USER_MTU)
    retval = BPF_REDIRECT;
    out:
    global_bpf_mtu_tc = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_exceed_mtu(ctx: *mut __sk_buff) -> c_int {
    int tc_exceed_mtu(struct __sk_buff *ctx)
    {
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    int retval = BPF_DROP; /* Fail */
    let mut skb_len: __u32 = ctx.len;
    let mut mtu_len: __u32 = 0;
    int delta;
    int err;
// Exceed MTU with 1 via delta adjust
    delta = GLOBAL_USER_MTU - (skb_len - ETH_HLEN) + 1;
    err = bpf_check_mtu(ctx, ifindex, &mtu_len, delta, 0);
    if (err) {
    retval = BPF_OK; /* Success in exceeding MTU check */
    if (err != BPF_MTU_CHK_RET_FRAG_NEEDED)
    retval = BPF_DROP;
    }
    global_bpf_mtu_tc = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_exceed_mtu_da(ctx: *mut __sk_buff) -> c_int {
    int tc_exceed_mtu_da(struct __sk_buff *ctx)
    {
// SKB Direct-Access variant
    void *data_end = (void *)(long)ctx.data_end;
    void *data = (void *)(long)ctx.data;
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let mut data_len: __u32 = data_end - data;
    int retval = BPF_DROP; /* Fail */
    let mut mtu_len: __u32 = 0;
    int delta;
    int err;
// Exceed MTU with 1 via delta adjust
    delta = GLOBAL_USER_MTU - (data_len - ETH_HLEN) + 1;
    err = bpf_check_mtu(ctx, ifindex, &mtu_len, delta, 0);
    if (err) {
    retval = BPF_OK; /* Success in exceeding MTU check */
    if (err != BPF_MTU_CHK_RET_FRAG_NEEDED)
    retval = BPF_DROP;
    }
    global_bpf_mtu_tc = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_minus_delta(ctx: *mut __sk_buff) -> c_int {
    int tc_minus_delta(struct __sk_buff *ctx)
    {
    int retval = BPF_OK; /* Expected retval on successful test */
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    let mut skb_len: __u32 = ctx.len;
    let mut mtu_len: __u32 = 0;
    int delta;
// Borderline test case: Minus delta exceeding packet length allowed
    delta = -((skb_len - ETH_HLEN) + 1);
// Minus length (adjusted via delta) still pass MTU check, other helpers
// are responsible for catching this, when doing actual size adjust
//
    if (bpf_check_mtu(ctx, ifindex, &mtu_len, delta, 0))
    retval = BPF_DROP;
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_input_len(ctx: *mut __sk_buff) -> c_int {
    int tc_input_len(struct __sk_buff *ctx)
    {
    int retval = BPF_OK; /* Expected retval on successful test */
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
// API allow user give length to check as input via mtu_len param,
// resulting MTU value is still output in mtu_len param after call.
//
// Input length value is L3 size.
//
    let mut mtu_len: __u32 = GLOBAL_USER_MTU;
    if (bpf_check_mtu(ctx, ifindex, &mtu_len, 0, 0))
    retval = BPF_DROP;
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_input_len_exceed(ctx: *mut __sk_buff) -> c_int {
    int tc_input_len_exceed(struct __sk_buff *ctx)
    {
    int retval = BPF_DROP; /* Fail */
    let mut ifindex: __u32 = GLOBAL_USER_IFINDEX;
    int err;
// API allow user give length to check as input via mtu_len param,
// resulting MTU value is still output in mtu_len param after call.
//
// Input length value is L3 size like MTU.
//
    let mut mtu_len: __u32 = GLOBAL_USER_MTU;
    mtu_len += 1; /* Exceed with 1 */
    err = bpf_check_mtu(ctx, ifindex, &mtu_len, 0, 0);
    if (err == BPF_MTU_CHK_RET_FRAG_NEEDED)
    retval = BPF_OK; /* Success in exceeding MTU check */
    global_bpf_mtu_xdp = mtu_len;
    return retval;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tc_chk_segs_flag(ctx: *mut __sk_buff) -> c_int {
    int tc_chk_segs_flag(struct __sk_buff *ctx)
    {
    let mut mtu_len: __u32 = 0;
    int err;
    err = bpf_check_mtu(ctx, GLOBAL_USER_IFINDEX, &mtu_len, 0, BPF_MTU_CHK_SEGS);
    let mut err: return = = -EINVAL ? BPF_OK : BPF_DROP;
    }
