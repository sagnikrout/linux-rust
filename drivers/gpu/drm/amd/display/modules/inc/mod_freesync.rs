//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/inc/mod_freesync.h
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

// Access structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_freesync {
    pub dummy: c_int,
}

// TODO: References to this should be removed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_freesync_caps {
    pub supported: bool,
    pub min_refresh_in_micro_hz: c_uint,
    pub max_refresh_in_micro_hz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mod_vrr_state {
    VRR_STATE_UNSUPPORTED = 0,
    VRR_STATE_DISABLED,
    VRR_STATE_INACTIVE,
    VRR_STATE_ACTIVE_VARIABLE,
    VRR_STATE_ACTIVE_FIXED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_freesync_config {
    pub state: mod_vrr_state,
    pub vsif_supported: bool,
    pub ramping: bool,
    pub btr: bool,
    pub min_refresh_in_uhz: c_uint,
    pub max_refresh_in_uhz: c_uint,
    pub fixed_refresh_in_uhz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_vrr_params_btr {
    pub btr_enabled: bool,
    pub btr_active: bool,
    pub mid_point_in_us: u32,
    pub inserted_duration_in_us: u32,
    pub frames_to_insert: u32,
    pub frame_counter: u32,
    pub margin_in_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_vrr_params_fixed_refresh {
    pub fixed_active: bool,
    pub ramping_active: bool,
    pub ramping_done: bool,
    pub target_refresh_in_uhz: u32,
    pub frame_counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_vrr_params_flip_interval {
    pub flip_interval_workaround_active: bool,
    pub program_flip_interval_workaround: bool,
    pub do_flip_interval_workaround_cleanup: bool,
    pub flip_interval_detect_counter: u32,
    pub vsyncs_between_flip: u32,
    pub vsync_to_flip_in_us: u32,
    pub v_update_timestamp_in_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_vrr_params {
    pub supported: bool,
    pub send_info_frame: bool,
    pub Module: bool m_const_engaged; // this is used when m_is set up in OPTC so no overriding happens from FreeSync,
    pub state: mod_vrr_state,
    pub min_refresh_in_uhz: u32,
    pub max_duration_in_us: u32,
    pub max_refresh_in_uhz: u32,
    pub min_duration_in_us: u32,
    pub fixed_refresh_in_uhz: u32,
    pub m_const: u32,
    pub adjust: dc_crtc_timing_adjust,
    pub fixed: mod_vrr_params_fixed_refresh,
    pub btr: mod_vrr_params_btr,
    pub flip_interval: mod_vrr_params_flip_interval,
}

extern "C" {
    pub fn mod_freesync_destroy(mod_freesync: *mut mod_freesync);
}
// Returns true when FreeSync is supported and enabled (even if it is inactive)
extern "C" {
    pub fn mod_freesync_get_freesync_enabled(pVrr: *mut mod_vrr_params) -> bool;
}
