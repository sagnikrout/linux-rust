//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/bios_parser_types.h
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

// TODO: include signal_types.h and remove this enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum as_signal_type {
    AS_SIGNAL_TYPE_NONE = 0L, /* no signal */
    AS_SIGNAL_TYPE_DVI,
    AS_SIGNAL_TYPE_HDMI,
    AS_SIGNAL_TYPE_LVDS,
    AS_SIGNAL_TYPE_DISPLAY_PORT,
    AS_SIGNAL_TYPE_GPU_PLL,
    AS_SIGNAL_TYPE_XGMI,
    AS_SIGNAL_TYPE_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_result {
    BP_RESULT_OK = 0, /* There was no error */
    BP_RESULT_BADINPUT, /*Bad input parameter */
    BP_RESULT_BADBIOSTABLE, /* Bad BIOS table */
    BP_RESULT_UNSUPPORTED, /* BIOS Table is not supported */
    BP_RESULT_NORECORD, /* Record can't be found */
    BP_RESULT_FAILURE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_encoder_control_action {
// direct VBIOS translation! Just to simplify the translation
    ENCODER_CONTROL_DISABLE = 0,
    ENCODER_CONTROL_ENABLE,
    ENCODER_CONTROL_SETUP,
    ENCODER_CONTROL_INIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_transmitter_control_action {
// direct VBIOS translation! Just to simplify the translation
    TRANSMITTER_CONTROL_DISABLE = 0,
    TRANSMITTER_CONTROL_ENABLE,
    TRANSMITTER_CONTROL_BACKLIGHT_OFF,
    TRANSMITTER_CONTROL_BACKLIGHT_ON,
    TRANSMITTER_CONTROL_BACKLIGHT_BRIGHTNESS,
    TRANSMITTER_CONTROL_LCD_SETF_TEST_START,
    TRANSMITTER_CONTROL_LCD_SELF_TEST_STOP,
    TRANSMITTER_CONTROL_INIT,
    TRANSMITTER_CONTROL_DEACTIVATE,
    TRANSMITTER_CONTROL_ACTIAVATE,
    TRANSMITTER_CONTROL_SETUP,
    TRANSMITTER_CONTROL_SET_VOLTAGE_AND_PREEMPASIS,
// ATOM_TRANSMITTER_ACTION_POWER_ON. This action is for eDP only
// (power up the panel)
//
    TRANSMITTER_CONTROL_POWER_ON,
// ATOM_TRANSMITTER_ACTION_POWER_OFF. This action is for eDP only
// (power down the panel)
//
    TRANSMITTER_CONTROL_POWER_OFF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_external_encoder_control_action {
    EXTERNAL_ENCODER_CONTROL_DISABLE = 0,
    EXTERNAL_ENCODER_CONTROL_ENABLE = 1,
    EXTERNAL_ENCODER_CONTROL_INIT = 0x7,
    EXTERNAL_ENCODER_CONTROL_SETUP = 0xf,
    EXTERNAL_ENCODER_CONTROL_UNBLANK = 0x10,
    EXTERNAL_ENCODER_CONTROL_BLANK = 0x11,
    EXTERNAL_ENCODER_CONTROL_DAC_LOAD_DETECT = 0x12,
    EXTERNAL_ENCODER_CONTROL_DDC_SETUP = 0x14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_pipe_control_action {
    ASIC_PIPE_DISABLE = 0,
    ASIC_PIPE_ENABLE,
    ASIC_PIPE_INIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_lvtma_control_action {
    LVTMA_CONTROL_LCD_BLOFF = 2,
    LVTMA_CONTROL_LCD_BLON = 3,
    LVTMA_CONTROL_POWER_ON = 12,
    LVTMA_CONTROL_POWER_OFF = 13
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_encoder_control {
    pub action: bp_encoder_control_action,
    pub engine_id: engine_id,
    pub transmitter: transmitter,
    pub signal: signal_type,
    pub lanes_number: dc_lane_count,
    pub color_depth: dc_color_depth,
    pub enable_dp_audio: bool,
    pub /: *mut *mut uint32_t pixel_clock; / khz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_external_encoder_control {
    pub action: bp_external_encoder_control_action,
    pub engine_id: engine_id,
    pub link_rate: dc_link_rate,
    pub lanes_number: dc_lane_count,
    pub signal: signal_type,
    pub color_depth: dc_color_depth,
    pub coherent: bool,
    pub encoder_id: graphics_object_id,
    pub connector_obj_id: graphics_object_id,
    pub /: *mut *mut uint32_t pixel_clock; / in KHz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_crtc_source_select {
    pub engine_id: engine_id,
    pub controller_id: controller_id,
    pub sink_signal: signal_type,
    pub color_depth: dc_color_depth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_transmitter_control {
    pub action: bp_transmitter_control_action,
    pub engine_id: engine_id,
    pub /: *mut *mut transmitter transmitter; / PhyId,
    pub lanes_number: dc_lane_count,
    pub /: *mut *mut clock_source_id pll_id; / needed for DCE 4.0,
    pub signal: signal_type,
    pub /: *mut *mut dc_color_depth color_depth; / not used for DCE6.0,
    pub /: *mut *mut hpd_source_id hpd_sel; / ucHPDSel, used for DCe6.0,
    pub /: *mut *mut tx_ffe_id txffe_sel; / used for DCN3,
    pub /: *mut *mut engine_id hpo_engine_id; / used for DCN3,
    pub connector_obj_id: graphics_object_id,
// symClock; in 10kHz, pixel clock, in HDMI deep color mode, it should
// be pixel clock * deep_color_ratio (in KHz)
//
    pub pixel_clock: u32,
    pub lane_select: u32,
    pub lane_settings: u32,
    pub coherent: bool,
    pub multi_path: bool,
    pub single_pll_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_load_detection_parameters {
    pub engine_id: engine_id,
    pub device_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_hw_crtc_timing_parameters {
    pub controller_id: controller_id,
// horizontal part
    pub h_total: u32,
    pub h_addressable: u32,
    pub h_overscan_left: u32,
    pub h_overscan_right: u32,
    pub h_sync_start: u32,
    pub h_sync_width: u32,
// vertical part
    pub v_total: u32,
    pub v_addressable: u32,
    pub v_overscan_top: u32,
    pub v_overscan_bottom: u32,
    pub v_sync_start: u32,
    pub v_sync_width: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timing_flags {
    pub INTERLACE:1: u32,
    pub PIXEL_REPETITION:4: u32,
    pub HSYNC_POSITIVE_POLARITY:1: u32,
    pub VSYNC_POSITIVE_POLARITY:1: u32,
    pub HORZ_COUNT_BY_TWO:1: u32,
    pub flags: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_adjust_pixel_clock_parameters {
// Input: Signal Type - to be converted to Encoder mode
    pub signal_type: signal_type,
// Input: Encoder object id
    pub encoder_object_id: graphics_object_id,
// Input: Pixel Clock (requested Pixel clock based on Video timing
// standard used) in KHz
//
    pub pixel_clock: u32,
// Output: Adjusted Pixel Clock (after VBIOS exec table) in KHz
    pub adjusted_pixel_clock: u32,
// Output: If non-zero, this refDiv value should be used to calculate
// other ppll params
    pub reference_divider: u32,
// Output: If non-zero, this postDiv value should be used to calculate
// other ppll params
    pub pixel_clock_post_divider: u32,
// Input: Enable spread spectrum
    pub ss_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_pixel_clock_parameters {
    pub /: *mut *mut controller_id controller_id; / (Which CRTC uses this PLL),
    pub /: *mut *mut clock_source_id pll_id; / Clock Source Id,
// signal_type -> Encoder Mode - needed by VBIOS Exec table
    pub signal_type: signal_type,
// Adjusted Pixel Clock (after VBIOS exec table)
// that becomes Target Pixel Clock (100 Hz units)
    pub target_pixel_clock_100hz: u32,
// Calculated Reference divider of Display PLL
    pub reference_divider: u32,
// Calculated Feedback divider of Display PLL
    pub feedback_divider: u32,
// Calculated Fractional Feedback divider of Display PLL
    pub fractional_feedback_divider: u32,
// Calculated Pixel Clock Post divider of Display PLL
    pub pixel_clock_post_divider: u32,
    pub /: *mut *mut graphics_object_id encoder_object_id; / Encoder object id,
// VBIOS returns a fixed display clock when DFS-bypass feature
// is enabled (KHz)
    pub dfs_bypass_display_clock: u32,
// color depth to support HDMI deep color
    pub color_depth: transmitter_color_depth,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_pixel_clock_flags {
    pub FORCE_PROGRAMMING_OF_PLL:1: u32,
// Use Engine Clock as source for Display Clock when
// programming PLL
    pub USE_E_CLOCK_AS_SOURCE_FOR_D_CLOCK:1: u32,
// Use external reference clock (refDivSrc for PLL)
    pub SET_EXTERNAL_REF_DIV_SRC:1: u32,
// Use DFS bypass for Display clock.
    pub SET_DISPCLK_DFS_BYPASS:1: u32,
// Force program PHY PLL only
    pub PROGRAM_PHY_PLL_ONLY:1: u32,
// Support for YUV420
    pub SUPPORT_YUV_420:1: u32,
// Use XTALIN reference clock source
    pub SET_XTALIN_REF_SRC:1: u32,
// Use GENLK reference clock source
    pub SET_GENLOCK_REF_DIV_SRC:1: u32,
    pub flags: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bp_dce_clock_type {
    DCECLOCK_TYPE_DISPLAY_CLOCK = 0,
    DCECLOCK_TYPE_DPREFCLK      = 1
}

// DCE Clock Parameters structure for SetDceClock Exec command table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_set_dce_clock_parameters {
    pub /: *mut *mut clock_source_id pll_id; / Clock Source Id,
// Display clock or DPREFCLK value
    pub target_clock_frequency: u32,
// Clock to set: =0: DISPCLK  =1: DPREFCLK  =2: PIXCLK
    pub clock_type: bp_dce_clock_type,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_dce_clock_flags {
    pub USE_GENERICA_AS_SOURCE_FOR_DPREFCLK:1: u32,
// Use XTALIN reference clock source
    pub USE_XTALIN_AS_SOURCE_FOR_DPREFCLK:1: u32,
// Use PCIE reference clock source
    pub USE_PCIE_AS_SOURCE_FOR_DPREFCLK:1: u32,
// Use GENLK reference clock source
    pub USE_GENLOCK_AS_SOURCE_FOR_DPREFCLK:1: u32,
    pub flags: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spread_spectrum_flags {
// 1 = Center Spread; 0 = down spread
    pub CENTER_SPREAD:1: u32,
// 1 = external; 0 = internal
    pub EXTERNAL_SS:1: u32,
// 1 = delta-sigma type parameter; 0 = ver1
    pub DS_TYPE:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_spread_spectrum_parameters {
    pub pll_id: clock_source_id,
    pub percentage: u32,
    pub ds_frac_amount: u32,
    pub step: u32,
    pub delay: u32,
    pub /: *mut *mut uint32_t range; / In Hz unit,
    pub ver1: },
    pub feedback_amount: u32,
    pub nfrac_amount: u32,
    pub ds_frac_size: u32,
    pub ds: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_disp_connector_caps_info {
    pub 1: uint32_t INTERNAL_DISPLAY :,
    pub 1: uint32_t INTERNAL_DISPLAY_BL :,
    pub 1: uint32_t NO_DDC_PIN :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_encoder_cap_info {
    pub DP_HBR2_CAP:1: u32,
    pub DP_HBR2_EN:1: u32,
    pub DP_HBR3_EN:1: u32,
    pub HDMI_6GB_EN:1: u32,
    pub IS_DP2_CAPABLE:1: u32,
    pub DP_UHBR10_EN:1: u32,
    pub DP_UHBR13_5_EN:1: u32,
    pub DP_UHBR20_EN:1: u32,
    pub DP_IS_USB_C:1: u32,
    pub IS_HDMI_FRL_CAPABLE:1: u32,
    pub FRL_8G_EN:1: u32,
    pub FRL_10G_EN:1: u32,
    pub FRL_12G_EN:1: u32,
    pub RESERVED:19: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_soc_bb_info {
    pub dram_clock_change_latency_100ns: u32,
    pub dram_sr_exit_latency_100ns: u32,
    pub dram_sr_enter_exit_latency_100ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bp_connector_speed_cap_info {
    pub DP_HBR2_EN:1: u32,
    pub DP_HBR3_EN:1: u32,
    pub HDMI_6GB_EN:1: u32,
    pub DP_UHBR10_EN:1: u32,
    pub DP_UHBR13_5_EN:1: u32,
    pub DP_UHBR20_EN:1: u32,
    pub DP_IS_USB_C:1: u32,
    pub FRL_8G_EN:1: u32,
    pub FRL_10G_EN:1: u32,
    pub FRL_12G_EN:1: u32,
    pub FRL_16G_EN:1: u32,
    pub FRL_20G_EN:1: u32,
    pub FRL_24G_EN:1: u32,
    pub RESERVED:19: u32,
}
