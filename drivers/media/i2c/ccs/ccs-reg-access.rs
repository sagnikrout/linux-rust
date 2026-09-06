//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs/ccs-reg-access.h
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
// include/media/ccs/ccs-reg-access.h
//
// Generic driver for MIPI CCS/SMIA/SMIA++ compliant camera sensors
//
// Copyright (C) 2020 Intel Corporation
// Copyright (C) 2011--2012 Nokia Corporation
// Contact: Sakari Ailus <sakari.ailus@linux.intel.com>
//

extern "C" {
    pub fn ccs_read_addr(sensor: *mut ccs_sensor, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ccs_read_addr_8only(sensor: *mut ccs_sensor, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ccs_read_addr_noconv(sensor: *mut ccs_sensor, reg: u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ccs_write_addr(sensor: *mut ccs_sensor, reg: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn ccs_reg_conv(sensor: *mut ccs_sensor, reg: u32, val: u32) -> u32;
}

