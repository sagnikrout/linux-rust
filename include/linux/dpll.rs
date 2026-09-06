//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dpll.h
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates
// Copyright (c) 2023 Intel and affiliates
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_device_ops {
    pub extack): *mut *mut dpll_mode mode, struct netlink_ext_ack,
    pub extack): *mut dpll_mode mode, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut *mut s32 temp, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpll_ffo_type {
    DPLL_FFO_PORT_RXTX_RATE,
    DPLL_FFO_PIN_DEVICE,

    __DPLL_FFO_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_ffo_param {
    pub type: dpll_ffo_type,
    pub ffo: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_ops {
    pub supported_ffo: c_ulong,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut *mut u64 frequency, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut *mut u32 prio, struct netlink_ext_ack,
    pub extack): *const u32 prio, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut u64 freq, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_frequency {
    pub min: u64,
    pub max: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_phase_adjust_range {
    pub min: i32,
    pub max: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_esync {
    pub freq: u64,
    pub range: *const dpll_pin_frequency,
    pub range_num: u8,
    pub pulse: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_properties {
    pub board_label: *const c_char,
    pub panel_label: *const c_char,
    pub package_label: *const c_char,
    pub type: dpll_pin_type,
    pub capabilities: c_ulong,
    pub freq_supported_num: u32,
    pub freq_supported: *mut dpll_pin_frequency,
    pub phase_range: dpll_pin_phase_adjust_range,
    pub phase_gran: u32,
}

pub const DPLL_DEVICE_CREATED: c_int = 1;
pub const DPLL_DEVICE_DELETED: c_int = 2;
pub const DPLL_DEVICE_CHANGED: c_int = 3;
pub const DPLL_PIN_CREATED: c_int = 4;
pub const DPLL_PIN_DELETED: c_int = 5;
pub const DPLL_PIN_CHANGED: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_device_notifier_info {
    pub dpll: *mut dpll_device,
    pub id: u32,
    pub idx: u32,
    pub clock_id: u64,
    pub type: dpll_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_notifier_info {
    pub pin: *mut dpll_pin,
    pub id: u32,
    pub idx: u32,
    pub clock_id: u64,
    pub fwnode: *const fwnode_handle,
    pub prop: *const dpll_pin_properties,
    pub src_clock_id: u64,
}

extern "C" {
    pub fn dpll_netdev_pin_set(dev: *mut net_device, dpll_pin: *mut dpll_pin);
}
extern "C" {
    pub fn dpll_netdev_pin_clear(dev: *mut net_device);
}

extern "C" {
    pub fn dpll_device_put(dpll: *mut dpll_device, tracker: *mut dpll_tracker);
}

extern "C" {
    pub fn dpll_pin_put(pin: *mut dpll_pin, tracker: *mut dpll_tracker);
}
extern "C" {
    pub fn dpll_pin_fwnode_set(pin: *mut dpll_pin, fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn __dpll_device_change_ntf(dpll: *mut dpll_device) -> c_int;
}
extern "C" {
    pub fn dpll_device_change_ntf(dpll: *mut dpll_device) -> c_int;
}
extern "C" {
    pub fn __dpll_pin_change_ntf(pin: *mut dpll_pin) -> c_int;
}
extern "C" {
    pub fn dpll_pin_change_ntf(pin: *mut dpll_pin) -> c_int;
}
extern "C" {
    pub fn register_dpll_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_dpll_notifier(nb: *mut notifier_block) -> c_int;
}
