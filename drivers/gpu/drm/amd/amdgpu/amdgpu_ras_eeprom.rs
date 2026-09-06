//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ras_eeprom.h
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


//
// Copyright 2019 Advanced Micro Devices, Inc.
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

pub const RAS_TABLE_VER_V1: c_uint = 0x00010000;
pub const RAS_TABLE_VER_V2_1: c_uint = 0x00021000;
pub const RAS_TABLE_VER_V3: c_uint = 0x00030000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_gpu_health_status {
    GPU_HEALTH_USABLE = 0,
    GPU_RETIRED__ECC_REACH_THRESHOLD = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ras_eeprom_err_type {
    AMDGPU_RAS_EEPROM_ERR_NA,
    AMDGPU_RAS_EEPROM_ERR_RECOVERABLE,
    AMDGPU_RAS_EEPROM_ERR_NON_RECOVERABLE,
    AMDGPU_RAS_EEPROM_ERR_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_eeprom_table_header {
    pub header: u32,
    pub version: u32,
    pub first_rec_offset: u32,
    pub tbl_size: u32,
    pub checksum: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_eeprom_table_ras_info {
    pub rma_status: u8,
    pub health_percent: u8,
    pub ecc_page_threshold: u16,
    pub 1]: u32 padding[64 -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ras_eeprom_control {
    pub tbl_hdr: amdgpu_ras_eeprom_table_header,
    pub tbl_rai: amdgpu_ras_eeprom_table_ras_info,
// Base I2C EEPPROM 19-bit memory address,
// where the table is located. For more information,
// see top of amdgpu_eeprom.c.
//
    pub i2c_address: u32,
// The byte offset off of @i2c_address
// where the table header is found,
// and where the records start--always
// right after the header.
//
    pub ras_header_offset: u32,
    pub ras_info_offset: u32,
    pub ras_record_offset: u32,
// Number of records in the table.
//
    pub ras_num_recs: u32,
// the bad page number is ras_num_recs or
// ras_num_recs * umc.retire_unit
//
    pub ras_num_bad_pages: u32,
// Number of records store mca address
    pub ras_num_mca_recs: u32,
// Number of records store physical address
    pub ras_num_pa_recs: u32,
// First record index to read, 0-based.
// Range is [0, num_recs-1]. This is
// an absolute index, starting right after
// the table header.
//
    pub ras_fri: u32,
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
    pub is_eeprom_valid: bool,
}

//
// Represents single table record. Packed to be easily serialized into byte
// stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_table_record {
    pub address: u64,
    pub offset: u64,
}

extern "C" {
    pub fn amdgpu_ras_eeprom_init(control: *mut amdgpu_ras_eeprom_control) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_eeprom_reset_table(control: *mut amdgpu_ras_eeprom_control) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_eeprom_check_err_threshold(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_ras_eeprom_max_record_count(control: *mut amdgpu_ras_eeprom_control) -> u32;
}
extern "C" {
    pub fn amdgpu_ras_debugfs_set_ret_size(control: *mut amdgpu_ras_eeprom_control);
}
extern "C" {
    pub fn amdgpu_ras_eeprom_check(control: *mut amdgpu_ras_eeprom_control) -> c_int;
}
extern "C" {
    pub fn amdgpu_ras_eeprom_check_and_recover(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ras_check_bad_page_status(adev: *mut amdgpu_device);
}
