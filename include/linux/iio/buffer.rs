//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/buffer.h
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
// The industrial I/O core - generic buffer interfaces.
//
// Copyright (c) 2008 Jonathan Cameron
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_buffer_direction {
    IIO_BUFFER_DIRECTION_IN,
    IIO_BUFFER_DIRECTION_OUT,
}

extern "C" {
    pub fn iio_push_to_buffers(indio_dev: *mut iio_dev, data: *const c_void) -> c_int;
}
extern "C" {
    pub fn iio_pop_from_buffer(buffer: *mut iio_buffer, data: *mut c_void) -> c_int;
}
//
// iio_push_to_buffers_with_timestamp() - push data and timestamp to buffers
// @indio_dev:		iio_dev structure for device.
// @data:		sample data
// @timestamp:		timestamp for the sample data
//
// DEPRECATED: Use iio_push_to_buffers_with_ts() instead.
//
// Returns 0 on success, a negative error code otherwise.
//
// The size of indio_dev->scan_bytes is always aligned to the
// largest scan element's alignment (see iio_compute_scan_bytes()).
// So there may be padding after the timestamp. ts_offset contains
// the offset in bytes that was already computed for correctly
// aligning the timestamp.
//
// (int64_t *)(data + ts_offset) = timestamp;
extern "C" {
    pub fn iio_push_to_buffers(_arg: indio_dev, _arg: data) -> return;
}
//
// iio_push_to_buffers_with_ts() - push data and timestamp to buffers
// @indio_dev:		iio_dev structure for device.
// @data:		Pointer to sample data buffer.
// @data_total_len:	The size of @data in bytes.
// @timestamp:		Timestamp for the sample data.
//
// Pushes data to the IIO device's buffers. If timestamps are enabled for the
// device the function will store the supplied timestamp as the last element in
// the sample data buffer before pushing it to the device buffers. The sample
// data buffer needs to be large enough to hold the additional timestamp
// (usually the buffer should be at least indio->scan_bytes bytes large).
//
// Context: Any context.
// Return: 0 on success, a negative error code otherwise.
//
extern "C" {
    pub fn iio_push_to_buffers_with_timestamp(_arg: indio_dev, _arg: data, _arg: timestamp) -> return;
}
