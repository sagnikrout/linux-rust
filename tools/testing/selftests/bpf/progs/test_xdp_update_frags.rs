//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_update_frags.c
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
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

    int _version SEC("version") = 1;
    SEC("xdp.frags")
#[no_mangle]
pub unsafe extern "C" fn xdp_adjust_frags(xdp: *mut xdp_md) -> c_int {
    int xdp_adjust_frags(struct xdp_md *xdp)
    {
    __u8 *data_end = (void *)(long)xdp.data_end;
    __u8 *data = (void *)(long)xdp.data;
    __u8 val[16] = {};
    __u32 offset;
    int err;
    if (data + sizeof(__u32) > data_end)
    return XDP_DROP;
    offset = *(__u32 *)data;
    err = bpf_xdp_load_bytes(xdp, offset, val, sizeof(val));
    if (err < 0)
    return XDP_DROP;
    if (val[0] != 0xaa || val[15] != 0xaa) /* marker */
    return XDP_DROP;
    val[0] = 0xbb; /* update the marker */
    val[15] = 0xbb;
    err = bpf_xdp_store_bytes(xdp, offset, val, sizeof(val));
    if (err < 0)
    return XDP_DROP;
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
