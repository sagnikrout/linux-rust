//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/siena/efx_common.h
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
    pub fn efx_siena_fini_io(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_fini_struct(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_siena_link_clear_advertising(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_link_set_wanted_fc(efx: *mut efx_nic, wanted_fc: u8);
}
extern "C" {
    pub fn efx_siena_start_all(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_stop_all(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_create_reset_workqueue() -> c_int;
}
extern "C" {
    pub fn efx_siena_queue_reset_work(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_flush_reset_workqueue(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_destroy_reset_workqueue();
}
extern "C" {
    pub fn efx_siena_start_monitor(efx: *mut efx_nic);
}
extern "C" {
    pub fn __efx_siena_reconfigure_port(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_reconfigure_port(efx: *mut efx_nic) -> c_int;
}

extern "C" {
    pub fn efx_siena_try_recovery(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_reset_down(efx: *mut efx_nic, method: reset_type);
}
extern "C" {
    pub fn efx_siena_watchdog(net_dev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn efx_siena_reset_up(efx: *mut efx_nic, method: reset_type, ok: bool) -> c_int;
}
extern "C" {
    pub fn efx_siena_reset(efx: *mut efx_nic, method: reset_type) -> c_int;
}
extern "C" {
    pub fn efx_siena_schedule_reset(efx: *mut efx_nic, type: reset_type);
}
// Dummy PHY ops for PHY drivers
extern "C" {
    pub fn efx_siena_port_dummy_op_int(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_siena_port_dummy_op_void(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_siena_init_mcdi_logging(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_fini_mcdi_logging(efx: *mut efx_nic);
}

extern "C" {
    pub fn efx_siena_mac_reconfigure(efx: *mut efx_nic, mtu_only: bool);
}
extern "C" {
    pub fn efx_siena_set_mac_address(net_dev: *mut net_device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn efx_siena_set_rx_mode(net_dev: *mut net_device);
}
extern "C" {
    pub fn efx_siena_set_features(net_dev: *mut net_device, data: netdev_features_t) -> c_int;
}
extern "C" {
    pub fn efx_siena_link_status_changed(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_siena_xdp_max_mtu(efx: *mut efx_nic) -> c_uint;
}
extern "C" {
    pub fn efx_siena_change_mtu(net_dev: *mut net_device, new_mtu: c_int) -> c_int;
}
