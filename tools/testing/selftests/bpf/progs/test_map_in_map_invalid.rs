//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_map_in_map_invalid.c
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
// Copyright (c) 2021 Isovalent, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub __u32): __type(key,,
    pub int): __type(value,,
    pub 4): __uint(max_entries,,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 0); /* This will make map creation to fail */
    __type(key, __u32);
    __array(values, struct inner);
    } mim SEC(".maps");
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_noop0(ctx: *mut xdp_md) -> c_int {
    int xdp_noop0(struct xdp_md *ctx)
    {
    return XDP_PASS;
    }
    char _license[] SEC("license") = "GPL";
