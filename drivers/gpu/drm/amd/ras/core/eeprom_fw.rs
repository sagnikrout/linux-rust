//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/eeprom_fw.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_fw_eeprom_control {
    pub version: u32,
// record threshold
    pub record_threshold_config: c_int,
    pub record_threshold_count: u32,
    pub update_channel_flag: bool,
// Number of records in the table.
//
    pub ras_num_recs: u32,
// Maximum possible number of records
// we could store, i.e. the maximum capacity
// of the table.
//
    pub ras_max_record_count: u32,
// Protect table access via this mutex.
//
    pub ras_tbl_mutex: mutex,
// Record channel info which occurred bad pages
//
    pub bad_channel_bitmap: u32,
}

extern "C" {
    pub fn ras_fw_init_feature_flags(ras_core: *mut ras_core_context);
}
extern "C" {
    pub fn ras_fw_eeprom_supported(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_fw_eeprom_reset_table(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_fw_eeprom_check_safety_watermark(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_fw_eeprom_get_record_count(ras_core: *mut ras_core_context) -> u32;
}
extern "C" {
    pub fn ras_fw_eeprom_hw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_fw_eeprom_hw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_fw_eeprom_check_storage_status(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_fw_eeprom_sync_info(ras_core: *mut ras_core_context);
}
