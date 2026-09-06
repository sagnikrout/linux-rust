//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/link_service_types.h
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

// struct mst_mgr_callback_object;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_power_state {
    DP_POWER_STATE_D0 = 1,
    DP_POWER_STATE_D3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum edp_revision {
// eDP version 1.1 or lower
    EDP_REVISION_11 = 0x00,
// eDP version 1.2
    EDP_REVISION_12 = 0x01,
// eDP version 1.3
    EDP_REVISION_13 = 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lttpr_mode {
    LTTPR_MODE_UNKNOWN,
    LTTPR_MODE_NON_LTTPR,
    LTTPR_MODE_TRANSPARENT,
    LTTPR_MODE_NON_TRANSPARENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_training_settings {
    pub link_settings: dc_link_settings,
// TODO: turn lane settings below into mandatory fields
// as initial lane configuration
//
    pub voltage_swing: *mut dc_voltage_swing,
    pub pre_emphasis: *mut dc_pre_emphasis,
    pub post_cursor2: *mut dc_post_cursor2,
    pub should_set_fec_ready: bool,
    pub ffe_preset: *mut dc_dp_ffe_preset,
    pub cr_pattern_time: u16,
    pub eq_pattern_time: u16,
    pub cds_pattern_time: u16,
    pub pattern_for_cr: dc_dp_training_pattern,
    pub pattern_for_eq: dc_dp_training_pattern,
    pub pattern_for_cds: dc_dp_training_pattern,
    pub eq_wait_time_limit: u32,
    pub eq_loop_count_limit: u8,
    pub cds_wait_time_limit: u32,
    pub enhanced_framing: bool,
    pub lttpr_mode: lttpr_mode,
    pub lttpr_early_tps2: bool,
// disallow different lanes to have different lane settings
    pub disallow_per_lane_settings: bool,
// dpcd lane settings will always use the same hw lane settings
// even if it doesn't match requested lane adjust
    pub always_match_dpcd_with_hw_lane_settings: bool,
//
// training states - parameters that can change in link training
//
// TODO: Move hw_lane_settings and dpcd_lane_settings
// along with lane adjust, lane align, offset and all
// other training states into a new structure called
// training states, so link_training_settings becomes
// a constant input pre-decided prior to link training.
//
// The goal is to strictly decouple link training settings
// decision making process from link training states to
// prevent it from messy code practice of changing training
// decision on the fly.
//
    pub hw_lane_settings: [dc_lane_settings; LANE_COUNT_DP_MAX],
    pub dpcd_lane_settings: [dpcd_training_lane; LANE_COUNT_DP_MAX],
}

// TODO: Move this enum test harness
// Test patterns
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_test_pattern {
// Input data is pass through Scrambler
// and 8b10b Encoder straight to output
    DP_TEST_PATTERN_VIDEO_MODE = 0,

// phy test patterns
    DP_TEST_PATTERN_PHY_PATTERN_BEGIN,
    DP_TEST_PATTERN_D102 = DP_TEST_PATTERN_PHY_PATTERN_BEGIN,
    DP_TEST_PATTERN_SYMBOL_ERROR,
    DP_TEST_PATTERN_PRBS7,
    DP_TEST_PATTERN_80BIT_CUSTOM,
    DP_TEST_PATTERN_CP2520_1,
    DP_TEST_PATTERN_CP2520_2,
    DP_TEST_PATTERN_HBR2_COMPLIANCE_EYE = DP_TEST_PATTERN_CP2520_2,
    DP_TEST_PATTERN_CP2520_3,
    DP_TEST_PATTERN_128b_132b_TPS1,
    DP_TEST_PATTERN_128b_132b_TPS2,
    DP_TEST_PATTERN_PRBS9,
    DP_TEST_PATTERN_PRBS11,
    DP_TEST_PATTERN_PRBS15,
    DP_TEST_PATTERN_PRBS23,
    DP_TEST_PATTERN_PRBS31,
    DP_TEST_PATTERN_264BIT_CUSTOM,
    DP_TEST_PATTERN_SQUARE_BEGIN,
    DP_TEST_PATTERN_SQUARE = DP_TEST_PATTERN_SQUARE_BEGIN,
    DP_TEST_PATTERN_SQUARE_PRESHOOT_DISABLED,
    DP_TEST_PATTERN_SQUARE_DEEMPHASIS_DISABLED,
    DP_TEST_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED,
    DP_TEST_PATTERN_SQUARE_END = DP_TEST_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED,

// Link Training Patterns
    DP_TEST_PATTERN_TRAINING_PATTERN1,
    DP_TEST_PATTERN_TRAINING_PATTERN2,
    DP_TEST_PATTERN_TRAINING_PATTERN3,
    DP_TEST_PATTERN_TRAINING_PATTERN4,
    DP_TEST_PATTERN_128b_132b_TPS1_TRAINING_MODE,
    DP_TEST_PATTERN_128b_132b_TPS2_TRAINING_MODE,
    DP_TEST_PATTERN_PHY_PATTERN_END = DP_TEST_PATTERN_128b_132b_TPS2_TRAINING_MODE,

// link test patterns
    DP_TEST_PATTERN_COLOR_SQUARES,
    DP_TEST_PATTERN_COLOR_SQUARES_CEA,
    DP_TEST_PATTERN_VERTICAL_BARS,
    DP_TEST_PATTERN_HORIZONTAL_BARS,
    DP_TEST_PATTERN_COLOR_RAMP,

// audio test patterns
    DP_TEST_PATTERN_AUDIO_OPERATOR_DEFINED,
    DP_TEST_PATTERN_AUDIO_SAWTOOTH,

    DP_TEST_PATTERN_UNSUPPORTED
}

// Macro flag: #define IS_DP_PHY_SQUARE_PATTERN(test_pattern)\
// Macro flag: #define IS_DP_PHY_PATTERN(test_pattern)\
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_test_pattern_color_space {
    DP_TEST_PATTERN_COLOR_SPACE_RGB,
    DP_TEST_PATTERN_COLOR_SPACE_YCBCR601,
    DP_TEST_PATTERN_COLOR_SPACE_YCBCR709,
    DP_TEST_PATTERN_COLOR_SPACE_UNDEFINED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_panel_mode {
// not required
    DP_PANEL_MODE_DEFAULT,
// standard mode for eDP
    DP_PANEL_MODE_EDP,
// external chips specific settings
    DP_PANEL_MODE_SPECIAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_source_sequence {
    DPCD_SOURCE_SEQ_AFTER_CONNECT_DIG_FE_OTG = 1, /*done in apply_single_controller_ctx_to_hw */
    DPCD_SOURCE_SEQ_AFTER_DP_STREAM_ATTR,         /*done in core_link_enable_stream */
    DPCD_SOURCE_SEQ_AFTER_UPDATE_INFO_FRAME,      /*done in core_link_enable_stream/dcn20_enable_stream */
    DPCD_SOURCE_SEQ_AFTER_CONNECT_DIG_FE_BE,      /*done in perform_link_training_with_retries/dcn20_enable_stream */
    DPCD_SOURCE_SEQ_AFTER_ENABLE_LINK_PHY,        /*done in dp_enable_link_phy */
    DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN,     /*done in dp_set_hw_test_pattern */
    DPCD_SOURCE_SEQ_AFTER_ENABLE_AUDIO_STREAM,    /*done in dce110_enable_audio_stream */
    DPCD_SOURCE_SEQ_AFTER_ENABLE_DP_VID_STREAM,   /*done in enc1_stream_encoder_dp_unblank */
    DPCD_SOURCE_SEQ_AFTER_DISABLE_DP_VID_STREAM,  /*done in enc1_stream_encoder_dp_blank */
    DPCD_SOURCE_SEQ_AFTER_FIFO_STEER_RESET,       /*done in enc1_stream_encoder_dp_blank */
    DPCD_SOURCE_SEQ_AFTER_DISABLE_AUDIO_STREAM,   /*done in dce110_disable_audio_stream */
    DPCD_SOURCE_SEQ_AFTER_DISABLE_LINK_PHY,       /*done in dp_disable_link_phy */
    DPCD_SOURCE_SEQ_AFTER_DISCONNECT_DIG_FE_BE,   /*done in dce110_disable_stream */
}

// DPCD_ADDR_TRAINING_LANEx_SET registers value
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_training_lane_set {

    pub VOLTAGE_SWING_SET:2: u8,
    pub MAX_SWING_REACHED:1: u8,
    pub PRE_EMPHASIS_SET:2: u8,
    pub MAX_PRE_EMPHASIS_REACHED:1: u8,
// following is reserved in DP 1.1
    pub POST_CURSOR2_SET:2: u8,

    pub POST_CURSOR2_SET:2: u8,
    pub MAX_PRE_EMPHASIS_REACHED:1: u8,
    pub PRE_EMPHASIS_SET:2: u8,
    pub MAX_SWING_REACHED:1: u8,
    pub VOLTAGE_SWING_SET:2: u8,

    pub bits: },
    pub raw: u8,
}

// AMD's copy of various payload data for MST. We have two copies of the payload table (one in DRM,
// one in DC) since DRM's MST helpers can't be accessed here. This stream allocation table should
// _ONLY_ be filled out from DM and then passed to DC, do NOT use these for _any_ kind of atomic
// state calculations in DM, or you will break something.
//
// DP MST stream allocation (payload bandwidth number)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dp_mst_stream_allocation {
    pub vcp_id: u8,
// number of slots required for the DP stream in
// transport packet
    pub slot_count: u8,
}

// DP MST stream allocation table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dp_mst_stream_allocation_table {
// number of DP video streams
    pub stream_count: c_int,
// array of stream allocations
    pub stream_allocations: [dc_dp_mst_stream_allocation; MAX_CONTROLLER_NUM],
}
