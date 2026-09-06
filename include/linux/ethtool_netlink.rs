//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ethtool_netlink.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ethtool_multicast_groups {
    ETHNL_MCGRP_MONITOR,
}

extern "C" {
    pub fn ethnl_cable_test_alloc(phydev: *mut phy_device, cmd: u8) -> c_int;
}
extern "C" {
    pub fn ethnl_cable_test_free(phydev: *mut phy_device);
}
extern "C" {
    pub fn ethnl_cable_test_finished(phydev: *mut phy_device);
}
extern "C" {
    pub fn ethnl_cable_test_amplitude(phydev: *mut phy_device, pair: u8, mV: i16) -> c_int;
}
extern "C" {
    pub fn ethnl_cable_test_pulse(phydev: *mut phy_device, mV: u16) -> c_int;
}
extern "C" {
    pub fn ethtool_dev_mm_supported(dev: *mut net_device) -> bool;
}
extern "C" {
    pub fn ethnl_pse_send_ntf(netdev: *mut net_device, notif: c_ulong);
}

