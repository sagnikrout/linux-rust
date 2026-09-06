//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/dell/dell-wmi-descriptor.h
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
// Dell WMI descriptor driver
//
// Copyright (c) 2017 Dell Inc.
//

// possible return values:
// -ENODEV: Descriptor GUID missing from WMI bus
// -EPROBE_DEFER: probing for dell-wmi-descriptor not yet run
// 0: valid descriptor, successfully probed
// < 0: invalid descriptor, don't probe dependent devices
//
extern "C" {
    pub fn dell_wmi_get_descriptor_valid() -> c_int;
}
extern "C" {
    pub fn dell_wmi_get_interface_version(version: *mut u32) -> bool;
}
extern "C" {
    pub fn dell_wmi_get_size(size: *mut u32) -> bool;
}
extern "C" {
    pub fn dell_wmi_get_hotfix(hotfix: *mut u32) -> bool;
}
