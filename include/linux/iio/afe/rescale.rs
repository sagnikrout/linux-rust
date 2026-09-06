//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/afe/rescale.h
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
// Copyright (C) 2018 Axentia Technologies AB
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rescale_cfg {
    pub type: iio_chan_type,
    pub rescale): *mut *mut *mut int (props)(struct device dev, struct rescale,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rescale {
    pub cfg: *const rescale_cfg,
    pub source: *mut iio_channel,
    pub chan: iio_chan_spec,
    pub ext_info: *mut iio_chan_spec_ext_info,
    pub chan_processed: bool,
    pub numerator: i32,
    pub denominator: i32,
    pub offset: i32,
}
