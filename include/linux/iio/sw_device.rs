//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/sw_device.h
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
// Industrial I/O software device interface
//
// Copyright (c) 2016 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_sw_device_type {
    pub name: *const c_char,
    pub owner: *mut module,
    pub ops: *const iio_sw_device_ops,
    pub list: list_head,
    pub group: *mut config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_sw_device {
    pub device: *mut iio_dev,
    pub device_type: *mut iio_sw_device_type,
    pub group: config_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_sw_device_ops {
    pub ): *const *const *const iio_sw_device (probe)(char,
    pub ): *mut *mut int (remove)(struct iio_sw_device,
}

extern "C" {
    pub fn iio_register_sw_device_type(dt: *mut iio_sw_device_type) -> c_int;
}
extern "C" {
    pub fn iio_unregister_sw_device_type(dt: *mut iio_sw_device_type);
}
extern "C" {
    pub fn iio_sw_device_destroy(: *mut iio_sw_device);
}

