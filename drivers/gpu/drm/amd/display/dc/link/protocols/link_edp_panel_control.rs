//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/link/protocols/link_edp_panel_control.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

extern "C" {
    pub fn dp_get_panel_mode(link: *mut dc_link) -> dp_panel_mode;
}
extern "C" {
    pub fn dp_set_panel_mode(link: *mut dc_link, panel_mode: dp_panel_mode);
}
extern "C" {
    pub fn set_default_brightness_aux(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn is_smartmux_suported(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn edp_panel_backlight_power_on(link: *mut dc_link, wait_for_hpd: bool);
}
extern "C" {
    pub fn edp_get_backlight_level(link: *const dc_link) -> c_int;
}
extern "C" {
    pub fn edp_get_target_backlight_pwm(link: *const dc_link) -> c_int;
}
extern "C" {
    pub fn edp_get_psr_state(link: *const dc_link, state: *mut dc_psr_state) -> bool;
}
extern "C" {
    pub fn edp_get_psr_residency(link: *const dc_link, residency: *mut u32, mode: psr_residency_mode);
}
extern "C" {
    pub fn edp_set_coasting_vtotal(link: *mut dc_link, coasting_vtotal: u32, frame_skip_number: u16) -> bool;
}
extern "C" {
    pub fn edp_get_replay_state(link: *const dc_link, state: *mut u64) -> bool;
}
extern "C" {
    pub fn edp_wait_for_t12(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn edp_is_ilr_optimization_enabled(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn get_max_edp_link_rate(link: *mut dc_link) -> dc_link_rate;
}
extern "C" {
    pub fn edp_backlight_enable_aux(link: *mut dc_link, enable: bool) -> bool;
}
extern "C" {
    pub fn edp_add_delay_for_T9(link: *mut dc_link);
}
extern "C" {
    pub fn edp_receiver_ready_T9(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn edp_receiver_ready_T7(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn edp_power_alpm_dpcd_enable(link: *mut dc_link, enable: bool) -> bool;
}
extern "C" {
    pub fn edp_setup_freesync_replay(link: *mut dc_link, stream: *const dc_stream_state) -> bool;
}
extern "C" {
    pub fn edp_set_panel_power(link: *mut dc_link, powerOn: bool);
}
