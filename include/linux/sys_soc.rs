//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sys_soc.h
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
// Copyright (C) ST-Ericsson SA 2011
// Author: Lee Jones <lee.jones@linaro.org> for ST-Ericsson.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_device_attribute {
    pub machine: *const c_char,
    pub family: *const c_char,
    pub revision: *const c_char,
    pub serial_number: *const c_char,
    pub soc_id: *const c_char,
    pub data: *const c_void,
    pub custom_attr_group: *const attribute_group,
}

//
// soc_device_register - register SoC as a device
// @soc_plat_dev_attr: Attributes passed from platform to be attributed to a SoC
//
// Returns:
// - %NULL if the SoC bus is not yet registered;
// - on success, the newly allocated &struct soc_device pointer;
// - on failure, a negative error code as an ERR_PTR().
//
// soc_device_unregister - unregister SoC device
// @soc_dev: SoC device to be unregistered
//
extern "C" {
    pub fn soc_device_unregister(soc_dev: *mut soc_device);
}
//
// soc_device_to_device - helper function to fetch struct device
// @soc: Previously registered SoC device container
//
// Returns: &struct device pointer for this @soc
//
// soc_attr_read_machine - retrieve the machine model and store it in
// the soc_device_attribute structure
// @soc_dev_attr: SoC attribute structure to store the model in
//
// Returns:
// 0 on success, negative error number on failure.
//
extern "C" {
    pub fn soc_attr_read_machine(soc_dev_attr: *mut soc_device_attribute) -> c_int;
}

