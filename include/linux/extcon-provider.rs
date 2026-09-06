//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/extcon-provider.h
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
// External Connector (extcon) framework
// - linux/include/linux/extcon-provider.h for extcon provider device driver.
//
// Copyright (C) 2017 Samsung Electronics
// Author: Chanwoo Choi <cw00.choi@samsung.com>
//

// Following APIs register/unregister the extcon device.
extern "C" {
    pub fn extcon_dev_register(edev: *mut extcon_dev) -> c_int;
}
extern "C" {
    pub fn extcon_dev_unregister(edev: *mut extcon_dev);
}
// Following APIs allocate/free the memory of the extcon device.
extern "C" {
    pub fn extcon_dev_free(edev: *mut extcon_dev);
}
extern "C" {
    pub fn devm_extcon_dev_free(dev: *mut device, edev: *mut extcon_dev);
}
// Synchronize the state and property value for each external connector.
extern "C" {
    pub fn extcon_sync(edev: *mut extcon_dev, id: c_uint) -> c_int;
}
//
// Following APIs set the connected state of each external connector.
// The 'id' argument indicates the defined external connector.
//
// Following APIs set the property of each external connector.
// The 'id' argument indicates the defined external connector
// and the 'prop' indicates the extcon property.
//
// And extcon_set_property_capability() set the capability of the property
// for each external connector. They are used to set the capability of the
// property of each external connector based on the id and property.
//

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

