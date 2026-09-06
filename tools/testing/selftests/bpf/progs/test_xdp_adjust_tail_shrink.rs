//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_xdp_adjust_tail_shrink.c
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
// Copyright (c) 2018 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn _xdp_adjust_tail_shrink(xdp: *mut xdp_md) -> c_int {
    int _xdp_adjust_tail_shrink(struct xdp_md *xdp)
    {
    __u8 *data_end = (void *)(long)xdp.data_end;
    __u8 *data = (void *)(long)xdp.data;
    let mut offset: c_int = 0;
    switch (bpf_xdp_get_buff_len(xdp)) {
    case 54:
// sizeof(pkt_v4)
    offset = 256; /* shrink too much */
    break;
    case 9000:
// non-linear buff test cases
    if (data + 1 > data_end)
    return XDP_DROP;
    switch (data[0]) {
    case 0:
    offset = 10;
    break;
    case 1:
    offset = 4100;
    break;
    case 2:
    offset = 8200;
    break;
    default:
    return XDP_DROP;
    }
    break;
    default:
    offset = 20;
    break;
    }
    if (bpf_xdp_adjust_tail(xdp, 0 - offset))
    return XDP_DROP;
    return XDP_TX;
    }
    char _license[] SEC("license") = "GPL";
