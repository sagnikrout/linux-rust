//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/optc.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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
// DOC: overview
//
// Output Pipe Timing Combiner (OPTC) includes two major functional blocks:
// Output Data Mapper (ODM) and Output Timing Generator (OTG).
//
// - ODM: It is Output Data Mapping block. It can combine input data from
// multiple OPP data pipes into one single data stream or split data from one
// OPP data pipe into multiple data streams or just bypass OPP data to DIO.
// - OTG: It is Output Timing Generator. It generates display timing signals to
// drive the display output.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct optc {
    pub base: timing_generator,
    pub tg_regs: *const dcn_optc_registers,
    pub tg_shift: *const dcn_optc_shift,
    pub tg_mask: *const dcn_optc_mask,
    pub opp_count: c_int,
    pub max_h_total: u32,
    pub max_v_total: u32,
    pub min_h_blank: u32,
    pub min_h_sync_width: u32,
    pub min_v_sync_width: u32,
    pub min_v_blank: u32,
    pub min_v_blank_interlace: u32,
    pub vstartup_start: c_int,
    pub vupdate_offset: c_int,
    pub vupdate_width: c_int,
    pub vready_offset: c_int,
    pub pstate_keepout: c_int,
    pub orginal_patched_timing: dc_crtc_timing,
    pub signal: signal_type,
}

extern "C" {
    pub fn optc1_read_otg_state(optc: *mut timing_generator, s: *mut dcn_otg_state);
}
extern "C" {
    pub fn optc1_get_hw_timing(tg: *mut timing_generator, hw_crtc_timing: *mut dc_crtc_timing) -> bool;
}
extern "C" {
    pub fn optc1_disable_crtc(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_is_counter_moving(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_get_vblank_counter(optc: *mut timing_generator) -> u32;
}
extern "C" {
    pub fn optc1_is_blanked(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_did_triggered_reset_occur(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_enable_reset_trigger(optc: *mut timing_generator, source_tg_inst: c_int);
}
extern "C" {
    pub fn optc1_disable_reset_trigger(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc1_lock(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc1_unlock(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc1_enable_optc_clock(optc: *mut timing_generator, enable: bool);
}
extern "C" {
    pub fn optc1_set_vtotal_min_max(optc: *mut timing_generator, vtotal_min: c_int, vtotal_max: c_int);
}
extern "C" {
    pub fn optc1_is_stereo_left_eye(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_clear_optc_underflow(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc1_tg_init(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc1_is_tg_enabled(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_is_optc_underflow_occurred(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc1_set_blank_data_double_buffer(optc: *mut timing_generator, enable: bool);
}
extern "C" {
    pub fn optc1_set_timing_double_buffer(optc: *mut timing_generator, enable: bool);
}
extern "C" {
    pub fn optc1_configure_crc(optc: *mut timing_generator, params: *const crc_params) -> bool;
}
extern "C" {
    pub fn optc1_is_two_pixels_per_container(timing: *const dc_crtc_timing) -> bool;
}
