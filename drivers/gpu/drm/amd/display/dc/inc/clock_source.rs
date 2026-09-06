//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/clock_source.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spread_spectrum_data {
    pub 0.001%*/: *mut *mut uint32_t percentage; /> In unit of 0.01% or,
    pub /: *mut *mut uint32_t percentage_divider; /> 100 or 1000,
    pub freq_range_khz: u32,
    pub modulation_freq_hz: u32,
    pub flags: spread_spectrum_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct delta_sigma_data {
    pub feedback_amount: u32,
    pub nfrac_amount: u32,
    pub ds_frac_size: u32,
    pub ds_frac_amount: u32,
}

//
// Pixel Clock Parameters structure
// These parameters are required as input
// when calculating Pixel Clock Dividers for requested Pixel Clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_clk_flags {
    pub ENABLE_SS:1: u32,
    pub DISPLAY_BLANKED:1: u32,
    pub PROGRAM_PIXEL_CLOCK:1: u32,
    pub PROGRAM_ID_CLOCK:1: u32,
    pub SUPPORT_YCBCR420:1: u32,
}

//
// Display Port HW De spread of Reference Clock related Parameters structure
// Store it once at boot for later usage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csdp_ref_clk_ds_params {
    pub hw_dso_n_dp_ref_clk: bool,
// Flag for HW De Spread enabled (if enabled SS on DP Reference Clock)
    pub avg_dp_ref_clk_khz: u32,
// Average DP Reference clock (in KHz)
    pub ss_percentage_on_dp_ref_clk: u32,
// DP Reference clock SS percentage
// (not to be mixed with DP IDCLK SS from PLL Settings)
    pub ss_percentage_divider: u32,
// DP Reference clock SS percentage divider
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_clk_params {
    pub requested_pix_clk_100hz: u32,
// > Requested Pixel Clock
// (based on Video Timing standard used for requested mode)
    pub /: *mut *mut uint32_t requested_sym_clk; / in KHz,
// > Requested Sym Clock (relevant only for display port)
    pub /: *mut *mut uint32_t dp_ref_clk; / in KHz,
// > DP reference clock - calculated only for DP signal for specific cases
    pub encoder_object_id: graphics_object_id,
// > Encoder object Id - needed by VBIOS Exec table
    pub signal_type: signal_type,
// > signalType -> Encoder Mode - needed by VBIOS Exec table
    pub controller_id: controller_id,
// > ControllerId - which controller using this PLL
    pub color_depth: dc_color_depth,
    pub de_spread_params: csdp_ref_clk_ds_params,
// > de-spread info, relevant only for on-the-fly tune-up pixel rate
    pub pixel_encoding: dc_pixel_encoding,
    pub flags: pixel_clk_flags,
    pub dio_se_pix_per_cycle: u32,
}

//
// Pixel Clock Dividers structure with desired Pixel Clock
// (adjusted after VBIOS exec table),
// with actually calculated Clock and reference Crystal frequency
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_settings {
    pub actual_pix_clk_100hz: u32,
    pub adjusted_pix_clk_100hz: u32,
    pub calculated_pix_clk_100hz: u32,
    pub vco_freq: u32,
    pub reference_freq: u32,
    pub reference_divider: u32,
    pub feedback_divider: u32,
    pub fract_feedback_divider: u32,
    pub pix_clk_post_divider: u32,
    pub ss_percentage: u32,
    pub use_external_clk: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calc_pll_clock_source_init_data {
    pub bp: *mut dc_bios,
    pub min_pix_clk_pll_post_divider: u32,
    pub max_pix_clk_pll_post_divider: u32,
    pub min_pll_ref_divider: u32,
    pub max_pll_ref_divider: u32,
    pub min_override_input_pxl_clk_pll_freq_khz: u32,
// if not 0, override the firmware info
    pub max_override_input_pxl_clk_pll_freq_khz: u32,
// if not 0, override the firmware info
    pub num_fract_fb_divider_decimal_point: u32,
// number of decimal point for fractional feedback divider value
    pub num_fract_fb_divider_decimal_point_precision: u32,
// number of decimal point to round off for fractional feedback divider value
    pub ctx: *mut dc_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calc_pll_clock_source {
    pub ref_freq_khz: u32,
    pub min_pix_clock_pll_post_divider: u32,
    pub max_pix_clock_pll_post_divider: u32,
    pub min_pll_ref_divider: u32,
    pub max_pll_ref_divider: u32,
    pub max_vco_khz: u32,
    pub min_vco_khz: u32,
    pub min_pll_input_freq_khz: u32,
    pub max_pll_input_freq_khz: u32,
    pub fract_fb_divider_decimal_points_num: u32,
    pub fract_fb_divider_factor: u32,
    pub fract_fb_divider_precision: u32,
    pub fract_fb_divider_precision_factor: u32,
    pub ctx: *mut dc_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_source_funcs {
    pub ): *mut clock_source,
    pub ): *mut pll_settings,
    pub ): *mut pll_settings,
    pub pixel_clk_100hz): *mut c_uint,
    pub ref_clk): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_source {
    pub funcs: *const clock_source_funcs,
    pub ctx: *mut dc_context,
    pub id: clock_source_id,
    pub dp_clk_src: bool,
}
