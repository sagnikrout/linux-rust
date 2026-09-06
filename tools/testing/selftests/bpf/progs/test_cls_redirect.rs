//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/test_cls_redirect.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright 2019, 2020 Cloudflare

// offsetof() is used in static asserts, and the libbpf-redefined CO-RE
// friendly version breaks compilation for older clang versions <= 15
// when invoked in a static assert.  Restore original here.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_base_hdr {
    pub flags: u16,
    pub protocol: u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr {

    pub 2: uint8_t hlen : 5, control : 1, variant :,

    pub 5: uint8_t variant : 2, control : 1, hlen :,

    pub proto_ctype: u8,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unigue {

    pub 4: uint8_t _r : 2, last_hop_gre : 1, forward_syn : 1, version :,

    pub 2: uint8_t version : 4, forward_syn : 1, last_hop_gre : 1, _r :,

    pub reserved: u8,
    pub next_hop: u8,
    pub hop_count: u8,
// Next hops go here
    pub __attribute__((packed)): },
    pub eth: ethhdr,
    pub ip: iphdr,
    pub gre: gre_base_hdr,
// C attribute field omitted
    pub eth: ethhdr,
    pub ip: iphdr,
    pub udp: udphdr,
    pub gue: guehdr,
    pub unigue: unigue,
// C attribute field omitted
