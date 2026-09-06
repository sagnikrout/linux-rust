//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/raspberrypi/vchiq_bus.h
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
// Copyright (c) 2023 Ideas On Board Oy
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_device {
    pub dev: device,
    pub drv_mgmt: *mut vchiq_drv_mgmt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_driver {
    pub device): *mut *mut int (probe)(struct vchiq_device,
    pub device): *mut *mut void (remove)(struct vchiq_device,
    pub device): *mut *mut int (resume)(struct vchiq_device,
    pub state): pm_message_t,
    pub id_table: *const vchiq_device_id,
    pub driver: device_driver,
}

extern "C" {
    pub fn container_of(_arg: d, vchiq_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: d, vchiq_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn vchiq_device_unregister(dev: *mut vchiq_device);
}
extern "C" {
    pub fn vchiq_driver_register(vchiq_drv: *mut vchiq_driver) -> c_int;
}
extern "C" {
    pub fn vchiq_driver_unregister(vchiq_drv: *mut vchiq_driver);
}
//
// module_vchiq_driver() - Helper macro for registering a vchiq driver
// @__vchiq_driver: vchiq driver struct
//
// Helper macro for vchiq drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

