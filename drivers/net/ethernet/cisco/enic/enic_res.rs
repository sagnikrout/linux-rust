//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/enic_res.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const ENIC_MIN_WQ_DESCS: c_int = 64;
pub const ENIC_MAX_WQ_DESCS_DEFAULT: c_int = 4096;
pub const ENIC_MAX_WQ_DESCS: c_int = 16384;
pub const ENIC_MIN_RQ_DESCS: c_int = 64;
pub const ENIC_MAX_RQ_DESCS: c_int = 16384;
pub const ENIC_MAX_RQ_DESCS_DEFAULT: c_int = 4096;

pub const ENIC_MAX_MTU: c_int = 9000;
pub const ENIC_MULTICAST_PERFECT_FILTERS: c_int = 32;
pub const ENIC_UNICAST_PERFECT_FILTERS: c_int = 32;
pub const ENIC_NON_TSO_MAX_DESC: c_int = 16;

extern "C" {
    pub fn enic_get_vnic_config(: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_add_vlan(enic: *mut enic, vlanid: u16) -> c_int;
}
extern "C" {
    pub fn enic_del_vlan(enic: *mut enic, vlanid: u16) -> c_int;
}
extern "C" {
    pub fn enic_set_rss_key(enic: *mut enic, key_pa: dma_addr_t, len: u64) -> c_int;
}
extern "C" {
    pub fn enic_set_rss_cpu(enic: *mut enic, cpu_pa: dma_addr_t, len: u64) -> c_int;
}
extern "C" {
    pub fn enic_get_res_counts(enic: *mut enic);
}
extern "C" {
    pub fn enic_init_vnic_resources(enic: *mut enic);
}
extern "C" {
    pub fn enic_alloc_vnic_resources(: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_free_vnic_resources(: *mut enic);
}
