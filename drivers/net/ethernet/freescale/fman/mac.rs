//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fman/mac.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2008 - 2015 Freescale Semiconductor Inc.
//

pub const PORT_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_device {
    pub vaddr: *mut void __iomem,
    pub dev: *mut device,
    pub res: *mut resource,
    pub addr: [u8; ETH_ALEN],
    pub port: [*mut fman_port; PORT_NUM],
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub phy_if: phy_interface_t,
    pub promisc: bool,
    pub allmulti: bool,
    pub phylink_ops: *const phylink_mac_ops,
    pub mac_dev): *mut *mut int (enable)(struct fman_mac,
    pub mac_dev): *mut *mut void (disable)(struct fman_mac,
    pub enable): *mut *mut *mut int (set_promisc)(struct fman_mac mac_dev, bool,
    pub enet_addr): *const *const *const int (change_addr)(struct fman_mac mac_dev, enet_addr_t,
    pub enable): *mut *mut *mut int (set_allmulti)(struct fman_mac mac_dev, bool,
    pub enable): *mut *mut *mut int (set_tstamp)(struct fman_mac mac_dev, bool,
    pub enable): fman_mac_exceptions exception, bool,
    pub eth_addr): *mut enet_addr_t,
    pub eth_addr): *mut enet_addr_t,
    pub s): *mut ethtool_pause_stats,
    pub ranges): *const ethtool_rmon_hist_range,
    pub s): *mut ethtool_eth_ctrl_stats,
    pub s): *mut ethtool_eth_mac_stats,
    pub speed): *mut *mut *mut void (update_speed)(struct mac_device mac_dev, int,
    pub fman_mac: *mut fman_mac,
    pub priv: *mut mac_priv_s,
    pub fman_dev: *mut device,
    pub fman_port_devs: [*mut device; PORT_NUM],
}

// fman_config_to_mac(struct phylink_config *config)
extern "C" {
    pub fn container_of(_arg: config, mac_device: struct, _arg: phylink_config) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa_eth_data {
    pub mac_dev: *mut mac_device,
    pub mac_hw_id: c_int,
    pub fman_hw_id: c_int,
}

extern "C" {
    pub fn fman_set_multi(net_dev: *mut net_device, mac_dev: *mut mac_device) -> c_int;
}
