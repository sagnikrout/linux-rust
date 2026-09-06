//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/lenovo/wmi-capdata.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2025 Derek J. Clark <derekjohn.clark@gmail.com>

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lwmi_device_id {
    LWMI_DEVICE_ID_CPU = 0x01,
    LWMI_DEVICE_ID_GPU = 0x02,
    LWMI_DEVICE_ID_PSU = 0x03,
    LWMI_DEVICE_ID_FAN = 0x04,
}

pub const LWMI_TYPE_ID_NONE: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct capdata00 {
    pub id: u32,
    pub supported: u32,
    pub default_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct capdata01 {
    pub capdata00: struct,
    pub cd00: capdata00,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct capdata_fan {
    pub id: u32,
    pub min_rpm: u32,
    pub max_rpm: u32,
}

extern "C" {
    pub fn void(master_dev: *mut *mut cd_list_cb_t)(struct device, cd_list: *mut cd_list) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lwmi_cd_binder {
    pub cd00_list: *mut cd_list,
    pub cd01_list: *mut cd_list,
//
// May be called during or after the bind callback.
// Will be called with NULL if capdata_fan does not exist.
// The pointer is only valid in the callback; never keep it for later use!
//
    pub cd_fan_list_cb: cd_list_cb_t,
}

//
// lwmi_attr_id() - Formats a capability data attribute ID
// @dev_id: The u8 corresponding to the device ID.
// @feat_id: The u8 corresponding to the feature ID on the device.
// @mode_id: The u8 corresponding to the wmi-gamezone mode for set/get.
// @type_id: The u8 corresponding to the sub-device.
//
// Return: encoded capability data attribute ID.
//
extern "C" {
    pub fn lwmi_cd_match_add_all(master: *mut device, matchptr: *mut component_match);
}
extern "C" {
    pub fn lwmi_cd00_get_data(list: *mut cd_list, attribute_id: u32, output: *mut capdata00) -> c_int;
}
extern "C" {
    pub fn lwmi_cd01_get_data(list: *mut cd_list, attribute_id: u32, output: *mut capdata01) -> c_int;
}
extern "C" {
    pub fn lwmi_cd_fan_get_data(list: *mut cd_list, attribute_id: u32, output: *mut capdata_fan) -> c_int;
}
