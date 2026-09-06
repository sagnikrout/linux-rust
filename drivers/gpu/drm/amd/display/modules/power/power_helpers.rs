//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/power/power_helpers.h
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


// Copyright 2018 Advanced Micro Devices, Inc.
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
pub enum abm_defines {
    abm_defines_max_level = 4,
    abm_defines_max_config = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu_iram_parameters {
    pub backlight_lut_array: *mut c_uint,
    pub backlight_lut_array_size: c_uint,
    pub backlight_ramping_override: bool,
    pub backlight_ramping_reduction: c_uint,
    pub backlight_ramping_start: c_uint,
    pub min_abm_backlight: c_uint,
    pub set: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct backlight_state {
// HW uses u16.16 format for backlight PWM
    pub backlight_pwm: c_uint,
// DM may call power module to set backlight
// targeting percent brightness
//
    pub backlight_millipercent: c_uint,
// DM may call power module to set backlight based on an explicit
// nits value.
//
    pub backlight_millinit: c_uint,
    pub frame_ramp: c_uint,
    pub smooth_brightness_enabled: bool,
    pub isHDR: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_entity {
    pub stream: *mut dc_stream_state,
    pub caps: *mut psr_caps,
    pub psr_context: *mut mod_power_psr_context,
// PSR cached properties
    pub psr_enabled: bool,
    pub psr_events: c_uint,
    pub psr_power_opt: c_uint,
    pub replay_events: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwr_backlight_properties {
    pub use_nits_based_brightness: bool,
    pub disable_fractional_pwm: bool,
    pub min_abm_backlight: c_uint,
    pub num_backlight_levels: c_uint,
    pub backlight_ramping_override: bool,
    pub backlight_ramping_reduction: c_uint,
    pub backlight_ramping_start: c_uint,
// Backlight cached properties
    pub ac_backlight_percent: c_uint,
    pub dc_backlight_percent: c_uint,
// backlight LUT stored in HW u16.16 format
    pub backlight_lut: *mut c_uint,
    pub min_backlight_pwm: c_uint,
    pub max_backlight_pwm: c_uint,
    pub backlight_range: c_uint,
// Describes the panel's min and max luminance in millinits measured
// on full white screen, in min and max backlight settings.
//
    pub min_brightness_millinits: c_uint,
    pub max_brightness_millinits: c_uint,
    pub nits_range: c_uint,
// Backlight control type of the associated link. Cached here so the
// brightness translation helpers can select the correct mapping
// (legacy vs. VESA AUX zero-anchored) without threading the type
// through every call.
//
    pub backlight_control_type: backlight_control_type,
    pub backlight_caps_valid: bool,
    pub use_custom_backlight_caps: bool,
    pub custom_backlight_caps_config_no: c_uint,
    pub use_linear_backlight_curve: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcu_varibright_cached_properties {
    pub varibright_config_setting: c_uint,
    pub varibright_level: c_uint,
    pub varibright_hw_level: c_uint,
    pub def_varibright_level: c_uint,
    pub varibright_user_enable: bool,
    pub varibright_active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_power {
    pub mod_public: mod_power,
    pub dc: *mut dc,
    pub map: *mut power_entity,
    pub varibright_prop: dmcu_varibright_cached_properties,
    pub bl_prop: [pwr_backlight_properties; MAX_NUM_EDP],
    pub bl_state: [backlight_state; MAX_NUM_EDP],
    pub edp_num: c_uint,
    pub psr_smu_optimizations_support: bool,
    pub multi_disp_optimizations_support: bool,
    pub num_entities: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmcu_abm_set_bl_params {
    pub /: *mut *mut unsigned int gradual_change : 1; / [0:0],
    pub /: *mut *mut unsigned int reserved : 15; / [15:1],
    pub /: *mut *mut unsigned int frame_ramp : 16; / [31:16],
    pub bits: },
    pub u32All: c_uint,
}

extern "C" {
    pub fn init_replay_config(link: *mut dc_link, pr_config: *mut replay_config);
}
extern "C" {
    pub fn set_replay_low_rr_full_screen_video_src_vtotal(link: *mut dc_link, vtotal: u16);
}
extern "C" {
    pub fn is_psr_su_specific_panel(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn reset_replay_dsync_error_count(link: *mut dc_link);
}
extern "C" {
    pub fn change_replay_to_psr(link: *mut dc_link);
}
extern "C" {
    pub fn change_psr_to_replay(link: *mut dc_link);
}
extern "C" {
    pub fn initialize_backlight_caps(core_power: *mut core_power, inst: c_uint);
}
extern "C" {
    pub fn mod_power_hw_init_backlight(mod_power: *mut mod_power) -> bool;
}
