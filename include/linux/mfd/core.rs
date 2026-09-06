//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/core.h
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
// drivers/mfd/mfd-core.h
//
// core MFD support
// Copyright (c) 2006 Ian Molton
// Copyright (c) 2007 Dmitry Baryshkov
//

pub const MFD_DEP_LEVEL_NORMAL: c_int = 0;
pub const MFD_DEP_LEVEL_HIGH: c_int = 1;
// Matches ACPI PNP id, either _HID or _CID, or ACPI _ADR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfd_cell_acpi_match {
    pub pnpid: *const c_char,
    pub adr: c_ulonglong,
}

//
// This struct describes the MFD part ("cell").
// After registration the copy of this structure will become the platform data
// of the resulting platform_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfd_cell {
    pub name: *const c_char,
    pub id: c_int,
    pub level: c_int,
    pub dev): *mut *mut int (suspend)(struct platform_device,
    pub dev): *mut *mut int (resume)(struct platform_device,
// platform data passed to the sub devices drivers
    pub platform_data: *const c_void,
    pub pdata_size: usize,
// Matches ACPI
    pub acpi_match: *const mfd_cell_acpi_match,
// Software node for the device.
    pub swnode: *const software_node,
//
// Device Tree compatible string
// See: Documentation/devicetree/usage-model.rst Chapter 2.2 for details
//
    pub of_compatible: *const c_char,
//
// Address as defined in Device Tree.  Used to complement 'of_compatible'
// (above) when matching OF nodes with devices that have identical
// compatible strings
//
    pub of_reg: u64,
// Set to 'true' to use 'of_reg' (above) - allows for of_reg=0
    pub use_of_reg: bool,
//
// These resources can be specified relative to the parent device.
// For accessing hardware you should use resources from the platform dev
//
    pub num_resources: c_int,
    pub resources: *const resource,
// don't check for resource conflicts
    pub ignore_resource_conflicts: bool,
//
// Disable runtime PM callbacks for this subdevice - see
// pm_runtime_no_callbacks().
//
    pub pm_runtime_no_callbacks: bool,
// A list of regulator supplies that should be mapped to the MFD
// device rather than the child device when requested
//
    pub num_parent_supplies: c_int,
    pub parent_supplies: *const *const c_char,
}

//
// Given a platform device that's been created by mfd_add_devices(), fetch
// the mfd_cell that created it.
//
extern "C" {
    pub fn mfd_remove_devices(parent: *mut device);
}
extern "C" {
    pub fn mfd_remove_devices_late(parent: *mut device);
}
