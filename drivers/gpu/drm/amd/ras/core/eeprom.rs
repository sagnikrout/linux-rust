//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/core/eeprom.h
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
// Copyright 2025 Advanced Micro Devices, Inc.
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

pub const DISABLE_RETIRE_PAGE: c_int = 0;
//
// Bad address pfn : eeprom_umc_record.retired_row_pfn[39:0],
// nps mode: eeprom_umc_record.retired_row_pfn[46:40]
//
pub const EEPROM_RECORD_UMC_ADDR_MASK: c_uint = 0xFFFFFFFFFFULL;
pub const EEPROM_RECORD_UMC_NPS_MASK: c_uint = 0x7F0000000000ULL;
pub const EEPROM_RECORD_UMC_NPS_SHIFT: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ras_eeprom_err_type {
    RAS_EEPROM_ERR_NA,
    RAS_EEPROM_ERR_RECOVERABLE,
    RAS_EEPROM_ERR_NON_RECOVERABLE,
    RAS_EEPROM_ERR_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_eeprom_table_header {
    pub header: u32,
    pub version: u32,
    pub first_rec_offset: u32,
    pub tbl_size: u32,
    pub checksum: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_eeprom_table_ras_info {
    pub rma_status: u8,
    pub health_percent: u8,
    pub ecc_page_threshold: u16,
    pub 1]: u32 padding[64 -,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ras_eeprom_control {
    pub tbl_hdr: ras_eeprom_table_header,
    pub tbl_rai: ras_eeprom_table_ras_info,
// record threshold
    pub record_threshold_config: c_int,
    pub record_threshold_count: u32,
    pub update_channel_flag: bool,
    pub sys_func: *const ras_eeprom_sys_func,
    pub i2c_adapter: *mut c_void,
    pub i2c_port: u32,
    pub max_read_len: u16,
    pub max_write_len: u16,
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
}

//
// Represents single table record. Packed to be easily serialized into byte
// stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_umc_record {
    pub address: u64,
    pub offset: u64,
}

// The following variables will not be saved to eeprom.
//
extern "C" {
    pub fn ras_eeprom_hw_init(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_eeprom_hw_fini(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_eeprom_reset_table(ras_core: *mut ras_core_context) -> c_int;
}
extern "C" {
    pub fn ras_eeprom_check_safety_watermark(ras_core: *mut ras_core_context) -> bool;
}
extern "C" {
    pub fn ras_eeprom_max_record_count(ras_core: *mut ras_core_context) -> u32;
}
extern "C" {
    pub fn ras_eeprom_get_record_count(ras_core: *mut ras_core_context) -> u32;
}
extern "C" {
    pub fn ras_eeprom_sync_info(ras_core: *mut ras_core_context);
}
extern "C" {
    pub fn ras_eeprom_check_storage_status(ras_core: *mut ras_core_context) -> c_int;
}
