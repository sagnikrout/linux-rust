//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_pull_data.c
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

    int xdpf_sz;
    int sinfo_sz;
    int data_len;
    int pull_len;
pub const XDP_PACKET_HEADROOM: c_int = 256;
    SEC("xdp.frags")
#[no_mangle]
pub unsafe extern "C" fn xdp_find_sizes(ctx: *mut xdp_md) -> c_int {
    int xdp_find_sizes(struct xdp_md *ctx)
    {
    xdpf_sz = sizeof(struct xdp_frame);
    sinfo_sz = __PAGE_SIZE - XDP_PACKET_HEADROOM -
    (ctx.data_end - ctx.data);
    return XDP_PASS;
    }
    SEC("xdp.frags")
#[no_mangle]
pub unsafe extern "C" fn xdp_pull_data_prog(ctx: *mut xdp_md) -> c_int {
    int xdp_pull_data_prog(struct xdp_md *ctx)
    {
    __u8 *data_end = (void *)(long)ctx.data_end;
    __u8 *data = (void *)(long)ctx.data;
    __u8 *val_p;
    int err;
    if (data_len != data_end - data)
    return XDP_DROP;
    err = bpf_xdp_pull_data(ctx, pull_len);
    if (err)
    return XDP_DROP;
    val_p = (void *)(long)ctx.data + 1024;
    if (val_p + 1 > (void *)(long)ctx.data_end)
    return XDP_DROP;
    if (*val_p != 0xbb)
    return XDP_DROP;
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
