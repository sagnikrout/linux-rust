//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpaa2-mac.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2019, 2024-2026 NXP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_mac_stats {
    pub idx_dma_mem: *mut __le32,
    pub values_dma_mem: *mut __le64,
    pub values_iova: dma_addr_t idx_iova,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_mac {
    pub mc_dev: *mut fsl_mc_device,
    pub state: dpmac_link_state,
    pub net_dev: *mut net_device,
    pub mc_io: *mut fsl_mc_io,
    pub attr: dpmac_attr,
    pub ver_minor: u16 ver_major,,
    pub features: c_ulong,
    pub phylink_config: phylink_config,
    pub phylink: *mut phylink,
    pub if_mode: phy_interface_t,
    pub if_link_type: dpmac_link_type,
    pub pcs: *mut phylink_pcs,
    pub fw_node: *mut fwnode_handle,
    pub serdes_phy: *mut phy,
    pub ethtool_stats: dpaa2_mac_stats,
    pub rmon_stats: dpaa2_mac_stats,
    pub pause_stats: dpaa2_mac_stats,
    pub eth_ctrl_stats: dpaa2_mac_stats,
    pub eth_mac_stats: dpaa2_mac_stats,
}

extern "C" {
    pub fn dpaa2_mac_open(mac: *mut dpaa2_mac) -> c_int;
}
extern "C" {
    pub fn dpaa2_mac_close(mac: *mut dpaa2_mac);
}
extern "C" {
    pub fn dpaa2_mac_connect(mac: *mut dpaa2_mac) -> c_int;
}
extern "C" {
    pub fn dpaa2_mac_disconnect(mac: *mut dpaa2_mac);
}
extern "C" {
    pub fn dpaa2_mac_get_sset_count() -> c_int;
}
extern "C" {
    pub fn dpaa2_mac_get_strings(data: *mut u8);
}
extern "C" {
    pub fn dpaa2_mac_get_ethtool_stats(mac: *mut dpaa2_mac, data: *mut u64);
}
extern "C" {
    pub fn dpaa2_mac_start(mac: *mut dpaa2_mac);
}
extern "C" {
    pub fn dpaa2_mac_stop(mac: *mut dpaa2_mac);
}
