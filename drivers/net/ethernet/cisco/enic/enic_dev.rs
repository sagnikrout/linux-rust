//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/enic_dev.h
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
// Copyright 2011 Cisco Systems, Inc.  All rights reserved.

//
// Calls the devcmd function given by argument vnicdevcmdfn.
// If vf argument is valid, it proxies the devcmd
//

extern "C" {
    pub fn enic_dev_fw_info(enic: *mut enic, fw_info: *mut vnic_devcmd_fw_info) -> c_int;
}
extern "C" {
    pub fn enic_dev_stats_dump(enic: *mut enic, vstats: *mut vnic_stats) -> c_int;
}
extern "C" {
    pub fn enic_dev_add_station_addr(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_del_station_addr(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_add_addr(enic: *mut enic, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn enic_dev_del_addr(enic: *mut enic, addr: *const u8) -> c_int;
}
extern "C" {
    pub fn enic_vlan_rx_add_vid(netdev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn enic_vlan_rx_kill_vid(netdev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn enic_dev_notify_unset(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_hang_notify(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_set_ig_vlan_rewrite_mode(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_enable(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_disable(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_intr_coal_timer_info(enic: *mut enic) -> c_int;
}
extern "C" {
    pub fn enic_dev_status_to_errno(devcmd_status: c_int) -> c_int;
}
