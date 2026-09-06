//! Automatically rewritten from C Header to Rust Module
//! Source: net/ethtool/common.h
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

// compose link mode index from speed, type and duplex

extern "C" {
    pub fn __ethtool_get_link(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ethtool_check_rss_ctx_busy(dev: *mut net_device, rss_context: u32) -> c_int;
}
extern "C" {
    pub fn ethtool_rxfh_config_is_sym(rxfh: u64) -> c_int;
}
extern "C" {
    pub fn ethtool_get_rx_ring_count(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn __ethtool_get_ts_info(dev: *mut net_device, info: *mut kernel_ethtool_ts_info) -> c_int;
}
extern "C" {
    pub fn __ethtool_dev_mm_supported(dev: *mut net_device) -> bool;
}
//
// ethtool_nl_msg_needs_rtnl() - does this Netlink cmd need rtnl_lock?
// @dev: target device
// @cmd: ETHTOOL_MSG_* Netlink command value
//
// Return: true if @cmd is a command for which @dev has opted-in to
// keeping rtnl_lock held across the call (via op_needs_rtnl).
//
// tsconfig calls ndos (ndo_hwtstamp_set/get), not ethtool ops.
// Also, there is no corresponding ethtool ioctl, therefore
// these cases are Netlink-only.
//
// ethtool_ioctl_needs_rtnl() - does this legacy ioctl cmd need rtnl_lock?
// @dev: target device
// @ethcmd: ETHTOOL_* ioctl command value
//
// Return: true if @ethcmd is a command for which @dev has opted-in to
// keeping rtnl_lock held across the call (via op_needs_rtnl).
//

extern "C" {
    pub fn ethtool_rss_notify(dev: *mut net_device, type: u32, rss_context: u32);
}

