//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs/ccs-data.h
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
// CCS static data in-memory data structure definitions
//
// Copyright 2019--2020 Intel Corporation
//

//
// struct ccs_data_block_version - CCS static data version
// @version_major: Major version number
// @version_minor: Minor version number
// @date_year: Year
// @date_month: Month
// @date_day: Day
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_data_block_version {
    pub version_major: u16,
    pub version_minor: u16,
    pub date_year: u16,
    pub date_month: u8,
    pub date_day: u8,
}

//
// struct ccs_reg - CCS register value
// @addr: The 16-bit address of the register
// @len: Length of the data
// @value: Data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_reg {
    pub addr: u16,
    pub len: u16,
    pub value: *mut u8,
}

//
// struct ccs_if_rule - CCS static data if rule
// @addr: Register address
// @value: Register value
// @mask: Value applied to both actual register value and @value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_if_rule {
    pub addr: u16,
    pub value: u8,
    pub mask: u8,
}

//
// struct ccs_frame_format_desc - CCS frame format descriptor
// @pixelcode: The pixelcode; CCS_DATA_BLOCK_FFD_PIXELCODE_
// @value: Value related to the pixelcode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_frame_format_desc {
    pub pixelcode: u8,
    pub value: u16,
}

//
// struct ccs_frame_format_descs - A series of CCS frame format descriptors
// @num_column_descs: Number of column descriptors
// @num_row_descs: Number of row descriptors
// @column_descs: Column descriptors
// @row_descs: Row descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_frame_format_descs {
    pub num_column_descs: u8,
    pub num_row_descs: u8,
    pub column_descs: *mut ccs_frame_format_desc,
    pub row_descs: *mut ccs_frame_format_desc,
}

//
// struct ccs_pdaf_readout - CCS PDAF data readout descriptor
// @pdaf_readout_info_order: PDAF readout order
// @ffd: Frame format of PDAF data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pdaf_readout {
    pub pdaf_readout_info_order: u8,
    pub ffd: *mut ccs_frame_format_descs,
}

//
// struct ccs_rule - A CCS static data rule
// @num_if_rules: Number of if rules
// @if_rules: If rules
// @num_read_only_regs: Number of read-only registers
// @read_only_regs: Read-only registers
// @num_manufacturer_regs: Number of manufacturer-specific registers
// @manufacturer_regs: Manufacturer-specific registers
// @frame_format: Frame format
// @pdaf_readout: PDAF readout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_rule {
    pub num_if_rules: usize,
    pub if_rules: *mut ccs_if_rule,
    pub num_read_only_regs: usize,
    pub read_only_regs: *mut ccs_reg,
    pub num_manufacturer_regs: usize,
    pub manufacturer_regs: *mut ccs_reg,
    pub frame_format: *mut ccs_frame_format_descs,
    pub pdaf_readout: *mut ccs_pdaf_readout,
}

//
// struct ccs_pdaf_pix_loc_block_desc - PDAF pixel location block descriptor
// @block_type_id: Block type identifier, from 0 to n
// @repeat_x: Number of times this block is repeated to right
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pdaf_pix_loc_block_desc {
    pub block_type_id: u8,
    pub repeat_x: u16,
}

//
// struct ccs_pdaf_pix_loc_block_desc_group - PDAF pixel location block
// descriptor group
// @repeat_y: Number of times the group is repeated down
// @num_block_descs: Number of block descriptors in @block_descs
// @block_descs: Block descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pdaf_pix_loc_block_desc_group {
    pub repeat_y: u8,
    pub num_block_descs: u16,
    pub block_descs: *mut ccs_pdaf_pix_loc_block_desc,
}

//
// struct ccs_pdaf_pix_loc_pixel_desc - PDAF pixel location block descriptor
// @pixel_type: Type of the pixel; CCS_DATA_PDAF_PIXEL_TYPE_
// @small_offset_x: offset X coordinate
// @small_offset_y: offset Y coordinate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pdaf_pix_loc_pixel_desc {
    pub pixel_type: u8,
    pub small_offset_x: u8,
    pub small_offset_y: u8,
}

//
// struct ccs_pdaf_pix_loc_pixel_desc_group - PDAF pixel location pixel
// descriptor group
// @num_descs: Number of descriptors in @descs
// @descs: PDAF pixel location pixel descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pdaf_pix_loc_pixel_desc_group {
    pub num_descs: u8,
    pub descs: *mut ccs_pdaf_pix_loc_pixel_desc,
}

//
// struct ccs_pdaf_pix_loc - PDAF pixel locations
// @main_offset_x: Start X coordinate of PDAF pixel blocks
// @main_offset_y: Start Y coordinate of PDAF pixel blocks
// @global_pdaf_type: PDAF pattern type
// @block_width: Width of a block in pixels
// @block_height: Heigth of a block in pixels
// @num_block_desc_groups: Number of block descriptor groups
// @block_desc_groups: Block descriptor groups
// @num_pixel_desc_grups: Number of pixel descriptor groups
// @pixel_desc_groups: Pixel descriptor groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_pdaf_pix_loc {
    pub main_offset_x: u16,
    pub main_offset_y: u16,
    pub global_pdaf_type: u8,
    pub block_width: u8,
    pub block_height: u8,
    pub num_block_desc_groups: u16,
    pub block_desc_groups: *mut ccs_pdaf_pix_loc_block_desc_group,
    pub num_pixel_desc_grups: u8,
    pub pixel_desc_groups: *mut ccs_pdaf_pix_loc_pixel_desc_group,
}

//
// struct ccs_data_container - In-memory CCS static data
// @version: CCS static data version
// @num_sensor_read_only_regs: Number of the read-only registers for the sensor
// @sensor_read_only_regs: Read-only registers for the sensor
// @num_sensor_manufacturer_regs: Number of the manufacturer-specific registers
// for the sensor
// @sensor_manufacturer_regs: Manufacturer-specific registers for the sensor
// @num_sensor_rules: Number of rules for the sensor
// @sensor_rules: Rules for the sensor
// @num_module_read_only_regs: Number of the read-only registers for the module
// @module_read_only_regs: Read-only registers for the module
// @num_module_manufacturer_regs: Number of the manufacturer-specific registers
// for the module
// @module_manufacturer_regs: Manufacturer-specific registers for the module
// @num_module_rules: Number of rules for the module
// @module_rules: Rules for the module
// @sensor_pdaf: PDAF data for the sensor
// @module_pdaf: PDAF data for the module
// @license_length: Lenght of the license data
// @license: License data
// @end: Whether or not there's an end block
// @backing: Raw data, pointed to from elsewhere so keep it around
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_data_container {
    pub version: *mut ccs_data_block_version,
    pub num_sensor_read_only_regs: usize,
    pub sensor_read_only_regs: *mut ccs_reg,
    pub num_sensor_manufacturer_regs: usize,
    pub sensor_manufacturer_regs: *mut ccs_reg,
    pub num_sensor_rules: usize,
    pub sensor_rules: *mut ccs_rule,
    pub num_module_read_only_regs: usize,
    pub module_read_only_regs: *mut ccs_reg,
    pub num_module_manufacturer_regs: usize,
    pub module_manufacturer_regs: *mut ccs_reg,
    pub num_module_rules: usize,
    pub module_rules: *mut ccs_rule,
    pub sensor_pdaf: *mut ccs_pdaf_pix_loc,
    pub module_pdaf: *mut ccs_pdaf_pix_loc,
    pub license_length: usize,
    pub license: *mut c_char,
    pub end: bool,
    pub backing: *mut c_void,
}
