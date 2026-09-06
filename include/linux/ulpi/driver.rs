//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ulpi/driver.h
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
// struct ulpi - describes ULPI PHY device
// @id: vendor and product ids for ULPI device
// @ops: I/O access
// @dev: device interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulpi {
    pub dev: device,
    pub id: ulpi_device_id,
    pub ops: *const ulpi_ops,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &ulpi->dev) -> return;
}
//
// struct ulpi_driver - describes a ULPI PHY driver
// @id_table: array of device identifiers supported by this driver
// @probe: binds this driver to ULPI device
// @remove: unbinds this driver from ULPI device
// @driver: the name and owner members must be initialized by the drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulpi_driver {
    pub id_table: *const ulpi_device_id,
    pub ulpi): *mut *mut int (probe)(struct ulpi,
    pub ulpi): *mut *mut void (remove)(struct ulpi,
    pub driver: device_driver,
}

//
// use a macro to avoid include chaining to get THIS_MODULE
//

extern "C" {
    pub fn __ulpi_register_driver(drv: *mut ulpi_driver, module: *mut module) -> c_int;
}
extern "C" {
    pub fn ulpi_unregister_driver(drv: *mut ulpi_driver);
}

extern "C" {
    pub fn ulpi_read(ulpi: *mut ulpi, addr: u8) -> c_int;
}
extern "C" {
    pub fn ulpi_write(ulpi: *mut ulpi, addr: u8, val: u8) -> c_int;
}
