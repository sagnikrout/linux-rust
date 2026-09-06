//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/xdp_lb_bench_common.h
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const MAX_VIPS: c_int = 16;

pub const MAX_REALS: c_int = 512;

pub const PCKT_FRAGMENTED: c_uint = 0x3FFF;

pub const IPIP_V6_PREFIX2: c_int = 0;
pub const IPIP_V6_PREFIX3: c_int = 0;
// Stats indices (0..MAX_VIPS-1 are per-VIP packet/byte counters)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_key {
    pub src: __be32,
    pub srcv6: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_definition {
    pub vip: __be32,
    pub vipv6: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vip_meta {
    pub flags: __u32,
    pub vip_num: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_pos_lru {
    pub pos: __u32,
    pub atime: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_definition {
    pub dst: __be32,
    pub dstv6: [__be32; 4],
    pub flags: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lb_stats {
    pub v1: __u64,
    pub v2: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_value {
    pub mac: [__u8; 6],
    pub pad: [__u8; 2],
}
