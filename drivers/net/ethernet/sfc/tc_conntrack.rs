//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/tc_conntrack.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for Solarflare network controllers and boards
// Copyright 2023, Advanced Micro Devices, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_ct_zone {
    pub zone: u16,
    pub linkage: rhash_head,
    pub ref: refcount_t,
    pub nf_ft: *mut nf_flowtable,
    pub efx: *mut efx_nic,
    pub /: *mut *mut mutex mutex; / protects cts list,
    pub /: *mut *mut list_head cts; / list of efx_tc_ct_entry in this zone,
}

// create/uncreate/teardown hashtables
extern "C" {
    pub fn efx_tc_init_conntrack(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_tc_destroy_conntrack(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_tc_fini_conntrack(efx: *mut efx_nic);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_ct_entry {
    pub cookie: c_ulong,
    pub linkage: rhash_head,
    pub eth_proto: __be16,
    pub ip_proto: u8,
    pub dnat: bool,
    pub nat_ip: __be32 src_ip, dst_ip,,
    pub dst_ip6: in6_addr src_ip6,,
    pub /: *mut *mut __be16 l4_sport, l4_dport, l4_natport; / Ports (UDP, TCP),
    pub zone: *mut efx_tc_ct_zone,
    pub mark: u32,
    pub cnt: *mut efx_tc_counter,
    pub /: *mut *mut list_head list; / entry on zone->cts,
}

