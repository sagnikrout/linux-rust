//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/acpi.h
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

pub const ACPI_ID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_id {
    pub id: [__u8; ACPI_ID_LEN],
    pub driver_data: kernel_ulong_t,
    pub cls: __u32,
    pub cls_msk: __u32,
}

//
// ACPI_DEVICE_CLASS - macro used to describe an ACPI device with
// the PCI-defined class-code information
//
// @_cls : the class, subclass, prog-if triple for this device
// @_msk : the class mask for this device
//
// This macro is used to create a struct acpi_device_id that matches a
// specific PCI class. The .id and .driver_data fields will be left
// initialized with the default value.
//

