//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acpi_drivers.h
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
//
// acpi_drivers.h  ($Revision: 31 $)
//
// Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
// Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
//
pub const ACPI_MAX_STRING: c_int = 80;
//
// _HID definitions
// HIDs must conform to ACPI spec(6.1.4)
// Linux specific HIDs do not apply to this and begin with LNX:
//

// SMBUS HID definition as supported by Microsoft Windows

// Quirk for broken IBM BIOSes

//
// For fixed hardware buttons, we fabricate acpi_devices with HID
// ACPI_BUTTON_HID_POWERF or ACPI_BUTTON_HID_SLEEPF.  Fixed hardware
// signals only an event; it doesn't supply a notification value.
// To allow drivers to treat notifications from fixed hardware the
// same as those from real devices, we turn the events into this
// notification value.
//
pub const ACPI_FIXED_HARDWARE_EVENT: c_uint = 0x100;
// --------------------------------------------------------------------------
// ACPI PCI Interrupt Link
extern "C" {
    pub fn acpi_irq_penalty_init() -> c_int;
}
extern "C" {
    pub fn acpi_pci_link_free_irq(handle: acpi_handle) -> c_int;
}
// ACPI PCI Device Binding

extern "C" {
    pub fn acpi_dev_get_pci_dev(_arg: acpi_fetch_acpi_dev(handle)) -> return;
}
// Arch-defined function to add a bus to the system

extern "C" {
    pub fn pci_acpi_crs_quirks();
}

// --------------------------------------------------------------------------

extern "C" {
    pub fn is_dock_device(adev: *mut acpi_device) -> c_int;
}

