//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsi.h
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
// FSI device & driver interfaces
//
// Copyright (C) IBM Corporation 2016
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_device {
    pub dev: device,
    pub engine_type: u8,
    pub version: u8,
    pub unit: u8,
    pub slave: *mut fsi_slave,
    pub addr: u32,
    pub size: u32,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &fsi_dev->dev) -> return;
}
extern "C" {
    pub fn fsi_device_peek(dev: *mut fsi_device, val: *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_device_id {
    pub engine_type: u8,
    pub version: u8,
}

pub const FSI_VERSION_ANY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_driver {
    pub fsidev): *mut *mut int (probe)(struct fsi_device,
    pub fsidev): *mut *mut void (remove)(struct fsi_device,
    pub drv: device_driver,
    pub id_table: *const fsi_device_id,
}

extern "C" {
    pub fn fsi_driver_register(fsi_drv: *mut fsi_driver) -> c_int;
}
extern "C" {
    pub fn fsi_driver_unregister(fsi_drv: *mut fsi_driver);
}
// module_fsi_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

// direct slave API
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsi_dev_type {
    fsi_dev_cfam,
    fsi_dev_sbefifo,
    fsi_dev_scom,
    fsi_dev_occ
}

extern "C" {
    pub fn fsi_free_minor(dev: dev_t);
}
