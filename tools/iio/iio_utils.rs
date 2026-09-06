//! Automatically rewritten from C Header to Rust Module
//! Source: tools/iio/iio_utils.h
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
// IIO - useful set of util functionality
//
// Copyright (c) 2008 Jonathan Cameron
//

// Made up value to limit allocation sizes
pub const IIO_MAX_NAME_LENGTH: c_int = 64;

//
// struct iio_channel_info - information about a given channel
// @name: channel name
// @generic_name: general name for channel type
// @scale: scale factor to be applied for conversion to si units
// @offset: offset to be applied for conversion to si units
// @index: the channel index in the buffer output
// @bytes: number of bytes occupied in buffer output
// @bits_used: number of valid bits of data
// @shift: amount of bits to shift right data before applying bit mask
// @mask: a bit mask for the raw output
// @be: flag if data is big endian
// @format: format of the raw value
// @location: data offset for this channel inside the buffer (in bytes)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_channel_info {
    pub name: *mut c_char,
    pub generic_name: *mut c_char,
    pub scale: float,
    pub offset: float,
    pub index: unsigned,
    pub bytes: unsigned,
    pub bits_used: unsigned,
    pub shift: unsigned,
    pub mask: u64,
    pub be: unsigned,
    pub format: c_char,
    pub location: unsigned,
}

extern "C" {
    pub fn iioutils_break_up_name(full_name: *const c_char, generic_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn bsort_channel_array_by_index(ci_array: *mut iio_channel_info, cnt: c_int);
}
extern "C" {
    pub fn find_type_by_name(name: *const c_char, type: *const c_char) -> c_int;
}
extern "C" {
    pub fn write_sysfs_int(filename: *const c_char, basedir: *const c_char, val: c_int) -> c_int;
}
extern "C" {
    pub fn read_sysfs_posint(filename: *const c_char, basedir: *const c_char) -> c_int;
}
extern "C" {
    pub fn read_sysfs_float(filename: *const c_char, basedir: *const c_char, val: *mut float) -> c_int;
}
extern "C" {
    pub fn read_sysfs_string(filename: *const c_char, basedir: *const c_char, str: *mut c_char) -> c_int;
}
