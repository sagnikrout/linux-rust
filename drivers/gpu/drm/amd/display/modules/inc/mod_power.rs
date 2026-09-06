//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/inc/mod_power.h
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


// Copyright (c) 2019 Advanced Micro Devices, Inc. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_power_init_params {
    pub disable_fractional_pwm: bool,
// Use nits based brightness instead of brightness percentage
//
    pub use_nits_based_brightness: bool,
    pub panel_min_millinits: c_uint,
    pub panel_max_millinits: c_uint,
    pub min_backlight_pwm: c_uint,
    pub max_backlight_pwm: c_uint,
    pub min_abm_backlight: c_uint,
    pub num_backlight_levels: c_uint,
    pub backlight_ramping_override: bool,
    pub backlight_ramping_reduction: c_uint,
    pub backlight_ramping_start: c_uint,
    pub def_varibright_enable: bool,
    pub def_varibright_level: c_uint,
    pub varibright_level: c_uint,
    pub abm_config_setting: c_uint,
    pub allow_psr_smu_optimizations: bool,
    pub allow_psr_multi_disp_optimizations: bool,
    pub use_custom_backlight_caps: bool,
    pub custom_backlight_caps_config_no: c_uint,
    pub use_linear_backlight_curve: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_power {
    pub dummy: c_int,
}

// VariBright settings structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct varibright_info {
    pub level: c_uint,
    pub enable: bool,
    pub activate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_power_psr_context {
// ddc line
    pub channel: c_uint,
// Transmitter id
    pub transmitter_id: c_uint,
// Engine Id is used for Dig Be source select
    pub engine_id: c_uint,
// Controller Id used for Dig Fe source select
    pub controller_id: c_uint,
// Pcie or Uniphy
    pub phy_type: c_uint,
// Physical PHY Id used by SMU interpretation
    pub smu_phy_id: c_uint,
// Vertical total pixels from crtc timing.
// This is used for static screen detection.
// ie. If we want to detect half a frame,
// we use this to determine the hyst lines.
//
    pub crtc_timing_vertical_total: c_uint,
// PSR supported from panel capabilities and
// current display configuration
//
    pub psr_supported_display_config: bool,
// Whether fast link training is supported by the panel
    pub psr_exit_link_training_required: bool,
// If RFB setup time is greater than the total VBLANK time,
// it is not possible for the sink to capture the video frame
// in the same frame the SDP is sent. In this case,
// the frame capture indication bit should be set and an extra
// static frame should be transmitted to the sink.
//
    pub psr_frame_capture_indication_req: bool,
// Set the last possible line SDP may be transmitted without violating
// the RFB setup time or entering the active video frame.
//
    pub sdp_transmit_line_num_deadline: c_uint,
// The VSync rate in Hz used to calculate the
// step size for smooth brightness feature
//
    pub vsync_rate_hz: c_uint,
    pub skip_psr_wait_for_pll_lock: c_uint,
    pub number_of_controllers: c_uint,
// Unused, for future use. To indicate that first changed frame from
// state3 shouldn't result in psr_inactive, but rather to perform
// an automatic single frame rfb_update.
//
    pub rfb_update_auto_en: bool,
// Number of frame before entering static screen
    pub timehyst_frames: c_uint,
// Partial frames before entering static screen
    pub hyst_lines: c_uint,
// # of repeated AUX transaction attempts to make before
// indicating failure to the driver
//
    pub aux_repeats: c_uint,
// Controls hw blocks to power down during PSR active state
    pub psr_level: c_uint,
// Controls additional delay after remote frame capture before
// continuing powerd own
//
    pub frame_delay: c_uint,
    pub allow_smu_optimizations: bool,
    pub allow_multi_disp_optimizations: bool,
    pub line_time_in_us: c_uint,
// Panel self refresh 2 selective update granularity required
    pub su_granularity_required: bool,
// psr2 selective update y granularity capability
    pub su_y_granularity: u8,
    pub rate_control_caps: u8,
    pub os_request_force_ffu: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum psr_event {
    psr_event_invalid = 0x0,
    psr_event_vsync = 0x1,
    psr_event_full_screen = 0x2,
    psr_event_defer_enable = 0x4,
    psr_event_hw_programming = 0x8,
    psr_event_test_harness_enable_psr = 0x10,
    psr_event_test_harness_disable_psr = 0x20,
    psr_event_mpo_video_selective_update = 0x40,
    psr_event_edp_panel_off_disable_psr = 0x80,
    psr_event_dynamic_display_switch = 0x100,
    psr_event_big_screen_video = 0x200,
    psr_event_dds_defer_stream_enable = 0x800,
    psr_event_dynamic_link_rate_control = 0x1000,
    psr_event_vrr_transition = 0x2000,
    psr_event_pause = 0x4000,
    psr_event_immediate_flip = 0x8000,
    psr_event_os_request_disable = 0x10000,
    psr_event_os_request_force_ffu = 0x20000,
    psr_event_os_override_hold = 0x40000,
    psr_event_crc_window_active = 0x80000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_event {
    replay_event_invalid = 0x0,
    replay_event_vsync = 0x1,
    replay_event_full_screen = 0x2,
    replay_event_mpo_video_selective_update = 0x4,
    replay_event_big_screen_video = 0x8,
    replay_event_hw_programming = 0x10,
    replay_event_edp_panel_off_disable_psr = 0x20,
    replay_event_general_ui = 0x40,
    replay_event_vrr = 0x80,
    replay_event_prepare_vtotal = 0x100,
    replay_event_test_harness_enable_replay = 0x200,
    replay_event_test_harness_disable_replay = 0x400,
    replay_event_test_harness_ultra_sleep = 0x800,
    replay_event_immediate_flip = 0x1000,
    replay_event_vrr_transition = 0x2000,
    replay_event_pause = 0x4000,
    replay_event_disable_replay_while_DPMS = 0x8000,
    replay_event_test_harness_mode = 0x10000,
    replay_event_cursor_updating = 0x20000,
    replay_event_sleep_resume = 0x40000,
    replay_event_disable_in_AC = 0x80000,
    replay_event_disable_replay_while_detect_display = 0x100000,
    replay_event_disable_replay_while_switching_mux = 0x400000,
    replay_event_infopacket = 0x800000,
    replay_event_os_request_disable = 0x1000000,
    replay_event_os_request_force_ffu = 0x2000000,
    replay_event_os_override_hold = 0x4000000,
    replay_event_crc_window_active = 0x8000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_enable_option {
    pr_enable_option_static_screen = 0x1,
    pr_enable_option_mpo_video = 0x2,
    pr_enable_option_full_screen_video = 0x4,
    pr_enable_option_general_ui = 0x8,
    pr_enable_option_full_screen = 0x10,
    pr_enable_option_static_screen_coasting = 0x10000,
    pr_enable_option_mpo_video_coasting = 0x20000,
    pr_enable_option_full_screen_video_coasting = 0x40000,
    pr_enable_option_full_screen_coasting = 0x100000,
}

extern "C" {
    pub fn mod_power_destroy(mod_power: *mut mod_power);
}
extern "C" {
    pub fn mod_power_hw_init(mod_power: *mut mod_power) -> bool;
}
