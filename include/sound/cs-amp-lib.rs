//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs-amp-lib.h
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
// Copyright (C) 2024 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_amp_cal_data {
    pub calTarget: [u32; 2],
    pub calTime: [u32; 2],
    pub calAmbient: i8,
    pub calStatus: u8,
    pub calR: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_amp_efi_data {
    pub size: u32,
    pub count: u32,
    pub __counted_by(count): cirrus_amp_cal_data data[],
    pub __packed: },
//
// struct cirrus_amp_cal_controls - definition of firmware calibration controls
// @alg_id:	ID of algorithm containing the controls.
// @mem_region:	DSP memory region containing the controls.
// @ambient:	Name of control for calAmbient value.
// @calr:	Name of control for calR value.
// @status:	Name of control for calStatus value.
// @checksum:	Name of control for checksum value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_amp_cal_controls {
    pub alg_id: c_uint,
    pub mem_region: c_int,
    pub ambient: *const c_char,
    pub calr: *const c_char,
    pub status: *const c_char,
    pub checksum: *const c_char,
}

extern "C" {
    pub fn cs_amp_get_vendor_spkid(dev: *mut device) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_amp_test_hooks {
    pub buf): *mut c_void,
    pub buf): *mut c_void,
    pub val): *const *const char ctl_name, u32,
    pub val): *const *const char ctl_name, u32,
}
