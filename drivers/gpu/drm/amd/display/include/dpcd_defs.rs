//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/dpcd_defs.h
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

pub const DP_SINK_HW_REVISION_START: c_uint = 0x409;

// Panel Replay

pub const DP_PANEL_REPLAY_CAPABILITY_SUPPORT: c_uint = 0x0b0;

pub const DP_PANEL_REPLAY_CAPABILITY: c_uint = 0x0b1;

pub const DP_PR_SU_X_GRANULARITY_LOW: c_uint = 0x0b2;

pub const DP_PR_SU_X_GRANULARITY_HIGH: c_uint = 0x0b3;

pub const DP_PR_SU_Y_GRANULARITY: c_uint = 0x0b4;

pub const DP_PR_SU_Y_GRANULARITY_EXTENDED_CAP_LOW: c_uint = 0x0b5;

pub const DP_PR_SU_Y_GRANULARITY_EXTENDED_CAP_HIGH: c_uint = 0x0b6;

pub const DP_PANEL_REPLAY_ENABLE_AND_CONFIGURATION_1: c_uint = 0x1b0;

pub const DP_PANEL_REPLAY_ENABLE_AND_CONFIGURATION_2: c_uint = 0x1b1;

pub const DP_PR_ERROR_STATUS: c_uint = 0x2020  /* DP 2.0 */;

pub const DP_PR_REPLAY_SINK_STATUS: c_uint = 0x2022;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_revision {
    DPCD_REV_10 = 0x10,
    DPCD_REV_11 = 0x11,
    DPCD_REV_12 = 0x12,
    DPCD_REV_13 = 0x13,
    DPCD_REV_14 = 0x14
}

// these are the types stored at DOWNSTREAMPORT_PRESENT
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_downstream_port_type {
    DOWNSTREAM_DP = 0,
    DOWNSTREAM_VGA,
    DOWNSTREAM_DVI_HDMI_DP_PLUS_PLUS,/* DVI, HDMI, DP++ */
    DOWNSTREAM_NONDDC /* has no EDID (TV,CV) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_link_test_patterns {
    LINK_TEST_PATTERN_NONE = 0,
    LINK_TEST_PATTERN_COLOR_RAMP,
    LINK_TEST_PATTERN_VERTICAL_BARS,
    LINK_TEST_PATTERN_COLOR_SQUARES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_test_color_format {
    TEST_COLOR_FORMAT_RGB = 0,
    TEST_COLOR_FORMAT_YCBCR422,
    TEST_COLOR_FORMAT_YCBCR444
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_test_bit_depth {
    TEST_BIT_DEPTH_6 = 0,
    TEST_BIT_DEPTH_8,
    TEST_BIT_DEPTH_10,
    TEST_BIT_DEPTH_12,
    TEST_BIT_DEPTH_16
}

// PHY (encoder) test patterns
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_phy_test_patterns {
    PHY_TEST_PATTERN_NONE = 0,
    PHY_TEST_PATTERN_D10_2,
    PHY_TEST_PATTERN_SYMBOL_ERROR,
    PHY_TEST_PATTERN_PRBS7,
    PHY_TEST_PATTERN_80BIT_CUSTOM,/* For DP1.2 only */
    PHY_TEST_PATTERN_CP2520_1,
    PHY_TEST_PATTERN_CP2520_2,
    PHY_TEST_PATTERN_CP2520_3, /* same as TPS4 */
    PHY_TEST_PATTERN_128b_132b_TPS1 = 0x8,
    PHY_TEST_PATTERN_128b_132b_TPS2 = 0x10,
    PHY_TEST_PATTERN_PRBS9 = 0x18,
    PHY_TEST_PATTERN_PRBS11 = 0x20,
    PHY_TEST_PATTERN_PRBS15 = 0x28,
    PHY_TEST_PATTERN_PRBS23 = 0x30,
    PHY_TEST_PATTERN_PRBS31 = 0x38,
    PHY_TEST_PATTERN_264BIT_CUSTOM = 0x40,
    PHY_TEST_PATTERN_SQUARE = 0x48,
    PHY_TEST_PATTERN_SQUARE_PRESHOOT_DISABLED = 0x49,
    PHY_TEST_PATTERN_SQUARE_DEEMPHASIS_DISABLED = 0x4A,
    PHY_TEST_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED = 0x4B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_test_dyn_range {
    TEST_DYN_RANGE_VESA = 0,
    TEST_DYN_RANGE_CEA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_audio_test_pattern {
    AUDIO_TEST_PATTERN_OPERATOR_DEFINED = 0,/* direct HW translation */
    AUDIO_TEST_PATTERN_SAWTOOTH
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_audio_sampling_rate {
    AUDIO_SAMPLING_RATE_32KHZ = 0,/* direct HW translation */
    AUDIO_SAMPLING_RATE_44_1KHZ,
    AUDIO_SAMPLING_RATE_48KHZ,
    AUDIO_SAMPLING_RATE_88_2KHZ,
    AUDIO_SAMPLING_RATE_96KHZ,
    AUDIO_SAMPLING_RATE_176_4KHZ,
    AUDIO_SAMPLING_RATE_192KHZ
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_audio_channels {
    AUDIO_CHANNELS_1 = 0,/* direct HW translation */
    AUDIO_CHANNELS_2,
    AUDIO_CHANNELS_3,
    AUDIO_CHANNELS_4,
    AUDIO_CHANNELS_5,
    AUDIO_CHANNELS_6,
    AUDIO_CHANNELS_7,
    AUDIO_CHANNELS_8,

    AUDIO_CHANNELS_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_audio_test_pattern_periods {
    DPCD_AUDIO_TEST_PATTERN_PERIOD_NOTUSED = 0,/* direct HW translation */
    DPCD_AUDIO_TEST_PATTERN_PERIOD_3,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_6,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_12,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_24,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_48,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_96,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_192,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_384,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_768,
    DPCD_AUDIO_TEST_PATTERN_PERIOD_1536
}

// This enum is for programming DPCD TRAINING_PATTERN_SET
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_training_patterns {
    DPCD_TRAINING_PATTERN_VIDEOIDLE = 0,/* direct HW translation! */
    DPCD_TRAINING_PATTERN_1,
    DPCD_TRAINING_PATTERN_2,
    DPCD_TRAINING_PATTERN_3,
    DPCD_TRAINING_PATTERN_4 = 7,
    DPCD_128b_132b_TPS1 = 1,
    DPCD_128b_132b_TPS2 = 2,
    DPCD_128b_132b_TPS2_CDS = 3,
}

// This enum is for use with PsrSinkPsrStatus.bits.sinkSelfRefreshStatus
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_psr_sink_states {
    PSR_SINK_STATE_INACTIVE = 0,
    PSR_SINK_STATE_ACTIVE_CAPTURE_DISPLAY_ON_SOURCE_TIMING = 1,
    PSR_SINK_STATE_ACTIVE_DISPLAY_FROM_SINK_RFB = 2,
    PSR_SINK_STATE_ACTIVE_CAPTURE_DISPLAY_ON_SINK_TIMING = 3,
    PSR_SINK_STATE_ACTIVE_CAPTURE_TIMING_RESYNC = 4,
    PSR_SINK_STATE_SINK_INTERNAL_ERROR = 7,
}

pub const DP_SOURCE_SEQUENCE: c_uint = 0x30C;
pub const DP_SOURCE_TABLE_REVISION: c_uint = 0x310;
pub const DP_SOURCE_PAYLOAD_SIZE: c_uint = 0x311;
pub const DP_SOURCE_SINK_CAP: c_uint = 0x317;
pub const DP_SOURCE_BACKLIGHT_LEVEL: c_uint = 0x320;
pub const DP_SOURCE_BACKLIGHT_CURRENT_PEAK: c_uint = 0x326;
pub const DP_SOURCE_BACKLIGHT_CONTROL: c_uint = 0x32E;
pub const DP_SOURCE_BACKLIGHT_ENABLE: c_uint = 0x32F;
pub const DP_SINK_DRR_GRANULARITY: c_uint = 0x33B;
pub const DP_SOURCE_MINIMUM_HBLANK_SUPPORTED: c_uint = 0x340;
pub const DP_SINK_PR_REPLAY_STATUS: c_uint = 0x378;
pub const DP_SINK_PR_PIXEL_DEVIATION_PER_LINE: c_uint = 0x379;
pub const DP_SINK_PR_MAX_NUMBER_OF_DEVIATION_LINE: c_uint = 0x37A;
pub const DP_SINK_EMISSION_RATE: c_uint = 0x37E;
pub const DP_SINK_PR_FRAME_SKIP_COUNT: c_uint = 0x337;
// Remove once drm_dp_helper.h is updated upstream

pub const DP_TOTAL_LTTPR_CNT: c_uint = 0xF000A /* 2.1 */;

