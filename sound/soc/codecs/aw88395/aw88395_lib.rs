//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/aw88395/aw88395_lib.h
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
// aw88395_lib.h  -- ACF bin parsing and check library file for aw88395
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bin_header_version_enum {
    HEADER_VERSION_V1 = 0x01000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_type_enum {
    DATA_TYPE_REGISTER   = 0x00000000,
    DATA_TYPE_DSP_REG    = 0x00000010,
    DATA_TYPE_DSP_CFG    = 0x00000011,
    DATA_TYPE_SOC_REG    = 0x00000020,
    DATA_TYPE_SOC_APP    = 0x00000021,
    DATA_TYPE_DSP_FW     = 0x00000022,
    DATA_TYPE_MULTI_BINS = 0x00002000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_version_enum {
    DATA_VERSION_V1 = 0x00000001,
    DATA_VERSION_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_header_info {
    pub check_sum: c_uint,
    pub header_ver: c_uint,
    pub bin_data_type: c_uint,
    pub bin_data_ver: c_uint,
    pub bin_data_len: c_uint,
    pub ui_ver: c_uint,
    pub chip_type: [c_uchar; 8],
    pub reg_byte_len: c_uint,
    pub data_byte_len: c_uint,
    pub device_addr: c_uint,
    pub valid_data_len: c_uint,
    pub valid_data_addr: c_uint,
    pub reg_num: c_uint,
    pub reg_data_byte_len: c_uint,
    pub download_addr: c_uint,
    pub app_version: c_uint,
    pub header_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_container {
    pub len: c_uint,
    pub data: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_bin {
    pub p_addr: *mut c_uchar,
    pub all_bin_parse_num: c_uint,
    pub multi_bin_parse_num: c_uint,
    pub single_bin_parse_num: c_uint,
    pub header_info: [bin_header_info; BIN_NUM_MAX],
    pub info: bin_container,
}
