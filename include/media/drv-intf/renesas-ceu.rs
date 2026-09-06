//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/renesas-ceu.h
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


// SPDX-License-Identifier: GPL-2.0
//
// renesas-ceu.h - Renesas CEU driver interface
//
// Copyright 2017-2018 Jacopo Mondi <jacopo+renesas@jmondi.org>
//
pub const CEU_MAX_SUBDEVS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceu_async_subdev {
    pub flags: c_ulong,
    pub bus_width: c_uchar,
    pub bus_shift: c_uchar,
    pub i2c_adapter_id: c_uint,
    pub i2c_address: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceu_platform_data {
    pub num_subdevs: c_uint,
    pub subdevs: [ceu_async_subdev; CEU_MAX_SUBDEVS],
}
