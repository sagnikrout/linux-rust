//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/svc.h
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
// Greybus SVC code
//
// Copyright 2015 Google Inc.
// Copyright 2015 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_svc_state {
    GB_SVC_STATE_RESET,
    GB_SVC_STATE_PROTOCOL_VERSION,
    GB_SVC_STATE_SVC_HELLO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_svc_watchdog_bite {
    GB_SVC_WATCHDOG_BITE_RESET_UNIPRO = 0,
    GB_SVC_WATCHDOG_BITE_PANIC_KERNEL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svc_debugfs_pwrmon_rail {
    pub id: u8,
    pub svc: *mut gb_svc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc {
    pub dev: device,
    pub hd: *mut gb_host_device,
    pub connection: *mut gb_connection,
    pub state: gb_svc_state,
    pub device_id_map: ida,
    pub wq: *mut workqueue_struct,
    pub endo_id: u16,
    pub ap_intf_id: u8,
    pub protocol_major: u8,
    pub protocol_minor: u8,
    pub watchdog: *mut gb_svc_watchdog,
    pub action: gb_svc_watchdog_bite,
    pub debugfs_dentry: *mut dentry,
    pub pwrmon_rails: *mut svc_debugfs_pwrmon_rail,
}

extern "C" {
    pub fn gb_svc_add(svc: *mut gb_svc) -> c_int;
}
extern "C" {
    pub fn gb_svc_del(svc: *mut gb_svc);
}
extern "C" {
    pub fn gb_svc_put(svc: *mut gb_svc);
}
extern "C" {
    pub fn gb_svc_intf_device_id(svc: *mut gb_svc, intf_id: u8, device_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_svc_route_destroy(svc: *mut gb_svc, intf1_id: u8, intf2_id: u8);
}
extern "C" {
    pub fn gb_svc_intf_eject(svc: *mut gb_svc, intf_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_svc_intf_vsys_set(svc: *mut gb_svc, intf_id: u8, enable: bool) -> c_int;
}
extern "C" {
    pub fn gb_svc_intf_refclk_set(svc: *mut gb_svc, intf_id: u8, enable: bool) -> c_int;
}
extern "C" {
    pub fn gb_svc_intf_unipro_set(svc: *mut gb_svc, intf_id: u8, enable: bool) -> c_int;
}
extern "C" {
    pub fn gb_svc_intf_activate(svc: *mut gb_svc, intf_id: u8, intf_type: *mut u8) -> c_int;
}
extern "C" {
    pub fn gb_svc_intf_resume(svc: *mut gb_svc, intf_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_svc_intf_set_power_mode_hibernate(svc: *mut gb_svc, intf_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_svc_ping(svc: *mut gb_svc) -> c_int;
}
extern "C" {
    pub fn gb_svc_watchdog_create(svc: *mut gb_svc) -> c_int;
}
extern "C" {
    pub fn gb_svc_watchdog_destroy(svc: *mut gb_svc);
}
extern "C" {
    pub fn gb_svc_watchdog_enabled(svc: *mut gb_svc) -> bool;
}
extern "C" {
    pub fn gb_svc_watchdog_enable(svc: *mut gb_svc) -> c_int;
}
extern "C" {
    pub fn gb_svc_watchdog_disable(svc: *mut gb_svc) -> c_int;
}
