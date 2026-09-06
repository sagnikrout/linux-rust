//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip.h
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


//
// Copyright (C) 2012 Thomas Petazzoni
//
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

extern "C" {
    pub fn int(: *mut *mut platform_irq_probe_t)(struct platform_device, : *mut device_node) -> typedef;
}
// Undefined on purpose

//
// This macro must be used by the different irqchip drivers to declare
// the association between their DT compatible string and their
// initialization function.
//
// @name: name that must be unique across all IRQCHIP_DECLARE of the
// same file.
// @compat: compatible string of the irqchip driver
// @fn: initialization function
//

extern "C" {
    pub fn platform_irqchip_probe(pdev: *mut platform_device) -> c_int;
}

//
// This macro must be used by the different irqchip drivers to declare
// the association between their version and their initialization function.
//
// @name: name that must be unique across all IRQCHIP_ACPI_DECLARE of the
// same file.
// @subtable: Subtable to be identified in MADT
// @validate: Function to be called on that subtable to check its validity.
// Can be NULL.
// @data: data to be checked by the validate function.
// @fn: initialization function
//

extern "C" {
    pub fn irqchip_init();
}

