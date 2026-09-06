//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/aperture.h
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


// SPDX-License-Identifier: MIT

extern "C" {
    pub fn __aperture_remove_legacy_vga_devices(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn aperture_remove_conflicting_pci_devices(pdev: *mut pci_dev, name: *const c_char) -> c_int;
}

//
// aperture_remove_all_conflicting_devices - remove all existing framebuffers
// @name: a descriptive name of the requesting driver
//
// This function removes all graphics device drivers. Use this function on systems
// that can have their framebuffer located anywhere in memory.
//
// Returns:
// 0 on success, or a negative errno code otherwise
//
extern "C" {
    pub fn aperture_remove_conflicting_devices(_arg: 0, _arg: (resource_size_t)-1, _arg: name) -> return;
}
