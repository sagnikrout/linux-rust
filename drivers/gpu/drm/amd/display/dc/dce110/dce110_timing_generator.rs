//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce110/dce110_timing_generator.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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

// GSL Sync related values
// In VSync mode, after 4 units of time, master pipe will generate
// flip_ready signal
pub const VFLIP_READY_DELAY: c_int = 4;
// In HSync mode, after 2 units of time, master pipe will generate
// flip_ready signal
pub const HFLIP_READY_DELAY: c_int = 2;
// 6 lines delay between forcing flip and checking all pipes ready
pub const HFLIP_CHECK_DELAY: c_int = 6;
// 3 lines before end of frame
pub const FLIP_READY_BACK_LOOKUP: c_int = 3;
// Trigger Source Select - ASIC-defendant, actual values for the
// register programming
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trigger_source_select {
    TRIGGER_SOURCE_SELECT_LOGIC_ZERO = 0,
    TRIGGER_SOURCE_SELECT_CRTC_VSYNCA = 1,
    TRIGGER_SOURCE_SELECT_CRTC_HSYNCA = 2,
    TRIGGER_SOURCE_SELECT_CRTC_VSYNCB = 3,
    TRIGGER_SOURCE_SELECT_CRTC_HSYNCB = 4,
    TRIGGER_SOURCE_SELECT_GENERICF = 5,
    TRIGGER_SOURCE_SELECT_GENERICE = 6,
    TRIGGER_SOURCE_SELECT_VSYNCA = 7,
    TRIGGER_SOURCE_SELECT_HSYNCA = 8,
    TRIGGER_SOURCE_SELECT_VSYNCB = 9,
    TRIGGER_SOURCE_SELECT_HSYNCB = 10,
    TRIGGER_SOURCE_SELECT_HPD1 = 11,
    TRIGGER_SOURCE_SELECT_HPD2 = 12,
    TRIGGER_SOURCE_SELECT_GENERICD = 13,
    TRIGGER_SOURCE_SELECT_GENERICC = 14,
    TRIGGER_SOURCE_SELECT_VIDEO_CAPTURE = 15,
    TRIGGER_SOURCE_SELECT_GSL_GROUP0 = 16,
    TRIGGER_SOURCE_SELECT_GSL_GROUP1 = 17,
    TRIGGER_SOURCE_SELECT_GSL_GROUP2 = 18,
    TRIGGER_SOURCE_SELECT_BLONY = 19,
    TRIGGER_SOURCE_SELECT_GENERICA = 20,
    TRIGGER_SOURCE_SELECT_GENERICB = 21,
    TRIGGER_SOURCE_SELECT_GSL_ALLOW_FLIP = 22,
    TRIGGER_SOURCE_SELECT_MANUAL_TRIGGER = 23
}

// Trigger Source Select - ASIC-dependant, actual values for the
// register programming
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trigger_polarity_select {
    TRIGGER_POLARITY_SELECT_LOGIC_ZERO = 0,
    TRIGGER_POLARITY_SELECT_CRTC = 1,
    TRIGGER_POLARITY_SELECT_GENERICA = 2,
    TRIGGER_POLARITY_SELECT_GENERICB = 3,
    TRIGGER_POLARITY_SELECT_HSYNCA = 4,
    TRIGGER_POLARITY_SELECT_HSYNCB = 5,
    TRIGGER_POLARITY_SELECT_VIDEO_CAPTURE = 6,
    TRIGGER_POLARITY_SELECT_GENERICC = 7
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_timing_generator_offsets {
    pub crtc: i32,
    pub dcp: i32,
// DCE80 use only
    pub dmif: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_timing_generator {
    pub base: timing_generator,
    pub offsets: dce110_timing_generator_offsets,
    pub derived_offsets: dce110_timing_generator_offsets,
    pub controller_id: controller_id,
    pub max_h_total: u32,
    pub max_v_total: u32,
    pub min_h_blank: u32,
    pub min_h_front_porch: u32,
    pub min_h_back_porch: u32,
// DCE 12
    pub min_h_sync_width: u32,
    pub min_v_sync_width: u32,
    pub min_v_blank: u32,
}

// Macro flag: #define DCE110TG_FROM_TG(tg)\
// determine if given timing can be supported by TG
// HW programming
// Program timing generator with given timing
// Disable/Enable Timing Generator
extern "C" {
    pub fn dce110_timing_generator_enable_crtc(tg: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn dce110_timing_generator_disable_crtc(tg: *mut timing_generator) -> bool;
}
// TG current status
// return the current frame counter. Used by Linux kernel DRM
// return true if TG counter is moving. false if TG is stopped
extern "C" {
    pub fn dce110_timing_generator_is_counter_moving(tg: *mut timing_generator) -> bool;
}
// wait until TG is in beginning of vertical blank region
extern "C" {
    pub fn dce110_timing_generator_wait_for_vblank(tg: *mut timing_generator);
}
// wait until TG is in beginning of active region
extern "C" {
    pub fn dce110_timing_generator_wait_for_vactive(tg: *mut timing_generator);
}
// Timing Generator Synchronization routines
// Setups Global Swap Lock group, TimingServer or TimingClient
// Clear all the register writes done by setup_global_swap_lock
// Reset crtc position on master VSync
// Reset slave controllers on master VSync
// disabling trigger-reset
// Checks whether CRTC triggered reset occurred
// Stuff to move to other virtual HW objects
// Move to enable accelerated mode
extern "C" {
    pub fn dce110_timing_generator_disable_vga(tg: *mut timing_generator);
}
// TODO: Should we move it to transform
// Fully program CRTC timing in timing generator
// TODO: Should we move it to opp?
// Combine with below and move YUV/RGB color conversion to SW layer
// Combine with above and move YUV/RGB color conversion to SW layer
// End-of-move
// Not called yet
// TODO: replace 'controller_dp_test_pattern' by 'test_pattern_mode'
// because this is not DP-specific (which is probably somewhere in DP
// encoder)
extern "C" {
    pub fn dce110_tg_is_blanked(tg: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn dce110_is_two_pixels_per_container(timing: *const dc_crtc_timing) -> bool;
}
