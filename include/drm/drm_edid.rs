//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_edid.h
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
// Copyright © 2007-2008 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
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

pub const EDID_LENGTH: c_int = 128;
pub const DDC_ADDR: c_uint = 0x50;
pub const DDC_ADDR2: c_uint = 0x52 /* E-DDC 1.2 - where DisplayID can hide */;
pub const CEA_EXT: c_uint = 0x02;
pub const VTB_EXT: c_uint = 0x10;
pub const DI_EXT: c_uint = 0x40;
pub const LS_EXT: c_uint = 0x50;
pub const MI_EXT: c_uint = 0x60;
pub const DISPLAYID_EXT: c_uint = 0x70;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct est_timings {
    pub t1: u8,
    pub t2: u8,
    pub mfg_rsvd: u8,
    pub __packed: },
// 00=16:10, 01=4:3, 10=5:4, 11=16:9
pub const EDID_TIMING_ASPECT_SHIFT: c_int = 6;

// need to add 60
pub const EDID_TIMING_VFREQ_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct std_timing {
    pub /: *mut *mut u8 hsize; / need to multiply by 8 then add 248,
    pub vfreq_aspect: u8,
    pub __packed: },

// If detailed data is pixel timing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_pixel_timing {
    pub hactive_lo: u8,
    pub hblank_lo: u8,
    pub hactive_hblank_hi: u8,
    pub vactive_lo: u8,
    pub vblank_lo: u8,
    pub vactive_vblank_hi: u8,
    pub hsync_offset_lo: u8,
    pub hsync_pulse_width_lo: u8,
    pub vsync_offset_pulse_width_lo: u8,
    pub hsync_vsync_offset_pulse_width_hi: u8,
    pub width_mm_lo: u8,
    pub height_mm_lo: u8,
    pub width_height_mm_hi: u8,
    pub hborder: u8,
    pub vborder: u8,
    pub misc: u8,
    pub __packed: },
// If it's not pixel timing, it'll be one of the below
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_data_string {
    pub str: [u8; 13],
    pub __packed: },

pub const DRM_EDID_DEFAULT_GTF_SUPPORT_FLAG: c_uint = 0x00 /* 1.3 */;
pub const DRM_EDID_RANGE_LIMITS_ONLY_FLAG: c_uint = 0x01 /* 1.4 */;
pub const DRM_EDID_SECONDARY_GTF_SUPPORT_FLAG: c_uint = 0x02 /* 1.3 */;
pub const DRM_EDID_CVT_SUPPORT_FLAG: c_uint = 0x04 /* 1.4 */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_edid_quirk {
// Do a dummy read before DPCD accesses, to prevent corruption.
    DRM_EDID_QUIRK_DP_DPCD_PROBE,

    DRM_EDID_QUIRK_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_data_monitor_range {
    pub min_vfreq: u8,
    pub max_vfreq: u8,
    pub min_hfreq_khz: u8,
    pub max_hfreq_khz: u8,
    pub /: *mut *mut u8 pixel_clock_mhz; / need to multiply by 10,
    pub flags: u8,
    pub reserved: u8,
    pub /: *mut *mut u8 hfreq_start_khz; / need to multiply by 2,
    pub /: *mut *mut u8 c; / need to divide by 2,
    pub m: __le16,
    pub k: u8,
    pub /: *mut *mut u8 j; / need to divide by 2,
    pub gtf2: } __packed,
    pub version: u8,
    pub /: *mut *mut u8 data1; / high 6 bits: extra clock resolution,
    pub /: *mut *mut u8 data2; / plus low 2 of above: max hactive,
    pub supported_aspects: u8,
    pub /: *mut *mut u8 flags; / preferred aspect and blanking support,
    pub supported_scalings: u8,
    pub preferred_refresh: u8,
    pub cvt: } __packed,
    pub formula: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_data_wpindex {
    pub /: *mut *mut u8 white_yx_lo; / Lower 2 bits each,
    pub white_x_hi: u8,
    pub white_y_hi: u8,
    pub /: *mut *mut u8 gamma; / need to divide by 100 then add 1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_data_color_point {
    pub windex1: u8,
    pub wpindex1: [u8; 3],
    pub windex2: u8,
    pub wpindex2: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvt_timing {
    pub code: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_non_pixel {
    pub pad1: u8,
    pub name: *mut *mut u8 type; / ff=serial, fe=string, fd=monitor range, fc=monitor,
    pub pad2: u8,
    pub str: detailed_data_string,
    pub range: detailed_data_monitor_range,
    pub color: detailed_data_wpindex,
    pub timings: [std_timing; 6],
    pub cvt: [cvt_timing; 4],
    pub data: } __packed,
    pub __packed: },
pub const EDID_DETAIL_EST_TIMINGS: c_uint = 0xf7;
pub const EDID_DETAIL_CVT_3BYTE: c_uint = 0xf8;
pub const EDID_DETAIL_COLOR_MGMT_DATA: c_uint = 0xf9;
pub const EDID_DETAIL_STD_MODES: c_uint = 0xfa;
pub const EDID_DETAIL_MONITOR_CPDATA: c_uint = 0xfb;
pub const EDID_DETAIL_MONITOR_NAME: c_uint = 0xfc;
pub const EDID_DETAIL_MONITOR_RANGE: c_uint = 0xfd;
pub const EDID_DETAIL_MONITOR_STRING: c_uint = 0xfe;
pub const EDID_DETAIL_MONITOR_SERIAL: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct detailed_timing {
    pub /: *mut *mut __le16 pixel_clock; / need to multiply by 10 KHz,
    pub pixel_data: detailed_pixel_timing,
    pub other_data: detailed_non_pixel,
    pub data: } __packed,
    pub __packed: },

// If analog

// If digital

// YCBCR 420 deep color modes

// HDMI 2.1 additional fields
pub const DRM_EDID_MAX_FRL_RATE_MASK: c_uint = 0xf0;

// Deep Color specific

// VRR specific

pub const DRM_EDID_VRR_MAX_UPPER_MASK: c_uint = 0xc0;
pub const DRM_EDID_VRR_MAX_LOWER_MASK: c_uint = 0xff;
pub const DRM_EDID_VRR_MIN_MASK: c_uint = 0x3f;
// DSC specific

pub const DRM_EDID_DSC_MAX_FRL_RATE_MASK: c_uint = 0xf0;
pub const DRM_EDID_DSC_MAX_SLICES: c_uint = 0xf;
pub const DRM_EDID_DSC_TOTAL_CHUNK_KBYTES: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_edid_product_id {
    pub manufacturer_name: __be16,
    pub product_code: __le16,
    pub serial_number: __le32,
    pub week_of_manufacture: u8,
    pub year_of_manufacture: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edid {
    pub header: [u8; 8],
// Vendor & product info
    pub product_id: drm_edid_product_id,
    pub mfg_id: [u8; 2],
    pub prod_code: [u8; 2],
    pub /: *mut *mut u32 serial; / FIXME: byte order,
    pub mfg_week: u8,
    pub mfg_year: u8,
    pub __packed: },
    pub __packed: },
// EDID version
    pub version: u8,
    pub revision: u8,
// Display info:
    pub input: u8,
    pub width_cm: u8,
    pub height_cm: u8,
    pub gamma: u8,
    pub features: u8,
// Color characteristics
    pub red_green_lo: u8,
    pub blue_white_lo: u8,
    pub red_x: u8,
    pub red_y: u8,
    pub green_x: u8,
    pub green_y: u8,
    pub blue_x: u8,
    pub blue_y: u8,
    pub white_x: u8,
    pub white_y: u8,
// Est. timings and mfg rsvd timings
    pub established_timings: est_timings,
// Standard timings 1-8
    pub standard_timings: [std_timing; 8],
// Detailing timings 1-4
    pub detailed_timings: [detailed_timing; 4],
// Number of 128 byte ext. blocks
    pub extensions: u8,
// Checksum
    pub checksum: u8,
    pub __packed: },
// EDID matching
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_edid_ident {
// ID encoded by drm_edid_encode_panel_id()
    pub panel_id: u32,
    pub name: *const c_char,
}

// Short Audio Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cea_sad {
    pub format: u8,
    pub /: *mut *mut u8 channels; / max number of channels - 1,
    pub freq: u8,
    pub /: *mut *mut u8 byte2; / meaning depends on format,
}

extern "C" {
    pub fn drm_edid_to_sad(edid: *const edid, sads: *mut cea_sad) -> c_int;
}
extern "C" {
    pub fn drm_edid_to_speaker_allocation(edid: *const edid, sadb: *mut u8) -> c_int;
}
//
// drm_edid_decode_mfg_id - Decode the manufacturer ID
// @mfg_id: The manufacturer ID
// @vend: A 4-byte buffer to store the 3-letter vendor string plus a '\0'
// termination
//
// drm_edid_encode_panel_id - Encode an ID for matching against drm_edid_get_panel_id()
// @vend_chr_0: First character of the vendor string.
// @vend_chr_1: Second character of the vendor string.
// @vend_chr_2: Third character of the vendor string.
// @product_id: The 16-bit product ID.
//
// This is a macro so that it can be calculated at compile time and used
// as an initializer.
//
// For instance:
// drm_edid_encode_panel_id('B', 'O', 'E', 0x2d08) => 0x09e52d08
//
// Return: a 32-bit ID per panel.
//

//
// drm_edid_decode_panel_id - Decode a panel ID from drm_edid_encode_panel_id()
// @panel_id: The panel ID to decode.
// @vend: A 4-byte buffer to store the 3-letter vendor string plus a '\0'
// termination
// @product_id: The product ID will be returned here.
//
// For instance, after:
// drm_edid_decode_panel_id(0x09e52d08, vend, &product_id)
// These will be true:
// vend[0] = 'B'
// vend[1] = 'O'
// vend[2] = 'E'
// vend[3] = '\0'
// product_id = 0x2d08
//
// product_id = (u16)(panel_id & 0xffff);
extern "C" {
    pub fn drm_probe_ddc(adapter: *mut i2c_adapter) -> bool;
}
extern "C" {
    pub fn drm_add_edid_modes(connector: *mut drm_connector, edid: *mut edid) -> c_int;
}
extern "C" {
    pub fn drm_edid_override_connector_update(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn drm_match_cea_mode(to_match: *const drm_display_mode) -> u8;
}
extern "C" {
    pub fn drm_detect_hdmi_monitor(edid: *const edid) -> bool;
}
extern "C" {
    pub fn drm_detect_monitor_audio(edid: *const edid) -> bool;
}
extern "C" {
    pub fn drm_edid_header_is_valid(edid: *const c_void) -> c_int;
}
extern "C" {
    pub fn drm_edid_is_valid(edid: *mut edid) -> bool;
}
// Interface based on struct drm_edid
extern "C" {
    pub fn drm_edid_free(drm_edid: *const drm_edid);
}
extern "C" {
    pub fn drm_edid_valid(drm_edid: *const drm_edid) -> bool;
}
extern "C" {
    pub fn drm_edid_connector_add_modes(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn drm_edid_is_digital(drm_edid: *const drm_edid) -> bool;
}
extern "C" {
    pub fn drm_edid_get_panel_id(drm_edid: *const drm_edid) -> u32;
}
extern "C" {
    pub fn drm_edid_has_quirk(connector: *mut drm_connector, quirk: drm_edid_quirk) -> bool;
}
