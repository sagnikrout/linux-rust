//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mdev.h
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
// Mediated device definition
//
// Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
// Author: Neo Jia <cjia@nvidia.com>
// Kirti Wankhede <kwankhede@nvidia.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdev_device {
    pub dev: device,
    pub uuid: guid_t,
    pub next: list_head,
    pub type: *mut mdev_type,
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdev_type {
// set by the driver before calling mdev_register parent:
    pub sysfs_name: *const c_char,
    pub pretty_name: *const c_char,
// set by the core, can be used drivers
    pub parent: *mut mdev_parent,
// internal only
    pub kobj: kobject,
    pub devices_kobj: *mut kobject,
}

// embedded into the struct device that the mdev devices hang off
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdev_parent {
    pub dev: *mut device,
    pub mdev_driver: *mut mdev_driver,
    pub mdev_types_kset: *mut kset,
// Synchronize device creation/removal with parent unregistration
    pub unreg_sem: rw_semaphore,
    pub types: *mut mdev_type,
    pub nr_types: c_uint,
    pub available_instances: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn container_of(_arg: dev, mdev_device: struct, _arg: dev) -> return;
}
//
// struct mdev_driver - Mediated device driver
// @device_api: string to return for the device_api sysfs
// @max_instances: maximum number of instances supported (optional)
// @probe: called when new device created
// @remove: called when device removed
// @get_available: Return the max number of instances that can be created
// @show_description: Print a description of the mtype
// @driver: device driver structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdev_driver {
    pub device_api: *const c_char,
    pub max_instances: c_uint,
    pub dev): *mut *mut int (probe)(struct mdev_device,
    pub dev): *mut *mut void (remove)(struct mdev_device,
    pub mtype): *mut *mut unsigned int (get_available)(struct mdev_type,
    pub buf): *mut *mut *mut ssize_t (show_description)(struct mdev_type mtype, char,
    pub driver: device_driver,
}

extern "C" {
    pub fn mdev_unregister_parent(parent: *mut mdev_parent);
}
extern "C" {
    pub fn mdev_register_driver(drv: *mut mdev_driver) -> c_int;
}
extern "C" {
    pub fn mdev_unregister_driver(drv: *mut mdev_driver);
}
