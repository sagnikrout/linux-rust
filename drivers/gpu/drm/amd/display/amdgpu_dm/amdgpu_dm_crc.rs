//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_crc.h
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
// Authors: AMD
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_dm_pipe_crc_source {
    AMDGPU_DM_PIPE_CRC_SOURCE_NONE = 0,
    AMDGPU_DM_PIPE_CRC_SOURCE_CRTC,
    AMDGPU_DM_PIPE_CRC_SOURCE_CRTC_DITHER,
    AMDGPU_DM_PIPE_CRC_SOURCE_DPRX,
    AMDGPU_DM_PIPE_CRC_SOURCE_DPRX_DITHER,
    AMDGPU_DM_PIPE_CRC_SOURCE_MAX,
    AMDGPU_DM_PIPE_CRC_SOURCE_INVALID = -1,
}

pub const MAX_CRTC: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum secure_display_mode {
// via dmub + psp
    LEGACY_MODE = 0,
// driver directly
    DISPLAY_CRC_MODE,
    SECURE_DISPLAY_MODE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_id_mapping {
    pub assigned: bool,
    pub is_mst: bool,
    pub enc_hw_inst: u8,
    pub lct: u8,
    pub port_num: u8,
    pub rad: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_data {
    pub crc_R: u32,
    pub crc_G: u32,
    pub crc_B: u32,
    pub frame_count: u32,
    pub crc_ready: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_info {
    pub crc: [crc_data; MAX_CRC_WINDOW_NUM],
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_window_param {
    pub x_start: u16,
    pub y_start: u16,
    pub x_end: u16,
    pub y_end: u16,
// CRC window is activated or not
    pub enable: bool,
// Update crc window during vertical blank or not
    pub update_win: bool,
// skip reading/writing for few frames
    pub skip_frame_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secure_display_crtc_context {
// work to notify PSP TA
    pub notify_ta_work: work_struct,
// work to forward ROI to dmcu/dmub
    pub forward_roi_work: work_struct,
    pub crtc: *mut drm_crtc,
// Region of Interest (ROI)
    pub roi: [crc_window; MAX_CRC_WINDOW_NUM],
    pub crc_info: crc_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secure_display_context {
    pub crtc_ctx: *mut secure_display_crtc_context,
// Whether dmub support multiple ROI setting
    pub support_mul_roi: bool,
    pub op_mode: secure_display_mode,
    pub phy_mapping_updated: bool,
    pub phy_id_mapping_cnt: c_int,
    pub phy_id_mapping: [phy_id_mapping; MAX_CRTC],
}

// amdgpu_dm_crc.c

extern "C" {
    pub fn amdgpu_dm_crtc_set_crc_source(crtc: *mut drm_crtc, src_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_crtc_handle_crc_irq(crtc: *mut drm_crtc);
}

// Macro flag: #define amdgpu_dm_crtc_handle_crc_irq(x)

extern "C" {
    pub fn amdgpu_dm_crc_window_is_activated(crtc: *mut drm_crtc) -> bool;
}
extern "C" {
    pub fn amdgpu_dm_crtc_handle_crc_window_irq(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn amdgpu_dm_crtc_secure_display_create_contexts(adev: *mut amdgpu_device);
}

// Macro flag: #define amdgpu_dm_crc_window_is_activated(x)
// Macro flag: #define amdgpu_dm_crtc_handle_crc_window_irq(x)
// Macro flag: #define amdgpu_dm_crtc_secure_display_create_contexts(x)

extern "C" {
    pub fn dm_parse_crc_source(source: *const c_char) -> amdgpu_dm_pipe_crc_source;
}
extern "C" {
    pub fn dm_is_crc_source_crtc(src: amdgpu_dm_pipe_crc_source) -> bool;
}
extern "C" {
    pub fn dm_is_crc_source_dprx(src: amdgpu_dm_pipe_crc_source) -> bool;
}
extern "C" {
    pub fn dm_need_crc_dither(src: amdgpu_dm_pipe_crc_source) -> bool;
}

