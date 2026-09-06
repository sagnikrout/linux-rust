//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/sysfs.h
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
// The industrial I/O core
//
// Copyright (c) 2008 Jonathan Cameron
//
// General attributes
//
// struct iio_dev_attr - iio specific device attribute
// @dev_attr:	underlying device attribute
// @address:	associated register address
// @l:		list head for maintaining list of dynamically created attrs
// @c:		specification for the underlying channel
// @buffer:	the IIO buffer to which this attribute belongs to (if any)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dev_attr {
    pub dev_attr: device_attribute,
    pub address: u64,
    pub l: list_head,
    pub c: *const iio_chan_spec,
    pub buffer: *mut iio_buffer,
}

//
// struct iio_const_attr - constant device specific attribute
// often used for things like available modes
// @string:	attribute string
// @dev_attr:	underlying device attribute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_const_attr {
    pub string: *const c_char,
    pub dev_attr: device_attribute,
}

// Some attributes will be hard coded (device dependent) and not require an

// Generic attributes of onetype or another
//
// IIO_DEV_ATTR_SAMP_FREQ - sets any internal clock frequency
// @_mode: sysfs file mode/permissions
// @_show: output method for the attribute
// @_store: input method for the attribute
//

//
// IIO_DEV_ATTR_SAMP_FREQ_AVAIL - list available sampling frequencies
// @_show: output method for the attribute
//
// May be mode dependent on some devices
//

//
// IIO_CONST_ATTR_SAMP_FREQ_AVAIL - list available sampling frequencies
// @_string: frequency string for the attribute
//
// Constant version
//

//
// IIO_DEV_ATTR_INT_TIME_AVAIL - list available integration times
// @_show: output method for the attribute
//

//
// IIO_CONST_ATTR_INT_TIME_AVAIL - list available integration times
// @_string: frequency string for the attribute
//
// Constant version
//

