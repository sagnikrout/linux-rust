//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/misc/ad714x.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AD714X CapTouch Programmable Controller driver (bus interfaces)
//
// Copyright 2009-2011 Analog Devices Inc.
//

pub const STAGE_NUM: c_int = 12;
extern "C" {
    pub fn int(: *mut *mut ad714x_read_t)(struct ad714x_chip, short: unsigned, : *mut c_ushort, _arg: usize) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut ad714x_write_t)(struct ad714x_chip, short: unsigned, short: unsigned) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad714x_chip {
    pub l_state: c_ushort,
    pub h_state: c_ushort,
    pub c_state: c_ushort,
    pub adc_reg: [c_ushort; STAGE_NUM],
    pub amb_reg: [c_ushort; STAGE_NUM],
    pub sensor_val: [c_ushort; STAGE_NUM],
    pub hw: *mut ad714x_platform_data,
    pub sw: *mut ad714x_driver_data,
    pub irq: c_int,
    pub dev: *mut device,
    pub read: ad714x_read_t,
    pub write: ad714x_write_t,
    pub mutex: mutex,
    pub product: unsigned,
    pub version: unsigned,
    pub ____cacheline_aligned: __be16 xfer_buf[16],
}
