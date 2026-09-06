//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/iio_core.h
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
// The industrial I/O core function defs.
//
// Copyright (c) 2008 Jonathan Cameron
//
// These definitions are meant for use only within the IIO core, not individual
// drivers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dev_buffer_pair {
    pub indio_dev: *mut iio_dev,
    pub buffer: *mut iio_buffer,
}

pub const IIO_IOCTL_UNHANDLED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_ioctl_handler {
    pub entry: list_head,
    pub arg): unsigned int cmd, unsigned long,
}

extern "C" {
    pub fn iio_device_ioctl_handler_unregister(h: *mut iio_ioctl_handler);
}
extern "C" {
    pub fn iio_free_chan_devattr_list(attr_list: *mut list_head);
}
extern "C" {
    pub fn iio_format_value(buf: *mut c_char, type: c_uint, size: c_int, vals: *mut c_int) -> isize;
}
// Event interface flags
pub const IIO_BUSY_BIT_POS: c_int = 1;

extern "C" {
    pub fn iio_buffers_alloc_sysfs_and_mask(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_buffers_free_sysfs_and_mask(indio_dev: *mut iio_dev);
}

extern "C" {
    pub fn iio_disable_all_buffers(indio_dev: *mut iio_dev);
}
extern "C" {
    pub fn iio_buffer_wakeup_poll(indio_dev: *mut iio_dev);
}
extern "C" {
    pub fn iio_device_detach_buffers(indio_dev: *mut iio_dev);
}

extern "C" {
    pub fn iio_device_register_eventset(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_device_unregister_eventset(indio_dev: *mut iio_dev);
}
extern "C" {
    pub fn iio_device_wakeup_eventset(indio_dev: *mut iio_dev);
}
extern "C" {
    pub fn iio_event_enabled(ev_int: *const iio_event_interface) -> bool;
}
