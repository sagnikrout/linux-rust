//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/xen/xen-pciback/conf_space_quirks.h
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
// PCI Backend - Data structures for special overlays for broken devices.
//
// Ryan Wilson <hap9@epoch.ncsc.mil>
// Chris Bookholt <hap10@epoch.ncsc.mil>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pcibk_config_quirk {
    pub quirks_list: list_head,
    pub devid: pci_device_id,
    pub pdev: *mut pci_dev,
}

// field);
extern "C" {
    pub fn xen_pcibk_config_quirks_init(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn xen_pcibk_config_field_free(field: *mut config_field);
}
extern "C" {
    pub fn xen_pcibk_config_quirk_release(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn xen_pcibk_field_is_dup(dev: *mut pci_dev, reg: c_uint) -> c_int;
}
