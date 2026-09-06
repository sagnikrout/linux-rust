//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/peci/internal.h
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
// Copyright (c) 2018-2021 Intel Corporation

// PECI CPU address range 0x30-0x37
pub const PECI_BASE_ADDR: c_uint = 0x30;
pub const PECI_DEVICE_NUM_MAX: c_int = 8;
extern "C" {
    pub fn peci_request_free(req: *mut peci_request);
}
extern "C" {
    pub fn peci_request_status(req: *mut peci_request) -> c_int;
}
extern "C" {
    pub fn peci_request_dib_read(req: *mut peci_request) -> u64;
}
extern "C" {
    pub fn peci_request_temp_read(req: *mut peci_request) -> i16;
}
extern "C" {
    pub fn peci_request_data_readb(req: *mut peci_request) -> u8;
}
extern "C" {
    pub fn peci_request_data_readw(req: *mut peci_request) -> u16;
}
extern "C" {
    pub fn peci_request_data_readl(req: *mut peci_request) -> u32;
}
extern "C" {
    pub fn peci_request_data_readq(req: *mut peci_request) -> u64;
}
//
// struct peci_device_id - PECI device data to match
// @data: pointer to driver private data specific to device
// @x86_vfm: device vendor-family-model
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_device_id {
    pub data: *const c_void,
    pub x86_vfm: u32,
}

extern "C" {
    pub fn peci_device_create(controller: *mut peci_controller, addr: u8) -> c_int;
}
extern "C" {
    pub fn peci_device_destroy(device: *mut peci_device);
}
//
// struct peci_driver - PECI driver
// @driver: inherit device driver
// @probe: probe callback
// @remove: remove callback
// @id_table: PECI device match table to decide which device to bind
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peci_driver {
    pub driver: device_driver,
    pub id): *const *const *const int (probe)(struct peci_device device, struct peci_device_id,
    pub device): *mut *mut void (remove)(struct peci_device,
    pub id_table: *const peci_device_id,
}

//
// peci_driver_register() - register PECI driver
// @driver: the driver to be registered
//
// PECI drivers that don't need to do anything special in module init should
// use the convenience "module_peci_driver" macro instead
//
// Return: zero on success, else a negative error code.
//

extern "C" {
    pub fn peci_driver_unregister(driver: *mut peci_driver);
}
//
// module_peci_driver() - helper macro for registering a modular PECI driver
// @__peci_driver: peci_driver struct
//
// Helper macro for PECI drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

extern "C" {
    pub fn peci_controller_scan_devices(controller: *mut peci_controller) -> c_int;
}
