//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/iio_core_trigger.h
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
// The industrial I/O core, trigger consumer handling functions
//
// Copyright (c) 2008 Jonathan Cameron
//

//
// iio_device_register_trigger_consumer() - set up an iio_dev to use triggers
// @indio_dev: iio_dev associated with the device that will consume the trigger
//
// Return 0 if successful, negative otherwise
//
extern "C" {
    pub fn iio_device_register_trigger_consumer(indio_dev: *mut iio_dev) -> c_int;
}
//
// iio_device_unregister_trigger_consumer() - reverse the registration process
// @indio_dev: iio_dev associated with the device that consumed the trigger
//
extern "C" {
    pub fn iio_device_unregister_trigger_consumer(indio_dev: *mut iio_dev);
}

//
// iio_device_register_trigger_consumer() - set up an iio_dev to use triggers
// @indio_dev: iio_dev associated with the device that will consume the trigger
//
// iio_device_unregister_trigger_consumer() - reverse the registration process
// @indio_dev: iio_dev associated with the device that consumed the trigger
//
