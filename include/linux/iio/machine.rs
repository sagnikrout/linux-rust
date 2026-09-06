//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/machine.h
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
// Industrial I/O in kernel access map definitions for board files.
//
// Copyright (c) 2011 Jonathan Cameron
//
// struct iio_map - description of link between consumer and device channels
// @adc_channel_label:	Label used to identify the channel on the provider.
// This is matched against the datasheet_name element
// of struct iio_chan_spec.
// @consumer_dev_name:	Name to uniquely identify the consumer device.
// @consumer_channel:	Unique name used to identify the channel on the
// consumer side.
// @consumer_data:	Data about the channel for use by the consumer driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_map {
    pub adc_channel_label: *const c_char,
    pub consumer_dev_name: *const c_char,
    pub consumer_channel: *const c_char,
    pub consumer_data: *mut c_void,
}

