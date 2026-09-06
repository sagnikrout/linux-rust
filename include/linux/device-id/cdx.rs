//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/cdx.h
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
// struct cdx_device_id - CDX device identifier
// @vendor: Vendor ID
// @device: Device ID
// @subvendor: Subsystem vendor ID (or CDX_ANY_ID)
// @subdevice: Subsystem device ID (or CDX_ANY_ID)
// @class: Device class
// Most drivers do not need to specify class/class_mask
// as vendor/device is normally sufficient.
// @class_mask: Limit which sub-fields of the class field are compared.
// @override_only: Match only when dev->driver_override is this driver.
//
// Type of entries in the "device Id" table for CDX devices supported by
// a CDX device driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdx_device_id {
    pub vendor: __u16,
    pub device: __u16,
    pub subvendor: __u16,
    pub subdevice: __u16,
    pub class: __u32,
    pub class_mask: __u32,
    pub override_only: __u32,
}
