//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_platform.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2006 Benjamin Herrenschmidt, IBM Corp.
// <benh@kernel.crashing.org>
//

//
// struct of_dev_auxdata - lookup table entry for device names & platform_data
// @compatible: compatible value of node to match against node
// @phys_addr: Start address of registers to match against node
// @name: Name to assign for matching nodes
// @platform_data: platform_data to assign for matching nodes
//
// This lookup table allows the caller of of_platform_populate() to override
// the names of devices when creating devices from the device tree.  The table
// should be terminated with an empty entry.  It also allows the platform_data
// pointer to be set.
//
// The reason for this functionality is that some Linux infrastructure uses
// the device name to look up a specific device, but the Linux-specific names
// are not encoded into the device tree, so the kernel needs to provide specific
// values.
//
// Note: Using an auxdata lookup table should be considered a last resort when
// converting a platform to use the DT.  Normally the automatically generated
// device name will not matter, and drivers should obtain data from the device
// node instead of from an anonymous platform_data pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_dev_auxdata {
    pub compatible: *mut c_char,
    pub phys_addr: resource_size_t,
    pub name: *mut c_char,
    pub platform_data: *mut c_void,
}

// Macro to simplify populating a lookup table

// Platform drivers register/unregister
extern "C" {
    pub fn of_device_add(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn of_device_register(ofdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn of_device_unregister(ofdev: *mut platform_device);
}

// Platform devices and busses creation
extern "C" {
    pub fn of_platform_device_destroy(dev: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn of_platform_depopulate(parent: *mut device);
}
extern "C" {
    pub fn devm_of_platform_populate(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn devm_of_platform_depopulate(dev: *mut device);
}

// Platform devices and busses creation

