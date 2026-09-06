//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/inc/mod_shared.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
// Authors: AMD
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum color_transfer_func {
    TRANSFER_FUNC_UNKNOWN,
    TRANSFER_FUNC_SRGB,
    TRANSFER_FUNC_BT709,
    TRANSFER_FUNC_PQ2084,
    TRANSFER_FUNC_PQ2084_INTERIM,
    TRANSFER_FUNC_LINEAR_0_1,
    TRANSFER_FUNC_LINEAR_0_125,
    TRANSFER_FUNC_GAMMA_22,
    TRANSFER_FUNC_GAMMA_26
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vrr_packet_type {
    PACKET_TYPE_VRR,
    PACKET_TYPE_FS_V1,
    PACKET_TYPE_FS_V2,
    PACKET_TYPE_FS_V3,
    PACKET_TYPE_VTEM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lut3d_control_flags {
    pub raw: c_uint,
    pub :1: unsigned int do_chroma_scale,
    pub :3: unsigned int spec_version,
    pub :1: unsigned int use_zero_display_black,
    pub :1: unsigned int use_zero_source_black,
    pub :6: unsigned int force_display_black,
    pub :1: unsigned int apply_display_gamma,
    pub :6: unsigned int exp_shaper_max,
    pub :1: unsigned int unity_3dlut,
    pub :1: unsigned int bypass_3dlut,
    pub :1: unsigned int use_3dlut,
    pub :1: unsigned int less_than_dcip3,
    pub :1: unsigned int override_lum,
    pub :1: unsigned int use_gamut_map_lib,
    pub :1: unsigned int chromatic_adaptation_src,
    pub :1: unsigned int chromatic_adaptation_dst,
    pub :1: unsigned int do_blender_lut_degamma,
    pub :4: unsigned int reseved,
    pub bits: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tm_show_option_internal {
    tm_show_option_internal_single_file		= 0,/*flags2 not in use*/
    tm_show_option_internal_duplicate_file,		/*use flags2*/
    tm_show_option_internal_duplicate_sidebyside/*use flags2*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lut3d_control_gamut_map {
    lut3d_control_gamut_map_none = 0,
    lut3d_control_gamut_map_tonemap,
    lut3d_control_gamut_map_chto,
    lut3d_control_gamut_map_chso,
    lut3d_control_gamut_map_chci
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lut3d_control_rotation_mode {
    lut3d_control_rotation_mode_none = 0,
    lut3d_control_rotation_mode_hue,
    lut3d_control_rotation_mode_cc,
    lut3d_control_rotation_mode_hue_cc
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lut3d_settings {
    pub version: c_uchar,
    pub flags: lut3d_control_flags,
    pub flags2: lut3d_control_flags,
    pub option: tm_show_option_internal,
    pub 100*/: *mut *mut unsigned int min_lum;/multiplied by,
    pub max_lum: c_uint,
    pub min_lum2: c_uint,
    pub max_lum2: c_uint,
    pub map: lut3d_control_gamut_map,
    pub rotation: lut3d_control_rotation_mode,
    pub map2: lut3d_control_gamut_map,
    pub rotation2: lut3d_control_rotation_mode,
}
