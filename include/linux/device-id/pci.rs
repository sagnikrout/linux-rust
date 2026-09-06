//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/pci.h
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

pub type kernel_ulong_t = c_ulong;

//
// struct pci_device_id - PCI device ID structure
// @vendor:		Vendor ID to match (or PCI_ANY_ID)
// @device:		Device ID to match (or PCI_ANY_ID)
// @subvendor:		Subsystem vendor ID to match (or PCI_ANY_ID)
// @subdevice:		Subsystem device ID to match (or PCI_ANY_ID)
// @class:		Device class, subclass, and "interface" to match.
// See Appendix D of the PCI Local Bus Spec or
// include/linux/pci_ids.h for a full list of classes.
// Most drivers do not need to specify class/class_mask
// as vendor/device is normally sufficient.
// @class_mask:		Limit which sub-fields of the class field are compared.
// See drivers/scsi/sym53c8xx_2/ for example of usage.
// @driver_data:	Data private to the driver.
// Most drivers don't need to use driver_data field.
// Best practice is to use driver_data as an index
// into a static list of equivalent device types,
// instead of using it as a pointer.
// @override_only:	Match only when dev->driver_override is this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_id {
    pub PCI_ANY_ID*/: *mut *mut __u32 vendor, device; / Vendor and device ID or,
    pub /: *mut *mut __u32 subvendor, subdevice; / Subsystem ID's or PCI_ANY_ID,
    pub /: *mut *mut __u32 class, class_mask; / (class,subclass,prog-if) triplet,
    pub /: *mut *mut kernel_ulong_t driver_data; / Data private to the driver,
    pub override_only: __u32,
}

// pci_epf
pub const PCI_EPF_NAME_SIZE: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_device_id {
    pub name: [c_char; PCI_EPF_NAME_SIZE],
    pub driver_data: kernel_ulong_t,
}
