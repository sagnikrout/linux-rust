//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/platform_pci.h
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
pub const XEN_IOPORT_MAGIC_VAL: c_uint = 0x49d2;
pub const XEN_IOPORT_LINUX_PRODNUM: c_uint = 0x0003;
pub const XEN_IOPORT_LINUX_DRVVER: c_uint = 0x0001;
pub const XEN_IOPORT_BASE: c_uint = 0x10;

extern "C" {
    pub fn xen_has_pv_devices() -> bool;
}
extern "C" {
    pub fn xen_has_pv_disk_devices() -> bool;
}
extern "C" {
    pub fn xen_has_pv_nic_devices() -> bool;
}
extern "C" {
    pub fn xen_has_pv_and_legacy_disk_devices() -> bool;
}

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_XEN) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_XEN) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_XEN) -> return;
}

