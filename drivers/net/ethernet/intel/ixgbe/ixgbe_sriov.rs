//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_sriov.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
// ixgbe driver limit the max number of VFs could be enabled to
// 63 (IXGBE_MAX_VF_FUNCTIONS - 1)
//

pub const IXGBE_MAX_VFS_4TC: c_int = 32;
pub const IXGBE_MAX_VFS_8TC: c_int = 16;

extern "C" {
    pub fn ixgbe_restore_vf_multicasts(adapter: *mut ixgbe_adapter);
}

extern "C" {
    pub fn ixgbe_check_mdd_event(adapter: *mut ixgbe_adapter) -> bool;
}
extern "C" {
    pub fn ixgbe_msg_task(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_vf_configuration(pdev: *mut pci_dev, event_mask: c_uint) -> c_int;
}
extern "C" {
    pub fn ixgbe_ping_all_vfs(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_set_all_vfs(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_ndo_set_vf_mac(netdev: *mut net_device, queue: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn ixgbe_link_mbps(adapter: *mut ixgbe_adapter) -> c_int;
}
extern "C" {
    pub fn ixgbe_ndo_set_vf_spoofchk(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_ndo_set_vf_trust(netdev: *mut net_device, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn ixgbe_ndo_set_vf_link_state(netdev: *mut net_device, vf: c_int, state: c_int) -> c_int;
}
extern "C" {
    pub fn ixgbe_check_vf_rate_limit(adapter: *mut ixgbe_adapter);
}
extern "C" {
    pub fn ixgbe_set_vf_link_state(adapter: *mut ixgbe_adapter, vf: c_int, state: c_int);
}
extern "C" {
    pub fn ixgbe_disable_sriov(adapter: *mut ixgbe_adapter) -> c_int;
}

extern "C" {
    pub fn ixgbe_enable_sriov(adapter: *mut ixgbe_adapter, max_vfs: c_uint);
}

extern "C" {
    pub fn ixgbe_pci_sriov_configure(dev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
