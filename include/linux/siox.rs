//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/siox.h
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
// Copyright (C) 2015 Pengutronix, Uwe Kleine-König <kernel@pengutronix.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siox_device {
    pub /: *mut *mut list_head node; / node in smaster->devices,
    pub smaster: *mut siox_master,
    pub dev: device,
    pub type: *const c_char,
    pub inbytes: usize,
    pub outbytes: usize,
    pub statustype: u8,
    pub status_read_clean: u8,
    pub status_written: u8,
    pub status_written_lastcycle: u8,
    pub connected: bool,
// statistics
    pub watchdog_errors: c_uint,
    pub status_errors: c_uint,
    pub status_errors_kn: *mut kernfs_node,
    pub watchdog_kn: *mut kernfs_node,
    pub watchdog_errors_kn: *mut kernfs_node,
    pub connected_kn: *mut kernfs_node,
}

extern "C" {
    pub fn siox_device_synced(sdevice: *mut siox_device) -> bool;
}
extern "C" {
    pub fn siox_device_connected(sdevice: *mut siox_device) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siox_driver {
    pub sdevice): *mut *mut int (probe)(struct siox_device,
    pub sdevice): *mut *mut void (remove)(struct siox_device,
    pub sdevice): *mut *mut void (shutdown)(struct siox_device,
//
// buf is big enough to hold sdev->inbytes - 1 bytes, the status byte
// is in the scope of the framework.
//
    pub buf[]): *mut *mut *mut int (set_data)(struct siox_device sdevice, u8 status, u8,
//
// buf is big enough to hold sdev->outbytes - 1 bytes, the status byte
// is in the scope of the framework
//
    pub buf[]): *const *const *const int (get_data)(struct siox_device sdevice, u8,
    pub driver: device_driver,
}

extern "C" {
    pub fn container_of(_arg: driver, siox_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn __siox_driver_register(sdriver: *mut siox_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn __siox_driver_register(_arg: sdriver, _arg: THIS_MODULE) -> return;
}
extern "C" {
    pub fn driver_unregister(_arg: &sdriver->driver) -> return;
}
//
// module_siox_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//
