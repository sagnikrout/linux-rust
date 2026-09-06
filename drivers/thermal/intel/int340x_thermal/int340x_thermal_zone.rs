//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/intel/int340x_thermal/int340x_thermal_zone.h
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
// int340x_thermal_zone.h
// Copyright (c) 2015, Intel Corporation.
//

pub const INT340X_THERMAL_MAX_ACT_TRIP_COUNT: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct active_trip {
    pub temp: c_int,
    pub id: c_int,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct int34x_thermal_zone {
    pub adev: *mut acpi_device,
    pub aux_trip_nr: c_int,
    pub zone: *mut thermal_zone_device,
    pub priv_data: *mut c_void,
    pub lpat_table: *mut acpi_lpat_conversion_table,
}

extern "C" {
    pub fn int340x_thermal_zone_remove(: *mut int34x_thermal_zone);
}
extern "C" {
    pub fn int340x_thermal_update_trips(int34x_zone: *mut int34x_thermal_zone);
}
