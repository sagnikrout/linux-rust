//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/macio.h
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

// MacIO device driver is defined later
pub const MACIO_DEV_COUNT_RESOURCES: c_int = 8;
pub const MACIO_DEV_COUNT_IRQS: c_int = 8;
//
// the macio_bus structure is used to describe a "virtual" bus
// within a MacIO ASIC. It's typically provided by a macio_pci_asic
// PCI device, but could be provided differently as well (nubus
// machines using a fake OF tree).
//
// The pdev field can be NULL on non-PCI machines
//

//
// the macio_dev structure is used to describe a device
// within an Apple MacIO ASIC.
//

extern "C" {
    pub fn macio_dev_put(dev: *mut macio_dev);
}
//
// Accessors to resources & interrupts and other device
// fields
//
extern "C" {
    pub fn resource_size(_arg: res) -> return;
}
extern "C" {
    pub fn macio_enable_devres(dev: *mut macio_dev) -> c_int;
}
extern "C" {
    pub fn macio_request_resource(dev: *mut macio_dev, resource_no: c_int, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn macio_release_resource(dev: *mut macio_dev, resource_no: c_int);
}
extern "C" {
    pub fn macio_request_resources(dev: *mut macio_dev, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn macio_release_resources(dev: *mut macio_dev);
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->ofdev.dev) -> return;
}

//
// A driver for a mac-io chip based device
//

extern "C" {
    pub fn macio_register_driver(: *mut macio_driver) -> c_int;
}
extern "C" {
    pub fn macio_unregister_driver(: *mut macio_driver);
}

