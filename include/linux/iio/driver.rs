//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/driver.h
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
// Industrial I/O in kernel access map interface.
//
// Copyright (c) 2011 Jonathan Cameron
//
// iio_map_array_register() - tell the core about inkernel consumers
// @indio_dev:	provider device
// @map:	array of mappings specifying association of channel with client
//
// iio_map_array_unregister() - tell the core to remove consumer mappings for
// the given provider device
// @indio_dev:	provider device
//
extern "C" {
    pub fn iio_map_array_unregister(indio_dev: *mut iio_dev) -> c_int;
}
//
// devm_iio_map_array_register - device-managed version of iio_map_array_register
// @dev:	Device object to which to bind the unwinding of this registration
// @indio_dev:	Pointer to the iio_dev structure
// @maps:	Pointer to an IIO map object which is to be registered to this IIO device
//
// This function will call iio_map_array_register() to register an IIO map object
// and will also hook a callback to the iio_map_array_unregister() function to
// handle de-registration of the IIO map object when the device's refcount goes to
// zero.
//
