//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy_link_topology.h
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
//
// PHY device list allow maintaining a list of PHY devices that are
// part of a netdevice's link topology. PHYs can for example be chained,
// as is the case when using a PHY that exposes an SFP module, on which an
// SFP transceiver that embeds a PHY is connected.
//
// This list can then be used by userspace to leverage individual PHY
// capabilities.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_link_topology {
    pub phys: xarray,
    pub next_phy_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_device_node {
    pub upstream_type: phy_upstream,
    pub netdev: *mut net_device,
    pub phydev: *mut phy_device,
    pub upstream: },
    pub parent_sfp_bus: *mut sfp_bus,
    pub phy: *mut phy_device,
}

extern "C" {
    pub fn phy_link_topo_del_phy(dev: *mut net_device, phy: *mut phy_device);
}

