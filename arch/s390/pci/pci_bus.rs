//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/pci/pci_bus.h
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
// Copyright IBM Corp. 2020
//
// Author(s):
// Pierre Morel <pmorel@linux.ibm.com>
//

extern "C" {
    pub fn zpci_bus_device_register(zdev: *mut zpci_dev, ops: *mut pci_ops) -> c_int;
}
extern "C" {
    pub fn zpci_bus_device_unregister(zdev: *mut zpci_dev);
}
extern "C" {
    pub fn zpci_bus_scan_bus(zbus: *mut zpci_bus) -> c_int;
}
extern "C" {
    pub fn zpci_bus_get_next(pos: *mut zpci_bus);
}
//
// zpci_bus_for_each - iterate over all the registered zbus objects
// @pos:	a struct zpci_bus * as cursor
//
// Acquires and releases references as the cursor iterates over the registered
// objects. Is tolerant against concurrent removals of objects.
//
// Context: Process context. May sleep.
//

extern "C" {
    pub fn zpci_bus_scan_device(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_bus_remove_device(zdev: *mut zpci_dev, set_error: bool);
}
extern "C" {
    pub fn zpci_release_device(kref: *mut kref);
}
extern "C" {
    pub fn zpci_zdev_put(zdev: *mut zpci_dev);
}
extern "C" {
    pub fn zpci_alloc_domain(domain: c_int) -> c_int;
}
extern "C" {
    pub fn zpci_free_domain(domain: c_int);
}
extern "C" {
    pub fn zpci_setup_bus_resources(zdev: *mut zpci_dev) -> c_int;
}
