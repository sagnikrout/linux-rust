//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/mcdi_port_common.h
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
// Copyright 2018 Solarflare Communications Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_mcdi_phy_data {
    pub flags: u32,
    pub type: u32,
    pub supported_cap: u32,
    pub channel: u32,
    pub port: u32,
    pub stats_mask: u32,
    pub name: [u8; 20],
    pub media: u32,
    pub mmd_mask: u32,
    pub revision: [u8; 20],
    pub forced_cap: u32,
}

extern "C" {
    pub fn efx_mcdi_get_phy_cfg(efx: *mut efx_nic, cfg: *mut efx_mcdi_phy_data) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_loopback_modes(efx: *mut efx_nic, loopback_modes: *mut u64) -> c_int;
}
extern "C" {
    pub fn mcdi_to_ethtool_linkset(media: u32, cap: u32, linkset: *mut c_ulong);
}
extern "C" {
    pub fn ethtool_linkset_to_mcdi_cap(linkset: *const c_ulong) -> u32;
}
extern "C" {
    pub fn efx_get_mcdi_phy_flags(efx: *mut efx_nic) -> u32;
}
extern "C" {
    pub fn mcdi_to_ethtool_media(media: u32) -> u8;
}
extern "C" {
    pub fn ethtool_fec_caps_to_mcdi(supported_cap: u32, ethtool_cap: u32) -> u32;
}
extern "C" {
    pub fn mcdi_fec_caps_to_ethtool(caps: u32, is_25g: bool) -> u32;
}
extern "C" {
    pub fn efx_mcdi_phy_check_fcntl(efx: *mut efx_nic, lpa: u32);
}
extern "C" {
    pub fn efx_mcdi_phy_poll(efx: *mut efx_nic) -> bool;
}
extern "C" {
    pub fn efx_mcdi_phy_probe(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_remove(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_phy_get_link_ksettings(efx: *mut efx_nic, cmd: *mut ethtool_link_ksettings);
}
extern "C" {
    pub fn efx_mcdi_phy_set_link_ksettings(efx: *mut efx_nic, cmd: *const ethtool_link_ksettings) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_get_fecparam(efx: *mut efx_nic, fec: *mut ethtool_fecparam) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_set_fecparam(efx: *mut efx_nic, fec: *const ethtool_fecparam) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_test_alive(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_port_reconfigure(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_run_tests(efx: *mut efx_nic, results: *mut c_int, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_get_module_eeprom(efx: *mut efx_nic, ee: *mut ethtool_eeprom, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_phy_get_module_info(efx: *mut efx_nic, modinfo: *mut ethtool_modinfo) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_set_mac(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_set_mtu(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_mac_init_stats(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_mac_fini_stats(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_mcdi_port_get_number(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_mcdi_process_link_change(efx: *mut efx_nic, ev: *mut efx_qword_t);
}
