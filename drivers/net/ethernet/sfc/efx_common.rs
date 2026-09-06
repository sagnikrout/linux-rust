//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/efx_common.h
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
extern "C" {
    pub fn efx_fini_io(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_init_struct(efx: *mut efx_nic, pci_dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn efx_fini_struct(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_link_clear_advertising(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_link_set_wanted_fc(efx: *mut efx_nic, _arg: u8);
}
extern "C" {
    pub fn efx_start_all(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_stop_all(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_net_stats(net_dev: *mut net_device, stats: *mut rtnl_link_stats64);
}
extern "C" {
    pub fn efx_create_reset_workqueue() -> c_int;
}
extern "C" {
    pub fn efx_queue_reset_work(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_flush_reset_workqueue(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_destroy_reset_workqueue();
}
extern "C" {
    pub fn efx_start_monitor(efx: *mut efx_nic);
}
extern "C" {
    pub fn __efx_reconfigure_port(efx: *mut efx_nic) -> c_int;
}

extern "C" {
    pub fn efx_try_recovery(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_reset_down(efx: *mut efx_nic, method: reset_type);
}
extern "C" {
    pub fn efx_watchdog(net_dev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn efx_reset_up(efx: *mut efx_nic, method: reset_type, ok: bool) -> c_int;
}
extern "C" {
    pub fn efx_reset(efx: *mut efx_nic, method: reset_type) -> c_int;
}
extern "C" {
    pub fn efx_schedule_reset(efx: *mut efx_nic, type: reset_type);
}
// Dummy PHY ops for PHY drivers
extern "C" {
    pub fn efx_port_dummy_op_int(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_port_dummy_op_void(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_init_mcdi_logging(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_fini_mcdi_logging(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_mac_reconfigure(efx: *mut efx_nic, mtu_only: bool);
}
extern "C" {
    pub fn efx_set_mac_address(net_dev: *mut net_device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn efx_set_rx_mode(net_dev: *mut net_device);
}
extern "C" {
    pub fn efx_set_features(net_dev: *mut net_device, data: netdev_features_t) -> c_int;
}
extern "C" {
    pub fn efx_link_status_changed(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_xdp_max_mtu(efx: *mut efx_nic) -> c_uint;
}
extern "C" {
    pub fn efx_change_mtu(net_dev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn efx_detach_reps(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_attach_reps(efx: *mut efx_nic);
}
