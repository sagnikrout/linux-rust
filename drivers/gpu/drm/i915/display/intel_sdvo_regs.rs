//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_sdvo_regs.h
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
// Copyright © 2006-2007 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Authors:
// Eric Anholt <eric@anholt.net>
//

//
// SDVO command definitions and structures.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_caps {
    pub vendor_id: u8,
    pub device_id: u8,
    pub device_rev_id: u8,
    pub sdvo_version_major: u8,
    pub sdvo_version_minor: u8,
    pub sdvo_num_inputs:2: c_uint,
    pub smooth_scaling:1: c_uint,
    pub sharp_scaling:1: c_uint,
    pub up_scaling:1: c_uint,
    pub down_scaling:1: c_uint,
    pub stall_support:1: c_uint,
    pub pad:1: c_uint,
    pub output_flags: u16,
    pub __packed: },
// Note: SDVO detailed timing flags match EDID misc flags.

// This matches the EDID DTD structure, more or less
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_dtd {
    pub /: *mut *mut u16 clock; / pixel clock, in 10kHz units,
    pub /: *mut *mut u8 h_active; / lower 8 bits (pixels),
    pub /: *mut *mut u8 h_blank; / lower 8 bits (pixels),
    pub /: *mut *mut u8 h_high; / upper 4 bits each h_active, h_blank,
    pub /: *mut *mut u8 v_active; / lower 8 bits (lines),
    pub /: *mut *mut u8 v_blank; / lower 8 bits (lines),
    pub /: *mut *mut u8 v_high; / upper 4 bits each v_active, v_blank,
    pub part1: },
    pub /: *mut *mut u8 h_sync_off; / lower 8 bits, from hblank start,
    pub /: *mut *mut u8 h_sync_width; / lower 8 bits (pixels),
// lower 4 bits each vsync offset, vsync width
    pub v_sync_off_width: u8,
//
// 2 high bits of hsync offset, 2 high bits of hsync width,
// bits 4-5 of vsync offset, and 2 high bits of vsync width.
//
    pub sync_off_width_high: u8,
    pub dtd_flags: u8,
    pub sdvo_flags: u8,
// bits 6-7 of vsync offset at bits 6-7
    pub v_sync_off_high: u8,
    pub reserved: u8,
    pub part2: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_pixel_clock_range {
    pub /: *mut *mut u16 min; / pixel clock, in 10kHz units,
    pub /: *mut *mut u16 max; / pixel clock, in 10kHz units,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_preferred_input_timing_args {
    pub clock: u16,
    pub width: u16,
    pub height: u16,
    pub interlace:1: u8,
    pub scaled:1: u8,
    pub pad:6: u8,
    pub __packed: },
// I2C registers for SDVO
pub const SDVO_I2C_ARG_0: c_uint = 0x07;
pub const SDVO_I2C_ARG_1: c_uint = 0x06;
pub const SDVO_I2C_ARG_2: c_uint = 0x05;
pub const SDVO_I2C_ARG_3: c_uint = 0x04;
pub const SDVO_I2C_ARG_4: c_uint = 0x03;
pub const SDVO_I2C_ARG_5: c_uint = 0x02;
pub const SDVO_I2C_ARG_6: c_uint = 0x01;
pub const SDVO_I2C_ARG_7: c_uint = 0x00;
pub const SDVO_I2C_OPCODE: c_uint = 0x08;
pub const SDVO_I2C_CMD_STATUS: c_uint = 0x09;
pub const SDVO_I2C_RETURN_0: c_uint = 0x0a;
pub const SDVO_I2C_RETURN_1: c_uint = 0x0b;
pub const SDVO_I2C_RETURN_2: c_uint = 0x0c;
pub const SDVO_I2C_RETURN_3: c_uint = 0x0d;
pub const SDVO_I2C_RETURN_4: c_uint = 0x0e;
pub const SDVO_I2C_RETURN_5: c_uint = 0x0f;
pub const SDVO_I2C_RETURN_6: c_uint = 0x10;
pub const SDVO_I2C_RETURN_7: c_uint = 0x11;
pub const SDVO_I2C_VENDOR_BEGIN: c_uint = 0x20;
// Status results
pub const SDVO_CMD_STATUS_POWER_ON: c_uint = 0x0;
pub const SDVO_CMD_STATUS_SUCCESS: c_uint = 0x1;
pub const SDVO_CMD_STATUS_NOTSUPP: c_uint = 0x2;
pub const SDVO_CMD_STATUS_INVALID_ARG: c_uint = 0x3;
pub const SDVO_CMD_STATUS_PENDING: c_uint = 0x4;
pub const SDVO_CMD_STATUS_TARGET_NOT_SPECIFIED: c_uint = 0x5;
pub const SDVO_CMD_STATUS_SCALING_NOT_SUPP: c_uint = 0x6;
// SDVO commands, argument/result registers
pub const SDVO_CMD_RESET: c_uint = 0x01;
// Returns a struct intel_sdvo_caps
pub const SDVO_CMD_GET_DEVICE_CAPS: c_uint = 0x02;
pub const SDVO_CMD_GET_FIRMWARE_REV: c_uint = 0x86;

//
// Reports which inputs are trained (managed to sync).
//
// Devices must have trained within 2 vsyncs of a mode change.
//
pub const SDVO_CMD_GET_TRAINED_INPUTS: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_get_trained_inputs_response {
    pub input0_trained:1: c_uint,
    pub input1_trained:1: c_uint,
    pub pad:6: c_uint,
    pub __packed: },
// Returns a struct intel_sdvo_output_flags of active outputs.
pub const SDVO_CMD_GET_ACTIVE_OUTPUTS: c_uint = 0x04;
//
// Sets the current set of active outputs.
//
// Takes a struct intel_sdvo_output_flags.  Must be preceded by a SET_IN_OUT_MAP
// on multi-output devices.
//
pub const SDVO_CMD_SET_ACTIVE_OUTPUTS: c_uint = 0x05;
//
// Returns the current mapping of SDVO inputs to outputs on the device.
//
// Returns two struct intel_sdvo_output_flags structures.
//
pub const SDVO_CMD_GET_IN_OUT_MAP: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_in_out_map {
    pub in1: u16 in0,,
}

//
// Sets the current mapping of SDVO inputs to outputs on the device.
//
// Takes two struct i380_sdvo_output_flags structures.
//
pub const SDVO_CMD_SET_IN_OUT_MAP: c_uint = 0x07;
//
// Returns a struct intel_sdvo_output_flags of attached displays.
//
pub const SDVO_CMD_GET_ATTACHED_DISPLAYS: c_uint = 0x0b;
//
// Returns a struct intel_sdvo_ouptut_flags of displays supporting hot plugging.
//
pub const SDVO_CMD_GET_HOT_PLUG_SUPPORT: c_uint = 0x0c;
//
// Takes a struct intel_sdvo_output_flags.
//
pub const SDVO_CMD_SET_ACTIVE_HOT_PLUG: c_uint = 0x0d;
//
// Returns a struct intel_sdvo_output_flags of displays with hot plug
// interrupts enabled.
//
pub const SDVO_CMD_GET_ACTIVE_HOT_PLUG: c_uint = 0x0e;
pub const SDVO_CMD_GET_INTERRUPT_EVENT_SOURCE: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_get_interrupt_event_source_response {
    pub interrupt_status: u16,
    pub ambient_light_interrupt:1: c_uint,
    pub hdmi_audio_encrypt_change:1: c_uint,
    pub pad:6: c_uint,
    pub __packed: },
//
// Selects which input is affected by future input commands.
//
// Commands affected include SET_INPUT_TIMINGS_PART[12],
// GET_INPUT_TIMINGS_PART[12], GET_PREFERRED_INPUT_TIMINGS_PART[12],
// GET_INPUT_PIXEL_CLOCK_RANGE, and CREATE_PREFERRED_INPUT_TIMINGS.
//
pub const SDVO_CMD_SET_TARGET_INPUT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_set_target_input_args {
    pub target_1:1: c_uint,
    pub pad:7: c_uint,
    pub __packed: },
//
// Takes a struct intel_sdvo_output_flags of which outputs are targeted by
// future output commands.
//
// Affected commands include SET_OUTPUT_TIMINGS_PART[12],
// GET_OUTPUT_TIMINGS_PART[12], and GET_OUTPUT_PIXEL_CLOCK_RANGE.
//
pub const SDVO_CMD_SET_TARGET_OUTPUT: c_uint = 0x11;
pub const SDVO_CMD_GET_INPUT_TIMINGS_PART1: c_uint = 0x12;
pub const SDVO_CMD_GET_INPUT_TIMINGS_PART2: c_uint = 0x13;
pub const SDVO_CMD_SET_INPUT_TIMINGS_PART1: c_uint = 0x14;
pub const SDVO_CMD_SET_INPUT_TIMINGS_PART2: c_uint = 0x15;
pub const SDVO_CMD_SET_OUTPUT_TIMINGS_PART1: c_uint = 0x16;
pub const SDVO_CMD_SET_OUTPUT_TIMINGS_PART2: c_uint = 0x17;
pub const SDVO_CMD_GET_OUTPUT_TIMINGS_PART1: c_uint = 0x18;
pub const SDVO_CMD_GET_OUTPUT_TIMINGS_PART2: c_uint = 0x19;
// Part 1

// Part 2

//
// Generates a DTD based on the given width, height, and flags.
//
// This will be supported by any device supporting scaling or interlaced
// modes.
//
pub const SDVO_CMD_CREATE_PREFERRED_INPUT_TIMING: c_uint = 0x1a;

pub const SDVO_CMD_GET_PREFERRED_INPUT_TIMING_PART1: c_uint = 0x1b;
pub const SDVO_CMD_GET_PREFERRED_INPUT_TIMING_PART2: c_uint = 0x1c;
// Returns a struct intel_sdvo_pixel_clock_range
pub const SDVO_CMD_GET_INPUT_PIXEL_CLOCK_RANGE: c_uint = 0x1d;
// Returns a struct intel_sdvo_pixel_clock_range
pub const SDVO_CMD_GET_OUTPUT_PIXEL_CLOCK_RANGE: c_uint = 0x1e;
// Returns a byte bitfield containing SDVO_CLOCK_RATE_MULT_* flags
pub const SDVO_CMD_GET_SUPPORTED_CLOCK_RATE_MULTS: c_uint = 0x1f;
// Returns a byte containing a SDVO_CLOCK_RATE_MULT_* flag
pub const SDVO_CMD_GET_CLOCK_RATE_MULT: c_uint = 0x20;
// Takes a byte containing a SDVO_CLOCK_RATE_MULT_* flag
pub const SDVO_CMD_SET_CLOCK_RATE_MULT: c_uint = 0x21;

pub const SDVO_CMD_GET_SUPPORTED_TV_FORMATS: c_uint = 0x27;
// 6 bytes of bit flags for TV formats shared by all TV format functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_tv_format {
    pub ntsc_m:1: c_uint,
    pub ntsc_j:1: c_uint,
    pub ntsc_443:1: c_uint,
    pub pal_b:1: c_uint,
    pub pal_d:1: c_uint,
    pub pal_g:1: c_uint,
    pub pal_h:1: c_uint,
    pub pal_i:1: c_uint,
    pub pal_m:1: c_uint,
    pub pal_n:1: c_uint,
    pub pal_nc:1: c_uint,
    pub pal_60:1: c_uint,
    pub secam_b:1: c_uint,
    pub secam_d:1: c_uint,
    pub secam_g:1: c_uint,
    pub secam_k:1: c_uint,
    pub secam_k1:1: c_uint,
    pub secam_l:1: c_uint,
    pub secam_60:1: c_uint,
    pub hdtv_std_smpte_240m_1080i_59:1: c_uint,
    pub hdtv_std_smpte_240m_1080i_60:1: c_uint,
    pub hdtv_std_smpte_260m_1080i_59:1: c_uint,
    pub hdtv_std_smpte_260m_1080i_60:1: c_uint,
    pub hdtv_std_smpte_274m_1080i_50:1: c_uint,
    pub hdtv_std_smpte_274m_1080i_59:1: c_uint,
    pub hdtv_std_smpte_274m_1080i_60:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_23:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_24:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_25:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_29:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_30:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_50:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_59:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_60:1: c_uint,
    pub hdtv_std_smpte_295m_1080i_50:1: c_uint,
    pub hdtv_std_smpte_295m_1080p_50:1: c_uint,
    pub hdtv_std_smpte_296m_720p_59:1: c_uint,
    pub hdtv_std_smpte_296m_720p_60:1: c_uint,
    pub hdtv_std_smpte_296m_720p_50:1: c_uint,
    pub hdtv_std_smpte_293m_480p_59:1: c_uint,
    pub hdtv_std_smpte_170m_480i_59:1: c_uint,
    pub hdtv_std_iturbt601_576i_50:1: c_uint,
    pub hdtv_std_iturbt601_576p_50:1: c_uint,
    pub hdtv_std_eia_7702a_480i_60:1: c_uint,
    pub hdtv_std_eia_7702a_480p_60:1: c_uint,
    pub pad:3: c_uint,
    pub __packed: },
pub const SDVO_CMD_GET_TV_FORMAT: c_uint = 0x28;
pub const SDVO_CMD_SET_TV_FORMAT: c_uint = 0x29;
// Returns the resolutiosn that can be used with the given TV format
pub const SDVO_CMD_GET_SDTV_RESOLUTION_SUPPORT: c_uint = 0x83;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_sdtv_resolution_request {
    pub ntsc_m:1: c_uint,
    pub ntsc_j:1: c_uint,
    pub ntsc_443:1: c_uint,
    pub pal_b:1: c_uint,
    pub pal_d:1: c_uint,
    pub pal_g:1: c_uint,
    pub pal_h:1: c_uint,
    pub pal_i:1: c_uint,
    pub pal_m:1: c_uint,
    pub pal_n:1: c_uint,
    pub pal_nc:1: c_uint,
    pub pal_60:1: c_uint,
    pub secam_b:1: c_uint,
    pub secam_d:1: c_uint,
    pub secam_g:1: c_uint,
    pub secam_k:1: c_uint,
    pub secam_k1:1: c_uint,
    pub secam_l:1: c_uint,
    pub secam_60:1: c_uint,
    pub pad:5: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_sdtv_resolution_reply {
    pub res_320x200:1: c_uint,
    pub res_320x240:1: c_uint,
    pub res_400x300:1: c_uint,
    pub res_640x350:1: c_uint,
    pub res_640x400:1: c_uint,
    pub res_640x480:1: c_uint,
    pub res_704x480:1: c_uint,
    pub res_704x576:1: c_uint,
    pub res_720x350:1: c_uint,
    pub res_720x400:1: c_uint,
    pub res_720x480:1: c_uint,
    pub res_720x540:1: c_uint,
    pub res_720x576:1: c_uint,
    pub res_768x576:1: c_uint,
    pub res_800x600:1: c_uint,
    pub res_832x624:1: c_uint,
    pub res_920x766:1: c_uint,
    pub res_1024x768:1: c_uint,
    pub res_1280x1024:1: c_uint,
    pub pad:5: c_uint,
    pub __packed: },
// Get supported resolution with squire pixel aspect ratio that can be
pub const SDVO_CMD_GET_SCALED_HDTV_RESOLUTION_SUPPORT: c_uint = 0x85;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_hdtv_resolution_request {
    pub hdtv_std_smpte_240m_1080i_59:1: c_uint,
    pub hdtv_std_smpte_240m_1080i_60:1: c_uint,
    pub hdtv_std_smpte_260m_1080i_59:1: c_uint,
    pub hdtv_std_smpte_260m_1080i_60:1: c_uint,
    pub hdtv_std_smpte_274m_1080i_50:1: c_uint,
    pub hdtv_std_smpte_274m_1080i_59:1: c_uint,
    pub hdtv_std_smpte_274m_1080i_60:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_23:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_24:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_25:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_29:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_30:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_50:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_59:1: c_uint,
    pub hdtv_std_smpte_274m_1080p_60:1: c_uint,
    pub hdtv_std_smpte_295m_1080i_50:1: c_uint,
    pub hdtv_std_smpte_295m_1080p_50:1: c_uint,
    pub hdtv_std_smpte_296m_720p_59:1: c_uint,
    pub hdtv_std_smpte_296m_720p_60:1: c_uint,
    pub hdtv_std_smpte_296m_720p_50:1: c_uint,
    pub hdtv_std_smpte_293m_480p_59:1: c_uint,
    pub hdtv_std_smpte_170m_480i_59:1: c_uint,
    pub hdtv_std_iturbt601_576i_50:1: c_uint,
    pub hdtv_std_iturbt601_576p_50:1: c_uint,
    pub hdtv_std_eia_7702a_480i_60:1: c_uint,
    pub hdtv_std_eia_7702a_480p_60:1: c_uint,
    pub pad:6: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_hdtv_resolution_reply {
    pub res_640x480:1: c_uint,
    pub res_800x600:1: c_uint,
    pub res_1024x768:1: c_uint,
    pub res_1280x960:1: c_uint,
    pub res_1400x1050:1: c_uint,
    pub res_1600x1200:1: c_uint,
    pub res_1920x1440:1: c_uint,
    pub res_2048x1536:1: c_uint,
    pub res_2560x1920:1: c_uint,
    pub res_3200x2400:1: c_uint,
    pub res_3840x2880:1: c_uint,
    pub pad1:5: c_uint,
    pub res_848x480:1: c_uint,
    pub res_1064x600:1: c_uint,
    pub res_1280x720:1: c_uint,
    pub res_1360x768:1: c_uint,
    pub res_1704x960:1: c_uint,
    pub res_1864x1050:1: c_uint,
    pub res_1920x1080:1: c_uint,
    pub res_2128x1200:1: c_uint,
    pub res_2560x1400:1: c_uint,
    pub res_2728x1536:1: c_uint,
    pub res_3408x1920:1: c_uint,
    pub res_4264x2400:1: c_uint,
    pub res_5120x2880:1: c_uint,
    pub pad2:3: c_uint,
    pub res_768x480:1: c_uint,
    pub res_960x600:1: c_uint,
    pub res_1152x720:1: c_uint,
    pub res_1124x768:1: c_uint,
    pub res_1536x960:1: c_uint,
    pub res_1680x1050:1: c_uint,
    pub res_1728x1080:1: c_uint,
    pub res_1920x1200:1: c_uint,
    pub res_2304x1440:1: c_uint,
    pub res_2456x1536:1: c_uint,
    pub res_3072x1920:1: c_uint,
    pub res_3840x2400:1: c_uint,
    pub res_4608x2880:1: c_uint,
    pub pad3:3: c_uint,
    pub res_1280x1024:1: c_uint,
    pub pad4:7: c_uint,
    pub res_1280x768:1: c_uint,
    pub pad5:7: c_uint,
    pub __packed: },
// Get supported power state returns info for encoder and monitor, rely on
pub const SDVO_CMD_GET_SUPPORTED_POWER_STATES: c_uint = 0x2a;
// Get power state returns info for encoder and monitor, rely on last
pub const SDVO_CMD_GET_POWER_STATE: c_uint = 0x2b;
pub const SDVO_CMD_GET_ENCODER_POWER_STATE: c_uint = 0x2b;
pub const SDVO_CMD_SET_ENCODER_POWER_STATE: c_uint = 0x2c;

pub const SDVO_CMD_GET_MAX_PANEL_POWER_SEQUENCING: c_uint = 0x2d;
pub const SDVO_CMD_GET_PANEL_POWER_SEQUENCING: c_uint = 0x2e;
pub const SDVO_CMD_SET_PANEL_POWER_SEQUENCING: c_uint = 0x2f;
//
// The panel power sequencing parameters are in units of milliseconds.
// The high fields are bits 8:9 of the 10-bit values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_panel_power_sequencing {
    pub t0: u8,
    pub t1: u8,
    pub t2: u8,
    pub t3: u8,
    pub t4: u8,
    pub t0_high:2: c_uint,
    pub t1_high:2: c_uint,
    pub t2_high:2: c_uint,
    pub t3_high:2: c_uint,
    pub t4_high:2: c_uint,
    pub pad:6: c_uint,
    pub __packed: },
pub const SDVO_CMD_GET_MAX_BACKLIGHT_LEVEL: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_max_backlight_reply {
    pub max_value: u8,
    pub default_value: u8,
    pub __packed: },
pub const SDVO_CMD_GET_BACKLIGHT_LEVEL: c_uint = 0x31;
pub const SDVO_CMD_SET_BACKLIGHT_LEVEL: c_uint = 0x32;
pub const SDVO_CMD_GET_AMBIENT_LIGHT: c_uint = 0x33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_get_ambient_light_reply {
    pub trip_low: u16,
    pub trip_high: u16,
    pub value: u16,
    pub __packed: },
pub const SDVO_CMD_SET_AMBIENT_LIGHT: c_uint = 0x34;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdvo_set_ambient_light_reply {
    pub trip_low: u16,
    pub trip_high: u16,
    pub enable:1: c_uint,
    pub pad:7: c_uint,
    pub __packed: },
// Set display power state
pub const SDVO_CMD_SET_DISPLAY_POWER_STATE: c_uint = 0x7d;

pub const SDVO_CMD_GET_SUPPORTED_ENHANCEMENTS: c_uint = 0x84;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_enhancements_reply {
    pub flicker_filter:1: c_uint,
    pub flicker_filter_adaptive:1: c_uint,
    pub flicker_filter_2d:1: c_uint,
    pub saturation:1: c_uint,
    pub hue:1: c_uint,
    pub brightness:1: c_uint,
    pub contrast:1: c_uint,
    pub overscan_h:1: c_uint,
    pub overscan_v:1: c_uint,
    pub hpos:1: c_uint,
    pub vpos:1: c_uint,
    pub sharpness:1: c_uint,
    pub dot_crawl:1: c_uint,
    pub dither:1: c_uint,
    pub tv_chroma_filter:1: c_uint,
    pub tv_luma_filter:1: c_uint,
    pub __packed: },
// Picture enhancement limits below are dependent on the current TV format,
// and thus need to be queried and set after it.
//
pub const SDVO_CMD_GET_MAX_FLICKER_FILTER: c_uint = 0x4d;
pub const SDVO_CMD_GET_MAX_FLICKER_FILTER_ADAPTIVE: c_uint = 0x7b;
pub const SDVO_CMD_GET_MAX_FLICKER_FILTER_2D: c_uint = 0x52;
pub const SDVO_CMD_GET_MAX_SATURATION: c_uint = 0x55;
pub const SDVO_CMD_GET_MAX_HUE: c_uint = 0x58;
pub const SDVO_CMD_GET_MAX_BRIGHTNESS: c_uint = 0x5b;
pub const SDVO_CMD_GET_MAX_CONTRAST: c_uint = 0x5e;
pub const SDVO_CMD_GET_MAX_OVERSCAN_H: c_uint = 0x61;
pub const SDVO_CMD_GET_MAX_OVERSCAN_V: c_uint = 0x64;
pub const SDVO_CMD_GET_MAX_HPOS: c_uint = 0x67;
pub const SDVO_CMD_GET_MAX_VPOS: c_uint = 0x6a;
pub const SDVO_CMD_GET_MAX_SHARPNESS: c_uint = 0x6d;
pub const SDVO_CMD_GET_MAX_TV_CHROMA_FILTER: c_uint = 0x74;
pub const SDVO_CMD_GET_MAX_TV_LUMA_FILTER: c_uint = 0x77;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_enhancement_limits_reply {
    pub max_value: u16,
    pub default_value: u16,
    pub __packed: },
pub const SDVO_CMD_GET_LVDS_PANEL_INFORMATION: c_uint = 0x7f;
pub const SDVO_CMD_SET_LVDS_PANEL_INFORMATION: c_uint = 0x80;

pub const SDVO_CMD_GET_FLICKER_FILTER: c_uint = 0x4e;
pub const SDVO_CMD_SET_FLICKER_FILTER: c_uint = 0x4f;
pub const SDVO_CMD_GET_FLICKER_FILTER_ADAPTIVE: c_uint = 0x50;
pub const SDVO_CMD_SET_FLICKER_FILTER_ADAPTIVE: c_uint = 0x51;
pub const SDVO_CMD_GET_FLICKER_FILTER_2D: c_uint = 0x53;
pub const SDVO_CMD_SET_FLICKER_FILTER_2D: c_uint = 0x54;
pub const SDVO_CMD_GET_SATURATION: c_uint = 0x56;
pub const SDVO_CMD_SET_SATURATION: c_uint = 0x57;
pub const SDVO_CMD_GET_HUE: c_uint = 0x59;
pub const SDVO_CMD_SET_HUE: c_uint = 0x5a;
pub const SDVO_CMD_GET_BRIGHTNESS: c_uint = 0x5c;
pub const SDVO_CMD_SET_BRIGHTNESS: c_uint = 0x5d;
pub const SDVO_CMD_GET_CONTRAST: c_uint = 0x5f;
pub const SDVO_CMD_SET_CONTRAST: c_uint = 0x60;
pub const SDVO_CMD_GET_OVERSCAN_H: c_uint = 0x62;
pub const SDVO_CMD_SET_OVERSCAN_H: c_uint = 0x63;
pub const SDVO_CMD_GET_OVERSCAN_V: c_uint = 0x65;
pub const SDVO_CMD_SET_OVERSCAN_V: c_uint = 0x66;
pub const SDVO_CMD_GET_HPOS: c_uint = 0x68;
pub const SDVO_CMD_SET_HPOS: c_uint = 0x69;
pub const SDVO_CMD_GET_VPOS: c_uint = 0x6b;
pub const SDVO_CMD_SET_VPOS: c_uint = 0x6c;
pub const SDVO_CMD_GET_SHARPNESS: c_uint = 0x6e;
pub const SDVO_CMD_SET_SHARPNESS: c_uint = 0x6f;
pub const SDVO_CMD_GET_TV_CHROMA_FILTER: c_uint = 0x75;
pub const SDVO_CMD_SET_TV_CHROMA_FILTER: c_uint = 0x76;
pub const SDVO_CMD_GET_TV_LUMA_FILTER: c_uint = 0x78;
pub const SDVO_CMD_SET_TV_LUMA_FILTER: c_uint = 0x79;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_enhancements_arg {
    pub value: u16,
    pub __packed: },
pub const SDVO_CMD_GET_DOT_CRAWL: c_uint = 0x70;
pub const SDVO_CMD_SET_DOT_CRAWL: c_uint = 0x71;

pub const SDVO_CMD_GET_DITHER: c_uint = 0x72;
pub const SDVO_CMD_SET_DITHER: c_uint = 0x73;

pub const SDVO_CMD_SET_CONTROL_BUS_SWITCH: c_uint = 0x7a;

// HDMI op codes
pub const SDVO_CMD_GET_SUPP_ENCODE: c_uint = 0x9d;
pub const SDVO_CMD_GET_ENCODE: c_uint = 0x9e;
pub const SDVO_CMD_SET_ENCODE: c_uint = 0x9f;
pub const SDVO_ENCODE_DVI: c_uint = 0x0;
pub const SDVO_ENCODE_HDMI: c_uint = 0x1;
pub const SDVO_CMD_SET_PIXEL_REPLI: c_uint = 0x8b;
pub const SDVO_CMD_GET_PIXEL_REPLI: c_uint = 0x8c;
pub const SDVO_CMD_GET_COLORIMETRY_CAP: c_uint = 0x8d;
pub const SDVO_CMD_SET_COLORIMETRY: c_uint = 0x8e;

pub const SDVO_CMD_GET_COLORIMETRY: c_uint = 0x8f;
pub const SDVO_CMD_GET_AUDIO_ENCRYPT_PREFER: c_uint = 0x90;
pub const SDVO_CMD_SET_AUDIO_STAT: c_uint = 0x91;
pub const SDVO_CMD_GET_AUDIO_STAT: c_uint = 0x92;

pub const SDVO_CMD_SET_HBUF_INDEX: c_uint = 0x93;
pub const SDVO_HBUF_INDEX_ELD: c_int = 0;
pub const SDVO_HBUF_INDEX_AVI_IF: c_int = 1;
pub const SDVO_CMD_GET_HBUF_INDEX: c_uint = 0x94;
pub const SDVO_CMD_GET_HBUF_INFO: c_uint = 0x95;
pub const SDVO_CMD_SET_HBUF_AV_SPLIT: c_uint = 0x96;
pub const SDVO_CMD_GET_HBUF_AV_SPLIT: c_uint = 0x97;
pub const SDVO_CMD_SET_HBUF_DATA: c_uint = 0x98;
pub const SDVO_CMD_GET_HBUF_DATA: c_uint = 0x99;
pub const SDVO_CMD_SET_HBUF_TXRATE: c_uint = 0x9a;
pub const SDVO_CMD_GET_HBUF_TXRATE: c_uint = 0x9b;

pub const SDVO_CMD_GET_AUDIO_TX_INFO: c_uint = 0x9c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_sdvo_encode {
    pub dvi_rev: u8,
    pub hdmi_rev: u8,
    pub __packed: },
