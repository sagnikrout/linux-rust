//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ccwgroup.h
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
// struct ccwgroup_device - ccw group device
// @state: online/offline state
// @count: number of attached slave devices
// @dev: embedded device structure
// @cdev: variable number of slave devices, allocated as needed
// @ungroup_work: used to ungroup the ccwgroup device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccwgroup_device {
    pub state: },
// private:
    pub onoff: core::sync::atomic::AtomicI32,
    pub reg_mutex: mutex,
// public:
    pub count: c_uint,
    pub dev: device,
    pub ungroup_work: work_struct,
    pub cdev: [*mut ccw_device; ],
}

//
// struct ccwgroup_driver - driver for ccw group devices
// @setup: function called during device creation to setup the device
// @remove: function called on remove
// @set_online: function called when device is set online
// @set_offline: function called when device is set offline
// @shutdown: function called when device is shut down
// @driver: embedded driver structure
// @ccw_driver: supported ccw_driver (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccwgroup_driver {
    pub ): *mut *mut int (setup) (struct ccwgroup_device,
    pub ): *mut *mut void (remove) (struct ccwgroup_device,
    pub ): *mut *mut int (set_online) (struct ccwgroup_device,
    pub ): *mut *mut int (set_offline) (struct ccwgroup_device,
    pub ): *mut *mut void (shutdown)(struct ccwgroup_device,
    pub driver: device_driver,
    pub ccw_driver: *mut ccw_driver,
}

extern "C" {
    pub fn ccwgroup_driver_register(cdriver: *mut ccwgroup_driver) -> c_int;
}
extern "C" {
    pub fn ccwgroup_driver_unregister(cdriver: *mut ccwgroup_driver);
}
extern "C" {
    pub fn ccwgroup_set_online(gdev: *mut ccwgroup_device) -> c_int;
}
extern "C" {
    pub fn ccwgroup_set_offline(gdev: *mut ccwgroup_device, call_gdrv: bool) -> c_int;
}
extern "C" {
    pub fn ccwgroup_probe_ccwdev(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccwgroup_remove_ccwdev(cdev: *mut ccw_device);
}

extern "C" {
    pub fn dev_is_ccwgroup(dev: *mut device) -> bool;
}

