//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/ethtool_common.h
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
// Copyright 2019 Solarflare Communications Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
extern "C" {
    pub fn efx_siena_ethtool_get_msglevel(net_dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn efx_siena_ethtool_set_msglevel(net_dev: *mut net_device, msg_enable: u32);
}
extern "C" {
    pub fn efx_siena_ethtool_get_sset_count(net_dev: *mut net_device, string_set: c_int) -> c_int;
}
extern "C" {
    pub fn efx_siena_ethtool_get_rx_ring_count(net_dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn efx_siena_ethtool_get_rxfh_indir_size(net_dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn efx_siena_ethtool_get_rxfh_key_size(net_dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn efx_siena_ethtool_reset(net_dev: *mut net_device, flags: *mut u32) -> c_int;
}
