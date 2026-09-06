//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mei_cl_bus.h
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
// Copyright (c) 2013-2016, Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn void(cldev: *mut *mut mei_cldev_cb_t)(struct mei_cl_device) -> typedef;
}
//
// struct mei_cl_device - MEI device handle
// An mei_cl_device pointer is returned from mei_add_device()
// and links MEI bus clients to their actual ME host client pointer.
// Drivers for MEI devices will get an mei_cl_device pointer
// when being probed and shall use it for doing ME bus I/O.
//
// @bus_list: device on the bus list
// @bus: parent mei device
// @dev: linux driver model device pointer
// @me_cl: me client
// @cl: mei client
// @name: device name
// @rx_work: async work to execute Rx event callback
// @rx_cb: Drivers register this callback to get asynchronous ME
// Rx buffer pending notifications.
// @notif_work: async work to execute FW notify event callback
// @notif_cb: Drivers register this callback to get asynchronous ME
// FW notification pending notifications.
//
// @do_match: whether the device can be matched with a driver
// @is_added: device is already scanned
// @priv_data: client private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_cl_device {
    pub bus_list: list_head,
    pub bus: *mut mei_device,
    pub dev: device,
    pub me_cl: *mut mei_me_client,
    pub cl: *mut mei_cl,
    pub name: [c_char; MEI_CL_NAME_SIZE],
    pub rx_work: work_struct,
    pub rx_cb: mei_cldev_cb_t,
    pub notif_work: work_struct,
    pub notif_cb: mei_cldev_cb_t,
    pub do_match:1: c_uint,
    pub is_added:1: c_uint,
    pub priv_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mei_cl_driver {
    pub driver: device_driver,
    pub name: *const c_char,
    pub id_table: *const mei_cl_device_id,
    pub id): *const mei_cl_device_id,
    pub cldev): *mut *mut void (remove)(struct mei_cl_device,
}

extern "C" {
    pub fn mei_cldev_driver_unregister(cldrv: *mut mei_cl_driver);
}
//
// module_mei_cl_driver - Helper macro for registering mei cl driver
//
// @__mei_cldrv: mei_cl_driver structure
//
// Helper macro for mei cl drivers which do not do anything special in module
// init/exit, for eliminating a boilerplate code.
//

extern "C" {
    pub fn mei_cldev_recv(cldev: *mut mei_cl_device, buf: *mut u8, length: usize) -> isize;
}
extern "C" {
    pub fn mei_cldev_register_rx_cb(cldev: *mut mei_cl_device, rx_cb: mei_cldev_cb_t) -> c_int;
}
extern "C" {
    pub fn mei_cldev_ver(cldev: *const mei_cl_device) -> u8;
}
extern "C" {
    pub fn mei_cldev_mtu(cldev: *const mei_cl_device) -> usize;
}
extern "C" {
    pub fn mei_cldev_set_drvdata(cldev: *mut mei_cl_device, data: *mut c_void);
}
extern "C" {
    pub fn mei_cldev_enable(cldev: *mut mei_cl_device) -> c_int;
}
extern "C" {
    pub fn mei_cldev_disable(cldev: *mut mei_cl_device) -> c_int;
}
extern "C" {
    pub fn mei_cldev_enabled(cldev: *const mei_cl_device) -> bool;
}
extern "C" {
    pub fn mei_cldev_dma_unmap(cldev: *mut mei_cl_device) -> c_int;
}
