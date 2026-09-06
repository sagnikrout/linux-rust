//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dmub/inc/dmub_cmd.h
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
// Note: This header is the authoritative source for the DMUB firmware
// command interface. In the Linux kernel, it is maintained as an
// auto-generated copy from an AMD internal repository.
//
// This file follows Doxygen formatting for firmware standards and
// is NOT intended to be kernel-doc compliant.
//
// Copyright 2019 Advanced Micro Devices, Inc.
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

// <DMUB_TYPES>==================================================================
// Basic type definitions.

//
// Flag from driver to indicate that ABM should be disabled gradually
// by slowly reversing all backlight programming and pixel compensation.
//
pub const SET_ABM_PIPE_GRADUALLY_DISABLE: c_int = 0;
//
// Flag from driver to indicate that ABM should be disabled immediately
// and undo all backlight programming and pixel compensation.
//
pub const SET_ABM_PIPE_IMMEDIATELY_DISABLE: c_int = 255;
//
// Flag from driver to indicate that ABM should be disabled immediately
// and keep the current backlight programming and pixel compensation.
//
pub const SET_ABM_PIPE_IMMEDIATE_KEEP_GAIN_DISABLE: c_int = 254;
//
// Flag from driver to set the current ABM pipe index or ABM operating level.
//
pub const SET_ABM_PIPE_NORMAL: c_int = 1;
//
// Number of ambient light levels in ABM algorithm.
//
pub const NUM_AMBI_LEVEL: c_int = 5;
//
// Number of operating/aggression levels in ABM algorithm.
//
pub const NUM_AGGR_LEVEL: c_int = 4;
//
// Number of segments in the gamma curve.
//
pub const NUM_POWER_FN_SEGS: c_int = 8;
//
// Number of segments in the backlight curve.
//
pub const NUM_BL_CURVE_SEGS: c_int = 16;
//
// Maximum number of segments in ABM ACE curve.
//
pub const ABM_MAX_NUM_OF_ACE_SEGMENTS: c_int = 64;
//
// Maximum number of bins in ABM histogram.
//
pub const ABM_MAX_NUM_OF_HG_BINS: c_int = 64;
//
// Flag from driver to indicate that CACP should be disabled gradually
// by slowly reversing all programming and pixel compensation.
//
pub const SET_CACP_PIPE_GRADUALLY_DISABLE: c_int = 0;
//
// Flag from driver to indicate that CACP should be disabled immediately
// and undo all programming and pixel compensation.
//
pub const SET_CACP_PIPE_IMMEDIATELY_DISABLE: c_int = 255;
//
// Flag from driver to indicate that CACP should be disabled immediately
// and keep the current programming and pixel compensation.
//
pub const SET_CACP_PIPE_IMMEDIATE_KEEP_GAIN_DISABLE: c_int = 254;
//
// Flag from driver to indicate that CACP should be disabled immediately
// on the next abm vertical interrupt
//
pub const SET_CACP_PIPE_IMMEDIATE_ON_NEXT_DISABLE: c_int = 253;
//
// Flag from driver to set the current CACP pipe index or CACP operating level.
//
pub const SET_CACP_PIPE_NORMAL: c_int = 1;
// Maximum number of SubVP streams
pub const DMUB_MAX_SUBVP_STREAMS: c_int = 2;
// Define max FPO streams as 4 for now. Current implementation today
// only supports 1, but could be more in the future. Reduce array
// size to ensure the command size remains less than 64 bytes if
// adding new fields.
//
pub const DMUB_MAX_FPO_STREAMS: c_int = 4;
// Define to ensure that the "common" members always appear in the same
// order in different structs for back compat purposes
//

// Maximum number of streams on any ASIC.
pub const DMUB_MAX_STREAMS: c_int = 6;
// Maximum number of planes on any ASIC.
pub const DMUB_MAX_PLANES: c_int = 6;
// Maximum number of phantom planes on any ASIC

// Trace buffer offset for entry
pub const TRACE_BUFFER_ENTRY_OFFSET: c_int = 16;
//
// Maximum number of dirty rects supported by FW.
//
pub const DMUB_MAX_DIRTY_RECTS: c_int = 3;
//
// PSR control version legacy
//
pub const DMUB_CMD_PSR_CONTROL_VERSION_UNKNOWN: c_uint = 0x0;
//
// PSR control version with multi edp support
//
pub const DMUB_CMD_PSR_CONTROL_VERSION_1: c_uint = 0x1;
//
// dirty rect cmd version legacy
//
pub const DMUB_CMD_DIRTY_RECTS_VERSION_UNKNOWN: c_uint = 0x0;
//
// dirty rect cmd version with multi edp support
//
pub const DMUB_CMD_DIRTY_RECTS_VERSION_1: c_uint = 0x1;
//
// dirty rect cmd version with external monitor support
//
pub const DMUB_CMD_DIRTY_RECTS_VERSION_2: c_uint = 0x2;
//
// Cursor update cmd version legacy
//
pub const DMUB_CMD_CURSOR_UPDATE_VERSION_UNKNOWN: c_uint = 0x0;
//
// Cursor update cmd version with multi edp support
//
pub const DMUB_CMD_CURSOR_UPDATE_VERSION_1: c_uint = 0x1;
//
// Cursor update cmd version with external monitor support
//
pub const DMUB_CMD_CURSOR_UPDATE_VERSION_2: c_uint = 0x2;
//
// ABM control version legacy
//
pub const DMUB_CMD_ABM_CONTROL_VERSION_UNKNOWN: c_uint = 0x0;
//
// ABM control version with multi edp support
//
pub const DMUB_CMD_ABM_CONTROL_VERSION_1: c_uint = 0x1;
//
// CACP control version legacy
//
pub const DMUB_CMD_CACP_CONTROL_VERSION_UNKNOWN: c_uint = 0x0;
//
// CACP control version with multi edp support
//
pub const DMUB_CMD_CACP_CONTROL_VERSION_1: c_uint = 0x1;
//
// CACP control mode with dynamic switch between v1 and v2
//
pub const DMUB_CMD_CACP_CONTROL_MODE_0: c_uint = 0x0;
//
// CACP control version with forced configure to v1 on new ABM HW
//
pub const DMUB_CMD_CACP_CONTROL_MODE_1: c_uint = 0x1;
//
// Physical framebuffer address location, 64-bit.
//

//
// OS/FW agnostic memcpy
//

//
// OS/FW agnostic memset
//

//
// OS/FW agnostic memcmp
//

//
// OS/FW agnostic udelay
//

pub const ABM_NUM_OF_ACE_SEGMENTS: c_int = 5;
//
// Debug FW state offset
//
pub const DMUB_DEBUG_FW_STATE_OFFSET: c_uint = 0x300;
#[repr(C)]
#[derive(Copy, Clone)]
pub union abm_flags {
//
// @abm_enabled: Indicates if ABM is enabled.
//
    pub 1: unsigned int abm_enabled :,
//
// @disable_abm_requested: Indicates if driver has requested ABM to be disabled.
//
    pub 1: unsigned int disable_abm_requested :,
//
// @disable_abm_immediately: Indicates if driver has requested ABM to be disabled immediately.
//
    pub 1: unsigned int disable_abm_immediately :,
//
// @disable_abm_immediate_keep_gain: Indicates if driver has requested ABM
// to be disabled immediately and keep gain.
//
    pub 1: unsigned int disable_abm_immediate_keep_gain :,
//
// @fractional_pwm: Indicates if fractional duty cycle for backlight PWM is enabled.
//
    pub 1: unsigned int fractional_pwm :,
//
// @abm_gradual_bl_change: Indicates if algorithm has completed gradual adjustment
// of user backlight level.
//
    pub 1: unsigned int abm_gradual_bl_change :,
//
// @abm_new_frame: Indicates if a new frame update needed for ABM to ramp up into steady
//
    pub 1: unsigned int abm_new_frame :,
//
// @vb_scaling_enabled: Indicates variBright Scaling Enable
//
    pub 1: unsigned int vb_scaling_enabled :,
    pub bitfields: },
    pub u32All: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abm_save_restore {
//
// @flags: Misc. ABM flags.
//
    pub flags: abm_flags,
//
// @pause: true:  pause ABM and get state
// false: unpause ABM after setting state
//
    pub pause: u32,
//
// @next_ace_slope: Next ACE slopes to be programmed in HW (u3.13)
//
    pub next_ace_slope: [u32; ABM_NUM_OF_ACE_SEGMENTS],
//
// @next_ace_thresh: Next ACE thresholds to be programmed in HW (u10.6)
//
    pub next_ace_thresh: [u32; ABM_NUM_OF_ACE_SEGMENTS],
//
// @next_ace_offset: Next ACE offsets to be programmed in HW (u10.6)
//
    pub next_ace_offset: [u32; ABM_NUM_OF_ACE_SEGMENTS],
//
// @knee_threshold: Current x-position of ACE knee (u0.16).
//
    pub knee_threshold: u32,
//
// @current_gain: Current backlight reduction (u16.16).
//
    pub current_gain: u32,
//
// @curr_bl_level: Current actual backlight level converging to target backlight level.
//
    pub curr_bl_level: u16,
//
// @curr_user_bl_level: Current nominal backlight level converging to level requested by user.
//
    pub curr_user_bl_level: u16,
}

//
// union dmub_addr - DMUB physical/virtual 64-bit address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_addr {
    pub /: *mut *mut *mut uint32_t low_part; /< Lower 32 bits,
    pub /: *mut *mut *mut uint32_t high_part; /< Upper 32 bits,
    pub /: *mut *mut } u; /<< Low/high bit access,
    pub /: *mut *mut uint64_t quad_part; /<< 64 bit address,
}

// Flattened structure containing SOC BB parameters stored in the VBIOS
// It is not practical to store the entire bounding box in VBIOS since the bounding box struct can gain new parameters.
// This also prevents alighment issues when new parameters are added to the SoC BB.
// The following parameters should be added since these values can't be obtained elsewhere:
// -dml2_soc_power_management_parameters
// -dml2_soc_vmin_clock_limits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_soc_bb_params {
    pub dram_clk_change_blackout_ns: u32,
    pub dram_clk_change_read_only_ns: u32,
    pub dram_clk_change_write_only_ns: u32,
    pub fclk_change_blackout_ns: u32,
    pub g7_ppt_blackout_ns: u32,
    pub stutter_enter_plus_exit_latency_ns: u32,
    pub stutter_exit_latency_ns: u32,
    pub z8_stutter_enter_plus_exit_latency_ns: u32,
    pub z8_stutter_exit_latency_ns: u32,
    pub z8_min_idle_time_ns: u32,
    pub type_b_dram_clk_change_blackout_ns: u32,
    pub type_b_ppt_blackout_ns: u32,
    pub vmin_limit_dispclk_khz: u32,
    pub vmin_limit_dcfclk_khz: u32,
    pub g7_temperature_read_blackout_ns: u32,
}

//
// Dirty rect definition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rect {
//
// Dirty rect x offset.
//
    pub x: u32,
//
// Dirty rect y offset.
//
    pub y: u32,
//
// Dirty rect width.
//
    pub width: u32,
//
// Dirty rect height.
//
    pub height: u32,
}

//
// Flags that can be set by driver to change some PSR behaviour.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_psr_debug_flags {
//
// Debug flags.
//
// Enable visual confirm in FW.
//
    pub 1: uint32_t visual_confirm :,
//
// Force all selective updates to bw full frame updates.
//
    pub 1: uint32_t force_full_frame_update :,
//
// Use HW Lock Mgr object to do HW locking in FW.
//
    pub 1: uint32_t use_hw_lock_mgr :,
//
// Use TPS3 signal when restore main link.
//
    pub 1: uint32_t force_wakeup_by_tps3 :,
//
// Back to back flip, therefore cannot power down PHY
//
    pub 1: uint32_t back_to_back_flip :,
//
// Enable visual confirm for IPS
//
    pub 1: uint32_t enable_ips_visual_confirm :,
    pub bitfields: },
//
// Union for debug flags.
//
    pub u32All: u32,
}

//
// Flags that can be set by driver to change some Replay behaviour.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_debug_flags {
//
// 0x1 (bit 0)
// Enable visual confirm in FW.
//
    pub 1: uint32_t visual_confirm :,
//
// 0x2 (bit 1)
// @skip_crc: Set if need to skip CRC.
//
    pub 1: uint32_t skip_crc :,
//
// 0x4 (bit 2)
// @force_link_power_on: Force disable ALPM control
//
    pub 1: uint32_t force_link_power_on :,
//
// 0x8 (bit 3)
// @force_phy_power_on: Force phy power on
//
    pub 1: uint32_t force_phy_power_on :,
//
// 0x10 (bit 4)
// @timing_resync_disabled: Disabled Replay normal sleep mode timing resync
//
    pub 1: uint32_t timing_resync_disabled :,
//
// 0x20 (bit 5)
// @skip_crtc_disabled: CRTC disable skipped
//
    pub 1: uint32_t skip_crtc_disabled :,
//
// 0x40 (bit 6)
// @force_defer_one_frame_update: Force defer one frame update in ultra sleep mode
//
    pub 1: uint32_t force_defer_one_frame_update :,
//
// 0x80 (bit 7)
// @disable_delay_alpm_on: Force disable delay alpm on
//
    pub 1: uint32_t disable_delay_alpm_on :,
//
// 0x100 (bit 8)
// @disable_desync_error_check: Force disable desync error check
//
    pub 1: uint32_t disable_desync_error_check :,
//
// 0x200 (bit 9)
// @force_self_update_when_abm_non_steady: Force self update if abm is not steady
//
    pub 1: uint32_t force_self_update_when_abm_non_steady :,
//
// 0x400 (bit 10)
// @enable_ips_visual_confirm: Enable IPS visual confirm when entering IPS
// If we enter IPS2, the Visual confirm bar will change to yellow
//
    pub 1: uint32_t enable_ips_visual_confirm :,
//
// 0x800 (bit 11)
// @enable_ips_residency_profiling: Enable IPS residency profiling
//
    pub 1: uint32_t enable_ips_residency_profiling :,
//
// 0x1000 (bit 12)
// @enable_coasting_vtotal_check: Enable Coasting_vtotal_check
//
    pub 1: uint32_t enable_coasting_vtotal_check :,
//
// 0x2000 (bit 13)
// @enable_visual_confirm_debug: Enable Visual Confirm Debug
//
    pub 1: uint32_t enable_visual_confirm_debug :,
//
// 0x4000 (bit 14)
// @debug_log_enabled: Debug Log Enabled
//
    pub 1: uint32_t debug_log_enabled :,
//
// 0x8000 (bit 15)
// @enable_sub_feature_visual_confirm: Enable Sub Feature Visual Confirm
//
    pub 1: uint32_t enable_sub_feature_visual_confirm :,
    pub 16: uint32_t reserved :,
    pub bitfields: },
    pub u32All: u32,
}

//
// Flags record error state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_visual_confirm_error_state_flags {
//
// 0x1 (bit 0) - Desync Error flag.
//
    pub 1: uint32_t desync_error :,
//
// 0x2 (bit 1) - State Transition Error flag.
//
    pub 1: uint32_t state_transition_error :,
//
// 0x4 (bit 2) - Crc Error flag
//
    pub 1: uint32_t crc_error :,
//
// 0x8 (bit 3) - Reserved
//
    pub 1: uint32_t reserved_3 :,
//
// 0x10 (bit 4) - Incorrect Coasting vtotal checking --> use debug flag to control DPCD write.
// Added new debug flag to control DPCD.
//
    pub 1: uint32_t incorrect_vtotal_in_static_screen :,
//
// 0x20 (bit 5) - No doubled Refresh Rate.
//
    pub 1: uint32_t no_double_rr :,
//
// Reserved bit 6-7
//
    pub 2: uint32_t reserved_6_7 :,
//
// Reserved bit 9-31
//
    pub 24: uint32_t reserved_9_31 :,
    pub bitfields: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_hw_flags {
//
// @allow_alpm_fw_standby_mode: To indicate whether the
// ALPM FW standby mode is allowed
//
    pub 1: uint32_t allow_alpm_fw_standby_mode :,
//
// @dsc_enable_status: DSC enable status in driver
//
    pub 1: uint32_t dsc_enable_status :,
//
// @fec_enable_status: receive fec enable/disable status from driver
//
    pub 1: uint32_t fec_enable_status :,
//
// @smu_optimizations_en: SMU power optimization.
// Only when active display is Replay capable and display enters Replay.
// Trigger interrupt to SMU to powerup/down.
//
    pub 1: uint32_t smu_optimizations_en :,
//
// @phy_power_state: Indicates current phy power state
//
    pub 1: uint32_t phy_power_state :,
//
// @link_power_state: Indicates current link power state
//
    pub 1: uint32_t link_power_state :,
//
// Use TPS3 signal when restore main link.
//
    pub 1: uint32_t force_wakeup_by_tps3 :,
//
// @is_alpm_initialized: Indicates whether ALPM is initialized
//
    pub 1: uint32_t is_alpm_initialized :,
//
// @alpm_mode: Indicates ALPM mode selected
//
    pub 2: uint32_t alpm_mode :,
    pub bitfields: },
    pub u32All: u32,
}

//
// Flags that can be set by driver to change some Panel Replay behaviour.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pr_debug_flags {
//
// 0x1 (bit 0)
// Enable visual confirm in FW.
//
    pub 1: uint32_t visual_confirm :,
//
// 0x2 (bit 1)
// @skip_crc: Set if need to skip CRC.
//
    pub 1: uint32_t skip_crc :,
//
// 0x4 (bit 2)
// @force_link_power_on: Force disable ALPM control
//
    pub 1: uint32_t force_link_power_on :,
//
// 0x8 (bit 3)
// @force_phy_power_on: Force phy power on
//
    pub 1: uint32_t force_phy_power_on :,
//
// 0x10 (bit 4)
// @visual_confirm_rate_control: Enable Visual Confirm rate control detection
//
    pub 1: uint32_t visual_confirm_rate_control :,
//
// 0x20 (bit 5)
// @force_full_frame_update: Force all selective updates to be full frame updates
//
    pub 1: uint32_t force_full_frame_update :,
//
// 0x40 (bit 6)
// @force_dpg_on: Force DPG on
//
    pub 1: uint32_t force_dpg_on :,
//
// 0x80 (bit 7)
// @force_hubp_on: Force Hubp on
//
    pub 1: uint32_t force_hubp_on :,
    pub 24: uint32_t reserved :,
    pub bitfields: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pr_hw_flags {
//
// @allow_alpm_fw_standby_mode: To indicate whether the
// ALPM FW standby mode is allowed
//
    pub 1: uint32_t allow_alpm_fw_standby_mode :,
//
// @dsc_enable_status: DSC enable status in driver
//
    pub 1: uint32_t dsc_enable_status :,
//
// @fec_enable_status: receive fec enable/disable status from driver
//
    pub 1: uint32_t fec_enable_status :,
//
// @smu_optimizations_en: SMU power optimization.
// Only when active display is Replay capable and display enters Replay.
// Trigger interrupt to SMU to powerup/down.
//
    pub 1: uint32_t smu_optimizations_en :,
//
// @link_power_state: Indicates current link power state
//
    pub 1: uint32_t link_power_state :,
//
// Use TPS3 signal when restore main link.
//
    pub 1: uint32_t force_wakeup_by_tps3 :,
//
// @is_alpm_initialized: Indicates whether ALPM is initialized
//
    pub 1: uint32_t is_alpm_initialized :,
//
// @alpm_mode: Indicates ALPM mode selected
//
    pub 2: uint32_t alpm_mode :,
    pub 23: uint32_t reserved :,
    pub bitfields: },
    pub u32All: u32,
}

//
// Definition of Panel Replay ML Activity Options
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pr_ml_activity_option {
    OPTION_DEFAULT	= 0x00, // VESA Option Default (1C)
    OPTION_1A		= 0x01, // VESA Option 1A
    OPTION_1B		= 0x02, // VESA Option 1B
    OPTION_1C		= 0x03, // VESA Option 1C
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_assisted_mclk_switch_version {
    pub 5: uint8_t minor :,
    pub 3: uint8_t major :,
}

//
// DMUB feature capabilities.
// After DMUB init, driver will query FW capabilities prior to enabling certain features.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_feature_caps {
//
// Max PSR version supported by FW.
//
    pub psr: u8,
    pub fw_assisted_mclk_switch_ver: u8,
    pub reserved: [u8; 4],
    pub subvp_psr_support: u8,
    pub gecc_enable: u8,
    pub replay_supported: u8,
    pub replay_reserved: [u8; 3],
    pub abm_aux_backlight_support: u8,
    pub lsdma_support_in_dmu: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_visual_confirm_color {
//
// Maximum 10 bits color value
//
    pub color_r_cr: u16,
    pub color_g_y: u16,
    pub color_b_cb: u16,
    pub panel_inst: u16,
}

//
// struct dmub_cursor_offload_pipe_data_dcn30_v1 - DCN30+ per pipe data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_pipe_data_dcn30_v1 {
    pub CURSOR0_0_CURSOR_SURFACE_ADDRESS: u32,
    pub CURSOR0_0_CURSOR_SURFACE_ADDRESS_HIGH: u32,
    pub 16: uint32_t CURSOR0_0_CURSOR_SIZE__CURSOR_WIDTH :,
    pub 16: uint32_t CURSOR0_0_CURSOR_SIZE__CURSOR_HEIGHT :,
    pub 16: uint32_t CURSOR0_0_CURSOR_POSITION__CURSOR_X_POSITION :,
    pub 16: uint32_t CURSOR0_0_CURSOR_POSITION__CURSOR_Y_POSITION :,
    pub 16: uint32_t CURSOR0_0_CURSOR_HOT_SPOT__CURSOR_HOT_SPOT_X :,
    pub 16: uint32_t CURSOR0_0_CURSOR_HOT_SPOT__CURSOR_HOT_SPOT_Y :,
    pub 13: uint32_t CURSOR0_0_CURSOR_DST_OFFSET__CURSOR_DST_X_OFFSET :,
    pub 1: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_ENABLE :,
    pub 3: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_MODE :,
    pub 1: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_2X_MAGNIFY :,
    pub 2: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_PITCH :,
    pub 5: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_LINES_PER_CHUNK :,
    pub reserved0: [u32; 4],
    pub 1: uint32_t CNVC_CUR0_CURSOR0_CONTROL__CUR0_ENABLE :,
    pub 3: uint32_t CNVC_CUR0_CURSOR0_CONTROL__CUR0_MODE :,
    pub 1: uint32_t CNVC_CUR0_CURSOR0_CONTROL__CUR0_EXPANSION_MODE :,
    pub 1: uint32_t CNVC_CUR0_CURSOR0_CONTROL__CUR0_ROM_EN :,
    pub 24: uint32_t CNVC_CUR0_CURSOR0_COLOR0__CUR0_COLOR0 :,
    pub 24: uint32_t CNVC_CUR0_CURSOR0_COLOR1__CUR0_COLOR1 :,
    pub 16: uint32_t CNVC_CUR0_CURSOR0_FP_SCALE_BIAS__CUR0_FP_BIAS :,
    pub 16: uint32_t CNVC_CUR0_CURSOR0_FP_SCALE_BIAS__CUR0_FP_SCALE, :,
    pub reserved1: [u32; 5],
    pub 8: uint32_t HUBPREQ0_CURSOR_SETTINGS__CURSOR0_DST_Y_OFFSET :,
    pub 8: uint32_t HUBPREQ0_CURSOR_SETTINGS__CURSOR0_CHUNK_HDL_ADJUST :,
    pub reserved2: [u32; 3],
}

//
// struct dmub_cursor_offload_pipe_data_dcn401_v1 - DCN401 per pipe data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_pipe_data_dcn401_v1 {
    pub CURSOR0_0_CURSOR_SURFACE_ADDRESS: u32,
    pub CURSOR0_0_CURSOR_SURFACE_ADDRESS_HIGH: u32,
    pub 16: uint32_t CURSOR0_0_CURSOR_SIZE__CURSOR_WIDTH :,
    pub 16: uint32_t CURSOR0_0_CURSOR_SIZE__CURSOR_HEIGHT :,
    pub 16: uint32_t CURSOR0_0_CURSOR_POSITION__CURSOR_X_POSITION :,
    pub 16: uint32_t CURSOR0_0_CURSOR_POSITION__CURSOR_Y_POSITION :,
    pub 16: uint32_t CURSOR0_0_CURSOR_HOT_SPOT__CURSOR_HOT_SPOT_X :,
    pub 16: uint32_t CURSOR0_0_CURSOR_HOT_SPOT__CURSOR_HOT_SPOT_Y :,
    pub 13: uint32_t CURSOR0_0_CURSOR_DST_OFFSET__CURSOR_DST_X_OFFSET :,
    pub 1: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_ENABLE :,
    pub 3: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_MODE :,
    pub 1: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_2X_MAGNIFY :,
    pub 2: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_PITCH :,
    pub 5: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_LINES_PER_CHUNK :,
    pub reserved0: [u32; 4],
    pub 1: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_ENABLE :,
    pub 3: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_MODE :,
    pub 1: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_EXPANSION_MODE :,
    pub 1: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_ROM_EN :,
    pub 24: uint32_t CM_CUR0_CURSOR0_COLOR0__CUR0_COLOR0 :,
    pub 24: uint32_t CM_CUR0_CURSOR0_COLOR1__CUR0_COLOR1 :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_G_Y__CUR0_FP_BIAS_G_Y :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_G_Y__CUR0_FP_SCALE_G_Y, :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_RB_CRCB__CUR0_FP_BIAS_RB_CRCB :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_RB_CRCB__CUR0_FP_SCALE_RB_CRCB :,
    pub reserved1: [u32; 4],
    pub 8: uint32_t HUBPREQ0_CURSOR_SETTINGS__CURSOR0_DST_Y_OFFSET :,
    pub 8: uint32_t HUBPREQ0_CURSOR_SETTINGS__CURSOR0_CHUNK_HDL_ADJUST :,
    pub 1: uint32_t HUBP0_DCHUBP_MALL_CONFIG__USE_MALL_FOR_CURSOR :,
    pub reserved2: [u32; 3],
}

//
// struct dmub_cursor_offload_pipe_data_dcn60_v1 - DCN60 per pipe data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_pipe_data_dcn60_v1 {
    pub CURSOR0_0_CURSOR_SURFACE_ADDRESS: u32,
    pub CURSOR0_0_CURSOR_SURFACE_ADDRESS_HIGH: u32,
    pub 16: uint32_t CURSOR0_0_CURSOR_SIZE__CURSOR_WIDTH :,
    pub 16: uint32_t CURSOR0_0_CURSOR_SIZE__CURSOR_HEIGHT :,
    pub 16: uint32_t CURSOR0_0_CURSOR_POSITION__CURSOR_X_POSITION :,
    pub 16: uint32_t CURSOR0_0_CURSOR_POSITION__CURSOR_Y_POSITION :,
    pub 16: uint32_t CURSOR0_0_CURSOR_HOT_SPOT__CURSOR_HOT_SPOT_X :,
    pub 16: uint32_t CURSOR0_0_CURSOR_HOT_SPOT__CURSOR_HOT_SPOT_Y :,
    pub 13: uint32_t CURSOR0_0_CURSOR_DST_OFFSET__CURSOR_DST_X_OFFSET :,
    pub 1: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_ENABLE :,
    pub 3: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_MODE :,
    pub 1: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_2X_MAGNIFY :,
    pub 2: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_PITCH :,
    pub 5: uint32_t CURSOR0_0_CURSOR_CONTROL__CURSOR_LINES_PER_CHUNK :,
    pub reserved0: [u32; 4],
    pub 1: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_ENABLE :,
    pub 3: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_MODE :,
    pub 1: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_EXPANSION_MODE :,
    pub 1: uint32_t CM_CUR0_CURSOR0_CONTROL__CUR0_ROM_EN :,
    pub 24: uint32_t CM_CUR0_CURSOR0_COLOR0__CUR0_COLOR0 :,
    pub 24: uint32_t CM_CUR0_CURSOR0_COLOR1__CUR0_COLOR1 :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_G_Y__CUR0_FP_BIAS_G_Y :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_G_Y__CUR0_FP_SCALE_G_Y, :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_RB_CRCB__CUR0_FP_BIAS_RB_CRCB :,
    pub 16: uint32_t CM_CUR0_CURSOR0_FP_SCALE_BIAS_RB_CRCB__CUR0_FP_SCALE_RB_CRCB :,
    pub reserved1: [u32; 4],
    pub 8: uint32_t HUBPREQ0_CURSOR_SETTINGS__CURSOR0_DST_Y_OFFSET :,
    pub 8: uint32_t HUBPREQ0_CURSOR_SETTINGS__CURSOR0_CHUNK_HDL_ADJUST :,
    pub 1: uint32_t HUBPREQ0_CURSOR_SETTINGS__FORCE_CURSOR_TO_DISP_PREF :,
    pub reserved2: [u32; 3],
}

//
// struct dmub_cursor_offload_pipe_data_v1 - Per pipe data for cursor offload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_pipe_data_v1 {
    pub /: *mut *mut *mut dmub_cursor_offload_pipe_data_dcn30_v1 dcn30; /< DCN30 cursor data.,
    pub /: *mut *mut *mut dmub_cursor_offload_pipe_data_dcn401_v1 dcn401; /< DCN401 cursor data.,
    pub /: *mut *mut *mut dmub_cursor_offload_pipe_data_dcn60_v1 dcn60; /< DCN60 cursor data.,
    pub /: *mut *mut *mut uint8_t payload[96]; /< Guarantees the cursor pipe data size per-pipe.,
}

//
// struct dmub_cursor_offload_payload_data_v1 - A payload of stream data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_payload_data_v1 {
    pub /: *mut *mut *mut uint32_t write_idx_start; /< Write index, updated before pipe_data is written.,
    pub /: *mut *mut *mut uint32_t write_idx_finish; /< Write index, updated after pipe_data is written.,
    pub /: *mut *mut *mut uint32_t pipe_mask; /< Mask of pipes to update.,
    pub /: *mut *mut *mut uint32_t reserved; /< Reserved for future use.,
    pub /: *mut *mut *mut dmub_cursor_offload_pipe_data_v1 pipe_data[6]; /< Per-pipe cursor data.,
}

//
// struct dmub_cursor_offload_stream_v1 - Per-stream data for cursor offload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_stream_v1 {
    pub /: *mut *mut *mut dmub_cursor_offload_payload_data_v1 payloads[4]; /< A small buffer of cursor payloads.,
    pub /: *mut *mut *mut uint32_t write_idx; /< The index of the last written payload.,
}

//
// struct dmub_cursor_offload_v1 - Cursor offload feature state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_offload_v1 {
    pub /: *mut *mut *mut dmub_cursor_offload_stream_v1 offload_streams[6]; /< Per-stream cursor offload data,
}

// ==============================================================================
// </DMUB_TYPES>=================================================================
// ==============================================================================
// < DMUB_META>==================================================================
// ==============================================================================

// Magic value for identifying dmub_fw_meta_info
pub const DMUB_FW_META_MAGIC: c_uint = 0x444D5542;
// Offset from the end of the file to the dmub_fw_meta_info
pub const DMUB_FW_META_OFFSET: c_uint = 0x24;
//
// union dmub_fw_meta_feature_bits - Static feature bits for pre-initialization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fw_meta_feature_bits {
    pub /: *mut *mut *mut uint32_t shared_state_link_detection : 1; /< 1 supports link detection via shared state,
    pub /: *mut *mut *mut uint32_t cursor_offload_v1_support: 1; /< 1 supports cursor offload,
    pub /: *mut *mut *mut uint32_t inbox0_lock_support: 1; /< 1 supports inbox0 lock mechanism,
    pub 29: uint32_t reserved :,
    pub /: *mut *mut *mut } bits; /< status bits,
    pub /: *mut *mut *mut uint32_t all; /< 32-bit access to status bits,
}

//
// struct dmub_fw_meta_info - metadata associated with fw binary
//
// NOTE: This should be considered a stable API. Fields should
// not be repurposed or reordered. New fields should be
// added instead to extend the structure.
//
// @magic_value: magic value identifying DMUB firmware meta info
// @fw_region_size: size of the firmware state region
// @trace_buffer_size: size of the tracebuffer region
// @fw_version: the firmware version information
// @dal_fw: 1 if the firmware is DAL
// @shared_state_size: size of the shared state region in bytes
// @shared_state_features: number of shared state features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fw_meta_info {
    pub /: *mut *mut *mut uint32_t magic_value; /< magic value identifying DMUB firmware meta info,
    pub /: *mut *mut *mut uint32_t fw_region_size; /< size of the firmware state region,
    pub /: *mut *mut *mut uint32_t trace_buffer_size; /< size of the tracebuffer region,
    pub /: *mut *mut *mut uint32_t fw_version; /< the firmware version information,
    pub /: *mut *mut *mut uint8_t dal_fw; /< 1 if the firmware is DAL,
    pub /: *mut *mut *mut uint8_t reserved[3]; /< padding bits,
    pub /: *mut *mut *mut uint32_t shared_state_size; /< size of the shared state region in bytes,
    pub /: *mut *mut *mut uint16_t shared_state_features; /< number of shared state features,
    pub /: *mut *mut *mut uint16_t reserved2; /< padding bytes,
    pub /: *mut *mut *mut dmub_fw_meta_feature_bits feature_bits; /< static feature bits,
}

//
// union dmub_fw_meta - ensures that dmub_fw_meta_info remains 64 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fw_meta {
    pub /: *mut *mut *mut dmub_fw_meta_info info; /< metadata info,
    pub /: *mut *mut *mut uint8_t reserved[64]; /< padding bits,
}

// ==============================================================================
// < DMUB Trace Buffer>================================================================
// ==============================================================================

//
// dmub_trace_code_t - firmware trace code, 32-bits
//
pub type dmub_trace_code_t = u32;
//
// struct dmcub_trace_buf_entry - Firmware trace entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmcub_trace_buf_entry {
    pub /: *mut *mut *mut dmub_trace_code_t trace_code; /< trace code for the event,
    pub /: *mut *mut *mut uint32_t tick_count; /< the tick count at time of trace,
    pub /: *mut *mut *mut uint32_t param0; /< trace defined parameter 0,
    pub /: *mut *mut *mut uint32_t param1; /< trace defined parameter 1,
}

// ==============================================================================
// < DMUB_STATUS>================================================================
// ==============================================================================
//
// DMCUB scratch registers can be used to determine firmware status.
// Current scratch register usage is as follows:
//
// SCRATCH0: FW Boot Status register
// SCRATCH5: LVTMA Status Register
// SCRATCH15: FW Boot Options register
//
// union dmub_fw_boot_status - Status bit definitions for SCRATCH0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fw_boot_status {
    pub /: *mut *mut *mut uint32_t dal_fw : 1; /< 1 if DAL FW,
    pub /: *mut *mut *mut uint32_t mailbox_rdy : 1; /< 1 if mailbox ready,
    pub /: *mut *mut *mut uint32_t optimized_init_done : 1; /< 1 if optimized init done,
    pub /: *mut *mut *mut uint32_t restore_required : 1; /< 1 if driver should call restore,
    pub /: *mut *mut *mut uint32_t defer_load : 1; /< 1 if VBIOS data is deferred programmed,
    pub /: *mut *mut *mut uint32_t fams_enabled : 1; /< 1 if VBIOS data is deferred programmed,
    pub /: *mut *mut *mut uint32_t detection_required: 1; /< if detection need to be triggered by driver,
    pub /: *mut *mut *mut uint32_t hw_power_init_done: 1; /< 1 if hw power init is completed,
    pub /: *mut *mut *mut uint32_t ono_regions_enabled: 1; /< 1 if ONO regions are enabled,
    pub /: *mut *mut *mut } bits; /< status bits,
    pub /: *mut *mut *mut uint32_t all; /< 32-bit access to status bits,
}

//
// enum dmub_fw_boot_status_bit - Enum bit definitions for SCRATCH0.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_fw_boot_status_bit {
    DMUB_FW_BOOT_STATUS_BIT_DAL_FIRMWARE = (1 << 0), /**< 1 if DAL FW */
    DMUB_FW_BOOT_STATUS_BIT_MAILBOX_READY = (1 << 1), /**< 1 if mailbox ready */
    DMUB_FW_BOOT_STATUS_BIT_OPTIMIZED_INIT_DONE = (1 << 2), /**< 1 if init done */
    DMUB_FW_BOOT_STATUS_BIT_RESTORE_REQUIRED = (1 << 3), /**< 1 if driver should call restore */
    DMUB_FW_BOOT_STATUS_BIT_DEFERRED_LOADED = (1 << 4), /**< 1 if VBIOS data is deferred programmed */
    DMUB_FW_BOOT_STATUS_BIT_FAMS_ENABLED = (1 << 5), /**< 1 if FAMS is enabled*/
    DMUB_FW_BOOT_STATUS_BIT_DETECTION_REQUIRED = (1 << 6), /**< 1 if detection need to be triggered by driver*/
    DMUB_FW_BOOT_STATUS_BIT_HW_POWER_INIT_DONE = (1 << 7), /**< 1 if hw power init is completed */
    DMUB_FW_BOOT_STATUS_BIT_ONO_REGIONS_ENABLED = (1 << 8), /**< 1 if ONO regions are enabled */
}

// Register bit definition for SCRATCH5
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_lvtma_status {
    pub 1: uint32_t psp_ok :,
    pub 1: uint32_t edp_on :,
    pub 30: uint32_t reserved :,
    pub bits: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_lvtma_status_bit {
    DMUB_LVTMA_STATUS_BIT_PSP_OK = (1 << 0),
    DMUB_LVTMA_STATUS_BIT_EDP_ON = (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_ips_disable_type {
    DMUB_IPS_ENABLE = 0,
    DMUB_IPS_DISABLE_ALL = 1,
    DMUB_IPS_DISABLE_IPS1 = 2,
    DMUB_IPS_DISABLE_IPS2 = 3,
    DMUB_IPS_DISABLE_IPS2_Z10 = 4,
    DMUB_IPS_DISABLE_DYNAMIC = 5,
    DMUB_IPS_RCG_IN_ACTIVE_IPS2_IN_OFF = 6,
    DMUB_IPS_DISABLE_Z8_RETENTION = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_ips_rcg_disable_type {
    DMUB_IPS_RCG_ENABLE = 0,
    DMUB_IPS0_RCG_DISABLE = 1,
    DMUB_IPS1_RCG_DISABLE = 2,
    DMUB_IPS_RCG_DISABLE = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_ips_in_vpb_disable_type {
    DMUB_IPS_VPB_RCG_ONLY = 0, // Legacy behaviour
    DMUB_IPS_VPB_DISABLE_ALL = 1,
    DMUB_IPS_VPB_ENABLE_IPS1_AND_RCG = 2,
    DMUB_IPS_VPB_ENABLE_ALL = 3 // Enable IPS1 Z8, IPS1 and RCG
}

pub const DMUB_IPS1_ALLOW_MASK: c_uint = 0x00000001;
pub const DMUB_IPS2_ALLOW_MASK: c_uint = 0x00000002;
pub const DMUB_IPS1_COMMIT_MASK: c_uint = 0x00000004;
pub const DMUB_IPS2_COMMIT_MASK: c_uint = 0x00000008;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_ips_comand_type {
//
// Start/stop IPS residency measurements for a given IPS mode
//
    DMUB_CMD__IPS_RESIDENCY_CNTL = 0,
//
// Query IPS residency information for a given IPS mode
//
    DMUB_CMD__IPS_QUERY_RESIDENCY_INFO = 1,
}

//
// enum dmub_cursor_offload_comand_type - Cursor offload subcommands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cursor_offload_comand_type {
//
// Initializes the cursor offload feature.
//
    DMUB_CMD__CURSOR_OFFLOAD_INIT = 0,
//
// Enables cursor offloading for a stream and updates the timing parameters.
//
    DMUB_CMD__CURSOR_OFFLOAD_STREAM_ENABLE = 1,
//
// Disables cursor offloading for a given stream.
//
    DMUB_CMD__CURSOR_OFFLOAD_STREAM_DISABLE = 2,
//
// Programs the latest data for a given stream.
//
    DMUB_CMD__CURSOR_OFFLOAD_STREAM_PROGRAM = 3,
}

//
// union dmub_fw_boot_options - Boot option definitions for SCRATCH14
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fw_boot_options {
    pub /: *mut *mut *mut uint32_t pemu_env : 1; /< 1 if PEMU,
    pub /: *mut *mut *mut uint32_t fpga_env : 1; /< 1 if FPGA,
    pub /: *mut *mut *mut uint32_t optimized_init : 1; /< 1 if optimized init,
    pub /: *mut *mut *mut uint32_t skip_phy_access : 1; /< 1 if PHY access should be skipped,
    pub /: *mut *mut *mut uint32_t disable_clk_gate: 1; /< 1 if clock gating should be disabled,
    pub /: *mut *mut *mut uint32_t skip_phy_init_panel_sequence: 1; /< 1 to skip panel init seq,
    pub /: *mut *mut *mut uint32_t z10_disable: 1; /< 1 to disable z10,
    pub /: *mut *mut *mut uint32_t enable_dpia: 1; /< 1 if DPIA should be enabled,
    pub /: *mut *mut *mut uint32_t invalid_vbios_data: 1; /< 1 if VBIOS data table is invalid,
    pub /: *mut *mut *mut uint32_t dpia_supported: 1; /< 1 if DPIA is supported on this platform,
    pub /: *mut *mut *mut uint32_t sel_mux_phy_c_d_phy_f_g: 1; /< 1 if PHYF/PHYG should be enabled on DCN31,
// < 1 if all root clock gating is enabled and low power memory is enabled
    pub 1: uint32_t power_optimization:,
    pub /: *mut *mut uint32_t diag_env: 1; / 1 if diagnostic environment,
    pub scratch8*/: *mut *mut uint32_t gpint_scratch8: 1; / 1 if GPINT is in,
    pub /: *mut *mut *mut uint32_t usb4_cm_version: 1; /< 1 CM support,
    pub /: *mut *mut uint32_t dpia_hpd_int_enable_supported: 1; / 1 if dpia hpd int enable supported,
    pub flow*/: *mut *mut uint32_t enable_non_transparent_setconfig: 1; / 1 if dpia use conventional dp lt,
    pub dppclk_ds*/: *mut *mut uint32_t disable_clk_ds: 1; / 1 if disallow dispclk_ds and,
    pub /: *mut *mut uint32_t disable_timeout_recovery : 1; / 1 if timeout recovery should be disabled,
    pub gating*/: *mut *mut uint32_t ips_pg_disable: 1; / 1 to disable ONO domains power,
    pub support*/: *mut *mut uint32_t ips_disable: 3; / options to disable ips,
    pub /: *mut *mut *mut uint32_t ips_sequential_ono: 1; /< 1 to enable sequential ONO IPS sequence,
    pub /: *mut *mut *mut uint32_t disable_sldo_opt: 1; /< 1 to disable SLDO optimizations,
    pub /: *mut *mut *mut uint32_t lower_hbr3_phy_ssc: 1; /< 1 to lower hbr3 phy ssc to 0.125 percent,
    pub /: *mut *mut *mut uint32_t override_hbr3_pll_vco: 1; /< 1 to override the hbr3 pll vco to 0,
    pub /: *mut *mut *mut uint32_t disable_dpia_bw_allocation: 1; /< 1 to disable the USB4 DPIA BW allocation,
    pub boot*/: *mut *mut *mut uint32_t bootcrc_en_at_preos: 1; /< 1 to run the boot time crc during warm/cold,
    pub boot*/: *mut *mut *mut uint32_t bootcrc_en_at_S0i3: 1; /< 1 to run the boot time crc during S0i3,
    pub boot*/: *mut *mut *mut uint32_t bootcrc_boot_mode: 1; /< 1 for S0i3 resume and 0 for Warm/cold,
    pub /: *mut *mut *mut uint32_t reserved : 1; /< reserved,
    pub /: *mut *mut *mut } bits; /< boot bits,
    pub /: *mut *mut *mut uint32_t all; /< 32-bit access to bits,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_fw_boot_options_bit {
    DMUB_FW_BOOT_OPTION_BIT_PEMU_ENV = (1 << 0), /**< 1 if PEMU */
    DMUB_FW_BOOT_OPTION_BIT_FPGA_ENV = (1 << 1), /**< 1 if FPGA */
    DMUB_FW_BOOT_OPTION_BIT_OPTIMIZED_INIT_DONE = (1 << 2), /**< 1 if optimized init done */
}

// ==============================================================================
// < DMUB_SHARED_STATE>==========================================================
// ==============================================================================
//
// Shared firmware state between driver and firmware for lockless communication
// in situations where the inbox/outbox may be unavailable.
//
// Each structure *must* be at most 256-bytes in size. The layout allocation is
// described below:
//
// [Header (256 Bytes)][Feature 1 (256 Bytes)][Feature 2 (256 Bytes)]...
//
// enum dmub_shared_state_feature_id - List of shared state features.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_shared_state_feature_id {
    DMUB_SHARED_SHARE_FEATURE__INVALID = 0,
    DMUB_SHARED_SHARE_FEATURE__IPS_FW = 1,
    DMUB_SHARED_SHARE_FEATURE__IPS_DRIVER = 2,
    DMUB_SHARED_SHARE_FEATURE__DEBUG_SETUP = 3,
    DMUB_SHARED_STATE_FEATURE__CURSOR_OFFLOAD_V1 = 4,
    DMUB_SHARED_STATE_FEATURE__LAST, /* Total number of features. */
}

//
// struct dmub_shared_state_ips_fw - Firmware signals for IPS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_shared_state_ips_fw_signals {
    pub /: *mut *mut *mut uint32_t ips1_commit : 1; /< 1 if in IPS1 or IPS0 RCG,
    pub /: *mut *mut *mut uint32_t ips2_commit : 1; /< 1 if in IPS2,
    pub /: *mut *mut *mut uint32_t in_idle : 1; /< 1 if DMCUB is in idle,
    pub /: *mut *mut *mut uint32_t detection_required : 1; /< 1 if detection is required,
    pub /: *mut *mut *mut uint32_t ips1z8_commit: 1; /< 1 if in IPS1 Z8 Retention,
    pub /: *mut *mut *mut uint32_t reserved_bits : 27; /< Reversed,
    pub bits: },
    pub all: u32,
}

//
// struct dmub_shared_state_ips_signals - Firmware signals for IPS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_shared_state_ips_driver_signals {
    pub /: *mut *mut *mut uint32_t allow_pg : 1; /< 1 if PG is allowed,
    pub /: *mut *mut *mut uint32_t allow_ips1 : 1; /< 1 is IPS1 is allowed,
    pub /: *mut *mut *mut uint32_t allow_ips2 : 1; /< 1 is IPS1 is allowed,
    pub /: *mut *mut *mut uint32_t allow_z10 : 1; /< 1 if Z10 is allowed,
    pub /: *mut *mut *mut uint32_t allow_idle: 1; /< 1 if driver is allowing idle,
    pub /: *mut *mut *mut uint32_t allow_ips0_rcg : 1; /< 1 is IPS0 RCG is allowed,
    pub /: *mut *mut *mut uint32_t allow_ips1_rcg : 1; /< 1 is IPS1 RCG is allowed,
    pub /: *mut *mut *mut uint32_t allow_ips1z8 : 1; /< 1 is IPS1 Z8 Retention is allowed,
    pub /: *mut *mut *mut uint32_t allow_dynamic_ips1 : 1; /< 1 if IPS1 is allowed in dynamic use cases such as VPB,
    pub /: *mut *mut *mut uint32_t allow_dynamic_ips1_z8: 1; /< 1 if IPS1 z8 ret is allowed in dynamic use cases such as VPB,
    pub /: *mut *mut *mut uint32_t reserved_bits : 22; /< Reversed bits,
    pub bits: },
    pub all: u32,
}

//
// IPS FW Version
//
pub const DMUB_SHARED_STATE__IPS_FW_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_debug_setup {
    pub exclude_points: [u32; 62],
    pub profile_mode: },
}

//
// struct dmub_shared_state_ips_fw - Firmware state for IPS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_ips_fw {
    pub /: *mut *mut *mut dmub_shared_state_ips_fw_signals signals; /< 4 bytes, IPS signal bits,
    pub /: *mut *mut *mut uint32_t rcg_entry_count; /< Entry counter for RCG,
    pub /: *mut *mut *mut uint32_t rcg_exit_count; /< Exit counter for RCG,
    pub /: *mut *mut *mut uint32_t ips1_entry_count; /< Entry counter for IPS1,
    pub /: *mut *mut *mut uint32_t ips1_exit_count; /< Exit counter for IPS1,
    pub /: *mut *mut *mut uint32_t ips2_entry_count; /< Entry counter for IPS2,
    pub /: *mut *mut *mut uint32_t ips2_exit_count; /< Exit counter for IPS2,
    pub /: *mut *mut *mut uint32_t ips1_z8ret_entry_count; /< Entry counter for IPS1 Z8 Retention,
    pub /: *mut *mut *mut uint32_t ips1_z8ret_exit_count; /< Exit counter for IPS1 Z8 Retention,
    pub /: *mut *mut *mut uint32_t reserved[53]; /< Reversed, to be updated when adding new fields.,
}

//
// IPS Driver Version
//
pub const DMUB_SHARED_STATE__IPS_DRIVER_VERSION: c_int = 1;
//
// struct dmub_shared_state_ips_driver - Driver state for IPS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_ips_driver {
    pub /: *mut *mut *mut dmub_shared_state_ips_driver_signals signals; /< 4 bytes, IPS signal bits,
    pub /: *mut *mut *mut uint32_t reserved[61]; /< Reversed, to be updated when adding new fields.,
}

//
// struct dmub_shared_state_cursor_offload_v1 - Header metadata for cursor offload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_cursor_offload_stream_v1 {
    pub /: *mut *mut *mut uint32_t last_write_idx; /< Last write index,
    pub /: *mut *mut *mut uint8_t reserved[28]; /< Reserved bytes.,
}

//
// struct dmub_shared_state_cursor_offload_v1 - Header metadata for cursor offload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_cursor_offload_v1 {
    pub /: *mut *mut *mut dmub_shared_state_cursor_offload_stream_v1 offload_streams[6]; /< stream state, 32-bytes each,
    pub /: *mut *mut *mut uint8_t reserved[56]; /< reserved for future use,
}

//
// enum dmub_shared_state_feature_common - Generic payload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_feature_common {
    pub padding: [u32; 62],
}

//
// enum dmub_shared_state_feature_header - Feature description.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_feature_header {
    pub /: *mut *mut *mut uint16_t id; /< Feature ID,
    pub /: *mut *mut *mut uint16_t version; /< Feature version,
    pub /: *mut *mut *mut uint32_t reserved; /< Reserved bytes.,
}

//
// struct dmub_shared_state_feature_block - Feature block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_shared_state_feature_block {
    pub /: *mut *mut *mut dmub_shared_state_feature_header header; /< Shared state header.,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_shared_feature_state_union {
    pub /: *mut *mut *mut dmub_shared_state_feature_common common; /< Generic data,
    pub /: *mut *mut *mut dmub_shared_state_ips_fw ips_fw; /< IPS firmware state,
    pub /: *mut *mut *mut dmub_shared_state_ips_driver ips_driver; /< IPS driver state,
    pub /: *mut *mut *mut dmub_shared_state_debug_setup debug_setup; /< Debug setup,
    pub /: *mut *mut *mut dmub_shared_state_cursor_offload_v1 cursor_offload_v1; /< Cursor offload,
    pub /: *mut *mut *mut } data; /< Shared state data.,
}

//
// Shared state size in bytes.
//

// ==============================================================================
// </DMUB_STATUS>================================================================
// ==============================================================================
// < DMUB_VBIOS>=================================================================
// ==============================================================================
//
// enum dmub_cmd_vbios_type - VBIOS commands.
//
// Command IDs should be treated as stable ABI.
// Do not reuse or modify IDs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_vbios_type {
//
// Configures the DIG encoder.
//
    DMUB_CMD__VBIOS_DIGX_ENCODER_CONTROL = 0,
//
// Controls the PHY.
//
    DMUB_CMD__VBIOS_DIG1_TRANSMITTER_CONTROL = 1,
//
// Sets the pixel clock/symbol clock.
//
    DMUB_CMD__VBIOS_SET_PIXEL_CLOCK = 2,
//
// Enables or disables power gating.
//
    DMUB_CMD__VBIOS_ENABLE_DISP_POWER_GATING = 3,
//
// Controls embedded panels.
//
    DMUB_CMD__VBIOS_LVTMA_CONTROL = 15,
//
// Query DP alt status on a transmitter.
//
    DMUB_CMD__VBIOS_TRANSMITTER_QUERY_DP_ALT  = 26,
//
// Control PHY FSM
//
    DMUB_CMD__VBIOS_TRANSMITTER_SET_PHY_FSM  = 29,
//
// Controls domain power gating
//
    DMUB_CMD__VBIOS_DOMAIN_CONTROL = 28,
}

// ==============================================================================
// </DMUB_VBIOS>=================================================================
// ==============================================================================
// < DMUB_GPINT>=================================================================
// ==============================================================================
//
// The shifts and masks below may alternatively be used to format and read
// the command register bits.
//
pub const DMUB_GPINT_DATA_PARAM_MASK: c_uint = 0xFFFF;
pub const DMUB_GPINT_DATA_PARAM_SHIFT: c_int = 0;
pub const DMUB_GPINT_DATA_COMMAND_CODE_MASK: c_uint = 0xFFF;
pub const DMUB_GPINT_DATA_COMMAND_CODE_SHIFT: c_int = 16;
pub const DMUB_GPINT_DATA_STATUS_MASK: c_uint = 0xF;
pub const DMUB_GPINT_DATA_STATUS_SHIFT: c_int = 28;
//
// Command responses.
//
// Return response for DMUB_GPINT__STOP_FW command.
//
pub const DMUB_GPINT__STOP_FW_RESPONSE: c_uint = 0xDEADDEAD;
//
// union dmub_gpint_data_register - Format for sending a command via the GPINT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_gpint_data_register {
    pub /: *mut *mut *mut uint32_t param : 16; /< 16-bit parameter,
    pub /: *mut *mut *mut uint32_t command_code : 12; /< GPINT command,
    pub /: *mut *mut *mut uint32_t status : 4; /< Command status bit,
    pub /: *mut *mut *mut } bits; /< GPINT bit access,
    pub /: *mut *mut *mut uint32_t all; /< GPINT 32-bit access,
}

//
// enum dmub_gpint_command - GPINT command to DMCUB FW
//
// Command IDs should be treated as stable ABI.
// Do not reuse or modify IDs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_gpint_command {
//
// Invalid command, ignored.
//
    DMUB_GPINT__INVALID_COMMAND = 0,
//
// DESC: Queries the firmware version.
// RETURN: Firmware version.
//
    DMUB_GPINT__GET_FW_VERSION = 1,
//
// DESC: Halts the firmware.
// RETURN: DMUB_GPINT__STOP_FW_RESPONSE (0xDEADDEAD) when halted
//
    DMUB_GPINT__STOP_FW = 2,
//
// DESC: Get PSR state from FW.
// RETURN: PSR state enum. This enum may need to be converted to the legacy PSR state value.
//
    DMUB_GPINT__GET_PSR_STATE = 7,
//
// DESC: Notifies DMCUB of the currently active streams.
// ARGS: Stream mask, 1 bit per active stream index.
//
    DMUB_GPINT__IDLE_OPT_NOTIFY_STREAM_MASK = 8,
//
// DESC: Start PSR residency counter. Stop PSR resdiency counter and get value.
// ARGS: We can measure residency from various points. The argument will specify the residency mode.
// By default, it is measured from after we powerdown the PHY, to just before we powerup the PHY.
// RETURN: PSR residency in milli-percent.
//
    DMUB_GPINT__PSR_RESIDENCY = 9,

//
// DESC: Notifies DMCUB detection is done so detection required can be cleared.
//
    DMUB_GPINT__NOTIFY_DETECTION_DONE = 12,

//
// DESC: Get REPLAY state from FW.
// RETURN: REPLAY state enum. This enum may need to be converted to the legacy REPLAY state value.
//
    DMUB_GPINT__GET_REPLAY_STATE = 13,

//
// DESC: Start REPLAY residency counter. Stop REPLAY resdiency counter and get value.
// ARGS: We can measure residency from various points. The argument will specify the residency mode.
// By default, it is measured from after we powerdown the PHY, to just before we powerup the PHY.
// RETURN: REPLAY residency in milli-percent.
//
    DMUB_GPINT__REPLAY_RESIDENCY = 14,

//
// DESC: Set CACP internal core configuration
//
    DMUB_GPINT__TRIGGER_CACP = 18,

//
// DESC: Copy bounding box to the host.
// ARGS: Version of bounding box to copy
// RETURN: Result of copying bounding box
//
    DMUB_GPINT__BB_COPY = 96,

//
// DESC: Updates the host addresses bit48~bit63 for bounding box.
// ARGS: The word3 for the 64 bit address
//
    DMUB_GPINT__SET_BB_ADDR_WORD3 = 97,

//
// DESC: Updates the host addresses bit32~bit47 for bounding box.
// ARGS: The word2 for the 64 bit address
//
    DMUB_GPINT__SET_BB_ADDR_WORD2 = 98,

//
// DESC: Updates the host addresses bit16~bit31 for bounding box.
// ARGS: The word1 for the 64 bit address
//
    DMUB_GPINT__SET_BB_ADDR_WORD1 = 99,

//
// DESC: Updates the host addresses bit0~bit15 for bounding box.
// ARGS: The word0 for the 64 bit address
//
    DMUB_GPINT__SET_BB_ADDR_WORD0 = 100,

//
// DESC: Updates the trace buffer lower 32-bit mask.
// ARGS: The new mask
// RETURN: Lower 32-bit mask.
//
    DMUB_GPINT__UPDATE_TRACE_BUFFER_MASK = 101,

//
// DESC: Updates the trace buffer mask bit0~bit15.
// ARGS: The new mask
// RETURN: Lower 32-bit mask.
//
    DMUB_GPINT__SET_TRACE_BUFFER_MASK_WORD0 = 102,

//
// DESC: Updates the trace buffer mask bit16~bit31.
// ARGS: The new mask
// RETURN: Lower 32-bit mask.
//
    DMUB_GPINT__SET_TRACE_BUFFER_MASK_WORD1 = 103,

//
// DESC: Updates the trace buffer mask bit32~bit47.
// ARGS: The new mask
// RETURN: Lower 32-bit mask.
//
    DMUB_GPINT__SET_TRACE_BUFFER_MASK_WORD2 = 114,

//
// DESC: Updates the trace buffer mask bit48~bit63.
// ARGS: The new mask
// RETURN: Lower 32-bit mask.
//
    DMUB_GPINT__SET_TRACE_BUFFER_MASK_WORD3 = 115,

//
// DESC: Read the trace buffer mask bi0~bit15.
//
    DMUB_GPINT__GET_TRACE_BUFFER_MASK_WORD0 = 116,

//
// DESC: Read the trace buffer mask bit16~bit31.
//
    DMUB_GPINT__GET_TRACE_BUFFER_MASK_WORD1 = 117,

//
// DESC: Read the trace buffer mask bi32~bit47.
//
    DMUB_GPINT__GET_TRACE_BUFFER_MASK_WORD2 = 118,

//
// DESC: Updates the trace buffer mask bit32~bit63.
//
    DMUB_GPINT__GET_TRACE_BUFFER_MASK_WORD3 = 119,

//
// DESC: Set IPS residency measurement
// ARGS: 0 - Disable ips measurement
// 1 - Enable ips measurement
//
    DMUB_GPINT__IPS_RESIDENCY = 121,
//
// DESC: Enable measurements for various task duration
// ARGS: 0 - Disable measurement
// 1 - Enable measurement
//
    DMUB_GPINT__TRACE_DMUB_WAKE_ACTIVITY = 123,
//
// DESC: Gets IPS residency in microseconds
// ARGS: 0 - Return IPS1 residency
// 1 - Return IPS2 residency
// 2 - Return IPS0_RCG residency
// 3 - Return IPS1_ONO2_ON residency
// 4 - Return IPS1_Z8_RETENTION residency
// RETURN: Total residency in microseconds - lower 32 bits
//
    DMUB_GPINT__GET_IPS_RESIDENCY_DURATION_US_LO = 124,
//
// DESC: Gets IPS1 histogram counts
// ARGS: Bucket index
// RETURN: Total count for the bucket
//
    DMUB_GPINT__GET_IPS1_HISTOGRAM_COUNTER = 125,
//
// DESC: Gets IPS2 histogram counts
// ARGS: Bucket index
// RETURN: Total count for the bucket
//
    DMUB_GPINT__GET_IPS2_HISTOGRAM_COUNTER = 126,
//
// DESC: Gets IPS residency
// ARGS: 0 - Return IPS1 residency
// 1 - Return IPS2 residency
// 2 - Return IPS0_RCG residency
// 3 - Return IPS1_ONO2_ON residency
// 4 - Return IPS1_Z8_RETENTION residency
// RETURN: Total residency in milli-percent.
//
    DMUB_GPINT__GET_IPS_RESIDENCY_PERCENT = 127,
//
// DESC: Gets IPS0_RCG histogram counts
// ARGS: Bucket index
// RETURN: Total count for the bucket
//
    DMUB_GPINT__GET_IPS0_RCG_HISTOGRAM_COUNTER = 128,
//
// DESC: Gets IPS1_ONO2_ON histogram counts
// ARGS: Bucket index
// RETURN: Total count for the bucket
//
    DMUB_GPINT__GET_IPS1_ONO2_ON_HISTOGRAM_COUNTER = 129,
//
// DESC: Gets IPS entry counter during residency measurement
// ARGS: 0 - Return IPS1 entry counts
// 1 - Return IPS2 entry counts
// 2 - Return IPS0_RCG entry counts
// 3 - Return IPS1_ONO2_ON entry counts
// 4 - Return IPS1_Z8_RETENTION entry counts
// RETURN: Entry counter for selected IPS mode
//
    DMUB_GPINT__GET_IPS_RESIDENCY_ENTRY_COUNTER = 130,
//
// DESC: Gets IPS inactive residency in microseconds
// ARGS: 0 - Return IPS1_MAX residency
// 1 - Return IPS2 residency
// 2 - Return IPS0_RCG residency
// 3 - Return IPS1_ONO2_ON residency
// 4 - Return IPS1_Z8_RETENTION residency
// RETURN: Total inactive residency in microseconds - lower 32 bits
//
    DMUB_GPINT__GET_IPS_INACTIVE_RESIDENCY_DURATION_US_LO = 131,
//
// DESC: Gets IPS inactive residency in microseconds
// ARGS: 0 - Return IPS1_MAX residency
// 1 - Return IPS2 residency
// 2 - Return IPS0_RCG residency
// 3 - Return IPS1_ONO2_ON residency
// 4 - Return IPS1_Z8_RETENTION residency
// RETURN: Total inactive residency in microseconds - upper 32 bits
//
    DMUB_GPINT__GET_IPS_INACTIVE_RESIDENCY_DURATION_US_HI = 132,
//
// DESC: Gets IPS residency in microseconds
// ARGS: 0 - Return IPS1 residency
// 1 - Return IPS2 residency
// 2 - Return IPS0_RCG residency
// 3 - Return IPS1_ONO2_ON residency
// 4 - Return IPS1_Z8_RETENTION residency
// RETURN: Total residency in microseconds - upper 32 bits
//
    DMUB_GPINT__GET_IPS_RESIDENCY_DURATION_US_HI = 133,
//
// DESC: Setup debug configs.
//
    DMUB_GPINT__SETUP_DEBUG_MODE = 136,
//
// DESC: Initiates IPS wake sequence.
//
    DMUB_GPINT__IPS_DEBUG_WAKE = 137,
//
// DESC: Do panel power off sequence
// ARGS: 1 - Power off
//
    DMUB_GPINT__PANEL_POWER_OFF_SEQ = 138,
//
// DESC: Gets panel polarity bias.
// ARGS: 0 - Get panel polarity bias
//
    DMUB_GPINT__PANEL_POLARITY_GET_BIAS = 139,
//
// DESC: Enables panel polarity.
// ARGS: 0 - Disable panel polarity
// 1 - Enable panel polarity
//
    DMUB_GPINT__PANEL_POLARITY_DEBUG_ENABLE = 140,
}

//
// INBOX0 generic command definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_inbox0_cmd_common {
    pub /: *mut *mut *mut uint32_t command_code: 8; /< INBOX0 command code,
    pub /: *mut *mut *mut uint32_t param: 24; /< 24-bit parameter,
    pub bits: },
    pub all: u32,
}

//
// INBOX0 hw_lock command definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_inbox0_cmd_lock_hw {
    pub 8: uint32_t command_code:,
// NOTE: Must be have enough bits to match: enum hw_lock_client
    pub 2: uint32_t hw_lock_client:,
// NOTE: Below fields must match with: struct dmub_hw_lock_inst_flags
    pub 3: uint32_t otg_inst:,
    pub 3: uint32_t opp_inst:,
    pub 3: uint32_t dig_inst:,
// NOTE: Below fields must match with: union dmub_hw_lock_flags
    pub 1: uint32_t lock_pipe:,
    pub 1: uint32_t lock_cursor:,
    pub 1: uint32_t lock_dig:,
    pub 1: uint32_t triple_buffer_lock:,
    pub /: *mut *mut *mut uint32_t lock: 1; /< Lock,
    pub /: *mut *mut *mut uint32_t should_release: 1; /< Release,
    pub /: *mut *mut *mut uint32_t reserved: 7; /< Reserved for extending more clients, HW, etc.,
    pub bits: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_inbox0_data_register {
    pub inbox0_cmd_common: dmub_inbox0_cmd_common,
    pub inbox0_cmd_lock_hw: dmub_inbox0_cmd_lock_hw,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_inbox0_command {
//
// DESC: Invalid command, ignored.
//
    DMUB_INBOX0_CMD__INVALID_COMMAND = 0,
//
// DESC: Notification to acquire/release HW lock
// ARGS:
//
    DMUB_INBOX0_CMD__HW_LOCK = 1,
}

// ==============================================================================
// </DMUB_GPINT>=================================================================
// ==============================================================================
// < DMUB_CMD>===================================================================
// ==============================================================================
//
// Size in bytes of each DMUB command.
//
pub const DMUB_RB_CMD_SIZE: c_int = 64;
//
// Maximum number of items in the DMUB ringbuffer.
//
pub const DMUB_RB_MAX_ENTRY: c_int = 128;
//
// Ringbuffer size in bytes.
//

//
// Maximum number of items in the DMUB REG INBOX0 internal ringbuffer.
//
pub const DMUB_REG_INBOX0_RB_MAX_ENTRY: c_int = 16;
//
// Ringbuffer size in bytes.
//

//
// REG_SET mask for reg offload.
//
pub const REG_SET_MASK: c_uint = 0xFFFF;
//
// enum dmub_cmd_type - DMUB inbox command.
//
// Command IDs should be treated as stable ABI.
// Do not reuse or modify IDs.
// Note that command IDs 1-4 have been deprecated.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_type {
//
// Invalid command.
//
    DMUB_CMD__NULL = 0,
//
// Workaround to avoid HUBP underflow during NV12 playback.
//
    DMUB_CMD__PLAT_54186_WA = 5,
//
// Command type used to query FW feature caps.
//
    DMUB_CMD__QUERY_FEATURE_CAPS = 6,
//
// Command type used to get visual confirm color.
//
    DMUB_CMD__GET_VISUAL_CONFIRM_COLOR = 8,
//
// Command type used for all PSR commands.
//
    DMUB_CMD__PSR = 64,
//
// Command type used for all MALL commands.
//
    DMUB_CMD__MALL = 65,
//
// Command type used for all ABM commands.
//
    DMUB_CMD__ABM = 66,
//
// Command type used to update dirty rects in FW.
//
    DMUB_CMD__UPDATE_DIRTY_RECT = 67,
//
// Command type used to update cursor info in FW.
//
    DMUB_CMD__UPDATE_CURSOR_INFO = 68,
//
// Command type used for HW locking in FW.
//
    DMUB_CMD__HW_LOCK = 69,
//
// Command type used to access DP AUX.
//
    DMUB_CMD__DP_AUX_ACCESS = 70,
//
// Command type used for OUTBOX1 notification enable
//
    DMUB_CMD__OUTBOX1_ENABLE = 71,

//
// Command type used for all idle optimization commands.
//
    DMUB_CMD__IDLE_OPT = 72,
//
// Command type used for all clock manager commands.
//
    DMUB_CMD__CLK_MGR = 73,
//
// Command type used for all panel control commands.
//
    DMUB_CMD__PANEL_CNTL = 74,

//
// Command type used for all CAB commands.
//
    DMUB_CMD__CAB_FOR_SS = 75,

    DMUB_CMD__FW_ASSISTED_MCLK_SWITCH = 76,

//
// Command type used for interfacing with DPIA.
//
    DMUB_CMD__DPIA = 77,
//
// Command type used for EDID CEA parsing
//
    DMUB_CMD__EDID_CEA = 79,
//
// Command type used for getting usbc cable ID
//
    DMUB_CMD_GET_USBC_CABLE_ID = 81,
//
// Command type used to query HPD state.
//
    DMUB_CMD__QUERY_HPD_STATE = 82,
//
// Command type used for all VBIOS interface commands.
//
// Command type used for all REPLAY commands.
//
    DMUB_CMD__REPLAY = 83,

//
// Command type used for all SECURE_DISPLAY commands.
//
    DMUB_CMD__SECURE_DISPLAY = 85,

//
// Command type used to set DPIA HPD interrupt state
//
    DMUB_CMD__DPIA_HPD_INT_ENABLE = 86,

//
// Command type used for all CACP commands.
//
    DMUB_CMD__CACP = 87,

//
// Command type used for all PSP commands.
//
    DMUB_CMD__PSP = 88,

//
// Command type used for all Fused IO commands.
//
    DMUB_CMD__FUSED_IO = 89,

//
// Command type used for all LSDMA commands.
//
    DMUB_CMD__LSDMA = 90,

//
// Command type use for all IPS commands.
//
    DMUB_CMD__IPS = 91,

//
// Command type use for Cursor offload.
//
    DMUB_CMD__CURSOR_OFFLOAD = 92,

//
// Command type used for all SMART_POWER_OLED commands.
//
    DMUB_CMD__SMART_POWER_OLED = 93,

//
// Command type use for all Panel Replay commands.
//
    DMUB_CMD__PR = 94,

//
// Command type used for all IHC commands.
//
    DMUB_CMD__IHC = 95,

//
// Command type use for boot time crc commands.
//
    DMUB_CMD__BOOT_TIME_CRC = 96,

//
// Command type use for all Panel Polarity commands.
//
    DMUB_CMD__PANEL_POLARITY = 97,

//
// Command type use for VBIOS shared commands.
//
    DMUB_CMD__VBIOS = 128,
}

//
// enum dmub_out_cmd_type - DMUB outbox commands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_out_cmd_type {
//
// Invalid outbox command, ignored.
//
    DMUB_OUT_CMD__NULL = 0,
//
// Command type used for DP AUX Reply data notification
//
    DMUB_OUT_CMD__DP_AUX_REPLY = 1,
//
// Command type used for DP HPD event notification
//
    DMUB_OUT_CMD__DP_HPD_NOTIFY = 2,
//
// Command type used for SET_CONFIG Reply notification
//
    DMUB_OUT_CMD__SET_CONFIG_REPLY = 3,
//
// Command type used for USB4 DPIA notification
//
    DMUB_OUT_CMD__DPIA_NOTIFICATION = 5,
//
// Command type used for HPD redetect notification
//
    DMUB_OUT_CMD__HPD_SENSE_NOTIFY = 6,
//
// Command type used for Fused IO notification
//
    DMUB_OUT_CMD__FUSED_IO = 7,
}

// DMUB_CMD__DPIA command sub-types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_dpia_type {
    DMUB_CMD__DPIA_DIG1_DPIA_CONTROL = 0,
    DMUB_CMD__DPIA_SET_CONFIG_ACCESS = 1, // will be replaced by DPIA_SET_CONFIG_REQUEST
    DMUB_CMD__DPIA_MST_ALLOC_SLOTS = 2,
    DMUB_CMD__DPIA_SET_TPS_NOTIFICATION = 3,
    DMUB_CMD__DPIA_SET_CONFIG_REQUEST = 4,
}

// DMUB_OUT_CMD__DPIA_NOTIFICATION command types.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_dpia_notification_type {
    DPIA_NOTIFY__BW_ALLOCATION = 0,
}

//
// struct dmub_cmd_header - Common command header fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_header {
    pub /: *mut *mut *mut unsigned int type : 8; /< command type,
    pub /: *mut *mut *mut unsigned int sub_type : 8; /< command sub type,
    pub /: *mut *mut *mut unsigned int ret_status : 1; /< 1 if returned data, 0 otherwise,
    pub /: *mut *mut *mut unsigned int multi_cmd_pending : 1; /< 1 if multiple commands chained together,
    pub /: *mut *mut *mut unsigned int is_reg_based : 1; /< 1 if register based mailbox cmd, 0 if FB based cmd,
    pub /: *mut *mut *mut unsigned int reserved0 : 5; /< reserved bits,
    pub /: *mut *mut unsigned int payload_bytes : 6; / payload excluding header - up to 60 bytes,
    pub /: *mut *mut *mut unsigned int reserved1 : 2; /< reserved bits,
}

//
// struct dmub_rb_cmd_common - Common command header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_common {
    pub /: *mut *mut *mut dmub_cmd_header header; /< command header,
//
// Padding to RB_CMD_SIZE
//
    pub dmub_cmd_header)]: uint8_t cmd_buffer[DMUB_RB_CMD_SIZE - sizeof(struct,
}

//
// struct dmub_cmd_PLAT_54186_wa - Underflow workaround
//
// Reprograms surface parameters to avoid underflow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_PLAT_54186_wa {
    pub /: *mut *mut *mut uint32_t DCSURF_SURFACE_CONTROL; /< reg value,
    pub /: *mut *mut *mut uint32_t DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH; /< reg value,
    pub /: *mut *mut *mut uint32_t DCSURF_PRIMARY_SURFACE_ADDRESS; /< reg value,
    pub /: *mut *mut *mut uint32_t DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH_C; /< reg value,
    pub /: *mut *mut *mut uint32_t DCSURF_PRIMARY_SURFACE_ADDRESS_C; /< reg value,
    pub /: *mut *mut *mut uint32_t hubp_inst : 4; /< HUBP instance,
    pub /: *mut *mut *mut uint32_t tmz_surface : 1; /< TMZ enable or disable,
    pub /: *mut *mut *mut uint32_t immediate :1; /< Immediate flip,
    pub /: *mut *mut *mut uint32_t vmid : 4; /< VMID,
    pub /: *mut *mut *mut uint32_t grph_stereo : 1; /< 1 if stereo,
    pub /: *mut *mut *mut uint32_t reserved : 21; /< Reserved,
    pub /: *mut *mut *mut } flip_params; /< Pageflip parameters,
    pub /: *mut *mut *mut uint32_t reserved[9]; /< Reserved bits,
}

//
// struct dmub_rb_cmd_PLAT_54186_wa - Underflow workaround command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_PLAT_54186_wa {
    pub /: *mut *mut *mut dmub_cmd_header header; /< Command header,
    pub /: *mut *mut *mut dmub_cmd_PLAT_54186_wa flip; /< Flip data,
}

//
// enum dmub_cmd_mall_type - MALL commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_mall_type {
//
// Allows display refresh from MALL.
//
    DMUB_CMD__MALL_ACTION_ALLOW = 0,
//
// Disallows display refresh from MALL.
//
    DMUB_CMD__MALL_ACTION_DISALLOW = 1,
//
// Cursor copy for MALL.
//
    DMUB_CMD__MALL_ACTION_COPY_CURSOR = 2,
//
// Controls DF requests.
//
    DMUB_CMD__MALL_ACTION_NO_DF_REQ = 3,
}

//
// struct dmub_rb_cmd_mall - MALL command data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_mall {
    pub /: *mut *mut *mut dmub_cmd_header header; /< Common command header,
    pub /: *mut *mut *mut dmub_addr cursor_copy_src; /< Cursor copy address,
    pub /: *mut *mut *mut dmub_addr cursor_copy_dst; /< Cursor copy destination,
    pub /: *mut *mut *mut uint32_t tmr_delay; /< Timer delay,
    pub /: *mut *mut *mut uint32_t tmr_scale; /< Timer scale,
    pub /: *mut *mut *mut uint16_t cursor_width; /< Cursor width in pixels,
    pub /: *mut *mut *mut uint16_t cursor_pitch; /< Cursor pitch in pixels,
    pub /: *mut *mut *mut uint16_t cursor_height; /< Cursor height in pixels,
    pub /: *mut *mut *mut uint8_t cursor_bpp; /< Cursor bits per pixel,
    pub /: *mut *mut *mut uint8_t debug_bits; /< Debug bits,
    pub /: *mut *mut *mut uint8_t reserved1; /< Reserved bits,
    pub /: *mut *mut *mut uint8_t reserved2; /< Reserved bits,
}

//
// enum dmub_cmd_cab_type - CAB command data.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_cab_type {
//
// No idle optimizations (i.e. no CAB)
//
    DMUB_CMD__CAB_NO_IDLE_OPTIMIZATION = 0,
//
// No DCN requests for memory
//
    DMUB_CMD__CAB_NO_DCN_REQ = 1,
//
// Fit surfaces in CAB (i.e. CAB enable)
//
    DMUB_CMD__CAB_DCN_SS_FIT_IN_CAB = 2,
//
// Do not fit surfaces in CAB (i.e. no CAB)
//
    DMUB_CMD__CAB_DCN_SS_NOT_FIT_IN_CAB = 3,
}

//
// struct dmub_rb_cmd_cab_for_ss - CAB command data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cab_for_ss {
    pub header: dmub_cmd_header,
    pub /: *mut *mut uint8_t cab_alloc_ways; / total number of ways,
    pub /: *mut *mut uint8_t debug_bits; / debug bits,
}

//
// Enum for indicating which MCLK switch mode per pipe
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mclk_switch_mode {
    NONE = 0,
    FPO = 1,
    SUBVP = 2,
    VBLANK = 3,
}

// Per pipe struct which stores the MCLK switch mode
// data to be sent to DMUB.
// Named "v2" for now -- once FPO and SUBVP are fully merged
// the type name can be updated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fw_assisted_mclk_switch_pipe_data_v2 {
    pub pix_clk_100hz: u32,
    pub main_vblank_start: u16,
    pub main_vblank_end: u16,
    pub mall_region_lines: u16,
    pub prefetch_lines: u16,
    pub prefetch_to_mall_start_lines: u16,
    pub processing_delay_lines: u16,
    pub cases: uint16_t htotal; // required to calculate line time for multi-display,
    pub vtotal: u16,
    pub main_pipe_index: u8,
    pub phantom_pipe_index: u8,
// Since the microschedule is calculated in terms of OTG lines,
// include any scaling factors to make sure when we get accurate
// conversion when programming MALL_START_LINE (which is in terms
// of HUBP lines). If 4K is being downscaled to 1080p, scale factor
// is 1/2 (numerator = 1, denominator = 2).
//
    pub scale_factor_numerator: u8,
    pub scale_factor_denominator: u8,
    pub is_drr: u8,
    pub main_split_pipe_index: u8,
    pub phantom_split_pipe_index: u8,
    pub subvp_data: },
    pub pix_clk_100hz: u32,
    pub vblank_start: u16,
    pub vblank_end: u16,
    pub vstartup_start: u16,
    pub vtotal: u16,
    pub htotal: u16,
    pub vblank_pipe_index: u8,
    pub padding: [u8; 1],
    pub drr_in_use: u8,
    pub frame: uint8_t drr_window_size_ms; // Indicates largest VMIN/VMAX adjustment per,
    pub VBLANK: uint16_t min_vtotal_supported; // Min VTOTAL that supports switching in,
    pub scheduling: uint16_t max_vtotal_supported; // Max VTOTAL that can support SubVP static,
    pub not: uint8_t use_ramping; // Use ramping or,
    pub drr_vblank_start_margin: u8,
    pub case: } drr_info; // DRR considered as part of SubVP + VBLANK,
    pub vblank_data: },
    pub pipe_config: },
// - subvp_data in the union (pipe_config) takes up 27 bytes.
// - Make the "mode" field a uint8_t instead of enum so we only use 1 byte (only
// for the DMCUB command, cast to enum once we populate the DMCUB subvp state).
//
    pub mclk_switch_mode: uint8_t mode; // enum,
}

//
// Config data for Sub-VP and FPO
// Named "v2" for now -- once FPO and SUBVP are fully merged
// the type name can be updated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fw_assisted_mclk_switch_config_v2 {
    pub watermark_a_cache: u16,
    pub vertical_int_margin_us: u8,
    pub pstate_allow_width_us: u8,
    pub pipe_data: [dmub_cmd_fw_assisted_mclk_switch_pipe_data_v2; DMUB_MAX_SUBVP_STREAMS],
}

//
// DMUB rb command definition for Sub-VP and FPO
// Named "v2" for now -- once FPO and SUBVP are fully merged
// the type name can be updated
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_fw_assisted_mclk_switch_v2 {
    pub header: dmub_cmd_header,
    pub config_data: dmub_cmd_fw_assisted_mclk_switch_config_v2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_flip_addr_info {
    pub surf_addr_lo: u32,
    pub surf_addr_c_lo: u32,
    pub meta_addr_lo: u32,
    pub meta_addr_c_lo: u32,
    pub surf_addr_hi: u16,
    pub surf_addr_c_hi: u16,
    pub meta_addr_hi: u16,
    pub meta_addr_c_hi: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_flip_info {
    pub 1: uint8_t is_immediate:,
    pub bits: },
    pub all: u8,
    pub config: },
    pub otg_inst: u8,
    pub pipe_mask: u8,
    pub pad: u8,
    pub addr_info: dmub_flip_addr_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_fams2_flip {
    pub header: dmub_cmd_header,
    pub flip_info: dmub_fams2_flip_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_lsdma_data {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_init_data {
    pub gpu_addr_base: dmub_addr,
    pub ring_size: u32,
    pub init_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_tiled_copy_data {
    pub src_addr_lo: u32,
    pub src_addr_hi: u32,
    pub dst_addr_lo: u32,
    pub dst_addr_hi: u32,
    pub 16: uint32_t src_x :,
    pub 16: uint32_t src_y :,
    pub 16: uint32_t dst_x :,
    pub 16: uint32_t dst_y :,
    pub 16: uint32_t src_width :,
    pub 16: uint32_t src_height :,
    pub 16: uint32_t dst_width :,
    pub 16: uint32_t dst_height :,
    pub 16: uint32_t rect_x :,
    pub 16: uint32_t rect_y :,
    pub 5: uint32_t src_swizzle_mode :,
    pub 5: uint32_t src_mip_max :,
    pub 5: uint32_t src_mip_id :,
    pub 5: uint32_t dst_mip_max :,
    pub 5: uint32_t dst_swizzle_mode :,
    pub 5: uint32_t dst_mip_id :,
    pub 1: uint32_t dcc :,
    pub 1: uint32_t padding1 :,
    pub 6: uint32_t data_format :,
    pub 4: uint32_t tmz :,
    pub 3: uint32_t dst_element_size :,
    pub 3: uint32_t num_type :,
    pub 3: uint32_t src_element_size :,
    pub 2: uint32_t write_compress :,
    pub 3: uint32_t cache_policy_dst :,
    pub 3: uint32_t cache_policy_src :,
    pub 2: uint32_t read_compress :,
    pub 2: uint32_t max_com :,
    pub 1: uint32_t max_uncom :,
    pub 2: uint32_t dst_dim :,
    pub 2: uint32_t src_dim :,
    pub 28: uint32_t padding :,
    pub tiled_copy_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_linear_copy_data {
    pub src_lo: u32,
    pub src_hi: u32,
    pub dst_lo: u32,
    pub dst_hi: u32,
    pub 30: uint32_t count :,
    pub 2: uint32_t pad0 :,
    pub 4: uint32_t tmz :,
    pub 3: uint32_t cache_policy_src :,
    pub 3: uint32_t cache_policy_dst :,
    pub 6: uint32_t data_format :,
    pub 3: uint32_t num_type :,
    pub 2: uint32_t read_compress :,
    pub 2: uint32_t write_compress :,
    pub 2: uint32_t max_com :,
    pub 1: uint32_t max_uncom :,
    pub 6: uint32_t pad1 :,
    pub linear_copy_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_linear_sub_window_copy_data {
    pub src_lo: u32,
    pub src_hi: u32,
    pub dst_lo: u32,
    pub dst_hi: u32,
    pub 16: uint32_t src_x :,
    pub 16: uint32_t src_y :,
    pub 16: uint32_t dst_x :,
    pub 16: uint32_t dst_y :,
    pub 16: uint32_t rect_x :,
    pub 16: uint32_t rect_y :,
    pub 16: uint32_t src_pitch :,
    pub 16: uint32_t dst_pitch :,
    pub src_slice_pitch: u32,
    pub dst_slice_pitch: u32,
    pub 4: uint32_t tmz :,
    pub 3: uint32_t element_size :,
    pub 3: uint32_t src_cache_policy :,
    pub 3: uint32_t dst_cache_policy :,
    pub 6: uint32_t data_format :,
    pub 3: uint32_t num_type :,
    pub 2: uint32_t read_compress :,
    pub 2: uint32_t write_compress :,
    pub 2: uint32_t max_com :,
    pub 1: uint32_t max_uncom :,
    pub 3: uint32_t reserved0 :,
    pub linear_sub_window_copy_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_reg_write_data {
    pub reg_addr: u32,
    pub reg_data: u32,
    pub reg_write_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_pio_copy_data {
    pub src_lo: u32,
    pub src_hi: u32,
    pub dst_lo: u32,
    pub dst_hi: u32,
    pub 26: uint32_t byte_count :,
    pub 1: uint32_t src_loc :,
    pub 1: uint32_t dst_loc :,
    pub 1: uint32_t src_addr_inc :,
    pub 1: uint32_t dst_addr_inc :,
    pub 1: uint32_t overlap_disable :,
    pub 1: uint32_t constant_fill :,
    pub fields: },
    pub raw: u32,
    pub packet: },
    pub pio_copy_data: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_pio_constfill_data {
    pub dst_lo: u32,
    pub dst_hi: u32,
    pub 26: uint32_t byte_count :,
    pub 1: uint32_t src_loc :,
    pub 1: uint32_t dst_loc :,
    pub 1: uint32_t src_addr_inc :,
    pub 1: uint32_t dst_addr_inc :,
    pub 1: uint32_t overlap_disable :,
    pub 1: uint32_t constant_fill :,
    pub fields: },
    pub raw: u32,
    pub packet: },
    pub data: u32,
    pub pio_constfill_data: },
    pub all: [u32; 14],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_lsdma {
    pub header: dmub_cmd_header,
    pub lsdma_data: dmub_cmd_lsdma_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_optc_state_v2 {
    pub v_total_min: u32,
    pub v_total_max: u32,
    pub v_total_mid: u32,
    pub v_total_mid_frame_num: u32,
    pub program_manual_trigger: u8,
    pub tg_inst: u8,
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_optc_position {
    pub vpos: u32,
    pub hpos: u32,
    pub frame: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_fams2_drr_update {
    pub header: dmub_cmd_header,
    pub dmub_optc_state_req: dmub_optc_state_v2,
}

// HW and FW global configuration data for FAMS2
// FAMS2 types and structs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fams2_stream_type {
    FAMS2_STREAM_TYPE_NONE = 0,
    FAMS2_STREAM_TYPE_VBLANK = 1,
    FAMS2_STREAM_TYPE_VACTIVE = 2,
    FAMS2_STREAM_TYPE_DRR = 3,
    FAMS2_STREAM_TYPE_SUBVP = 4,
    FAMS2_STREAM_TYPE_ALTERNATE = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rect16 {
//
// Dirty rect x offset.
//
    pub x: u16,
//
// Dirty rect y offset.
//
    pub y: u16,
//
// Dirty rect width.
//
    pub width: u16,
//
// Dirty rect height.
//
    pub height: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plane_pipe_rect {
    pub luma: dmub_rect16,
    pub chroma: dmub_rect16,
}

//
// Structure to hold the LSDMA source / dest copy parameters.
// Each field is an array of [2][4]:
// [2] - Instance 0 is the copy for current frame, instance 1 is the copy for next frame (instance 1 potentially unused if no next)
// [4] - One instance per pipe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_outputs {
    pub pipe: uint16_t src_x[2][4]; // src x position for the copy. Array of [2][4] for curr vs. next and each,
    pub copy: uint16_t src_y[2][4]; // src y position for the,
    pub next): uint16_t dst_x[2][4]; // dst x position for the copy (can change for curr vs.,
    pub next): uint16_t dst_y[2][4]; // dst y position for the copy (can change for curr vs.,
    pub match): uint16_t width[2][4]; // src and dst width for the copy (src and dst must,
    pub match): uint16_t height[2][4]; // src and dst height for the copy (src and dst must,
    pub next): uint16_t dst_pitch[4]; // dst pitch for the copy (same for curr and,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_alternate_stream_dynamic_state {
    pub is_tick_in_allow: uint64_t earliest_init_tick; // track earliest possible init tick for calculating,
    pub yet: uint32_t otg_frame_pending_clear[3]; // In this context pending means prefetch has never been completed for this frame,
    pub flip_pending_clear_order: [u8; 3],
    pub num_pending_flips: u8,
    pub times: uint32_t prefetch_start_line_x1000[3]; // can compute from existing params, but store because we use this multiple,
    pub times: uint16_t prefetch_end_line[3]; // can compute from existing params, but store because we use this multiple,
    pub recout_y: [u16; 3],
    pub flip_pending: [u8; 3],
    pub copy_from_earliest: [u8; 3],
    pub lsdma_bandwidth_mbps: u16,
    pub vstartup_line: u16,
    pub vready_line: u16,
    pub array: uint8_t cursor_size[3]; // Cursor array per plane for now - if we assume a single cursor, then we don't need an,
    pub finalized: uint8_t pad; // to maintain alignment for below fields - re-arrange structure once all fields are,
// outputs:
    pub subvp_start_line_a: [u16; 3],
    pub subvp_height_a: [u16; 3],
    pub subvp_next_start_line_a: [u16; 3],
    pub subvp_next_height_a: [u16; 3],
    pub subvp_start_line_b: [u16; 3],
    pub subvp_height_b: [u16; 3],
    pub subvp_next_start_line_b: [u16; 3],
    pub subvp_next_height_b: [u16; 3],
    pub subvp_c_start_line_a: [u16; 3],
    pub subvp_c_height_a: [u16; 3],
    pub subvp_c_next_start_line_a: [u16; 3],
    pub subvp_c_next_height_a: [u16; 3],
    pub subvp_c_start_line_b: [u16; 3],
    pub subvp_c_height_b: [u16; 3],
    pub subvp_c_next_start_line_b: [u16; 3],
    pub subvp_c_next_height_b: [u16; 3],
    pub subvp_position: [u8; 3],
    pub finalized: uint8_t pad1[1]; // to maintain alignment for below fields - re-arrange structure once all fields are,
    pub program_go_line: u32,
    pub program_go_frame_count: u32,
    pub svp0_start_dst_line: u16,
    pub svp0_end_dst_line: u16,
    pub svp1_start_dst_line: u16,
    pub svp1_end_dst_line: u16,
    pub SVP1: lsdma_outputs lsdma[2]; // [2] - instance per SVP0 and,
    pub SVP1: lsdma_outputs lsdma_c[2]; // [2] - instance per SVP0 and,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_alternate_stream_static_state {
    pub total_bytes_to_copy: u32,
    pub stream: uint16_t svp0_dst_lines; // per,
    pub stream: uint16_t svp1_dst_lines; // per,
    pub planes: uint16_t min_lead_dst_lines; // per stream, should be max(nominal_req_limit, vstartup_to_vactive). Does not have to be maxed over all,
    pub planes: uint16_t svp_req_limit; // per stream, should be the same value in time between all streams max(2 swaths, dst_y_pre) over all,
    pub fw_delays: u16,
    pub vstartup_start: u16,
    pub rec_height: [u16; 3],
    pub viewport_start: [u16; 3],
    pub 270): uint16_t viewport_size[3]; // for now size will be the number of lines perpendicular to scan direction (height for 0 / 180, width for 90 and,
    pub viewport_start_c: [u16; 3],
    pub viewport_size_c: [u16; 3],
    pub surface_pitch: [u16; 3],
    pub surface_pitch_c: [u16; 3],
    pub surface_height: [u16; 3],
    pub surface_height_c: [u16; 3],
    pub element_size: [u8; 3],
    pub element_size_c: [u8; 3],
    pub command: uint8_t swizzle_mode[3]; // TODO: Add mapping, should be value used in LSDMA,
    pub line): uint8_t vready_offset_lines; // vready offset from vstartup in lines (rounded up, as the actual offset may be a fraction of a,
    pub dst_y_prefetch_x1000: [u16; 3],
    pub total_swaths: [u16; 3],
    pub total_swaths_c: [u16; 3],
    pub prefetch_swaths: [u8; 3],
    pub prefetch_swaths_c: [u8; 3],
    pub swath_height: [u8; 3],
    pub swath_height_c: [u8; 3],
    pub block_256b_width: [u16; 3],
    pub block_256b_height: [u16; 3],
    pub block_256b_width_c: [u16; 3],
    pub block_256b_height_c: [u16; 3],
    pub macro_tile_width: [u16; 3],
    pub macro_tile_width_c: [u16; 3],
    pub 1: uint8_t is_multi_planar :,
    pub 1: uint8_t is_yuv420 :,
    pub 1: uint8_t prefetch_relative_vblank :,
    pub rotation: uint8_t vertical_access : 1; // vertical_access = 1 means 90 or 270,
    pub 2160): uint8_t access_direction : 1; // access_direction = 1 means bigger to smaller coordinations (e.g., scan from 2160 to 0 as opposed to regular 0 to,
    pub 1: uint8_t dcc :,
    pub TMZ): uint8_t tmz : 1; // TODO: Need to assign outside of DML (DML not aware of,
    pub bits: },
    pub all: u8,
    pub config: [}; 3],
    pub max_cursor_size: u8,
    pub pre_hdl_delta_x1000: [u16; 3],
    pub pre_hdl_delta_c_x1000: [u16; 3],
    pub rec_hdl_delta_x1000: [u16; 3],
    pub rec_hdl_delta_c_x1000: [u16; 3],
    pub dst_y_per_vm_vblank_x1000: [u16; 3],
    pub dst_y_per_row_vblank_x1000: [u16; 3],
    pub dst_y_after_scaler: [u16; 3],
    pub vinit_prefill: [u16; 3],
    pub vinit_prefill_c: [u16; 3],
    pub vratio_x1000: [u16; 3],
    pub vratio_c_x1000: [u16; 3],
    pub pipe_viewports: [plane_pipe_rect; 4],
// TODO - remove these deprecated vars
    pub pipes: uint32_t pipe_copy_offset[2][4]; // [2] - SVP0/1, [4] - 4,
    pub pipe_copy_offset_c: [u32; 2][4],
// bits 47:16 of the surface address
    pub pipes: uint32_t pipe_copy_addr_47_16[2][4]; // [2] - SVP0/1, [4] - 4,
    pub pipe_copy_addr_47_16_c: [u32; 2][4],
    pub pipe_copy_max_size: [u32; 2][4],
    pub pipe_copy_max_size_c: [u32; 2][4],
}

// static stream state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_legacy_stream_static_state {
    pub vactive_det_fill_delay_otg_vlines: u8,
    pub programming_delay_otg_vlines: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_subvp_stream_static_state {
    pub vratio_numerator: u16,
    pub vratio_denominator: u16,
    pub phantom_vtotal: u16,
    pub phantom_vactive: u16,
    pub 1: uint8_t is_multi_planar :,
    pub 1: uint8_t is_yuv420 :,
    pub bits: },
    pub all: u8,
    pub config: },
    pub programming_delay_otg_vlines: u8,
    pub prefetch_to_mall_otg_vlines: u8,
    pub phantom_otg_inst: u8,
    pub phantom_pipe_mask: u8,
    pub passthrough): uint8_t phantom_plane_pipe_masks[DMUB_MAX_PHANTOM_PLANES]; // phantom pipe mask per plane (for flip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_drr_stream_static_state {
    pub nom_stretched_vtotal: u16,
    pub programming_delay_otg_vlines: u8,
    pub only_stretch_if_required: u8,
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_legacy_stream_static_state {
    pub vactive_det_fill_delay_otg_vlines: u16,
    pub programming_delay_otg_vlines: u16,
    pub disallow_time_us: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_subvp_stream_static_state {
    pub vratio_numerator: u16,
    pub vratio_denominator: u16,
    pub phantom_vtotal: u16,
    pub phantom_vactive: u16,
    pub programming_delay_otg_vlines: u16,
    pub prefetch_to_mall_otg_vlines: u16,
    pub 1: uint8_t is_multi_planar :,
    pub 1: uint8_t is_yuv420 :,
    pub bits: },
    pub all: u8,
    pub config: },
    pub phantom_otg_inst: u8,
    pub phantom_pipe_mask: u8,
    pub pad0: u8,
    pub passthrough): uint8_t phantom_plane_pipe_masks[DMUB_MAX_PHANTOM_PLANES]; // phantom pipe mask per plane (for flip,
    pub 4)]: uint8_t pad1[4 - (DMUB_MAX_PHANTOM_PLANES %,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_drr_stream_static_state {
    pub nom_stretched_vtotal: u16,
    pub programming_delay_otg_vlines: u16,
    pub only_stretch_if_required: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_stream_static_sub_state {
    pub legacy: dmub_fams2_legacy_stream_static_state,
    pub subvp: dmub_fams2_subvp_stream_static_state,
    pub drr: dmub_fams2_drr_stream_static_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_cmd_stream_static_sub_state {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_stream_static_sub_state_v2 {
    pub alternate: dmub_fams2_cmd_alternate_stream_static_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_stream_static_state {
    pub type: fams2_stream_type,
    pub otg_vline_time_ns: u32,
    pub otg_vline_time_ticks: u32,
    pub htotal: u16,
    pub vtotal: uint16_t vtotal; // nominal,
    pub vblank_start: u16,
    pub vblank_end: u16,
    pub max_vtotal: u16,
    pub allow_start_otg_vline: u16,
    pub allow_end_otg_vline: u16,
    pub changed: uint16_t drr_keepout_otg_vline; // after this vline, vtotal cannot be,
    pub start: uint8_t scheduling_delay_otg_vlines; // min time to budget for ready to microschedule,
    pub execution: uint8_t contention_delay_otg_vlines; // time to budget for contention on,
    pub firing: uint8_t vline_int_ack_delay_otg_vlines; // min time to budget for vertical interrupt,
    pub vline: uint8_t allow_to_target_delay_otg_vlines; // time from allow vline to target,
    pub enabled: uint8_t is_drr : 1; // stream is DRR,
    pub nominal: uint8_t clamp_vtotal_min : 1; // clamp vtotal to min instead of,
    pub blank: uint8_t min_ttu_vblank_usable : 1; // if min ttu vblank is above wm, no force pstate is needed in,
    pub bits: },
    pub all: u8,
    pub config: },
    pub otg_inst: u8,
    pub config: uint8_t pipe_mask; // pipe mask for the whole,
    pub num_planes: u8,
    pub passthrough): uint8_t plane_pipe_masks[DMUB_MAX_PLANES]; // pipe mask per plane (for flip,
    pub 4)]: uint8_t pad[4 - (DMUB_MAX_PLANES %,
    pub sub_state: dmub_fams2_stream_static_sub_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_cmd_stream_static_base_state {
    pub type: fams2_stream_type,
    pub otg_vline_time_ns: u32,
    pub otg_vline_time_ticks: u32,
    pub htotal: u16,
    pub vtotal: uint16_t vtotal; // nominal,
    pub vblank_start: u16,
    pub vblank_end: u16,
    pub max_vtotal: u16,
    pub allow_start_otg_vline: u16,
    pub allow_end_otg_vline: u16,
    pub changed: uint16_t drr_keepout_otg_vline; // after this vline, vtotal cannot be,
    pub start: uint16_t scheduling_delay_otg_vlines; // min time to budget for ready to microschedule,
    pub execution: uint16_t contention_delay_otg_vlines; // time to budget for contention on,
    pub firing: uint16_t vline_int_ack_delay_otg_vlines; // min time to budget for vertical interrupt,
    pub vline: uint16_t allow_to_target_delay_otg_vlines; // time from allow vline to target,
    pub enabled: uint8_t is_drr : 1; // stream is DRR,
    pub nominal: uint8_t clamp_vtotal_min : 1; // clamp vtotal to min instead of,
    pub blank: uint8_t min_ttu_vblank_usable : 1; // if min ttu vblank is above wm, no force pstate is needed in,
    pub bits: },
    pub all: u8,
    pub config: },
    pub otg_inst: u8,
    pub config: uint8_t pipe_mask; // pipe mask for the whole,
    pub num_planes: u8,
    pub passthrough): uint8_t plane_pipe_masks[DMUB_MAX_PLANES]; // pipe mask per plane (for flip,
    pub 4)]: uint8_t pad[4 - (DMUB_MAX_PLANES %,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_stream_static_state_v1 {
    pub base: dmub_fams2_cmd_stream_static_base_state,
    pub sub_state: dmub_fams2_stream_static_sub_state_v2,
}

//
// enum dmub_fams2_allow_delay_check_mode - macroscheduler mode for breaking on excessive
// p-state request to allow latency
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_fams2_allow_delay_check_mode {
// No check for request to allow delay
    FAMS2_ALLOW_DELAY_CHECK_NONE = 0,
// Check for request to allow delay
    FAMS2_ALLOW_DELAY_CHECK_FROM_START = 1,
// Check for prepare to allow delay
    FAMS2_ALLOW_DELAY_CHECK_FROM_PREPARE = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_fams2_global_feature_config {
    pub 1: uint32_t enable:,
    pub 1: uint32_t enable_ppt_check:,
    pub 1: uint32_t enable_stall_recovery:,
    pub 1: uint32_t enable_debug:,
    pub 1: uint32_t enable_offload_flip:,
    pub 1: uint32_t enable_visual_confirm:,
    pub 2: uint32_t allow_delay_check_mode:,
    pub 1: uint32_t legacy_method_no_fams2 :,
    pub 1: uint32_t alternate_channel_workaround :,
    pub 22: uint32_t reserved:,
    pub bits: },
    pub all: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fams2_global_config {
    pub begin: uint32_t max_allow_delay_us; // max delay to assert allow from uclk change,
    pub lock: uint32_t lock_wait_time_us; // time to forecast acquisition of,
    pub num_streams: u32,
    pub features: dmub_fams2_global_feature_config,
    pub recovery_timeout_us: u32,
    pub hwfq_flip_programming_delay_us: u32,
    pub target: uint32_t max_allow_to_target_delta_us; // how early DCN could assert P-State allow compared to the P-State,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_fams2_config {
    pub global: dmub_cmd_fams2_global_config,
    pub //v0: dmub_fams2_stream_static_state stream;,
    pub base: dmub_fams2_cmd_stream_static_base_state,
    pub sub_state: dmub_fams2_cmd_stream_static_sub_state,
    pub //v1: } stream_v1;,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fams2_config_v2 {
    pub global: dmub_cmd_fams2_global_config,
    pub //v1: dmub_fams2_stream_static_state_v1 stream_v1[DMUB_MAX_STREAMS];,
}

//
// DMUB rb command definition for FAMS2 (merged SubVP, FPO, Legacy)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_fams2 {
    pub header: dmub_cmd_header,
    pub config: dmub_cmd_fams2_config,
}

//
// Indirect buffer descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_ib_data {
    pub memory: dmub_addr src; // location of indirect buffer in,
    pub bytes: uint16_t size; // indirect buffer size in,
}

//
// DMUB rb command definition for commands passed over indirect buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_ib {
    pub header: dmub_cmd_header,
    pub ib_data: dmub_ib_data,
}

//
// enum dmub_cmd_idle_opt_type - Idle optimization command type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_idle_opt_type {
//
// DCN hardware restore.
//
    DMUB_CMD__IDLE_OPT_DCN_RESTORE = 0,

//
// DCN hardware save.
//
    DMUB_CMD__IDLE_OPT_DCN_SAVE_INIT = 1,

//
// DCN hardware notify idle.
//
    DMUB_CMD__IDLE_OPT_DCN_NOTIFY_IDLE = 2,

//
// DCN hardware notify power state.
//
    DMUB_CMD__IDLE_OPT_SET_DC_POWER_STATE = 3,

//
// DCN notify to release HW.
//
    DMUB_CMD__IDLE_OPT_RELEASE_HW = 4,
}

//
// struct dmub_rb_cmd_idle_opt_dcn_restore - DCN restore command data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_idle_opt_dcn_restore {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
}

//
// struct dmub_dcn_notify_idle_cntl_data - Data passed to FW in a DMUB_CMD__IDLE_OPT_DCN_NOTIFY_IDLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_dcn_notify_idle_cntl_data {
    pub driver_idle: u8,
    pub skip_otg_disable: u8,
    pub reserved: [u8; 58],
}

//
// struct dmub_rb_cmd_idle_opt_dcn_notify_idle - Data passed to FW in a DMUB_CMD__IDLE_OPT_DCN_NOTIFY_IDLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_idle_opt_dcn_notify_idle {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub cntl_data: dmub_dcn_notify_idle_cntl_data,
}

//
// enum dmub_idle_opt_dc_power_state - DC power states.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_idle_opt_dc_power_state {
    DMUB_IDLE_OPT_DC_POWER_STATE_UNKNOWN = 0,
    DMUB_IDLE_OPT_DC_POWER_STATE_D0 = 1,
    DMUB_IDLE_OPT_DC_POWER_STATE_D1 = 2,
    DMUB_IDLE_OPT_DC_POWER_STATE_D2 = 4,
    DMUB_IDLE_OPT_DC_POWER_STATE_D3 = 8,
}

//
// struct dmub_idle_opt_set_dc_power_state_data - Data passed to FW in a DMUB_CMD__IDLE_OPT_SET_DC_POWER_STATE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_idle_opt_set_dc_power_state_data {
    pub /: *mut *mut *mut uint8_t power_state; /< power state,
    pub /: *mut *mut *mut uint8_t pad[3]; /< padding,
}

//
// struct dmub_rb_cmd_idle_opt_set_dc_power_state - Data passed to FW in a DMUB_CMD__IDLE_OPT_SET_DC_POWER_STATE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_idle_opt_set_dc_power_state {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub data: dmub_idle_opt_set_dc_power_state_data,
}

//
// struct dmub_clocks - Clock update notification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_clocks {
    pub /: *mut *mut *mut uint32_t dispclk_khz; /< dispclk kHz,
    pub /: *mut *mut *mut uint32_t dppclk_khz; /< dppclk kHz,
    pub /: *mut *mut *mut uint32_t dcfclk_khz; /< dcfclk kHz,
    pub /: *mut *mut *mut uint32_t dcfclk_deep_sleep_khz; /< dcfclk deep sleep kHz,
}

//
// enum dmub_cmd_clk_mgr_type - Clock manager commands.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_clk_mgr_type {
//
// Notify DMCUB of clock update.
//
    DMUB_CMD__CLK_MGR_NOTIFY_CLOCKS = 0,
}

//
// struct dmub_rb_cmd_clk_mgr_notify_clocks - Clock update notification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_clk_mgr_notify_clocks {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_clocks clocks; /< clock data,
}

//
// struct dmub_cmd_digx_encoder_control_data - Encoder control data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_digx_encoder_control_data {
    pub /: *mut *mut *mut dig_encoder_control_parameters_v1_5 dig; /< payload,
}

//
// struct dmub_rb_cmd_digx_encoder_control - Encoder control command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_digx_encoder_control {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_cmd_digx_encoder_control_data encoder_control; /< payload,
}

//
// struct dmub_cmd_set_pixel_clock_data - Set pixel clock data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_set_pixel_clock_data {
    pub /: *mut *mut *mut set_pixel_clock_parameter_v1_7 clk; /< payload,
}

//
// struct dmub_cmd_set_pixel_clock_data - Set pixel clock command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_set_pixel_clock {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_cmd_set_pixel_clock_data pixel_clock; /< payload,
}

//
// struct dmub_cmd_enable_disp_power_gating_data - Display power gating.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_enable_disp_power_gating_data {
    pub /: *mut *mut *mut enable_disp_power_gating_parameters_v2_1 pwr; /< payload,
}

//
// struct dmub_rb_cmd_enable_disp_power_gating - Display power command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_enable_disp_power_gating {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_cmd_enable_disp_power_gating_data power_gating; /< payload,
}

//
// struct dmub_dig_transmitter_control_data_v1_7 - Transmitter control.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_dig_transmitter_control_data_v1_7 {
    pub /: *mut *mut *mut uint8_t phyid; /< 0=UNIPHYA, 1=UNIPHYB, 2=UNIPHYC, 3=UNIPHYD, 4=UNIPHYE, 5=UNIPHYF,
    pub /: *mut *mut *mut uint8_t action; /< Defined as ATOM_TRANSMITER_ACTION_xxx,
    pub /: *mut *mut *mut uint8_t digmode; /< enum atom_encode_mode_def,
    pub /: *mut *mut *mut uint8_t dplaneset; /< DP voltage swing and pre-emphasis value, "DP_LANE_SET__xDB_y_zV",
    pub /: *mut *mut *mut uint8_t txffe; /< TxFFE settings for HDMI 2.1,
    pub mode_laneset: },
    pub /: *mut *mut *mut uint8_t lanenum; /< Number of lanes,
    pub /: *mut *mut *mut uint32_t symclk_10khz; /< Symbol Clock in 10Khz,
    pub /: *mut *mut *mut uint32_t symclk_Hz; /< Symbol clock in Hz for FRL,
    pub symclk_units: },
    pub /: *mut *mut *mut uint8_t hpdsel; /< =1: HPD1, =2: HPD2, ..., =6: HPD6, =0: HPD is not assigned,
    pub /: *mut *mut *mut uint8_t digfe_sel; /< DIG front-end selection, bit0 means DIG0 FE is enabled,
    pub /: *mut *mut *mut uint8_t connobj_id; /< Connector Object Id defined in ObjectId.h,
    pub /: *mut *mut *mut uint8_t HPO_instance; /< HPO instance (0: inst0, 1: inst1),
    pub [3:0]: *mut *mut *mut uint8_t TxFFELaneSel; /< TxFFE lane select,
    pub skip_phy_ssc_reduction: u8,
    pub /: *mut *mut *mut uint8_t reserved2[2]; /< For future use,
    pub /: *mut *mut *mut uint32_t reserved3[11]; /< For future use,
}

//
// union dmub_cmd_dig1_transmitter_control_data - Transmitter control data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_dig1_transmitter_control_data {
    pub /: *mut *mut *mut dig_transmitter_control_parameters_v1_6 dig; /< payload,
    pub /: *mut *mut *mut dmub_dig_transmitter_control_data_v1_7 dig_v1_7; /< payload 1.7,
}

//
// struct dmub_rb_cmd_dig1_transmitter_control - Transmitter control command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dig1_transmitter_control {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_cmd_dig1_transmitter_control_data transmitter_control; /< payload,
}

//
// struct dmub_rb_cmd_domain_control_data - Data for DOMAIN power control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_domain_control_data {
    pub /: *mut *mut *mut uint8_t inst : 6; /< DOMAIN instance to control,
    pub /: *mut *mut *mut uint8_t power_gate : 1; /< 1=power gate, 0=power up,
    pub /: *mut *mut *mut uint8_t reserved[3]; /< Reserved for future use,
}

//
// struct dmub_rb_cmd_domain_control - Controls DOMAIN power gating
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_domain_control {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_rb_cmd_domain_control_data data; /< payload,
}

//
// DPIA tunnel command parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_dig_dpia_control_data {
    pub /: *mut *mut *mut uint8_t enc_id; / 0 = ENGINE_ID_DIGA, ...,
    pub /: *mut *mut *mut uint8_t action; / ATOM_TRANSMITER_ACTION_DISABLE/ENABLE/SETUP_VSEMPH,
    pub /: *mut *mut *mut uint8_t digmode; / enum atom_encode_mode_def,
    pub /: *mut *mut *mut uint8_t dplaneset; / DP voltage swing and pre-emphasis value,
    pub mode_laneset: },
    pub /: *mut *mut *mut uint8_t lanenum; / Lane number 1, 2, 4, 8,
    pub /: *mut *mut *mut uint32_t symclk_10khz; / Symbol Clock in 10Khz,
    pub /: *mut *mut *mut uint8_t hpdsel; / =0: HPD is not assigned,
    pub /: *mut *mut *mut uint8_t digfe_sel; / DIG stream( front-end ) selection, bit0 - DIG0 FE,
    pub /: *mut *mut *mut uint8_t dpia_id; / Index of DPIA,
    pub 1: uint8_t fec_rdy :,
    pub 7: uint8_t reserved :,
    pub reserved1: u32,
}

//
// DMUB command for DPIA tunnel control.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dig1_dpia_control {
    pub header: dmub_cmd_header,
    pub dpia_control: dmub_cmd_dig_dpia_control_data,
}

//
// SET_CONFIG Command Payload (deprecated)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_config_cmd_payload {
    pub /: *mut *mut uint8_t msg_type; / set config message type,
    pub /: *mut *mut uint8_t msg_data; / set config message data,
}

//
// Data passed from driver to FW in a DMUB_CMD__DPIA_SET_CONFIG_ACCESS command. (deprecated)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_set_config_control_data {
    pub cmd_pkt: set_config_cmd_payload,
    pub /: *mut *mut uint8_t instance; / DPIA instance,
    pub /: *mut *mut uint8_t immed_status; / Immediate status returned in case of error,
}

//
// SET_CONFIG Request Command Payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_config_request_cmd_payload {
    pub /: *mut *mut uint8_t instance; / DPIA instance,
    pub /: *mut *mut uint8_t immed_status; / Immediate status returned in case of error,
    pub /: *mut *mut uint8_t msg_type; / set config message type,
    pub reserved: u8,
    pub /: *mut *mut uint32_t msg_data; / set config message data,
}

//
// DMUB command structure for SET_CONFIG command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_set_config_access {
    pub /: *mut *mut dmub_cmd_header header; / header,
    pub /: *mut *mut dmub_cmd_set_config_control_data set_config_control; / set config data,
}

//
// DMUB command structure for SET_CONFIG request command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_set_config_request {
    pub /: *mut *mut dmub_cmd_header header; / header,
    pub /: *mut *mut set_config_request_cmd_payload payload; / set config request payload,
}

//
// Data passed from driver to FW in a DMUB_CMD__DPIA_MST_ALLOC_SLOTS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_mst_alloc_slots_control_data {
    pub /: *mut *mut uint8_t mst_alloc_slots; / mst slots to be allotted,
    pub /: *mut *mut uint8_t instance; / DPIA instance,
    pub /: *mut *mut uint8_t immed_status; / Immediate status returned as there is no outbox msg posted,
    pub /: *mut *mut uint8_t mst_slots_in_use; / returns slots in use for error cases,
}

//
// DMUB command structure for SET_ command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_set_mst_alloc_slots {
    pub /: *mut *mut dmub_cmd_header header; / header,
    pub /: *mut *mut dmub_cmd_mst_alloc_slots_control_data mst_slots_control; / mst slots control,
}

//
// Data passed from driver to FW in a DMUB_CMD__SET_TPS_NOTIFICATION command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_tps_notification_data {
    pub /: *mut *mut uint8_t instance; / DPIA instance,
    pub /: *mut *mut uint8_t tps; / requested training pattern,
    pub reserved1: u8,
    pub reserved2: u8,
}

//
// DMUB command structure for SET_TPS_NOTIFICATION command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_set_tps_notification {
    pub /: *mut *mut dmub_cmd_header header; / header,
    pub /: *mut *mut dmub_cmd_tps_notification_data tps_notification; / set tps_notification data,
}

//
// DMUB command structure for DPIA HPD int enable control.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dpia_hpd_int_enable {
    pub /: *mut *mut dmub_cmd_header header; / header,
    pub /: *mut *mut uint32_t enable; / dpia hpd interrupt enable,
}

//
// struct dmub_rb_cmd_dpphy_init - DPPHY init.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dpphy_init {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut uint8_t reserved[60]; /< reserved bits,
}

//
// enum dp_aux_request_action - DP AUX request command listing.
//
// 4 AUX request command bits are shifted to high nibble.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_aux_request_action {
// I2C-over-AUX write request
    DP_AUX_REQ_ACTION_I2C_WRITE		= 0x00,
// I2C-over-AUX read request
    DP_AUX_REQ_ACTION_I2C_READ		= 0x10,
// I2C-over-AUX write status request
    DP_AUX_REQ_ACTION_I2C_STATUS_REQ	= 0x20,
// I2C-over-AUX write request with MOT=1
    DP_AUX_REQ_ACTION_I2C_WRITE_MOT		= 0x40,
// I2C-over-AUX read request with MOT=1
    DP_AUX_REQ_ACTION_I2C_READ_MOT		= 0x50,
// I2C-over-AUX write status request with MOT=1
    DP_AUX_REQ_ACTION_I2C_STATUS_REQ_MOT	= 0x60,
// Native AUX write request
    DP_AUX_REQ_ACTION_DPCD_WRITE		= 0x80,
// Native AUX read request
    DP_AUX_REQ_ACTION_DPCD_READ		= 0x90
}

//
// enum aux_return_code_type - DP AUX process return code listing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_return_code_type {
// AUX process succeeded
    AUX_RET_SUCCESS = 0,
// AUX process failed with unknown reason
    AUX_RET_ERROR_UNKNOWN,
// AUX process completed with invalid reply
    AUX_RET_ERROR_INVALID_REPLY,
// AUX process timed out
    AUX_RET_ERROR_TIMEOUT,
// HPD was low during AUX process
    AUX_RET_ERROR_HPD_DISCON,
// Failed to acquire AUX engine
    AUX_RET_ERROR_ENGINE_ACQUIRE,
// AUX request not supported
    AUX_RET_ERROR_INVALID_OPERATION,
// AUX process not available
    AUX_RET_ERROR_PROTOCOL_ERROR,
}

//
// enum aux_channel_type - DP AUX channel type listing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_channel_type {
// AUX thru Legacy DP AUX
    AUX_CHANNEL_LEGACY_DDC,
// AUX thru DPIA DP tunneling
    AUX_CHANNEL_DPIA
}

//
// struct aux_transaction_parameters - DP AUX request transaction data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_transaction_parameters {
    pub /: *mut *mut *mut uint8_t is_i2c_over_aux; /< 0=native AUX, 1=I2C-over-AUX,
    pub /: *mut *mut *mut uint8_t action; /< enum dp_aux_request_action,
    pub /: *mut *mut *mut uint8_t length; /< DP AUX request data length,
    pub /: *mut *mut *mut uint8_t reserved; /< For future use,
    pub /: *mut *mut *mut uint32_t address; /< DP AUX address,
    pub /: *mut *mut *mut uint8_t data[16]; /< DP AUX write data,
}

//
// Data passed from driver to FW in a DMUB_CMD__DP_AUX_ACCESS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_dp_aux_control_data {
    pub /: *mut *mut *mut uint8_t instance; /< AUX instance or DPIA instance,
    pub /: *mut *mut *mut uint8_t manual_acq_rel_enable; /< manual control for acquiring or releasing AUX channel,
    pub /: *mut *mut *mut uint8_t sw_crc_enabled; /< Use software CRC for tunneling packet instead of hardware CRC,
    pub /: *mut *mut *mut uint8_t reserved0; /< For future use,
    pub /: *mut *mut *mut uint16_t timeout; /< timeout time in us,
    pub /: *mut *mut *mut uint16_t reserved1; /< For future use,
    pub /: *mut *mut *mut aux_channel_type type; /< aux_channel_type,
    pub /: *mut *mut *mut aux_transaction_parameters dpaux; /< aux_transaction_parameters,
}

//
// Definition of a DMUB_CMD__DP_AUX_ACCESS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dp_aux_access {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__DP_AUX_ACCESS command.
//
    pub aux_control: dmub_cmd_dp_aux_control_data,
}

//
// Definition of a DMUB_CMD__OUTBOX1_ENABLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_outbox1_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// enable: 0x0 -> disable outbox1 notification (default value)
// 0x1 -> enable outbox1 notification
//
    pub enable: u32,
}

// DP AUX Reply command - OutBox Cmd
//
// Data passed to driver from FW in a DMUB_OUT_CMD__DP_AUX_REPLY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_reply_data {
//
// Aux cmd
//
    pub command: u8,
//
// Aux reply data length (max: 16 bytes)
//
    pub length: u8,
//
// Alignment only
//
    pub pad: [u8; 2],
//
// Aux reply data
//
    pub data: [u8; 16],
}

//
// Control Data passed to driver from FW in a DMUB_OUT_CMD__DP_AUX_REPLY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aux_reply_control_data {
//
// Reserved for future use
//
    pub handle: u32,
//
// Aux Instance
//
    pub instance: u8,
//
// Aux transaction result: definition in enum aux_return_code_type
//
    pub result: u8,
//
// Alignment only
//
    pub pad: u16,
}

//
// Definition of a DMUB_OUT_CMD__DP_AUX_REPLY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dp_aux_reply {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Control Data passed to driver from FW in a DMUB_OUT_CMD__DP_AUX_REPLY command.
//
    pub control: aux_reply_control_data,
//
// Data passed to driver from FW in a DMUB_OUT_CMD__DP_AUX_REPLY command.
//
    pub reply_data: aux_reply_data,
}

// DP HPD Notify command - OutBox Cmd
//
// DP HPD Type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_hpd_type {
//
// Normal DP HPD
//
    DP_HPD = 0,
//
// DP HPD short pulse
//
    DP_IRQ = 1,
//
// Failure to acquire DP HPD state
//
    DP_NONE_HPD = 2
}

//
// DP HPD Status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_hpd_status {
//
// DP_HPD status low
//
    DP_HPD_UNPLUG = 0,
//
// DP_HPD status high
//
    DP_HPD_PLUG
}

//
// Data passed to driver from FW in a DMUB_OUT_CMD__DP_HPD_NOTIFY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_hpd_data {
//
// DP HPD instance
//
    pub instance: u8,
//
// HPD type
//
    pub hpd_type: u8,
//
// HPD status: only for type: DP_HPD to indicate status
//
    pub hpd_status: u8,
//
// Alignment only
//
    pub pad: u8,
}

//
// Definition of a DMUB_OUT_CMD__DP_HPD_NOTIFY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dp_hpd_notify {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed to driver from FW in a DMUB_OUT_CMD__DP_HPD_NOTIFY command.
//
    pub hpd_data: dp_hpd_data,
}

//
// Definition of a SET_CONFIG reply from DPOA.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum set_config_status {
    SET_CONFIG_PENDING = 0,
    SET_CONFIG_ACK_RECEIVED,
    SET_CONFIG_RX_TIMEOUT,
    SET_CONFIG_UNKNOWN_ERROR,
}

//
// Definition of a set_config reply
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_config_reply_control_data {
    pub /: *mut *mut uint8_t instance; / DPIA Instance,
    pub /: *mut *mut uint8_t status; / Set Config reply,
    pub /: *mut *mut uint16_t pad; / Alignment,
}

//
// Definition of a DMUB_OUT_CMD__SET_CONFIG_REPLY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dp_set_config_reply {
    pub header: dmub_cmd_header,
    pub set_config_reply_control: set_config_reply_control_data,
}

//
// Definition of a DPIA notification header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpia_notification_header {
    pub /: *mut *mut *mut uint8_t instance; /< DPIA Instance,
    pub reserved: [u8; 3],
    pub /: *mut *mut *mut dmub_cmd_dpia_notification_type type; /< DPIA notification type,
}

//
// Definition of the common data struct of DPIA notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpia_notification_common {
    pub dpia_notification_header)]: - sizeof(struct,
}

//
// Definition of a DPIA notification data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpia_bw_allocation_notify_data {
    pub /: *mut *mut *mut uint16_t cm_bw_alloc_support: 1; /< USB4 CM BW Allocation mode support,
    pub /: *mut *mut *mut uint16_t bw_request_failed: 1; /< BW_Request_Failed,
    pub /: *mut *mut *mut uint16_t bw_request_succeeded: 1; /< BW_Request_Succeeded,
    pub /: *mut *mut *mut uint16_t est_bw_changed: 1; /< Estimated_BW changed,
    pub /: *mut *mut *mut uint16_t bw_alloc_cap_changed: 1; /< BW_Allocation_Capabiity_Changed,
    pub /: *mut *mut *mut uint16_t reserved: 11; /< Reserved,
    pub bits: },
    pub flags: u16,
}

//
// union dpia_notify_data_type - DPIA Notification in Outbox command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpia_notification_data {
//
// DPIA Notification for common data struct
//
    pub common_data: dpia_notification_common,
//
// DPIA Notification for DP BW Allocation support
//
    pub dpia_bw_alloc: dpia_bw_allocation_notify_data,
}

//
// Definition of a DPIA notification payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpia_notification_payload {
    pub header: dpia_notification_header,
    pub /: *mut *mut *mut dpia_notification_data data; /< DPIA notification payload data,
}

//
// Definition of a DMUB_OUT_CMD__DPIA_NOTIFICATION command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_dpia_notification {
    pub /: *mut *mut *mut dmub_cmd_header header; /< DPIA notification header,
    pub /: *mut *mut *mut dpia_notification_payload payload; /< DPIA notification payload,
}

//
// Data passed from driver to FW in a DMUB_CMD__QUERY_HPD_STATE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_hpd_state_query_data {
    pub /: *mut *mut *mut uint8_t instance; /< HPD instance or DPIA instance,
    pub /: *mut *mut *mut uint8_t result; /< For returning HPD state,
    pub /: *mut *mut *mut uint16_t pad; / < Alignment,
    pub /: *mut *mut *mut aux_channel_type ch_type; /< aux_channel_type,
    pub /: *mut *mut *mut aux_return_code_type status; /< for returning the status of command,
}

//
// Definition of a DMUB_CMD__QUERY_HPD_STATE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_query_hpd_state {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__QUERY_HPD_STATE command.
//
    pub data: dmub_cmd_hpd_state_query_data,
}

//
// struct dmub_rb_cmd_hpd_sense_notify - HPD sense notification data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_hpd_sense_notify_data {
    pub /: *mut *mut *mut uint32_t old_hpd_sense_mask; /< Old HPD sense mask,
    pub /: *mut *mut *mut uint32_t new_hpd_sense_mask; /< New HPD sense mask,
}

//
// struct dmub_rb_cmd_hpd_sense_notify - DMUB_OUT_CMD__HPD_SENSE_NOTIFY command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_hpd_sense_notify {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_rb_cmd_hpd_sense_notify_data data; /< payload,
}

//
// Command IDs should be treated as stable ABI.
// Do not reuse or modify IDs.
//
// PSR command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_psr_type {
//
// Set PSR version support.
//
    DMUB_CMD__PSR_SET_VERSION		= 0,
//
// Copy driver-calculated parameters to PSR state.
//
    DMUB_CMD__PSR_COPY_SETTINGS		= 1,
//
// Enable PSR.
//
    DMUB_CMD__PSR_ENABLE			= 2,

//
// Disable PSR.
//
    DMUB_CMD__PSR_DISABLE			= 3,

//
// Set PSR level.
// PSR level is a 16-bit value dicated by driver that
// will enable/disable different functionality.
//
    DMUB_CMD__PSR_SET_LEVEL			= 4,

//
// Forces PSR enabled until an explicit PSR disable call.
//
    DMUB_CMD__PSR_FORCE_STATIC		= 5,
//
// Set vtotal in psr active for FreeSync PSR.
//
    DMUB_CMD__SET_SINK_VTOTAL_IN_PSR_ACTIVE = 6,
//
// Set PSR power option
//
    DMUB_CMD__SET_PSR_POWER_OPT = 7,
}

//
// Different PSR residency modes.
// Different modes change the definition of PSR residency.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psr_residency_mode {
    PSR_RESIDENCY_MODE_PHY = 0,
    PSR_RESIDENCY_MODE_ALPM,
    PSR_RESIDENCY_MODE_ENABLEMENT_PERIOD,
// Do not add below.
    PSR_RESIDENCY_MODE_LAST_ELEMENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_fams_type {
    DMUB_CMD__FAMS_SETUP_FW_CTRL	= 0,
    DMUB_CMD__FAMS_DRR_UPDATE		= 1,
    DMUB_CMD__HANDLE_SUBVP_CMD	= 2, // specifically for SubVP cmd
//
// For SubVP set manual trigger in FW because it
// triggers DRR_UPDATE_PENDING which SubVP relies
// on (for any SubVP cases that use a DRR display)
//
    DMUB_CMD__FAMS_SET_MANUAL_TRIGGER = 3,
    DMUB_CMD__FAMS2_CONFIG = 4,
    DMUB_CMD__FAMS2_DRR_UPDATE = 5,
    DMUB_CMD__FAMS2_FLIP = 6,
    DMUB_CMD__FAMS2_IB_CONFIG = 7,
    DMUB_CMD__FAMS2_IB_DEBUG_META = 8,
}

//
// PSR versions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psr_version {
//
// PSR version 1.
//
    PSR_VERSION_1				= 0,
//
// Freesync PSR SU.
//
    PSR_VERSION_SU_1			= 1,
//
// PSR not supported.
//
    PSR_VERSION_UNSUPPORTED			= 0xFF,	// psr_version field is only 8 bits wide
}

//
// PHY Link rate for DP.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_link_rate {
//
// not supported.
//
    PHY_RATE_UNKNOWN = 0,
//
// Rate_1 (RBR)	- 1.62 Gbps/Lane
//
    PHY_RATE_162 = 1,
//
// Rate_2		- 2.16 Gbps/Lane
//
    PHY_RATE_216 = 2,
//
// Rate_3		- 2.43 Gbps/Lane
//
    PHY_RATE_243 = 3,
//
// Rate_4 (HBR)	- 2.70 Gbps/Lane
//
    PHY_RATE_270 = 4,
//
// Rate_5 (RBR2)- 3.24 Gbps/Lane
//
    PHY_RATE_324 = 5,
//
// Rate_6		- 4.32 Gbps/Lane
//
    PHY_RATE_432 = 6,
//
// Rate_7 (HBR2)- 5.40 Gbps/Lane
//
    PHY_RATE_540 = 7,
//
// Rate_8 (HBR3)- 8.10 Gbps/Lane
//
    PHY_RATE_810 = 8,
//
// UHBR10 - 10.0 Gbps/Lane
//
    PHY_RATE_1000 = 9,
//
// UHBR13.5 - 13.5 Gbps/Lane
//
    PHY_RATE_1350 = 10,
//
// UHBR10 - 20.0 Gbps/Lane
//
    PHY_RATE_2000 = 11,

    PHY_RATE_675 = 12,
//
// Rate 12 - 6.75 Gbps/Lane
//
}

//
// enum dmub_phy_fsm_state - PHY FSM states.
// PHY FSM state to transit to during PSR enable/disable.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_phy_fsm_state {
    DMUB_PHY_FSM_POWER_UP_DEFAULT = 0,
    DMUB_PHY_FSM_RESET,
    DMUB_PHY_FSM_RESET_RELEASED,
    DMUB_PHY_FSM_SRAM_LOAD_DONE,
    DMUB_PHY_FSM_INITIALIZED,
    DMUB_PHY_FSM_CALIBRATED,
    DMUB_PHY_FSM_CALIBRATED_LP,
    DMUB_PHY_FSM_CALIBRATED_PG,
    DMUB_PHY_FSM_POWER_DOWN,
    DMUB_PHY_FSM_PLL_EN,
    DMUB_PHY_FSM_TX_EN,
    DMUB_PHY_FSM_TX_EN_TEST_MODE,
    DMUB_PHY_FSM_FAST_LP,
    DMUB_PHY_FSM_P2_PLL_OFF_CPM,
    DMUB_PHY_FSM_P2_PLL_OFF_PG,
    DMUB_PHY_FSM_P2_PLL_OFF,
    DMUB_PHY_FSM_P2_PLL_ON,
}

//
// Data passed from driver to FW in a DMUB_CMD__PSR_COPY_SETTINGS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_psr_copy_settings_data {
//
// Flags that can be set by driver to change some PSR behaviour.
//
    pub debug: dmub_psr_debug_flags,
//
// 16-bit value dicated by driver that will enable/disable different functionality.
//
    pub psr_level: u16,
//
// DPP HW instance.
//
    pub dpp_inst: u8,
//
// MPCC HW instance.
// Not used in dmub fw,
// dmub fw will get active opp by reading odm registers.
//
    pub mpcc_inst: u8,
//
// OPP HW instance.
// Not used in dmub fw,
// dmub fw will get active opp by reading odm registers.
//
    pub opp_inst: u8,
//
// OTG HW instance.
//
    pub otg_inst: u8,
//
// DIG FE HW instance.
//
    pub digfe_inst: u8,
//
// DIG BE HW instance.
//
    pub digbe_inst: u8,
//
// DP PHY HW instance.
//
    pub dpphy_inst: u8,
//
// AUX HW instance.
//
    pub aux_inst: u8,
//
// Determines if SMU optimzations are enabled/disabled.
//
    pub smu_optimizations_en: u8,
//
// Unused.
// TODO: Remove.
//
    pub frame_delay: u8,
//
// If RFB setup time is greater than the total VBLANK time,
// it is not possible for the sink to capture the video frame
// in the same frame the SDP is sent. In this case,
// the frame capture indication bit should be set and an extra
// static frame should be transmitted to the sink.
//
    pub frame_cap_ind: u8,
//
// Granularity of Y offset supported by sink.
//
    pub su_y_granularity: u8,
//
// Indicates whether sink should start capturing
// immediately following active scan line,
// or starting with the 2nd active scan line.
//
    pub line_capture_indication: u8,
//
// Multi-display optimizations are implemented on certain ASICs.
//
    pub multi_disp_optimizations_en: u8,
//
// The last possible line SDP may be transmitted without violating
// the RFB setup time or entering the active video frame.
//
    pub init_sdp_deadline: u16,
//
// @ rate_control_caps : Indicate FreeSync PSR Sink Capabilities
//
    pub rate_control_caps: u8,
//
// Force PSRSU always doing full frame update
//
    pub force_ffu_mode: u8,
//
// Length of each horizontal line in us.
//
    pub line_time_in_us: u32,
//
// FEC enable status in driver
//
    pub fec_enable_status: u8,
//
// FEC re-enable delay when PSR exit.
// unit is 100us, range form 0~255(0xFF).
//
    pub fec_enable_delay_in100us: u8,
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// DSC enable status in driver
//
    pub dsc_enable_status: u8,
//
// Use FSM state for PSR power up/down
//
    pub use_phy_fsm: u8,
//
// frame delay for frame re-lock
//
    pub relock_delay_frame_cnt: u8,
//
// esd recovery indicate.
//
    pub esd_recovery: u8,
//
// DSC Slice height.
//
    pub dsc_slice_height: u16,
//
// Some panels request main link off before xth vertical line
//
    pub poweroff_before_vertical_line: u16,
//
// Some panels cannot handle idle pattern during PSR entry.
// To power down phy before disable stream to avoid sending
// idle pattern.
//
    pub power_down_phy_before_disable_stream: u8,
}

//
// Definition of a DMUB_CMD__PSR_COPY_SETTINGS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_copy_settings {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__PSR_COPY_SETTINGS command.
//
    pub psr_copy_settings_data: dmub_cmd_psr_copy_settings_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__PSR_SET_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_psr_set_level_data {
//
// 16-bit value dicated by driver that will enable/disable different functionality.
//
    pub psr_level: u16,
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
}

//
// Definition of a DMUB_CMD__PSR_SET_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_set_level {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__PSR_SET_LEVEL command.
//
    pub psr_set_level_data: dmub_cmd_psr_set_level_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_enable_data {
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Phy state to enter.
// Values to use are defined in dmub_phy_fsm_state
//
    pub phy_fsm_state: u8,
//
// Phy rate for DP - RBR/HBR/HBR2/HBR3.
// Set this using enum phy_link_rate.
// This does not support HDMI/DP2 for now.
//
    pub phy_rate: u8,
}

//
// Definition of a DMUB_CMD__PSR_ENABLE command.
// PSR enable/disable is controlled using the sub_type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
    pub data: dmub_rb_cmd_psr_enable_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__PSR_SET_VERSION command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_psr_set_version_data {
//
// PSR version that FW should implement.
//
    pub version: psr_version,
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__PSR_SET_VERSION command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_set_version {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__PSR_SET_VERSION command.
//
    pub psr_set_version_data: dmub_cmd_psr_set_version_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_psr_force_static_data {
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__PSR_FORCE_STATIC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_force_static {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__PSR_FORCE_STATIC command.
//
    pub psr_force_static_data: dmub_cmd_psr_force_static_data,
}

//
// PSR SU debug flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_psr_su_debug_flags {
//
// PSR SU debug flags.
//
// Update dirty rect in SW only.
//
    pub 1: uint8_t update_dirty_rect_only :,
//
// Reset the cursor/plane state before processing the call.
//
    pub 1: uint8_t reset_state :,
    pub bitfields: },
//
// Union for debug flags.
//
    pub u32All: u32,
}

//
// Data passed from driver to FW in a DMUB_CMD__UPDATE_DIRTY_RECT command.
// This triggers a selective update for PSR SU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_update_dirty_rect_data {
//
// Dirty rects from OS.
//
    pub src_dirty_rects: [dmub_rect; DMUB_MAX_DIRTY_RECTS],
//
// PSR SU debug flags.
//
    pub debug_flags: dmub_psr_su_debug_flags,
//
// Pipe index.
//
    pub pipe_idx: u8,
//
// Number of dirty rects.
//
    pub dirty_rect_count: u8,
//
// dirty rects cmd version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// OTG HW instance
//
    pub otg_inst: u8,
//
// Padding for 4 byte alignment
//
    pub padding: [u8; 3],
}

//
// Definition of a DMUB_CMD__UPDATE_DIRTY_RECT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_update_dirty_rect {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__UPDATE_DIRTY_RECT command.
//
    pub update_dirty_rect_data: dmub_cmd_update_dirty_rect_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__UPDATE_CURSOR_INFO command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_cursor_control_cfg {
    pub 1: uint32_t cur_enable:,
    pub 3: uint32_t reser0:,
    pub 1: uint32_t cur_2x_magnify:,
    pub 3: uint32_t reser1:,
    pub 3: uint32_t mode:,
    pub 5: uint32_t reser2:,
    pub 2: uint32_t pitch:,
    pub 6: uint32_t reser3:,
    pub 5: uint32_t line_per_chunk:,
    pub 3: uint32_t reser4:,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_position_cache_hubp {
    pub cur_ctl: dmub_reg_cursor_control_cfg,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_position_cfg {
    pub 16: uint32_t cur_x_pos:,
    pub 16: uint32_t cur_y_pos:,
    pub bits: },
    pub raw: u32,
    pub position: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_hot_spot_cfg {
    pub 16: uint32_t hot_x:,
    pub 16: uint32_t hot_y:,
    pub bits: },
    pub raw: u32,
    pub hot_spot: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_dst_offset_cfg {
    pub 13: uint32_t dst_x_offset:,
    pub 19: uint32_t reserved:,
    pub bits: },
    pub raw: u32,
    pub dst_offset: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_cur0_control_cfg {
    pub 1: uint32_t cur0_enable:,
    pub 1: uint32_t expansion_mode:,
    pub 1: uint32_t reser0:,
    pub 1: uint32_t cur0_rom_en:,
    pub 3: uint32_t mode:,
    pub 25: uint32_t reserved:,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_position_cache_dpp {
    pub cur0_ctl: dmub_reg_cur0_control_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_position_cfg {
    pub pHubp: dmub_cursor_position_cache_hubp,
    pub pDpp: dmub_cursor_position_cache_dpp,
    pub pipe_idx: u8,
//
// Padding is required. To be 4 Bytes Aligned.
//
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_attribute_cache_hubp {
    pub SURFACE_ADDR_HIGH: u32,
    pub SURFACE_ADDR: u32,
    pub cur_ctl: dmub_reg_cursor_control_cfg,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_cursor_size_cfg {
    pub 16: uint32_t width:,
    pub 16: uint32_t height:,
    pub bits: },
    pub raw: u32,
    pub size: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_reg_cursor_settings_cfg {
    pub 8: uint32_t dst_y_offset:,
    pub 2: uint32_t chunk_hdl_adjust:,
    pub 22: uint32_t reserved:,
    pub bits: },
    pub raw: u32,
    pub settings: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_attribute_cache_dpp {
    pub cur0_ctl: dmub_reg_cur0_control_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cursor_attributes_cfg {
    pub aHubp: dmub_cursor_attribute_cache_hubp,
    pub aDpp: dmub_cursor_attribute_cache_dpp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_update_cursor_payload0 {
//
// Cursor dirty rects.
//
    pub cursor_rect: dmub_rect,
//
// PSR SU debug flags.
//
    pub debug_flags: dmub_psr_su_debug_flags,
//
// Cursor enable/disable.
//
    pub enable: u8,
//
// Pipe index.
//
    pub pipe_idx: u8,
//
// Cursor update cmd version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Cursor Position Register.
// Registers contains Hubp & Dpp modules
//
    pub position_cfg: dmub_cursor_position_cfg,
//
// OTG HW instance
//
    pub otg_inst: u8,
//
// Padding for 4 byte alignment
//
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_update_cursor_payload1 {
    pub attribute_cfg: dmub_cursor_attributes_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_update_cursor_info_data {
    pub payload0: dmub_cmd_update_cursor_payload0,
    pub payload1: dmub_cmd_update_cursor_payload1,
}

//
// Definition of a DMUB_CMD__UPDATE_CURSOR_INFO command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_update_cursor_info {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__UPDATE_CURSOR_INFO command.
//
    pub update_cursor_info_data: dmub_cmd_update_cursor_info_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__SET_SINK_VTOTAL_IN_PSR_ACTIVE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_psr_set_vtotal_data {
//
// 16-bit value dicated by driver that indicates the vtotal in PSR active requirement when screen idle..
//
    pub psr_vtotal_idle: u16,
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// 16-bit value dicated by driver that indicates the vtotal in PSR active requirement when doing SU/FFU.
//
    pub psr_vtotal_su: u16,
//
// Explicit padding to 4 byte boundary.
//
    pub pad2: [u8; 2],
}

//
// Definition of a DMUB_CMD__SET_SINK_VTOTAL_IN_PSR_ACTIVE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_set_vtotal {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__SET_SINK_VTOTAL_IN_PSR_ACTIVE command.
//
    pub psr_set_vtotal_data: dmub_cmd_psr_set_vtotal_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__SET_PSR_POWER_OPT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_psr_set_power_opt_data {
//
// PSR control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel instance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
//
// PSR power option
//
    pub power_opt: u32,
}

//
// Definition of a DMUB_CMD__SET_PSR_POWER_OPT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_psr_set_power_opt {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__SET_PSR_POWER_OPT command.
//
    pub psr_set_power_opt_data: dmub_cmd_psr_set_power_opt_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_alpm_mode {
    ALPM_AUXWAKE = 0,
    ALPM_AUXLESS = 1,
    ALPM_UNSUPPORTED = 2,
}

//
// Definition of Replay Residency GPINT command.
// Bit[0] - Residency mode for Revision 0
// Bit[1] - Enable/Disable state
// Bit[2-3] - Revision number
// Bit[4-7] - Residency mode for Revision 1
// Bit[8] - Panel instance
// Bit[9-15] - Reserved
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pr_residency_mode {
    PR_RESIDENCY_MODE_PHY = 0x0,
    PR_RESIDENCY_MODE_ALPM,
    PR_RESIDENCY_MODE_IPS2,
    PR_RESIDENCY_MODE_FRAME_CNT,
    PR_RESIDENCY_MODE_ENABLEMENT_PERIOD,
}

//
// Definition of a replay_state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_state {
    REPLAY_STATE_0			= 0x0,
    REPLAY_STATE_1			= 0x10,
    REPLAY_STATE_1A			= 0x11,
    REPLAY_STATE_2			= 0x20,
    REPLAY_STATE_2A			= 0x21,
    REPLAY_STATE_3			= 0x30,
    REPLAY_STATE_3INIT		= 0x31,
    REPLAY_STATE_4			= 0x40,
    REPLAY_STATE_4A			= 0x41,
    REPLAY_STATE_4B			= 0x42,
    REPLAY_STATE_4C			= 0x43,
    REPLAY_STATE_4D			= 0x44,
    REPLAY_STATE_4E			= 0x45,
    REPLAY_STATE_4B_LOCKED		= 0x4A,
    REPLAY_STATE_4C_UNLOCKED	= 0x4B,
    REPLAY_STATE_5			= 0x50,
    REPLAY_STATE_5A			= 0x51,
    REPLAY_STATE_5B			= 0x52,
    REPLAY_STATE_5A_LOCKED		= 0x5A,
    REPLAY_STATE_5B_UNLOCKED	= 0x5B,
    REPLAY_STATE_6			= 0x60,
    REPLAY_STATE_6A			= 0x61,
    REPLAY_STATE_6B			= 0x62,
    REPLAY_STATE_INVALID		= 0xFF,
}

//
// Definition of a panel replay state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pr_state {
    PR_STATE_0									= 0x00, // State 0 steady state
// Pending SDP and Unlock before back to State 0
    PR_STATE_0_PENDING_SDP_AND_UNLOCK			= 0x01,
    PR_STATE_1									= 0x10, // State 1
    PR_STATE_2									= 0x20, // State 2 steady state
// Pending frame transmission before transition to State 2
    PR_STATE_2_PENDING_FRAME_TRANSMISSION		= 0x30,
// Active and Powered Up
    PR_STATE_2_POWERED							= 0x31,
// Active and Powered Down, but need to blank HUBP after DPG_EN latch
    PR_STATE_2_PENDING_HUBP_BLANK				= 0x32,
// Active and Pending Power Up
    PR_STATE_2_PENDING_POWER_UP					= 0x33,
// Active and Powered Up, Pending DPG latch
    PR_STATE_2_PENDING_LOCK	= 0x34,
// Active and Powered Up, Pending SDP and Unlock
    PR_STATE_2_PENDING_SDP_AND_UNLOCK			= 0x35,
// Pending transmission of AS SDP for timing sync, but no rfb update
    PR_STATE_2_PENDING_AS_SDP					= 0x36,
// Invalid
    PR_STATE_INVALID							= 0xFF,
}

//
// Replay command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_replay_type {
//
// Copy driver-calculated parameters to REPLAY state.
//
    DMUB_CMD__REPLAY_COPY_SETTINGS		= 0,
//
// Enable REPLAY.
//
    DMUB_CMD__REPLAY_ENABLE			= 1,
//
// Set Replay power option.
//
    DMUB_CMD__SET_REPLAY_POWER_OPT		= 2,
//
// Set coasting vtotal.
//
    DMUB_CMD__REPLAY_SET_COASTING_VTOTAL	= 3,
//
// Set power opt and coasting vtotal.
//
    DMUB_CMD__REPLAY_SET_POWER_OPT_AND_COASTING_VTOTAL	= 4,
//
// Set disabled iiming sync.
//
    DMUB_CMD__REPLAY_SET_TIMING_SYNC_SUPPORTED	= 5,
//
// Set Residency Frameupdate Timer.
//
    DMUB_CMD__REPLAY_SET_RESIDENCY_FRAMEUPDATE_TIMER = 6,
//
// Set pseudo vtotal
//
    DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL = 7,
//
// Set adaptive sync sdp enabled
//
    DMUB_CMD__REPLAY_DISABLED_ADAPTIVE_SYNC_SDP = 8,
//
// Set Replay General command.
//
    DMUB_CMD__REPLAY_SET_GENERAL_CMD = 16,
}

//
// Panel Polarity sub-types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_panel_polarity_type {
    DMUB_CMD__PANEL_POLARITY_ENABLE = 0,
    DMUB_CMD__PANEL_POLARITY_GET_BIAS = 1,
    DMUB_CMD__PANEL_POLARITY_RESET = 2,
}

//
// Panel Replay sub-types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_panel_replay_type {
    DMUB_CMD__PR_ENABLE = 0,
    DMUB_CMD__PR_COPY_SETTINGS = 1,
    DMUB_CMD__PR_UPDATE_STATE = 2,
    DMUB_CMD__PR_GENERAL_CMD = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_panel_replay_state_update_subtype {
    PR_STATE_UPDATE_COASTING_VTOTAL = 0x1,
    PR_STATE_UPDATE_SYNC_MODE = 0x2,
    PR_STATE_UPDATE_RUNTIME_FLAGS = 0x3,
    PR_STATE_UPDATE_PSEUDO_VTOTAL = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_panel_replay_general_subtype {
    PR_GENERAL_CMD_DEBUG_OPTION = 0x1,
}

//
// Replay general command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_replay_general_subtype {
    REPLAY_GENERAL_CMD_NOT_SUPPORTED = -1,
//
// TODO: For backward compatible, allow new command only.
// REPLAY_GENERAL_CMD_SET_TIMING_SYNC_SUPPORTED,
// REPLAY_GENERAL_CMD_SET_RESIDENCY_FRAMEUPDATE_TIMER,
// REPLAY_GENERAL_CMD_SET_PSEUDO_VTOTAL,
//
    REPLAY_GENERAL_CMD_DISABLED_ADAPTIVE_SYNC_SDP,
    REPLAY_GENERAL_CMD_DISABLED_DESYNC_ERROR_DETECTION,
    REPLAY_GENERAL_CMD_UPDATE_ERROR_STATUS,
    REPLAY_GENERAL_CMD_SET_LOW_RR_ACTIVATE,
    REPLAY_GENERAL_CMD_VIDEO_CONFERENCING,
    REPLAY_GENERAL_CMD_SET_CONTINUOUSLY_RESYNC,
    REPLAY_GENERAL_CMD_SET_COASTING_VTOTAL_WITHOUT_FRAME_UPDATE,
    REPLAY_GENERAL_CMD_LIVE_CAPTURE_WITH_CVT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_alpm_auxless_data {
    pub lfps_setup_ns: u16,
    pub lfps_period_ns: u16,
    pub lfps_silence_ns: u16,
    pub lfps_t1_t2_override_us: u16,
    pub lfps_t1_t2_offset_us: c_short,
    pub lttpr_count: u8,
//
// Padding to align structure to 4 byte boundary.
//
    pub pad: [u8; 1],
}

//
// Data passed from driver to FW in a DMUB_CMD__REPLAY_COPY_SETTINGS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_copy_settings_data {
//
// Flags that can be set by driver to change some replay behaviour.
//
    pub debug: replay_debug_flags,
//
// @flags: Flags used to determine feature functionality.
//
    pub flags: replay_hw_flags,
//
// DPP HW instance.
//
    pub dpp_inst: u8,
//
// OTG HW instance.
//
    pub otg_inst: u8,
//
// DIG FE HW instance.
//
    pub digfe_inst: u8,
//
// DIG BE HW instance.
//
    pub digbe_inst: u8,
//
// AUX HW instance.
//
    pub aux_inst: u8,
//
// Panel Instance.
// Panel isntance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// @pixel_deviation_per_line: Indicate the maximum pixel deviation per line compare
// to Source timing when Sink maintains coasting vtotal during the Replay normal sleep mode
//
    pub pixel_deviation_per_line: u8,
//
// @max_deviation_line: The max number of deviation line that can keep the timing
// synchronized between the Source and Sink during Replay normal sleep mode.
//
    pub max_deviation_line: u8,
//
// Length of each horizontal line in ns.
//
    pub line_time_in_ns: u32,
//
// PHY instance.
//
    pub dpphy_inst: u8,
//
// Determines if SMU optimzations are enabled/disabled.
//
    pub smu_optimizations_en: u8,
//
// Determines if timing sync are enabled/disabled.
//
    pub replay_timing_sync_supported: u8,
//
// Use FSM state for Replay power up/down
//
    pub use_phy_fsm: u8,
//
// Use for AUX-less ALPM LFPS wake operation
//
    pub auxless_alpm_data: dmub_alpm_auxless_data,
//
// @hpo_stream_enc_inst: HPO stream encoder instance
//
    pub hpo_stream_enc_inst: u8,
//
// @hpo_link_enc_inst: HPO link encoder instance
//
    pub hpo_link_enc_inst: u8,
//
// Determines if fast resync in ultra sleep mode is enabled/disabled.
//
    pub replay_support_fast_resync_in_ultra_sleep_mode: u8,
//
// @pad: Align structure to 4 byte boundary.
//
    pub pad: [u8; 1],
}

//
// Replay versions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_version {
//
// FreeSync Replay
//
    REPLAY_VERSION_FREESYNC_REPLAY	= 0,
//
// Panel Replay
//
    REPLAY_VERSION_PANEL_REPLAY		= 1,
//
// Replay not supported.
//
    REPLAY_VERSION_UNSUPPORTED		= 0xFF,
}

//
// Definition of a DMUB_CMD__REPLAY_COPY_SETTINGS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_copy_settings {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__REPLAY_COPY_SETTINGS command.
//
    pub replay_copy_settings_data: dmub_cmd_replay_copy_settings_data,
}

//
// Replay disable / enable state for dmub_rb_cmd_replay_enable_data.enable
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_enable {
//
// Disable REPLAY.
//
    REPLAY_DISABLE				= 0,
//
// Enable REPLAY.
//
    REPLAY_ENABLE				= 1,
}

//
// Data passed from driver to FW in a DMUB_CMD__SMART_POWER_OLED_ENABLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_smart_power_oled_enable_data {
//
// SMART_POWER_OLED enable or disable.
//
    pub enable: u8,
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
    pub peak_nits: u16,
//
// OTG HW instance.
//
    pub otg_inst: u8,
//
// DIG FE HW instance.
//
    pub digfe_inst: u8,
//
// DIG BE HW instance.
//
    pub digbe_inst: u8,
    pub debugcontrol: u8,
//
// vertical interrupt trigger line
//
    pub triggerline: u32,
    pub fixed_max_cll: u16,
    pub pad: [u8; 2],
}

//
// Data passed from driver to FW in a DMUB_CMD__REPLAY_ENABLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_enable_data {
//
// Replay enable or disable.
//
    pub enable: u8,
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Phy state to enter.
// Values to use are defined in dmub_phy_fsm_state
//
    pub phy_fsm_state: u8,
//
// Phy rate for DP - RBR/HBR/HBR2/HBR3.
// Set this using enum phy_link_rate.
// This does not support HDMI/DP2 for now.
//
    pub phy_rate: u8,
//
// @hpo_stream_enc_inst: HPO stream encoder instance
//
    pub hpo_stream_enc_inst: u8,
//
// @hpo_link_enc_inst: HPO link encoder instance
//
    pub hpo_link_enc_inst: u8,
//
// @pad: Align structure to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__REPLAY_ENABLE command.
// Replay enable/disable is controlled using action in data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
    pub data: dmub_rb_cmd_replay_enable_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__SET_REPLAY_POWER_OPT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_set_power_opt_data {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 3],
//
// REPLAY power option
//
    pub power_opt: u32,
}

//
// Data passed from driver to FW in a DMUB_CMD__REPLAY_SET_TIMING_SYNC_SUPPORTED command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_set_timing_sync_data {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// REPLAY set_timing_sync
//
    pub timing_sync_supported: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Data passed from driver to FW in a DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_set_pseudo_vtotal {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Source Vtotal that Replay + IPS + ABM full screen video src vtotal
//
    pub vtotal: u16,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_disabled_adaptive_sync_sdp_data {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// enabled: set adaptive sync sdp enabled
//
    pub force_disabled: u8,
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_set_general_cmd_data {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// subtype: replay general cmd sub type
//
    pub subtype: u8,
    pub pad: [u8; 2],
//
// config data with param1 and param2
//
    pub param1: u32,
    pub param2: u32,
}

//
// Definition of a DMUB_CMD__SET_REPLAY_POWER_OPT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_power_opt {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__SET_REPLAY_POWER_OPT command.
//
    pub replay_set_power_opt_data: dmub_cmd_replay_set_power_opt_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__REPLAY_SET_COASTING_VTOTAL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_set_coasting_vtotal_data {
//
// 16-bit value dicated by driver that indicates the coasting vtotal.
//
    pub coasting_vtotal: u16,
//
// REPLAY control version.
//
    pub cmd_version: u8,
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// 16-bit value dicated by driver that indicates the coasting vtotal high byte part.
//
    pub coasting_vtotal_high: u16,
//
// frame skip number.
//
    pub frame_skip_number: u16,
}

//
// Definition of a DMUB_CMD__REPLAY_SET_COASTING_VTOTAL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_coasting_vtotal {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__REPLAY_SET_COASTING_VTOTAL command.
//
    pub replay_set_coasting_vtotal_data: dmub_cmd_replay_set_coasting_vtotal_data,
}

//
// Definition of a DMUB_CMD__REPLAY_SET_POWER_OPT_AND_COASTING_VTOTAL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_power_opt_and_coasting_vtotal {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__SET_REPLAY_POWER_OPT command.
//
    pub replay_set_power_opt_data: dmub_cmd_replay_set_power_opt_data,
//
// Definition of a DMUB_CMD__REPLAY_SET_COASTING_VTOTAL command.
//
    pub replay_set_coasting_vtotal_data: dmub_cmd_replay_set_coasting_vtotal_data,
}

//
// Definition of a DMUB_CMD__REPLAY_SET_TIMING_SYNC_SUPPORTED command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_timing_sync {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of DMUB_CMD__REPLAY_SET_TIMING_SYNC_SUPPORTED command.
//
    pub replay_set_timing_sync_data: dmub_cmd_replay_set_timing_sync_data,
}

//
// Definition of a DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_pseudo_vtotal {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL command.
//
    pub data: dmub_cmd_replay_set_pseudo_vtotal,
}

//
// Definition of a DMUB_CMD__REPLAY_DISABLED_ADAPTIVE_SYNC_SDP command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_disabled_adaptive_sync_sdp {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of DMUB_CMD__REPLAY_DISABLED_ADAPTIVE_SYNC_SDP command.
//
    pub data: dmub_cmd_replay_disabled_adaptive_sync_sdp_data,
}

//
// Definition of a DMUB_CMD__REPLAY_SET_GENERAL_CMD command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_general_cmd {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of DMUB_CMD__REPLAY_SET_GENERAL_CMD command.
//
    pub data: dmub_cmd_replay_set_general_cmd_data,
}

//
// Data passed from driver to FW in  DMUB_CMD__REPLAY_SET_RESIDENCY_FRAMEUPDATE_TIMER command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_replay_frameupdate_timer_data {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Replay Frameupdate Timer Enable or not
//
    pub enable: u8,
//
// REPLAY force reflash frame update number
//
    pub frameupdate_count: u16,
}

//
// Definition of DMUB_CMD__REPLAY_SET_RESIDENCY_FRAMEUPDATE_TIMER
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_replay_set_frameupdate_timer {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Definition of a DMUB_CMD__SET_REPLAY_POWER_OPT command.
//
    pub data: dmub_cmd_replay_frameupdate_timer_data,
}

//
// Definition union of replay command set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_replay_cmd_set {
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Definition of DMUB_CMD__REPLAY_SET_TIMING_SYNC_SUPPORTED command data.
//
    pub sync_data: dmub_cmd_replay_set_timing_sync_data,
//
// Definition of DMUB_CMD__REPLAY_SET_RESIDENCY_FRAMEUPDATE_TIMER command data.
//
    pub timer_data: dmub_cmd_replay_frameupdate_timer_data,
//
// Definition of DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL command data.
//
    pub pseudo_vtotal_data: dmub_cmd_replay_set_pseudo_vtotal,
//
// Definition of DMUB_CMD__REPLAY_DISABLED_ADAPTIVE_SYNC_SDP command data.
//
    pub disabled_adaptive_sync_sdp_data: dmub_cmd_replay_disabled_adaptive_sync_sdp_data,
//
// Definition of DMUB_CMD__REPLAY_SET_GENERAL_CMD command data.
//
    pub set_general_cmd_data: dmub_cmd_replay_set_general_cmd_data,
}

//
// IHC command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_ihc_type {
//
// Set DIG HDCP interrupt destination.
//
    DMUB_CMD__IHC_SET_DIG_HDCP_INTERRUPT_DEST = 0,
}

//
// Data passed from driver to FW in a DMUB_CMD__IHC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_ihc_data {
//
// DIG engine ID (0-3).
//
    pub dig_id: u8,
//
// 1 = route to DMU, 0 = route to CPU.
//
    pub 1: uint8_t to_dmu :,
//
// Reserved bits.
//
    pub 7: uint8_t reserved :,
//
// Padding.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__IHC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_ihc {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// IHC command data.
//
    pub data: dmub_cmd_ihc_data,
}

//
// SMART POWER OLED command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_smart_power_oled_type {

//
// Enable/Disable SMART_POWER_OLED.
//
    DMUB_CMD__SMART_POWER_OLED_ENABLE = 1,
//
// Get current MaxCLL value if SMART POWER OLED is enabled.
//
    DMUB_CMD__SMART_POWER_OLED_GETMAXCLL = 2,
}

//
// Definition of a DMUB_CMD__SMART_POWER_OLED command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_smart_power_oled_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
    pub data: dmub_rb_cmd_smart_power_oled_enable_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_smart_power_oled_getmaxcll_input {
    pub panel_inst: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_smart_power_oled_getmaxcll_output {
    pub current_max_cll: u16,
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__SMART_POWER_OLED command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_smart_power_oled_getmaxcll {
    pub /: *mut *mut *mut dmub_cmd_header header; /< Command header,
//
// Data passed from driver to FW in a DMUB_CMD__SMART_POWER_OLED_GETMAXCLL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_smart_power_oled_getmaxcll_data {
    pub /: *mut *mut *mut dmub_cmd_smart_power_oled_getmaxcll_input input; /< Input,
    pub /: *mut *mut *mut dmub_cmd_smart_power_oled_getmaxcll_output output; /< Output,
    pub /: *mut *mut *mut uint32_t output_raw; /< Raw data output,
    pub data: },
}

//
// Set of HW components that can be locked.
//
// Note: If updating with more HW components, fields
// in dmub_inbox0_cmd_lock_hw must be updated to match.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_hw_lock_flags {
//
// Set of HW components that can be locked.
//
// Lock/unlock OTG master update lock.
//
    pub 1: uint8_t lock_pipe :,
//
// Lock/unlock cursor.
//
    pub 1: uint8_t lock_cursor :,
//
// Lock/unlock global update lock.
//
    pub 1: uint8_t lock_dig :,
//
// Triple buffer lock requires additional hw programming to usual OTG master lock.
//
    pub 1: uint8_t triple_buffer_lock :,
    pub bits: },
//
// Union for HW Lock flags.
//
    pub u8All: u8,
}

//
// Instances of HW to be locked.
//
// Note: If updating with more HW components, fields
// in dmub_inbox0_cmd_lock_hw must be updated to match.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_hw_lock_inst_flags {
//
// OTG HW instance for OTG master update lock.
//
    pub otg_inst: u8,
//
// OPP instance for cursor lock.
//
    pub opp_inst: u8,
//
// OTG HW instance for global update lock.
// TODO: Remove, and re-use otg_inst.
//
    pub dig_inst: u8,
//
// Explicit pad to 4 byte boundary.
//
    pub pad: u8,
}

//
// Clients that can acquire the HW Lock Manager.
//
// Note: If updating with more clients, fields in
// dmub_inbox0_cmd_lock_hw must be updated to match.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hw_lock_client {
//
// Driver is the client of HW Lock Manager.
//
    HW_LOCK_CLIENT_DRIVER = 0,
//
// PSR SU is the client of HW Lock Manager.
//
    HW_LOCK_CLIENT_PSR_SU		= 1,
    HW_LOCK_CLIENT_SUBVP = 3,
//
// Replay is the client of HW Lock Manager.
//
    HW_LOCK_CLIENT_REPLAY		= 4,
    HW_LOCK_CLIENT_FAMS2 = 5,
    HW_LOCK_CLIENT_CURSOR_OFFLOAD = 6,
//
// Invalid client.
//
    HW_LOCK_CLIENT_INVALID = 0xFFFFFFFF,
}

//
// Data passed to HW Lock Mgr in a DMUB_CMD__HW_LOCK command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_lock_hw_data {
//
// Specifies the client accessing HW Lock Manager.
//
    pub client: hw_lock_client,
//
// HW instances to be locked.
//
    pub inst_flags: dmub_hw_lock_inst_flags,
//
// Which components to be locked.
//
    pub hw_locks: dmub_hw_lock_flags,
//
// Specifies lock/unlock.
//
    pub lock: u8,
//
// HW can be unlocked separately from releasing the HW Lock Mgr.
// This flag is set if the client wishes to release the object.
//
    pub should_release: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

//
// Definition of a DMUB_CMD__HW_LOCK command.
// Command is used by driver and FW.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_lock_hw {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed to HW Lock Mgr in a DMUB_CMD__HW_LOCK command.
//
    pub lock_hw_data: dmub_cmd_lock_hw_data,
}

//
// ABM command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_abm_type {
//
// Initialize parameters for ABM algorithm.
// Data is passed through an indirect buffer.
//
    DMUB_CMD__ABM_INIT_CONFIG	= 0,
//
// Set OTG and panel HW instance.
//
    DMUB_CMD__ABM_SET_PIPE		= 1,
//
// Set user requested backklight level.
//
    DMUB_CMD__ABM_SET_BACKLIGHT	= 2,
//
// Set ABM operating/aggression level.
//
    DMUB_CMD__ABM_SET_LEVEL		= 3,
//
// Set ambient light level.
//
    DMUB_CMD__ABM_SET_AMBIENT_LEVEL	= 4,
//
// Enable/disable fractional duty cycle for backlight PWM.
//
    DMUB_CMD__ABM_SET_PWM_FRAC	= 5,

//
// unregister vertical interrupt after steady state is reached
//
    DMUB_CMD__ABM_PAUSE	= 6,

//
// Save and Restore ABM state. On save we save parameters, and
// on restore we update state with passed in data.
//
    DMUB_CMD__ABM_SAVE_RESTORE	= 7,

//
// Query ABM caps.
//
    DMUB_CMD__ABM_QUERY_CAPS	= 8,

//
// Set ABM Events
//
    DMUB_CMD__ABM_SET_EVENT	= 9,

//
// Get the current ACE curve.
//
    DMUB_CMD__ABM_GET_ACE_CURVE = 10,

//
// Get current histogram data
//
    DMUB_CMD__ABM_GET_HISTOGRAM_DATA = 11,
}

//
// LSDMA command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_lsdma_type {
//
// Initialize parameters for LSDMA.
// Ring buffer is mapped to the ring buffer
//
    DMUB_CMD__LSDMA_INIT_CONFIG	= 0,
//
// LSDMA copies data from source to destination linearly
//
    DMUB_CMD__LSDMA_LINEAR_COPY = 1,
//
// LSDMA copies data from source to destination linearly in sub window
//
    DMUB_CMD__LSDMA_LINEAR_SUB_WINDOW_COPY = 2,
//
// Send the tiled-to-tiled copy command
//
    DMUB_CMD__LSDMA_TILED_TO_TILED_COPY = 3,
//
// Send the poll reg write command
//
    DMUB_CMD__LSDMA_POLL_REG_WRITE = 4,
//
// Send the pio copy command
//
    DMUB_CMD__LSDMA_PIO_COPY = 5,
//
// Send the pio constfill command
//
    DMUB_CMD__LSDMA_PIO_CONSTFILL = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abm_ace_curve {
//
// @offsets: ACE curve offsets.
//
    pub offsets: [u32; ABM_MAX_NUM_OF_ACE_SEGMENTS],
//
// @thresholds: ACE curve thresholds.
//
    pub thresholds: [u32; ABM_MAX_NUM_OF_ACE_SEGMENTS],
//
// @slopes: ACE curve slopes.
//
    pub slopes: [u32; ABM_MAX_NUM_OF_ACE_SEGMENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed_pt_format {
//
// @sign_bit: Indicates whether one bit is reserved for the sign.
//
    pub sign_bit: bool,
//
// @num_int_bits: Number of bits used for integer part.
//
    pub num_int_bits: u8,
//
// @num_frac_bits: Number of bits used for fractional part.
//
    pub num_frac_bits: u8,
//
// @pad: Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abm_caps {
//
// @num_hg_bins: Number of histogram bins.
//
    pub num_hg_bins: u8,
//
// @num_ace_segments: Number of ACE curve segments.
//
    pub num_ace_segments: u8,
//
// @pad: Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
//
// @ace_thresholds_format: Format of the ACE thresholds. If not programmable, it is set to 0.
//
    pub ace_thresholds_format: fixed_pt_format,
//
// @ace_offsets_format: Format of the ACE offsets. If not programmable, it is set to 0.
//
    pub ace_offsets_format: fixed_pt_format,
//
// @ace_slopes_format: Format of the ACE slopes.
//
    pub ace_slopes_format: fixed_pt_format,
}

//
// Parameters for ABM2.4 algorithm. Passed from driver to FW via an indirect buffer.
// Requirements:
// - Padded explicitly to 32-bit boundary.
// - Must ensure this structure matches the one on driver-side,
// otherwise it won't be aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abm_config_table {
//
// Gamma curve thresholds, used for crgb conversion.
//
    pub 0B: uint16_t crgb_thresh[NUM_POWER_FN_SEGS]; //,
//
// Gamma curve offsets, used for crgb conversion.
//
    pub 16B: uint16_t crgb_offset[NUM_POWER_FN_SEGS]; //,
//
// Gamma curve slopes, used for crgb conversion.
//
    pub 32B: uint16_t crgb_slope[NUM_POWER_FN_SEGS]; //,
//
// Custom backlight curve thresholds.
//
    pub 48B: uint16_t backlight_thresholds[NUM_BL_CURVE_SEGS]; //,
//
// Custom backlight curve offsets.
//
    pub 78B: uint16_t backlight_offsets[NUM_BL_CURVE_SEGS]; //,
//
// Ambient light thresholds.
//
    pub 112B: uint16_t ambient_thresholds_lux[NUM_AMBI_LEVEL]; //,
//
// Minimum programmable backlight.
//
    pub 122B: uint16_t min_abm_backlight; //,
//
// Minimum reduction values.
//
    pub 124B: uint8_t min_reduction[NUM_AMBI_LEVEL][NUM_AGGR_LEVEL]; //,
//
// Maximum reduction values.
//
    pub 144B: uint8_t max_reduction[NUM_AMBI_LEVEL][NUM_AGGR_LEVEL]; //,
//
// Bright positive gain.
//
    pub 164B: uint8_t bright_pos_gain[NUM_AMBI_LEVEL][NUM_AGGR_LEVEL]; //,
//
// Dark negative gain.
//
    pub 184B: uint8_t dark_pos_gain[NUM_AMBI_LEVEL][NUM_AGGR_LEVEL]; //,
//
// Hybrid factor.
//
    pub 204B: uint8_t hybrid_factor[NUM_AGGR_LEVEL]; //,
//
// Contrast factor.
//
    pub 208B: uint8_t contrast_factor[NUM_AGGR_LEVEL]; //,
//
// Deviation gain.
//
    pub 212B: uint8_t deviation_gain[NUM_AGGR_LEVEL]; //,
//
// Minimum knee.
//
    pub 216B: uint8_t min_knee[NUM_AGGR_LEVEL]; //,
//
// Maximum knee.
//
    pub 220B: uint8_t max_knee[NUM_AGGR_LEVEL]; //,
//
// Unused.
//
    pub 224B: uint8_t iir_curve[NUM_AMBI_LEVEL]; //,
//
// Explicit padding to 4 byte boundary.
//
    pub 229B: uint8_t pad3[3]; //,
//
// Backlight ramp reduction.
//
    pub 232B: uint16_t blRampReduction[NUM_AGGR_LEVEL]; //,
//
// Backlight ramp start.
//
    pub 240B: uint16_t blRampStart[NUM_AGGR_LEVEL]; //,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_PIPE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_set_pipe_data {
//
// OTG HW instance.
//
    pub otg_inst: u8,
//
// Panel Control HW instance.
//
    pub panel_inst: u8,
//
// Controls how ABM will interpret a set pipe or set level command.
//
    pub set_pipe_option: u8,
//
// Unused.
// TODO: Remove.
//
    pub ramping_boundary: u8,
//
// PwrSeq HW Instance.
//
    pub pwrseq_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 3],
}

//
// Definition of a DMUB_CMD__ABM_SET_PIPE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_set_pipe {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_PIPE command.
//
    pub abm_set_pipe_data: dmub_cmd_abm_set_pipe_data,
}

//
// Type of backlight control method to be used by ABM module
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_backlight_control_type {
//
// PWM Backlight control
//
    DMU_BACKLIGHT_CONTROL_PWM = 0,
//
// VESA Aux-based backlight control
//
    DMU_BACKLIGHT_CONTROL_VESA_AUX = 1,
//
// AMD DPCD Aux-based backlight control
//
    DMU_BACKLIGHT_CONTROL_AMD_AUX = 2,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_BACKLIGHT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_set_backlight_data {
//
// Number of frames to ramp to backlight user level.
//
    pub frame_ramp: u32,
//
// Requested backlight level from user.
//
    pub backlight_user_level: u32,
//
// ABM control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// AUX HW Instance.
//
    pub aux_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 1],
//
// Backlight control type.
// Value 0 is PWM backlight control.
// Value 1 is VAUX backlight control.
// Value 2 is AMD DPCD AUX backlight control.
//
    pub backlight_control_type: dmub_backlight_control_type,
//
// Minimum luminance in nits.
//
    pub min_luminance: u32,
//
// Maximum luminance in nits.
//
    pub max_luminance: u32,
//
// Minimum backlight in pwm.
//
    pub min_backlight_pwm: u32,
//
// Maximum backlight in pwm.
//
    pub max_backlight_pwm: u32,
}

//
// Definition of a DMUB_CMD__ABM_SET_BACKLIGHT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_set_backlight {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_BACKLIGHT command.
//
    pub abm_set_backlight_data: dmub_cmd_abm_set_backlight_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_set_level_data {
//
// Set current ABM operating/aggression level.
//
    pub level: u32,
//
// ABM control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__ABM_SET_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_set_level {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_LEVEL command.
//
    pub abm_set_level_data: dmub_cmd_abm_set_level_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_AMBIENT_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_set_ambient_level_data {
//
// Ambient light sensor reading from OS.
//
    pub ambient_lux: u32,
//
// ABM control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__ABM_SET_AMBIENT_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_set_ambient_level {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_AMBIENT_LEVEL command.
//
    pub abm_set_ambient_level_data: dmub_cmd_abm_set_ambient_level_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_PWM_FRAC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_set_pwm_frac_data {
//
// Enable/disable fractional duty cycle for backlight PWM.
// TODO: Convert to uint8_t.
//
    pub fractional_pwm: u32,
//
// ABM control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__ABM_SET_PWM_FRAC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_set_pwm_frac {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_PWM_FRAC command.
//
    pub abm_set_pwm_frac_data: dmub_cmd_abm_set_pwm_frac_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_INIT_CONFIG command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_init_config_data {
//
// Location of indirect buffer used to pass init data to ABM.
//
    pub src: dmub_addr,
//
// Indirect buffer length.
//
    pub bytes: u16,
//
// ABM control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__ABM_INIT_CONFIG command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_init_config {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_INIT_CONFIG command.
//
    pub abm_init_config_data: dmub_cmd_abm_init_config_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_PAUSE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_pause_data {
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// OTG hw instance
//
    pub otg_inst: u8,
//
// Enable or disable ABM pause
//
    pub enable: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 1],
}

//
// Definition of a DMUB_CMD__ABM_PAUSE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_pause {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_PAUSE command.
//
    pub abm_pause_data: dmub_cmd_abm_pause_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_QUERY_CAPS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_query_caps_in {
//
// Panel instance.
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 3],
}

//
// Data passed from FW to driver in a DMUB_CMD__ABM_QUERY_CAPS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_query_caps_out {
//
// SW Algorithm caps.
//
    pub sw_caps: abm_caps,
//
// ABM HW caps.
//
    pub hw_caps: abm_caps,
}

//
// Definition of a DMUB_CMD__ABM_QUERY_CAPS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_query_caps {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed between FW and driver in a DMUB_CMD__ABM_QUERY_CAPS command.
//
    pub abm_query_caps_in: dmub_cmd_abm_query_caps_in,
    pub abm_query_caps_out: dmub_cmd_abm_query_caps_out,
    pub data: },
}

//
// enum dmub_abm_ace_curve_type - ACE curve type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_abm_ace_curve_type {
//
// ACE curve as defined by the SW layer.
//
    ABM_ACE_CURVE_TYPE__SW = 0,
//
// ACE curve as defined by the SW to HW translation interface layer.
//
    ABM_ACE_CURVE_TYPE__SW_IF = 1,
}

//
// enum dmub_abm_histogram_type - Histogram type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_abm_histogram_type {
//
// ACE curve as defined by the SW layer.
//
    ABM_HISTOGRAM_TYPE__SW = 0,
//
// ACE curve as defined by the SW to HW translation interface layer.
//
    ABM_HISTOGRAM_TYPE__SW_IF = 1,
}

//
// Definition of a DMUB_CMD__ABM_GET_ACE_CURVE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_get_ace_curve {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Address where ACE curve should be copied.
//
    pub dest: dmub_addr,
//
// Type of ACE curve being queried.
//
    pub ace_type: dmub_abm_ace_curve_type,
//
// Indirect buffer length.
//
    pub bytes: u16,
//
// eDP panel instance.
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

//
// Definition of a DMUB_CMD__ABM_GET_HISTOGRAM command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_get_histogram {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Address where Histogram should be copied.
//
    pub dest: dmub_addr,
//
// Type of Histogram being queried.
//
    pub histogram_type: dmub_abm_histogram_type,
//
// Indirect buffer length.
//
    pub bytes: u16,
//
// eDP panel instance.
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

//
// Definition of a DMUB_CMD__ABM_SAVE_RESTORE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_save_restore {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// OTG hw instance
//
    pub otg_inst: u8,
//
// Enable or disable ABM pause
//
    pub freeze: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub debug: u8,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_INIT_CONFIG command.
//
    pub abm_init_config_data: dmub_cmd_abm_init_config_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_EVENT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_abm_set_event_data {
//
// VB Scaling Init. Strength Mapping
// Byte 0: 0~255 for VB level 0
// Byte 1: 0~255 for VB level 1
// Byte 2: 0~255 for VB level 2
// Byte 3: 0~255 for VB level 3
//
    pub vb_scaling_strength_mapping: u32,
//
// VariBright Scaling Enable
//
    pub vb_scaling_enable: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__ABM_SET_EVENT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_abm_set_event {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__ABM_SET_EVENT command.
//
    pub abm_set_event_data: dmub_cmd_abm_set_event_data,
}

//
// CACP command sub-types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_cacp_type {
//
// Initialize parameters for CACP algorithm.
// Data is passed through an indirect buffer.
//
    DMUB_CMD__CACP_INIT_CONFIG	= 0,
//
// Set OTG and panel HW instance.
//
    DMUB_CMD__CACP_SET_PIPE		= 1,
//
// Set CACP operating/aggression level.
//
    DMUB_CMD__CACP_SET_LEVEL	= 2,
//
// Set event: full-screen/video mode/Hdr mode
//
    DMUB_CMD__CACP_SET_EVENT	= 3,

//
// unregister vertical interrupt after steady state is reached
//
    DMUB_CMD__CACP_PAUSE	= 4,
//
// Set backlight: only for miniLED
//
    DMUB_CMD__CACP_SET_BACKLIGHT	= 5,
//
// Enable/disable fractional duty cycle for backlight PWM.
//
    DMUB_CMD__CACP_SET_PWM_FRAC	= 6,
//
// Get CACP Histogram
//
    DMUB_CMD__CACP_GET_HISTOGRAM = 7,
//
// Get ACE curve area for IGT test validation.
//
    DMUB_CMD__CACP_GET_ACE_CURVE_AREA = 8,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_PIPE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_set_pipe_data {
//
// OTG HW instance.
//
    pub otg_inst: u8,
//
// Panel Control HW instance.
//
    pub panel_inst: u8,
//
// Controls how CACP will interpret a set pipe or set level command.
//
    pub set_pipe_option: u8,
//
// PwrSeq HW Instance.
//
    pub pwrseq_inst: u8,
}

//
// Definition of a DMUB_CMD__CACP_SET_PIPE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_set_pipe {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_PIPE command.
//
    pub cacp_set_pipe_data: dmub_cmd_cacp_set_pipe_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_set_level_data {
//
// Set current cacp operating/aggression level.
//
    pub level: u32,
//
// CACP control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__CACP_SET_LEVEL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_set_level {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_LEVEL command.
//
    pub cacp_set_level_data: dmub_cmd_cacp_set_level_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_EVENT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_set_event_data {
//
// Full screen mode
//
    pub full_screen_mode: u8,
//
// VariBright Scaling Enable
//
    pub vb_scaling_enable: u8,
//
// HDR mode
//
    pub hdr_mode: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Transition time info
//
    pub trans_info: u32,
}

//
// Definition of a DMUB_CMD__CACP_SET_EVENT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_set_event {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_EVENT command.
//
    pub cacp_set_event_data: dmub_cmd_cacp_set_event_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_INIT_CONFIG command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_init_config_data {
//
// Location of indirect buffer used to pass init data to CACP.
//
    pub src: dmub_addr,
//
// Indirect buffer length.
//
    pub bytes: u16,
//
// CACP control version.
//
    pub mode: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// CACP visual_confirm debug
//
    pub visual_confirm: u32,
//
// CACP customized str_scl params
//
    pub strscl_valid: u8,
    pub pad: [u8; 3],
    pub strscl_sdr: [u8; 4],
    pub strscl_hdr: [u8; 4],
}

//
// Definition of a DMUB_CMD__CACP_INIT_CONFIG command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_init_config {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_INIT_CONFIG command.
//
    pub cacp_init_config_data: dmub_cmd_cacp_init_config_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_PAUSE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_pause_data {
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// OTG hw instance
//
    pub otg_inst: u8,
//
// Enable or disable CACP pause
//
    pub enable: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

//
// Definition of a DMUB_CMD__CACP_PAUSE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_pause {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_PAUSE command.
//
    pub cacp_pause_data: dmub_cmd_cacp_pause_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_BACKLIGHT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_set_backlight_data {
//
// Number of frames to ramp to backlight user level.
//
    pub frame_ramp: u32,
//
// Requested backlight level from user.
//
    pub backlight_user_level: u32,
//
// ABM control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// AUX HW Instance.
//
    pub aux_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 1],
//
// Backlight control type.
// Value 0 is PWM backlight control.
// Value 1 is VAUX backlight control.
// Value 2 is AMD DPCD AUX backlight control.
//
    pub backlight_control_type: dmub_backlight_control_type,
//
// Minimum luminance in nits.
//
    pub min_luminance: u32,
//
// Maximum luminance in nits.
//
    pub max_luminance: u32,
//
// Minimum backlight in pwm.
//
    pub min_backlight_pwm: u32,
//
// Maximum backlight in pwm.
//
    pub max_backlight_pwm: u32,
}

//
// Definition of a DMUB_CMD__CACP_SET_BACKLIGHT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_set_backlight {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_BACKLIGHT command.
//
    pub cacp_set_backlight_data: dmub_cmd_cacp_set_backlight_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_PWM_FRAC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_set_pwm_frac_data {
//
// Enable/disable fractional duty cycle for backlight PWM.
// TODO: Convert to uint8_t.
//
    pub fractional_pwm: u32,
//
// CACP control version.
//
    pub version: u8,
//
// Panel Control HW instance mask.
// Bit 0 is Panel Control HW instance 0.
// Bit 1 is Panel Control HW instance 1.
//
    pub panel_mask: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 2],
}

//
// Definition of a DMUB_CMD__CACP_SET_PWM_FRAC command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_set_pwm_frac {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__CACP_SET_PWM_FRAC command.
//
    pub cacp_set_pwm_frac_data: dmub_cmd_cacp_set_pwm_frac_data,
}

//
// Definition of a DMUB_CMD__CACP_GET_HISTOGRAM command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_get_histogram {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Address where Histogram should be copied.
//
    pub dest: dmub_addr,
//
// Type of Histogram being queried.
//
    pub histogram_type: dmub_abm_histogram_type,
//
// Indirect buffer length.
//
    pub bytes: u16,
//
// eDP panel instance.
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: u8,
}

//
// Data passed from driver to FW in a DMUB_CMD__CACP_GET_ACE_CURVE_AREA command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_get_ace_curve_area_in {
//
// eDP panel instance (0-based).
//
    pub panel_inst: u8,
//
// Explicit padding to 4 byte boundary.
//
    pub pad: [u8; 3],
}

//
// Data returned from FW in a DMUB_CMD__CACP_GET_ACE_CURVE_AREA command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cacp_get_ace_curve_area_out {
//
// Computed area under the ACE PWL curve (trapezoidal integration).
// Lower value = stronger dimming (higher CACP level).
// Directly comparable across levels.
//
    pub area: u32,
}

//
// Definition of a DMUB_CMD__CACP_GET_ACE_CURVE_AREA command.
// Uses inline response pattern: input and output share the data union.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cacp_get_ace_curve_area {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data union for input/output.
//
    pub in: dmub_cmd_cacp_get_ace_curve_area_in,
    pub out: dmub_cmd_cacp_get_ace_curve_area_out,
    pub data: },
}

//
// Data passed from driver to FW in a DMUB_CMD__QUERY_FEATURE_CAPS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_query_feature_caps_data {
//
// DMUB feature capabilities.
// After DMUB init, driver will query FW capabilities prior to enabling certain features.
//
    pub feature_caps: dmub_feature_caps,
}

//
// Definition of a DMUB_CMD__QUERY_FEATURE_CAPS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_query_feature_caps {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__QUERY_FEATURE_CAPS command.
//
    pub query_feature_caps_data: dmub_cmd_query_feature_caps_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__GET_VISUAL_CONFIRM_COLOR command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_visual_confirm_color_data {
//
// DMUB visual confirm color
//
    pub visual_confirm_color: dmub_visual_confirm_color,
}

//
// Definition of a DMUB_CMD__GET_VISUAL_CONFIRM_COLOR command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_get_visual_confirm_color {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__GET_VISUAL_CONFIRM_COLOR command.
//
    pub visual_confirm_color_data: dmub_cmd_visual_confirm_color_data,
}

//
// enum dmub_cmd_panel_cntl_type - Panel control command.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_panel_cntl_type {
//
// Initializes embedded panel hardware blocks.
//
    DMUB_CMD__PANEL_CNTL_HW_INIT = 0,
//
// Queries backlight info for the embedded panel.
//
    DMUB_CMD__PANEL_CNTL_QUERY_BACKLIGHT_INFO = 1,
//
// Sets the PWM Freq as per user's requirement.
//
    DMUB_CMD__PANEL_DEBUG_PWM_FREQ = 2,
}

//
// struct dmub_cmd_panel_cntl_data - Panel control data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_panel_cntl_data {
    pub /: *mut *mut *mut uint32_t pwrseq_inst; /< pwrseq instance,
    pub /: *mut *mut uint32_t current_backlight; / in/out,
    pub /: *mut *mut uint32_t bl_pwm_cntl; / in/out,
    pub /: *mut *mut uint32_t bl_pwm_period_cntl; / in/out,
    pub /: *mut *mut uint32_t bl_pwm_ref_div1; / in/out,
    pub /: *mut *mut uint8_t is_backlight_on : 1; / in/out,
    pub /: *mut *mut uint8_t is_powered_on : 1; / in/out,
    pub padding: [u8; 3],
    pub /: *mut *mut uint32_t bl_pwm_ref_div2; / in/out,
    pub reserved: [u8; 4],
}

//
// struct dmub_rb_cmd_panel_cntl - Panel control command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_panel_cntl {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_cmd_panel_cntl_data data; /< payload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_optc_state {
    pub v_total_max: u32,
    pub v_total_min: u32,
    pub tg_inst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_drr_update {
    pub header: dmub_cmd_header,
    pub dmub_optc_state_req: dmub_optc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fw_assisted_mclk_switch_pipe_data {
    pub pix_clk_100hz: u32,
    pub max_ramp_step: u8,
    pub pipes: u8,
    pub min_refresh_in_hz: u8,
    pub pipe_count: u8,
    pub pipe_index: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fw_assisted_mclk_switch_config {
    pub fams_enabled: u8,
    pub visual_confirm_enabled: u8,
    pub Vactive: uint16_t vactive_stretch_margin_us; // Extra vblank stretch required when doing FPO +,
    pub pipe_data: [dmub_cmd_fw_assisted_mclk_switch_pipe_data; DMUB_MAX_FPO_STREAMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_fw_assisted_mclk_switch {
    pub header: dmub_cmd_header,
    pub config_data: dmub_cmd_fw_assisted_mclk_switch_config,
}

//
// Data passed from driver to FW in a DMUB_CMD__VBIOS_LVTMA_CONTROL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_lvtma_control_data {
    pub /: *mut *mut *mut uint8_t uc_pwr_action; /< LVTMA_ACTION,
    pub bypass_panel_control_wait: u8,
    pub /: *mut *mut *mut uint8_t reserved_0[2]; /< For future use,
    pub /: *mut *mut *mut uint8_t pwrseq_inst; /< LVTMA control instance,
    pub /: *mut *mut *mut uint8_t reserved_1[3]; /< For future use,
}

//
// Definition of a DMUB_CMD__VBIOS_LVTMA_CONTROL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_lvtma_control {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__VBIOS_LVTMA_CONTROL command.
//
    pub data: dmub_cmd_lvtma_control_data,
}

//
// Data passed in/out in a DMUB_CMD__VBIOS_TRANSMITTER_QUERY_DP_ALT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_transmitter_query_dp_alt_data {
    pub /: *mut *mut *mut uint8_t phy_id; /< 0=UNIPHYA, 1=UNIPHYB, 2=UNIPHYC, 3=UNIPHYD, 4=UNIPHYE, 5=UNIPHYF,
    pub /: *mut *mut *mut uint8_t is_usb; /< is phy is usb,
    pub /: *mut *mut *mut uint8_t is_dp_alt_disable; /< is dp alt disable,
    pub /: *mut *mut *mut uint8_t is_dp4; /< is dp in 4 lane,
}

//
// Definition of a DMUB_CMD__VBIOS_TRANSMITTER_QUERY_DP_ALT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_transmitter_query_dp_alt {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_rb_cmd_transmitter_query_dp_alt_data data; /< payload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_test_mode {
    pub mode: u8,
    pub pat0: u8,
    pub pad: [u8; 2],
}

//
// Data passed in/out in a DMUB_CMD__VBIOS_TRANSMITTER_SET_PHY_FSM command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_transmitter_set_phy_fsm_data {
    pub /: *mut *mut *mut uint8_t phy_id; /< 0=UNIPHYA, 1=UNIPHYB, 2=UNIPHYC, 3=UNIPHYD, 4=UNIPHYE, 5=UNIPHYF,
    pub /: *mut *mut *mut uint8_t mode; /< HDMI/DP/DP2 etc,
    pub /: *mut *mut *mut uint8_t lane_num; /< Number of lanes,
    pub /: *mut *mut *mut uint32_t symclk_100Hz; /< PLL symclock in 100hz,
    pub test_mode: phy_test_mode,
    pub state: dmub_phy_fsm_state,
    pub status: u32,
    pub pad: u8,
}

//
// Definition of a DMUB_CMD__VBIOS_TRANSMITTER_SET_PHY_FSM command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_transmitter_set_phy_fsm {
    pub /: *mut *mut *mut dmub_cmd_header header; /< header,
    pub /: *mut *mut *mut dmub_rb_cmd_transmitter_set_phy_fsm_data data; /< payload,
}

//
// Maximum number of bytes a chunk sent to DMUB for parsing
//
pub const DMUB_EDID_CEA_DATA_CHUNK_BYTES: c_int = 8;
//
// Represent a chunk of CEA blocks sent to DMUB for parsing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_send_edid_cea {
    pub /: *mut *mut *mut uint16_t offset; /< offset into the CEA block,
    pub /: *mut *mut *mut uint8_t length; /< number of bytes in payload to copy as part of CEA block,
    pub /: *mut *mut *mut uint16_t cea_total_length; /< total length of the CEA block,
    pub /: *mut *mut *mut uint8_t payload[DMUB_EDID_CEA_DATA_CHUNK_BYTES]; /< data chunk of the CEA block,
    pub /: *mut *mut *mut uint8_t pad[3]; /< padding and for future expansion,
}

//
// Result of VSDB parsing from CEA block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_edid_cea_amd_vsdb {
    pub /: *mut *mut *mut uint8_t vsdb_found; /< 1 if parsing has found valid AMD VSDB,
    pub /: *mut *mut *mut uint8_t freesync_supported; /< 1 if Freesync is supported,
    pub /: *mut *mut *mut uint16_t amd_vsdb_version; /< AMD VSDB version,
    pub /: *mut *mut *mut uint16_t min_frame_rate; /< Maximum frame rate,
    pub /: *mut *mut *mut uint16_t max_frame_rate; /< Minimum frame rate,
    pub /: *mut *mut *mut uint8_t freesync_mccs_vcp_code; /< Freesync MCCS VCP code,
}

//
// Result of sending a CEA chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_edid_cea_ack {
    pub /: *mut *mut *mut uint16_t offset; /< offset of the chunk into the CEA block,
    pub /: *mut *mut *mut uint8_t success; /< 1 if this sending of chunk succeeded,
    pub /: *mut *mut *mut uint8_t pad; /< padding and for future expansion,
}

//
// Specify whether the result is an ACK/NACK or the parsing has finished
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_edid_cea_reply_type {
    DMUB_CMD__EDID_CEA_AMD_VSDB	= 1, /**< VSDB parsing has finished */
    DMUB_CMD__EDID_CEA_ACK		= 2, /**< acknowledges the CEA sending is OK or failing */
}

//
// Definition of a DMUB_CMD__EDID_CEA command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_edid_cea {
    pub /: *mut *mut *mut dmub_cmd_header header; /< Command header,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_edid_cea_data {
    pub /: *mut *mut *mut dmub_cmd_send_edid_cea input; /< input to send CEA chunks,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_edid_cea_output {
    pub /: *mut *mut *mut uint8_t type; /< dmub_cmd_edid_cea_reply_type,
    pub amd_vsdb: dmub_cmd_edid_cea_amd_vsdb,
    pub ack: dmub_cmd_edid_cea_ack,
}

//
// struct dmub_cmd_cable_id_input - Defines the input of DMUB_CMD_GET_USBC_CABLE_ID command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cable_id_input {
    pub /: *mut *mut *mut uint8_t phy_inst; /< phy inst for cable id data,
}

//
// struct dmub_cmd_cable_id_input - Defines the output of DMUB_CMD_GET_USBC_CABLE_ID command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cable_id_output {
    pub /: *mut *mut *mut uint8_t UHBR10_20_CAPABILITY :2; /< b'01 for UHBR10 support, b'10 for both UHBR10 and UHBR20 support,
    pub /: *mut *mut *mut uint8_t UHBR13_5_CAPABILITY :1; /< b'1 for UHBR13.5 support,
    pub /: *mut *mut *mut uint8_t CABLE_TYPE :3; /< b'01 for passive cable, b'10 for active LRD cable, b'11 for active retimer cable,
    pub /: *mut *mut *mut uint8_t RESERVED :2; /< reserved means not defined,
}

//
// Definition of a DMUB_CMD_GET_USBC_CABLE_ID command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_get_usbc_cable_id {
    pub /: *mut *mut *mut dmub_cmd_header header; /< Command header,
//
// Data passed from driver to FW in a DMUB_CMD_GET_USBC_CABLE_ID command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_cable_id_data {
    pub /: *mut *mut *mut dmub_cmd_cable_id_input input; /< Input,
    pub /: *mut *mut *mut dmub_cmd_cable_id_output output; /< Output,
    pub /: *mut *mut *mut uint8_t output_raw; /< Raw data output,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_fused_io_sub_type {
    DMUB_CMD__FUSED_IO_EXECUTE = 0,
    DMUB_CMD__FUSED_IO_ABORT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_fused_request_type {
    FUSED_REQUEST_READ,
    FUSED_REQUEST_WRITE,
    FUSED_REQUEST_POLL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_fused_request_status {
    FUSED_REQUEST_STATUS_SUCCESS,
    FUSED_REQUEST_STATUS_BEGIN,
    FUSED_REQUEST_STATUS_SUBMIT,
    FUSED_REQUEST_STATUS_REPLY,
    FUSED_REQUEST_STATUS_POLL,
    FUSED_REQUEST_STATUS_ABORTED,
    FUSED_REQUEST_STATUS_FAILED = 0x80,
    FUSED_REQUEST_STATUS_INVALID,
    FUSED_REQUEST_STATUS_BUSY,
    FUSED_REQUEST_STATUS_TIMEOUT,
    FUSED_REQUEST_STATUS_POLL_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fused_request {
    pub status: u8,
    pub 2: uint8_t type :,
    pub 3: uint8_t _reserved0 :,
    pub comparing: uint8_t poll_mask_msb : 3; // Number of MSB to zero out from last byte before,
    pub identifier: u8,
    pub _reserved1: u8,
    pub timeout_us: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_fused_request_location {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fused_request_location_i2c {
    pub False: uint8_t is_aux : 1; //,
    pub 3: uint8_t ddc_line :,
    pub 1: uint8_t over_aux :,
    pub 3: uint8_t _reserved0 :,
    pub address: u8,
    pub offset: u8,
    pub length: u8,
    pub i2c: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_fused_request_location_aux {
    pub True: uint32_t is_aux : 1; //,
    pub 3: uint32_t ddc_line :,
    pub 20: uint32_t address :,
    pub transactions: uint32_t length : 8; // Automatically split into 16B,
    pub aux: },
    pub u: },
    pub expected: uint8_t buffer[0x30]; // Read: out, write: in, poll:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_fused_io {
    pub header: dmub_cmd_header,
    pub request: dmub_cmd_fused_request,
}

//
// Command type of a DMUB_CMD__SECURE_DISPLAY command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_secure_display_type {
    DMUB_CMD__SECURE_DISPLAY_TEST_CMD = 0,		/* test command to only check if inbox message works */
    DMUB_CMD__SECURE_DISPLAY_CRC_STOP_UPDATE,
    DMUB_CMD__SECURE_DISPLAY_CRC_WIN_NOTIFY,
    DMUB_CMD__SECURE_DISPLAY_MULTIPLE_CRC_STOP_UPDATE,
    DMUB_CMD__SECURE_DISPLAY_MULTIPLE_CRC_WIN_NOTIFY
}

pub const MAX_ROI_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_roi_info {
    pub x_start: u16,
    pub x_end: u16,
    pub y_start: u16,
    pub y_end: u16,
    pub otg_id: u8,
    pub phy_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_roi_window_ctl {
    pub x_start: u16,
    pub x_end: u16,
    pub y_start: u16,
    pub y_end: u16,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_roi_ctl_info {
    pub otg_id: u8,
    pub phy_id: u8,
    pub roi_ctl: [dmub_cmd_roi_window_ctl; MAX_ROI_NUM],
}

//
// Definition of a DMUB_CMD__SECURE_DISPLAY command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_secure_display {
    pub header: dmub_cmd_header,
//
// Data passed from driver to dmub firmware.
//
    pub roi_info: dmub_cmd_roi_info,
    pub mul_roi_ctl: dmub_cmd_roi_ctl_info,
}

//
// Command type of a DMUB_CMD__PSP command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_psp_type {
    DMUB_CMD__PSP_ASSR_ENABLE = 0
}

//
// Data passed from driver to FW in a DMUB_CMD__PSP_ASSR_ENABLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_assr_enable_data {
//
// ASSR enable or disable.
//
    pub enable: u8,
//
// PHY port type.
// Indicates eDP / non-eDP port type
//
    pub phy_port_type: u8,
//
// PHY port ID.
//
    pub phy_port_id: u8,
//
// Link encoder index.
//
    pub link_enc_index: u8,
//
// HPO mode.
//
    pub hpo_mode: u8,
//
// Reserved field.
//
    pub reserved: [u8; 7],
}

//
// Definition of a DMUB_CMD__PSP_ASSR_ENABLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_assr_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Assr data.
//
    pub assr_data: dmub_cmd_assr_enable_data,
//
// Reserved field.
//
    pub reserved: [u32; 3],
}

//
// Current definition of "ips_mode" from driver
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ips_residency_mode {
    IPS_RESIDENCY__IPS1_MAX,
    IPS_RESIDENCY__IPS2,
    IPS_RESIDENCY__IPS1_RCG, // refers to IPS0 + RCG
    IPS_RESIDENCY__IPS1_ONO2_ON,
    IPS_RESIDENCY__IPS1_Z8_RETENTION,
    IPS_RESIDENCY__PG_ONO_LAST_SEEN_IN_IPS,
    IPS_RESIDENCY__PG_ONO_CURRENT_STATE
}

pub const NUM_IPS_HISTOGRAM_BUCKETS: c_int = 16;
//
// IPS residency statistics to be sent to driver - subset of struct dmub_ips_residency_stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_ips_residency_info {
    pub residency_millipercent: u32,
    pub entry_counter: u32,
    pub histogram: [u32; NUM_IPS_HISTOGRAM_BUCKETS],
    pub total_time_us: u64,
    pub total_inactive_time_us: u64,
    pub ono_pg_state_at_collection: u32,
    pub ono_pg_state_last_seen_in_ips: u32,
}

//
// Data passed from driver to FW in a DMUB_CMD__IPS_RESIDENCY_CNTL command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_ips_residency_cntl_data {
    pub panel_inst: u8,
    pub start_measurement: u8,
    pub boundary: uint8_t padding[2]; // align to 4-byte,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_ips_residency_cntl {
    pub header: dmub_cmd_header,
    pub cntl_data: dmub_cmd_ips_residency_cntl_data,
}

//
// Data passed from FW to driver in a DMUB_CMD__IPS_QUERY_RESIDENCY_INFO command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_ips_query_residency_info_data {
    pub dest: dmub_addr,
    pub size: u32,
    pub ips_mode: u32,
    pub panel_inst: u8,
    pub boundary: uint8_t padding[3]; // align to 4-byte,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_ips_query_residency_info {
    pub header: dmub_cmd_header,
    pub info_data: dmub_cmd_ips_query_residency_info_data,
}

//
// struct dmub_cmd_cursor_offload_init_data - Payload for cursor offload init command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cursor_offload_init_data {
    pub /: *mut *mut *mut dmub_addr state_addr; /< State address for dmub_cursor_offload,
    pub /: *mut *mut *mut uint32_t state_size; /< State size for dmub_cursor_offload,
}

//
// struct dmub_rb_cmd_cursor_offload_init - Data for initializing cursor offload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cursor_offload_init {
    pub header: dmub_cmd_header,
    pub init_data: dmub_cmd_cursor_offload_init_data,
}

//
// struct dmub_cmd_cursor_offload_stream_data - Payload for cursor offload stream command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_cursor_offload_stream_data {
    pub /: *mut *mut *mut uint32_t otg_inst: 4; /< OTG instance to control,
    pub /: *mut *mut *mut uint32_t reserved: 28; /< Reserved for future use,
    pub /: *mut *mut *mut uint32_t line_time_in_ns; /< Line time in ns for the OTG,
    pub /: *mut *mut *mut uint32_t v_total_max; /< OTG v_total_max,
}

//
// struct dmub_rb_cmd_cursor_offload_stream_cntl - Controls a stream for cursor offload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_cursor_offload_stream_cntl {
    pub header: dmub_cmd_header,
    pub data: dmub_cmd_cursor_offload_stream_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__PR_ENABLE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_pr_enable_data {
//
// Panel Replay enable or disable.
//
    pub enable: u8,
//
// Panel Instance.
// Panel isntance to identify which replay_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// Phy state to enter.
// Values to use are defined in dmub_phy_fsm_state
//
    pub phy_fsm_state: u8,
//
// Phy rate for DP - RBR/HBR/HBR2/HBR3.
// Set this using enum phy_link_rate.
// This does not support HDMI/DP2 for now.
//
    pub phy_rate: u8,
//
// @hpo_stream_enc_inst: HPO stream encoder instance
//
    pub hpo_stream_enc_inst: u8,
//
// @hpo_link_enc_inst: HPO link encoder instance
//
    pub hpo_link_enc_inst: u8,
//
// @pad: Align structure to 4 byte boundary.
//
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_panel_polarity_enable_data {
//
// Panel Polarity enable or disable.
//
    pub enable: u8,
//
// OTG instance
//
    pub otg_inst: u8,
//
// @pad: Align structure to 4 byte boundary.
//
    pub pad: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_panel_polarity_reset_data {
//
// OTG instance
//
    pub otg_inst: u8,
//
// @pad: Align structure to 4 byte boundary.
//
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_panel_polarity_get_bias_input {
//
// OTG instance
//
    pub otg_inst: u8,
    pub pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_panel_polarity_get_bias_output {
//
// Accumulated Polarity Bias
//
    pub accumulated_bias: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_panel_polarity_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
    pub data: dmub_cmd_panel_polarity_enable_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_panel_polarity_get_bias {
//
// Command header.
//
    pub header: dmub_cmd_header,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_cmd_panel_polarity_get_bias_data {
    pub /: *mut *mut *mut dmub_cmd_panel_polarity_get_bias_input input; /< Input,
    pub /: *mut *mut *mut dmub_cmd_panel_polarity_get_bias_output output; /< Output,
    pub /: *mut *mut *mut uint32_t output_raw; /< Raw data output,
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_panel_polarity_reset {
//
// Command header.
//
    pub header: dmub_cmd_header,
    pub data: dmub_cmd_panel_polarity_reset_data,
}

//
// Definition of a DMUB_CMD__PR_ENABLE command.
// Panel Replay enable/disable is controlled using action in data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_pr_enable {
//
// Command header.
//
    pub header: dmub_cmd_header,
    pub data: dmub_cmd_pr_enable_data,
}

//
// Data passed from driver to FW in a DMUB_CMD__PR_COPY_SETTINGS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_pr_copy_settings_data {
//
// Flags that can be set by driver to change some replay behaviour.
//
    pub debug: pr_debug_flags,
//
// @flags: Flags used to determine feature functionality.
//
    pub flags: pr_hw_flags,
//
// DPP HW instance.
//
    pub dpp_inst: u8,
//
// OTG HW instance.
//
    pub otg_inst: u8,
//
// DIG FE HW instance.
//
    pub digfe_inst: u8,
//
// DIG BE HW instance.
//
    pub digbe_inst: u8,
//
// AUX HW instance.
//
    pub aux_inst: u8,
//
// Panel Instance.
// Panel isntance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// PHY instance.
//
    pub dpphy_inst: u8,
//
// Determines if SMU optimzations are enabled/disabled.
//
    pub smu_optimizations_en: u8,
//
// Length of each horizontal line in ns.
//
    pub line_time_in_ns: u32,
//
// Use FSFT afftet pixel clk
//
    pub pix_clk_100hz: u32,
//
// Use Original pixel clock
//
    pub sink_pix_clk_100hz: u32,
//
// Use for AUX-less ALPM LFPS wake operation
//
    pub auxless_alpm_data: dmub_alpm_auxless_data,
//
// DSC Slice height.
//
    pub dsc_slice_height: u16,
//
// Use FSM state for Replay power up/down
//
    pub use_phy_fsm: u8,
//
// @hpo_stream_enc_inst: HPO stream encoder instance
//
    pub hpo_stream_enc_inst: u8,
//
// @hpo_link_enc_inst: HPO link encoder instance
//
    pub hpo_link_enc_inst: u8,
//
// Selective Update granularity needed.
//
    pub su_granularity_needed: u8,
//
// Horizontal granularity for Selective Update.
//
    pub su_x_granularity: u16,
//
// Extended caps of vertical granularity for Selective Update.
//
    pub su_y_granularity_extended_caps: u16,
//
// Vertical granularity for Selective Update.
//
    pub su_y_granularity: u8,
//
// @main_link_activity_option: Indicates main link activity option selected
//
    pub main_link_activity_option: u8,
}

//
// Definition of a DMUB_CMD__PR_COPY_SETTINGS command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_pr_copy_settings {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__PR_COPY_SETTINGS command.
//
    pub data: dmub_cmd_pr_copy_settings_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_pr_runtime_flags {
    pub PR: uint32_t disable_abm_optimization : 1; // Disable ABM optimization for,
    pub FFU: uint32_t abm_periodic_ffu_allowed : 1; // DAL: scenario wants periodic ABM keep-alive,
    pub bitfields: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_pr_update_state_data {
//
// Panel Instance.
// Panel isntance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
    pub boundary: uint8_t pad[3]; // align to 4-byte,
//
// Update flags to control the update behavior.
//
    pub update_flag: u32,
//
// state/data to set.
//
    pub coasting_vtotal: u32,
    pub sync_mode: u32,
    pub pseudo_vtotal: u32,
    pub pr_runtime_flags: dmub_pr_runtime_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_pr_general_cmd_data {
//
// Panel Instance.
// Panel isntance to identify which psr_state to use
// Currently the support is only for 0 or 1
//
    pub panel_inst: u8,
//
// subtype: PR general cmd sub type
//
    pub subtype: u8,
    pub pad: [u8; 2],
//
// config data by different subtypes
//
    pub u32All: u32,
    pub data: },
}

//
// Definition of a DMUB_CMD__PR_UPDATE_STATE command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_pr_update_state {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__PR_UPDATE_STATE command.
//
    pub data: dmub_cmd_pr_update_state_data,
}

//
// Definition of a DMUB_CMD__PR_GENERAL_CMD command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_pr_general_cmd {
//
// Command header.
//
    pub header: dmub_cmd_header,
//
// Data passed from driver to FW in a DMUB_CMD__PR_GENERAL_CMD command.
//
    pub data: dmub_cmd_pr_general_cmd_data,
}

//
// Command type of a DMUB_CMD__BOOT_TIME_CRC command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_cmd_boot_time_crc_type {
    DMUB_CMD__BOOT_TIME_CRC_INIT_MEM = 0
}

//
// Data passed from driver to FW in a DMUB_CMD__BOOT_TIME_CRC_INIT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_cmd_boot_time_crc_init_data {
    pub buffer_addr: dmub_addr,
    pub buffer_size: u32,
}

//
// Definition of a DMUB_CMD__BOOT_TIME_CRC_INIT command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_cmd_boot_time_crc_init {
    pub header: dmub_cmd_header,
    pub data: dmub_cmd_boot_time_crc_init_data,
}

//
// union dmub_rb_cmd - DMUB inbox command.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_rb_cmd {
//
// Elements shared with all commands.
//
    pub cmd_common: dmub_rb_cmd_common,
//
// Definition of a DMUB_CMD__VBIOS_DIGX_ENCODER_CONTROL command.
//
    pub digx_encoder_control: dmub_rb_cmd_digx_encoder_control,
//
// Definition of a DMUB_CMD__VBIOS_SET_PIXEL_CLOCK command.
//
    pub set_pixel_clock: dmub_rb_cmd_set_pixel_clock,
//
// Definition of a DMUB_CMD__VBIOS_ENABLE_DISP_POWER_GATING command.
//
    pub enable_disp_power_gating: dmub_rb_cmd_enable_disp_power_gating,
//
// Definition of a DMUB_CMD__VBIOS_DPPHY_INIT command.
//
    pub dpphy_init: dmub_rb_cmd_dpphy_init,
//
// Definition of a DMUB_CMD__VBIOS_DIG1_TRANSMITTER_CONTROL command.
//
    pub dig1_transmitter_control: dmub_rb_cmd_dig1_transmitter_control,
//
// Definition of a DMUB_CMD__VBIOS_DOMAIN_CONTROL command.
//
    pub domain_control: dmub_rb_cmd_domain_control,
//
// Definition of a DMUB_CMD__PSR_SET_VERSION command.
//
    pub psr_set_version: dmub_rb_cmd_psr_set_version,
//
// Definition of a DMUB_CMD__PSR_COPY_SETTINGS command.
//
    pub psr_copy_settings: dmub_rb_cmd_psr_copy_settings,
//
// Definition of a DMUB_CMD__PSR_ENABLE command.
//
    pub psr_enable: dmub_rb_cmd_psr_enable,
//
// Definition of a DMUB_CMD__PSR_SET_LEVEL command.
//
    pub psr_set_level: dmub_rb_cmd_psr_set_level,
//
// Definition of a DMUB_CMD__PSR_FORCE_STATIC command.
//
    pub psr_force_static: dmub_rb_cmd_psr_force_static,
//
// Definition of a DMUB_CMD__UPDATE_DIRTY_RECT command.
//
    pub update_dirty_rect: dmub_rb_cmd_update_dirty_rect,
//
// Definition of a DMUB_CMD__UPDATE_CURSOR_INFO command.
//
    pub update_cursor_info: dmub_rb_cmd_update_cursor_info,
//
// Definition of a DMUB_CMD__HW_LOCK command.
// Command is used by driver and FW.
//
    pub lock_hw: dmub_rb_cmd_lock_hw,
//
// Definition of a DMUB_CMD__SET_SINK_VTOTAL_IN_PSR_ACTIVE command.
//
    pub psr_set_vtotal: dmub_rb_cmd_psr_set_vtotal,
//
// Definition of a DMUB_CMD__SET_PSR_POWER_OPT command.
//
    pub psr_set_power_opt: dmub_rb_cmd_psr_set_power_opt,
//
// Definition of a DMUB_CMD__PLAT_54186_WA command.
//
    pub PLAT_54186_wa: dmub_rb_cmd_PLAT_54186_wa,
//
// Definition of a DMUB_CMD__MALL command.
//
    pub mall: dmub_rb_cmd_mall,
//
// Definition of a DMUB_CMD__CAB command.
//
    pub cab: dmub_rb_cmd_cab_for_ss,
    pub fw_assisted_mclk_switch_v2: dmub_rb_cmd_fw_assisted_mclk_switch_v2,
//
// Definition of a DMUB_CMD__IDLE_OPT_DCN_RESTORE command.
//
    pub dcn_restore: dmub_rb_cmd_idle_opt_dcn_restore,
//
// Definition of a DMUB_CMD__CLK_MGR_NOTIFY_CLOCKS command.
//
    pub notify_clocks: dmub_rb_cmd_clk_mgr_notify_clocks,
//
// Definition of DMUB_CMD__PANEL_CNTL commands.
//
    pub panel_cntl: dmub_rb_cmd_panel_cntl,
//
// Definition of a DMUB_CMD__ABM_SET_PIPE command.
//
    pub abm_set_pipe: dmub_rb_cmd_abm_set_pipe,
//
// Definition of a DMUB_CMD__ABM_SET_BACKLIGHT command.
//
    pub abm_set_backlight: dmub_rb_cmd_abm_set_backlight,
//
// Definition of a DMUB_CMD__ABM_SET_LEVEL command.
//
    pub abm_set_level: dmub_rb_cmd_abm_set_level,
//
// Definition of a DMUB_CMD__ABM_SET_AMBIENT_LEVEL command.
//
    pub abm_set_ambient_level: dmub_rb_cmd_abm_set_ambient_level,
//
// Definition of a DMUB_CMD__ABM_SET_PWM_FRAC command.
//
    pub abm_set_pwm_frac: dmub_rb_cmd_abm_set_pwm_frac,
//
// Definition of a DMUB_CMD__ABM_INIT_CONFIG command.
//
    pub abm_init_config: dmub_rb_cmd_abm_init_config,
//
// Definition of a DMUB_CMD__ABM_PAUSE command.
//
    pub abm_pause: dmub_rb_cmd_abm_pause,
//
// Definition of a DMUB_CMD__ABM_SAVE_RESTORE command.
//
    pub abm_save_restore: dmub_rb_cmd_abm_save_restore,
//
// Definition of a DMUB_CMD__ABM_QUERY_CAPS command.
//
    pub abm_query_caps: dmub_rb_cmd_abm_query_caps,
//
// Definition of a DMUB_CMD__ABM_GET_ACE_CURVE command.
//
    pub abm_get_ace_curve: dmub_rb_cmd_abm_get_ace_curve,
//
// Definition of a DMUB_CMD__ABM_GET_HISTOGRAM command.
//
    pub abm_get_histogram: dmub_rb_cmd_abm_get_histogram,
//
// Definition of a DMUB_CMD__ABM_SET_EVENT command.
//
    pub abm_set_event: dmub_rb_cmd_abm_set_event,
//
// Definition of a DMUB_CMD__DP_AUX_ACCESS command.
//
    pub dp_aux_access: dmub_rb_cmd_dp_aux_access,
//
// Definition of a DMUB_CMD__OUTBOX1_ENABLE command.
//
    pub outbox1_enable: dmub_rb_cmd_outbox1_enable,
//
// Definition of a DMUB_CMD__QUERY_FEATURE_CAPS command.
//
    pub query_feature_caps: dmub_rb_cmd_query_feature_caps,
//
// Definition of a DMUB_CMD__GET_VISUAL_CONFIRM_COLOR command.
//
    pub visual_confirm_color: dmub_rb_cmd_get_visual_confirm_color,
    pub drr_update: dmub_rb_cmd_drr_update,
    pub fw_assisted_mclk_switch: dmub_rb_cmd_fw_assisted_mclk_switch,
//
// Definition of a DMUB_CMD__VBIOS_LVTMA_CONTROL command.
//
    pub lvtma_control: dmub_rb_cmd_lvtma_control,
//
// Definition of a DMUB_CMD__VBIOS_TRANSMITTER_QUERY_DP_ALT command.
//
    pub query_dp_alt: dmub_rb_cmd_transmitter_query_dp_alt,
//
// Definition of a DMUB_CMD__VBIOS_TRANSMITTER_SET_PHY_FSM command.
//
    pub set_phy_fsm: dmub_rb_cmd_transmitter_set_phy_fsm,
//
// Definition of a DMUB_CMD__DPIA_DIG1_CONTROL command.
//
    pub dig1_dpia_control: dmub_rb_cmd_dig1_dpia_control,
//
// Definition of a DMUB_CMD__DPIA_SET_CONFIG_ACCESS command.
//
    pub (deprecated): dmub_rb_cmd_set_config_access set_config_access; //,
//
// Definition of a DMUB_CMD__DPIA_SET_CONFIG_ACCESS command.
//
    pub set_config_request: dmub_rb_cmd_set_config_request,
//
// Definition of a DMUB_CMD__DPIA_MST_ALLOC_SLOTS command.
//
    pub set_mst_alloc_slots: dmub_rb_cmd_set_mst_alloc_slots,
//
// Definition of a DMUB_CMD__DPIA_SET_TPS_NOTIFICATION command.
//
    pub set_tps_notification: dmub_rb_cmd_set_tps_notification,
//
// Definition of a DMUB_CMD__EDID_CEA command.
//
    pub edid_cea: dmub_rb_cmd_edid_cea,
//
// Definition of a DMUB_CMD_GET_USBC_CABLE_ID command.
//
    pub cable_id: dmub_rb_cmd_get_usbc_cable_id,
//
// Definition of a DMUB_CMD__QUERY_HPD_STATE command.
//
    pub query_hpd: dmub_rb_cmd_query_hpd_state,
//
// Definition of a DMUB_CMD__SECURE_DISPLAY command.
//
    pub secure_display: dmub_rb_cmd_secure_display,
//
// Definition of a DMUB_CMD__DPIA_HPD_INT_ENABLE command.
//
    pub dpia_hpd_int_enable: dmub_rb_cmd_dpia_hpd_int_enable,
//
// Definition of a DMUB_CMD__CACP_SET_PIPE command.
//
    pub cacp_set_pipe: dmub_rb_cmd_cacp_set_pipe,
//
// Definition of a DMUB_CMD__CACP_SET_LEVEL command.
//
    pub cacp_set_level: dmub_rb_cmd_cacp_set_level,
//
// Definition of a DMUB_CMD__CACP_SET_EVENT command.
//
    pub cacp_set_event: dmub_rb_cmd_cacp_set_event,
//
// Definition of a DMUB_CMD__CACP_INIT_CONFIG command.
//
    pub cacp_init_config: dmub_rb_cmd_cacp_init_config,
//
// Definition of a DMUB_CMD__CACP_PAUSE command.
//
    pub cacp_pause: dmub_rb_cmd_cacp_pause,
//
// Definition of a DMUB_CMD__CACP_SET_BACKLIGHT command.
//
    pub cacp_set_backlight: dmub_rb_cmd_cacp_set_backlight,
//
// Definition of a DMUB_CMD__CACP_SET_PWM_FRAC command.
//
    pub cacp_set_pwm_frac: dmub_rb_cmd_cacp_set_pwm_frac,
//
// Definition of a DMUB_CMD__CACP_GET_HISTOGRAM command.
//
    pub cacp_get_histogram: dmub_rb_cmd_cacp_get_histogram,
//
// Definition of a DMUB_CMD__CACP_GET_ACE_CURVE_AREA command.
//
    pub cacp_get_ace_curve_area: dmub_rb_cmd_cacp_get_ace_curve_area,
//
// Definition of a DMUB_CMD__IDLE_OPT_DCN_NOTIFY_IDLE command.
//
    pub idle_opt_notify_idle: dmub_rb_cmd_idle_opt_dcn_notify_idle,
//
// Definition of a DMUB_CMD__IDLE_OPT_SET_DC_POWER_STATE command.
//
    pub idle_opt_set_dc_power_state: dmub_rb_cmd_idle_opt_set_dc_power_state,
//
// Definition of a DMUB_CMD__REPLAY_COPY_SETTINGS command.
//
    pub replay_copy_settings: dmub_rb_cmd_replay_copy_settings,
//
// Definition of a DMUB_CMD__REPLAY_ENABLE command.
//
    pub replay_enable: dmub_rb_cmd_replay_enable,
//
// Definition of a DMUB_CMD__SET_REPLAY_POWER_OPT command.
//
    pub replay_set_power_opt: dmub_rb_cmd_replay_set_power_opt,
//
// Definition of a DMUB_CMD__REPLAY_SET_COASTING_VTOTAL command.
//
    pub replay_set_coasting_vtotal: dmub_rb_cmd_replay_set_coasting_vtotal,
//
// Definition of a DMUB_CMD__REPLAY_SET_POWER_OPT_AND_COASTING_VTOTAL command.
//
    pub replay_set_power_opt_and_coasting_vtotal: dmub_rb_cmd_replay_set_power_opt_and_coasting_vtotal,
    pub replay_set_timing_sync: dmub_rb_cmd_replay_set_timing_sync,
//
// Definition of a DMUB_CMD__REPLAY_SET_RESIDENCY_FRAMEUPDATE_TIMER command.
//
    pub replay_set_frameupdate_timer: dmub_rb_cmd_replay_set_frameupdate_timer,
//
// Definition of a DMUB_CMD__REPLAY_SET_PSEUDO_VTOTAL command.
//
    pub replay_set_pseudo_vtotal: dmub_rb_cmd_replay_set_pseudo_vtotal,
//
// Definition of a DMUB_CMD__REPLAY_DISABLED_ADAPTIVE_SYNC_SDP command.
//
    pub replay_disabled_adaptive_sync_sdp: dmub_rb_cmd_replay_disabled_adaptive_sync_sdp,
//
// Definition of a DMUB_CMD__REPLAY_SET_GENERAL_CMD command.
//
    pub replay_set_general_cmd: dmub_rb_cmd_replay_set_general_cmd,
//
// Definition of a DMUB_CMD__PSP_ASSR_ENABLE command.
//
    pub assr_enable: dmub_rb_cmd_assr_enable,
    pub fams2_config: dmub_rb_cmd_fams2,
    pub ib_fams2_config: dmub_rb_cmd_ib,
    pub ib_fams2_debug_meta: dmub_rb_cmd_ib,
    pub fams2_drr_update: dmub_rb_cmd_fams2_drr_update,
    pub fams2_flip: dmub_rb_cmd_fams2_flip,
    pub fused_io: dmub_rb_cmd_fused_io,
//
// Definition of a DMUB_CMD__LSDMA command.
//
    pub lsdma: dmub_rb_cmd_lsdma,
    pub ips_residency_cntl: dmub_rb_cmd_ips_residency_cntl,
    pub ips_query_residency_info: dmub_rb_cmd_ips_query_residency_info,
//
// Definition of a DMUB_CMD__CURSOR_OFFLOAD_INIT command.
//
    pub cursor_offload_init: dmub_rb_cmd_cursor_offload_init,
//
// Definition of a DMUB_CMD__CURSOR_OFFLOAD control commands.
// - DMUB_CMD__CURSOR_OFFLOAD_STREAM_ENABLE
// - DMUB_CMD__CURSOR_OFFLOAD_STREAM_DISABLE
// - DMUB_CMD__CURSOR_OFFLOAD_STREAM_PROGRAM
// - DMUB_CMD__CURSOR_OFFLOAD_STREAM_UPDATE_DRR
//
    pub cursor_offload_stream_ctnl: dmub_rb_cmd_cursor_offload_stream_cntl,
//
// Definition of a DMUB_CMD__SMART_POWER_OLED_ENABLE command.
//
    pub smart_power_oled_enable: dmub_rb_cmd_smart_power_oled_enable,
//
// Definition of a DMUB_CMD__DMUB_CMD__SMART_POWER_OLED_GETMAXCLL command.
//
    pub smart_power_oled_getmaxcll: dmub_rb_cmd_smart_power_oled_getmaxcll,
//
// Definition of a DMUB_CMD__REPLAY_COPY_SETTINGS command.
//
    pub pr_copy_settings: dmub_rb_cmd_pr_copy_settings,
//
// Definition of a DMUB_CMD__REPLAY_ENABLE command.
//
    pub pr_enable: dmub_rb_cmd_pr_enable,
    pub pr_update_state: dmub_rb_cmd_pr_update_state,
    pub pr_general_cmd: dmub_rb_cmd_pr_general_cmd,
//
// Definition of a DMUB_CMD__IHC command.
//
    pub ihc: dmub_rb_cmd_ihc,
//
// Definition of a DMUB_CMD__BOOT_TIME_CRC_INIT command.
//
    pub boot_time_crc_init: dmub_rb_cmd_boot_time_crc_init,
//
// Definition of a DMUB_CMD__PANEL_POLARITY_ENABLE command.
//
    pub panel_polarity_enable: dmub_rb_cmd_panel_polarity_enable,
    pub panel_polarity_get_bias: dmub_rb_cmd_panel_polarity_get_bias,
    pub panel_polarity_reset: dmub_rb_cmd_panel_polarity_reset,
}

//
// union dmub_rb_out_cmd - Outbox command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dmub_rb_out_cmd {
//
// Parameters common to every command.
//
    pub cmd_common: dmub_rb_cmd_common,
//
// AUX reply command.
//
    pub dp_aux_reply: dmub_rb_cmd_dp_aux_reply,
//
// HPD notify command.
//
    pub dp_hpd_notify: dmub_rb_cmd_dp_hpd_notify,
//
// SET_CONFIG reply command.
//
    pub set_config_reply: dmub_rb_cmd_dp_set_config_reply,
//
// DPIA notification command.
//
    pub dpia_notification: dmub_rb_cmd_dpia_notification,
//
// HPD sense notification command.
//
    pub hpd_sense_notify: dmub_rb_cmd_hpd_sense_notify,
    pub fused_io: dmub_rb_cmd_fused_io,
}

// ==============================================================================
// </DMUB_CMD>===================================================================
// ==============================================================================
// < DMUB_RB>====================================================================
// ==============================================================================
//
// struct dmub_rb_init_params - Initialization params for DMUB ringbuffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb_init_params {
    pub /: *mut *mut *mut *mut void ctx; /< Caller provided context pointer,
    pub /: *mut *mut *mut *mut void base_address; /< CPU base address for ring's data,
    pub /: *mut *mut *mut uint32_t capacity; /< Ringbuffer capacity in bytes,
    pub /: *mut *mut *mut uint32_t read_ptr; /< Initial read pointer for consumer in bytes,
    pub /: *mut *mut *mut uint32_t write_ptr; /< Initial write pointer for producer in bytes,
}

//
// struct dmub_rb - Inbox or outbox DMUB ringbuffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_rb {
    pub /: *mut *mut *mut *mut void base_address; /< CPU address for the ring's data,
    pub /: *mut *mut *mut uint32_t rptr; /< Read pointer for consumer in bytes,
    pub /: *mut *mut *mut uint32_t wrpt; /< Write pointer for producer in bytes,
    pub /: *mut *mut *mut uint32_t capacity; /< Ringbuffer capacity in bytes,
    pub /: *mut *mut *mut *mut void ctx; /< Caller provided context pointer,
    pub /: *mut *mut *mut *mut void dmub; /< Pointer to the DMUB interface,
}

//
// @brief Checks if the ringbuffer is empty.
//
// @param rb DMUB Ringbuffer
// @return true if empty
// @return false otherwise
//
// @brief gets number of outstanding requests in the RB
//
// @param rb DMUB Ringbuffer
// @return true if full
//
// @brief gets number of free buffers in the RB
//
// @param rb DMUB Ringbuffer
// @return true if full
//
// +1 because 1 entry is always unusable
//
// @brief Checks if the ringbuffer is full
//
// @param rb DMUB Ringbuffer
// @return true if full
// @return false otherwise
//
// -1 because 1 entry is always unusable
//
// @brief Pushes a command into the ringbuffer
//
// @param rb DMUB ringbuffer
// @param cmd The command to push
// @return true if the ringbuffer was not full
// @return false otherwise
//
// copying data
// dst++ = *src++;
//
// @brief Pushes a command into the DMUB outbox ringbuffer
//
// @param rb DMUB outbox ringbuffer
// @param cmd Outbox command
// @return true if not full
// @return false otherwise
//
// @brief Returns the next unprocessed command in the ringbuffer.
//
// @param rb DMUB ringbuffer
// @param cmd The command to return
// @return true if not empty
// @return false otherwise
//
// cmd = (union dmub_rb_cmd *)rb_cmd;
//
// @brief Determines the next ringbuffer offset.
//
// @param rb DMUB inbox ringbuffer
// @param num_cmds Number of commands
// @param next_rptr The next offset in the ringbuffer
//
// next_rptr = rb->rptr + DMUB_RB_CMD_SIZE * num_cmds;
// next_rptr %= rb->capacity;
//
// @brief Returns a pointer to a command in the inbox.
//
// @param rb DMUB inbox ringbuffer
// @param cmd The inbox command to return
// @param rptr The ringbuffer offset
// @return true if not empty
// @return false otherwise
//
// cmd = (union dmub_rb_cmd *)rb_cmd;
//
// @brief Returns the next unprocessed command in the outbox.
//
// @param rb DMUB outbox ringbuffer
// @param cmd The outbox command to return
// @return true if not empty
// @return false otherwise
//
// copying data
// dst++ = *src++;
//
// @brief Removes the front entry in the ringbuffer.
//
// @param rb DMUB ringbuffer
// @return true if the command was removed
// @return false if there were no commands
//
// @brief Flushes commands in the ringbuffer to framebuffer memory.
//
// Avoids a race condition where DMCUB accesses memory while
// there are still writes in flight to framebuffer.
//
// @param rb DMUB ringbuffer
//
// @brief Initializes a DMCUB ringbuffer
//
// @param rb DMUB ringbuffer
// @param init_params initial configuration for the ringbuffer
//
// @brief Copies output data from in/out commands into the given command.
//
// @param rb DMUB ringbuffer
// @param cmd Command to copy data into
//
// Copy rb entry back into command
// ==============================================================================
// </DMUB_RB>====================================================================
// ==============================================================================
