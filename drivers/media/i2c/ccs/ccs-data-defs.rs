//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs/ccs-data-defs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// CCS static data binary format definitions
//
// Copyright 2019--2020 Intel Corporation
//

pub const CCS_STATIC_DATA_VERSION: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __ccs_data_length_specifier_id {
    CCS_DATA_LENGTH_SPECIFIER_1 = 0,
    CCS_DATA_LENGTH_SPECIFIER_2 = 1,
    CCS_DATA_LENGTH_SPECIFIER_3 = 2
}

pub const CCS_DATA_LENGTH_SPECIFIER_SIZE_SHIFT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_length_specifier {
    pub length: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_length_specifier2 {
    pub length: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_length_specifier3 {
    pub length: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block {
    pub id: u8,
    pub length: __ccs_data_length_specifier,
    pub __packed: },
pub const CCS_DATA_BLOCK_HEADER_ID_VERSION_SHIFT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block3 {
    pub id: u8,
    pub length: __ccs_data_length_specifier2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block4 {
    pub id: u8,
    pub length: __ccs_data_length_specifier3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __ccs_data_block_id {
    CCS_DATA_BLOCK_ID_DUMMY	= 1,
    CCS_DATA_BLOCK_ID_DATA_VERSION = 2,
    CCS_DATA_BLOCK_ID_SENSOR_READ_ONLY_REGS = 3,
    CCS_DATA_BLOCK_ID_MODULE_READ_ONLY_REGS = 4,
    CCS_DATA_BLOCK_ID_SENSOR_MANUFACTURER_REGS = 5,
    CCS_DATA_BLOCK_ID_MODULE_MANUFACTURER_REGS = 6,
    CCS_DATA_BLOCK_ID_SENSOR_RULE_BASED_BLOCK = 32,
    CCS_DATA_BLOCK_ID_MODULE_RULE_BASED_BLOCK = 33,
    CCS_DATA_BLOCK_ID_SENSOR_PDAF_PIXEL_LOCATION = 36,
    CCS_DATA_BLOCK_ID_MODULE_PDAF_PIXEL_LOCATION = 37,
    CCS_DATA_BLOCK_ID_LICENSE = 40,
    CCS_DATA_BLOCK_ID_END = 127,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_version {
    pub static_data_version_major: [u8; 2],
    pub static_data_version_minor: [u8; 2],
    pub year: [u8; 2],
    pub month: u8,
    pub day: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_regs {
    pub reg_len: u8,
    pub __packed: },
pub const CCS_DATA_BLOCK_REGS_ADDR_MASK: c_uint = 0x07;
pub const CCS_DATA_BLOCK_REGS_LEN_SHIFT: c_int = 3;
pub const CCS_DATA_BLOCK_REGS_LEN_MASK: c_uint = 0x38;
pub const CCS_DATA_BLOCK_REGS_SEL_SHIFT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ccs_data_block_regs_sel {
    CCS_DATA_BLOCK_REGS_SEL_REGS = 0,
    CCS_DATA_BLOCK_REGS_SEL_REGS2 = 1,
    CCS_DATA_BLOCK_REGS_SEL_REGS3 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_regs2 {
    pub reg_len: u8,
    pub addr: u8,
    pub __packed: },
pub const CCS_DATA_BLOCK_REGS_2_ADDR_MASK: c_uint = 0x01;
pub const CCS_DATA_BLOCK_REGS_2_LEN_SHIFT: c_int = 1;
pub const CCS_DATA_BLOCK_REGS_2_LEN_MASK: c_uint = 0x3e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_regs3 {
    pub reg_len: u8,
    pub addr: [u8; 2],
    pub __packed: },
pub const CCS_DATA_BLOCK_REGS_3_LEN_MASK: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __ccs_data_ffd_pixelcode {
    CCS_DATA_BLOCK_FFD_PIXELCODE_EMBEDDED = 1,
    CCS_DATA_BLOCK_FFD_PIXELCODE_DUMMY = 2,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BLACK = 3,
    CCS_DATA_BLOCK_FFD_PIXELCODE_DARK = 4,
    CCS_DATA_BLOCK_FFD_PIXELCODE_VISIBLE = 5,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_0 = 8,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_1 = 9,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_2 = 10,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_3 = 11,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_4 = 12,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_5 = 13,
    CCS_DATA_BLOCK_FFD_PIXELCODE_MS_6 = 14,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOP_OB = 16,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BOTTOM_OB = 17,
    CCS_DATA_BLOCK_FFD_PIXELCODE_LEFT_OB = 18,
    CCS_DATA_BLOCK_FFD_PIXELCODE_RIGHT_OB = 19,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOP_LEFT_OB = 20,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOP_RIGHT_OB = 21,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BOTTOM_LEFT_OB = 22,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BOTTOM_RIGHT_OB = 23,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOTAL = 24,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOP_PDAF = 32,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BOTTOM_PDAF = 33,
    CCS_DATA_BLOCK_FFD_PIXELCODE_LEFT_PDAF = 34,
    CCS_DATA_BLOCK_FFD_PIXELCODE_RIGHT_PDAF = 35,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOP_LEFT_PDAF = 36,
    CCS_DATA_BLOCK_FFD_PIXELCODE_TOP_RIGHT_PDAF = 37,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BOTTOM_LEFT_PDAF = 38,
    CCS_DATA_BLOCK_FFD_PIXELCODE_BOTTOM_RIGHT_PDAF = 39,
    CCS_DATA_BLOCK_FFD_PIXELCODE_SEPARATED_PDAF = 40,
    CCS_DATA_BLOCK_FFD_PIXELCODE_ORIGINAL_ORDER_PDAF = 41,
    CCS_DATA_BLOCK_FFD_PIXELCODE_VENDOR_PDAF = 41,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_ffd_entry {
    pub pixelcode: u8,
    pub reserved: u8,
    pub value: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_ffd {
    pub num_column_descs: u8,
    pub num_row_descs: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __ccs_data_block_rule_id {
    CCS_DATA_BLOCK_RULE_ID_IF = 1,
    CCS_DATA_BLOCK_RULE_ID_READ_ONLY_REGS = 2,
    CCS_DATA_BLOCK_RULE_ID_FFD = 3,
    CCS_DATA_BLOCK_RULE_ID_MSR = 4,
    CCS_DATA_BLOCK_RULE_ID_PDAF_READOUT = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_rule_if {
    pub addr: [u8; 2],
    pub value: u8,
    pub mask: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __ccs_data_block_pdaf_readout_order {
    CCS_DATA_BLOCK_PDAF_READOUT_ORDER_ORIGINAL = 1,
    CCS_DATA_BLOCK_PDAF_READOUT_ORDER_SEPARATE_WITHIN_LINE = 2,
    CCS_DATA_BLOCK_PDAF_READOUT_ORDER_SEPARATE_TYPES_SEPARATE_LINES = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_pdaf_readout {
    pub pdaf_readout_info_reserved: u8,
    pub pdaf_readout_info_order: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_pdaf_pix_loc_block_desc {
    pub block_type_id: u8,
    pub repeat_x: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_pdaf_pix_loc_block_desc_group {
    pub num_block_descs: [u8; 2],
    pub repeat_y: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __ccs_data_block_pdaf_pix_loc_pixel_type {
    CCS_DATA_PDAF_PIXEL_TYPE_LEFT_SEPARATED = 0,
    CCS_DATA_PDAF_PIXEL_TYPE_RIGHT_SEPARATED = 1,
    CCS_DATA_PDAF_PIXEL_TYPE_TOP_SEPARATED = 2,
    CCS_DATA_PDAF_PIXEL_TYPE_BOTTOM_SEPARATED = 3,
    CCS_DATA_PDAF_PIXEL_TYPE_LEFT_SIDE_BY_SIDE = 4,
    CCS_DATA_PDAF_PIXEL_TYPE_RIGHT_SIDE_BY_SIDE = 5,
    CCS_DATA_PDAF_PIXEL_TYPE_TOP_SIDE_BY_SIDE = 6,
    CCS_DATA_PDAF_PIXEL_TYPE_BOTTOM_SIDE_BY_SIDE = 7,
    CCS_DATA_PDAF_PIXEL_TYPE_TOP_LEFT = 8,
    CCS_DATA_PDAF_PIXEL_TYPE_TOP_RIGHT = 9,
    CCS_DATA_PDAF_PIXEL_TYPE_BOTTOM_LEFT = 10,
    CCS_DATA_PDAF_PIXEL_TYPE_BOTTOM_RIGHT = 11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_pdaf_pix_loc_pixel_desc {
    pub pixel_type: u8,
    pub small_offset_x: u8,
    pub small_offset_y: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_pdaf_pix_loc {
    pub main_offset_x: [u8; 2],
    pub main_offset_y: [u8; 2],
    pub global_pdaf_type: u8,
    pub block_width: u8,
    pub block_height: u8,
    pub num_block_desc_groups: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __ccs_data_block_end {
    pub crc: [u8; 4],
    pub __packed: },
