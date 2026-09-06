//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/drm_displayid_internal.h
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
// Copyright © 2014 Red Hat Inc.
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

pub const VESA_IEEE_OUI: c_uint = 0x3a0292;
// DisplayID Structure versions
pub const DISPLAY_ID_STRUCTURE_VER_20: c_uint = 0x20;
// DisplayID Structure v1r2 Data Blocks
pub const DATA_BLOCK_PRODUCT_ID: c_uint = 0x00;
pub const DATA_BLOCK_DISPLAY_PARAMETERS: c_uint = 0x01;
pub const DATA_BLOCK_COLOR_CHARACTERISTICS: c_uint = 0x02;
pub const DATA_BLOCK_TYPE_1_DETAILED_TIMING: c_uint = 0x03;
pub const DATA_BLOCK_TYPE_2_DETAILED_TIMING: c_uint = 0x04;
pub const DATA_BLOCK_TYPE_3_SHORT_TIMING: c_uint = 0x05;
pub const DATA_BLOCK_TYPE_4_DMT_TIMING: c_uint = 0x06;
pub const DATA_BLOCK_VESA_TIMING: c_uint = 0x07;
pub const DATA_BLOCK_CEA_TIMING: c_uint = 0x08;
pub const DATA_BLOCK_VIDEO_TIMING_RANGE: c_uint = 0x09;
pub const DATA_BLOCK_PRODUCT_SERIAL_NUMBER: c_uint = 0x0a;
pub const DATA_BLOCK_GP_ASCII_STRING: c_uint = 0x0b;
pub const DATA_BLOCK_DISPLAY_DEVICE_DATA: c_uint = 0x0c;
pub const DATA_BLOCK_INTERFACE_POWER_SEQUENCING: c_uint = 0x0d;
pub const DATA_BLOCK_TRANSFER_CHARACTERISTICS: c_uint = 0x0e;
pub const DATA_BLOCK_DISPLAY_INTERFACE: c_uint = 0x0f;
pub const DATA_BLOCK_STEREO_DISPLAY_INTERFACE: c_uint = 0x10;
pub const DATA_BLOCK_TILED_DISPLAY: c_uint = 0x12;
pub const DATA_BLOCK_VENDOR_SPECIFIC: c_uint = 0x7f;
pub const DATA_BLOCK_CTA: c_uint = 0x81;
// DisplayID Structure v2r0 Data Blocks
pub const DATA_BLOCK_2_PRODUCT_ID: c_uint = 0x20;
pub const DATA_BLOCK_2_DISPLAY_PARAMETERS: c_uint = 0x21;
pub const DATA_BLOCK_2_TYPE_7_DETAILED_TIMING: c_uint = 0x22;
pub const DATA_BLOCK_2_TYPE_8_ENUMERATED_TIMING: c_uint = 0x23;
pub const DATA_BLOCK_2_TYPE_9_FORMULA_TIMING: c_uint = 0x24;
pub const DATA_BLOCK_2_DYNAMIC_VIDEO_TIMING: c_uint = 0x25;
pub const DATA_BLOCK_2_DISPLAY_INTERFACE_FEATURES: c_uint = 0x26;
pub const DATA_BLOCK_2_STEREO_DISPLAY_INTERFACE: c_uint = 0x27;
pub const DATA_BLOCK_2_TILED_DISPLAY_TOPOLOGY: c_uint = 0x28;
pub const DATA_BLOCK_2_CONTAINER_ID: c_uint = 0x29;
pub const DATA_BLOCK_2_TYPE_10_FORMULA_TIMING: c_uint = 0x2a;
pub const DATA_BLOCK_2_VENDOR_SPECIFIC: c_uint = 0x7e;
pub const DATA_BLOCK_2_CTA_DISPLAY_ID: c_uint = 0x81;
// DisplayID Structure v1r2 Product Type
pub const PRODUCT_TYPE_EXTENSION: c_int = 0;
pub const PRODUCT_TYPE_TEST: c_int = 1;
pub const PRODUCT_TYPE_PANEL: c_int = 2;
pub const PRODUCT_TYPE_MONITOR: c_int = 3;
pub const PRODUCT_TYPE_TV: c_int = 4;
pub const PRODUCT_TYPE_REPEATER: c_int = 5;
pub const PRODUCT_TYPE_DIRECT_DRIVE: c_int = 6;
// DisplayID Structure v2r0 Display Product Primary Use Case (~Product Type)
pub const PRIMARY_USE_EXTENSION: c_int = 0;
pub const PRIMARY_USE_TEST: c_int = 1;
pub const PRIMARY_USE_GENERIC: c_int = 2;
pub const PRIMARY_USE_TV: c_int = 3;
pub const PRIMARY_USE_DESKTOP_PRODUCTIVITY: c_int = 4;
pub const PRIMARY_USE_DESKTOP_GAMING: c_int = 5;
pub const PRIMARY_USE_PRESENTATION: c_int = 6;
pub const PRIMARY_USE_HEAD_MOUNTED_VR: c_int = 7;
pub const PRIMARY_USE_HEAD_MOUNTED_AR: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_header {
    pub rev: u8,
    pub bytes: u8,
    pub prod_id: u8,
    pub ext_count: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_block {
    pub tag: u8,
    pub rev: u8,
    pub num_bytes: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_tiled_block {
    pub base: displayid_block,
    pub tile_cap: u8,
    pub topo: [u8; 3],
    pub tile_size: [u8; 4],
    pub tile_pixel_bezel: [u8; 5],
    pub topology_id: [u8; 9],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_detailed_timings_1 {
    pub pixel_clock: [u8; 3],
    pub flags: u8,
    pub hactive: __le16,
    pub hblank: __le16,
    pub hsync: __le16,
    pub hsw: __le16,
    pub vactive: __le16,
    pub vblank: __le16,
    pub vsync: __le16,
    pub vsw: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_detailed_timing_block {
    pub base: displayid_block,
    pub timings: [displayid_detailed_timings_1; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_formula_timings_9 {
    pub flags: u8,
    pub hactive: __le16,
    pub vactive: __le16,
    pub vrefresh: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_formula_timing_block {
    pub base: displayid_block,
    pub timings: [displayid_formula_timings_9; ],
    pub __packed: },
pub const DISPLAYID_DEVICE_TECH_UNSPECIFIED: c_int = 0;
pub const DISPLAYID_DEVICE_TECH_LCD: c_int = 1;
pub const DISPLAYID_DEVICE_TECH_OLED: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_display_params_block {
    pub base: displayid_block,
    pub horiz_image_size: __le16,
    pub vert_image_size: __le16,
    pub horiz_pixel_count: __le16,
    pub vert_pixel_count: __le16,
    pub features: u8,
    pub primary_color1: [u8; 3],
    pub primary_color2: [u8; 3],
    pub primary_color3: [u8; 3],
    pub white_point: [u8; 3],
    pub max_luminance_full: __le16,
    pub max_luminance_10: __le16,
    pub min_luminance: __le16,
    pub /: *mut *mut u8 color_depth_and_tech; / [2:0] depth, [6:4] device tech, [7] theme,
    pub gamma_eotf: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_vesa_vendor_specific_block {
    pub base: displayid_block,
    pub oui: [u8; 3],
    pub data_structure_type: u8,
    pub mso: u8,
    pub __packed: },
//
// DisplayID iteration.
//
// Do not access directly, this is private.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayid_iter {
    pub drm_edid: *const drm_edid,
    pub section: *const u8,
    pub length: c_int,
    pub idx: c_int,
    pub ext_index: c_int,
    pub version: u8,
    pub primary_use: u8,
    pub quirks: u8,
}

extern "C" {
    pub fn displayid_iter_end(iter: *mut displayid_iter);
}
extern "C" {
    pub fn displayid_version(iter: *const displayid_iter) -> u8;
}
extern "C" {
    pub fn displayid_primary_use(iter: *const displayid_iter) -> u8;
}
