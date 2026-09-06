//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/dml_top_display_cfg_types.h
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
// Copyright 2024 Advanced Micro Devices, Inc.

pub const DML2_MAX_PLANES: c_int = 8;
pub const DML2_MAX_DCN_PIPES: c_int = 8;

pub const DML2_MAX_WRITEBACK: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_swizzle_mode {
    dml2_sw_linear, // SW_LINEAR accepts 256 byte aligned pitch and also 128 byte aligned pitch if DCC is not enabled
    dml2_sw_256b_2d,
    dml2_sw_4kb_2d,
    dml2_sw_64kb_2d,
    dml2_sw_256kb_2d,

    dml2_gfx11_sw_linear,
    dml2_gfx11_sw_64kb_d,
    dml2_gfx11_sw_64kb_d_t,
    dml2_gfx11_sw_64kb_d_x,
    dml2_gfx11_sw_64kb_r_x,
    dml2_gfx11_sw_256kb_d_x,
    dml2_gfx11_sw_256kb_r_x,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_source_format_class {
    dml2_444_8 = 0,
    dml2_444_16 = 1,
    dml2_444_32 = 2,
    dml2_444_64 = 3,
    dml2_420_8 = 4,
    dml2_420_10 = 5,
    dml2_420_12 = 6,
    dml2_rgbe_alpha = 9,
    dml2_rgbe = 10,
    dml2_mono_8 = 11,
    dml2_mono_16 = 12,
    dml2_422_planar_8 = 13,
    dml2_422_planar_10 = 14,
    dml2_422_planar_12 = 15,
    dml2_422_packed_8 = 16,
    dml2_422_packed_10 = 17,
    dml2_422_packed_12 = 18
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_sample_positioning {
    dml2_interstitial = 0,
    dml2_cosited = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_rotation_angle {
    dml2_rotation_0 = 0,
    dml2_rotation_90 = 1,
    dml2_rotation_180 = 2,
    dml2_rotation_270 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_output_format_class {
    dml2_444 = 0,
    dml2_s422 = 1,
    dml2_n422 = 2,
    dml2_420 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_output_encoder_class {
    dml2_dp = 0,
    dml2_edp = 1,
    dml2_dp2p0 = 2,
    dml2_hdmi = 3,
    dml2_hdmifrl = 4,
    dml2_none = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_output_link_dp_rate {
    dml2_dp_rate_na = 0,
    dml2_dp_rate_hbr = 1,
    dml2_dp_rate_hbr2 = 2,
    dml2_dp_rate_hbr3 = 3,
    dml2_dp_rate_uhbr10 = 4,
    dml2_dp_rate_uhbr13p5 = 5,
    dml2_dp_rate_uhbr20 = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_pstate_type {
    dml2_pstate_type_uclk = 0,
    dml2_pstate_type_fclk = 1,
    dml2_pstate_type_ppt = 2,
    dml2_pstate_type_temp_read = 3,
    dml2_pstate_type_dummy_pstate = 4,
    dml2_pstate_type_count = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_uclk_pstate_change_strategy {
    dml2_uclk_pstate_change_strategy_auto = 0,
    dml2_uclk_pstate_change_strategy_force_vactive = 1,
    dml2_uclk_pstate_change_strategy_force_vblank = 2,
    dml2_uclk_pstate_change_strategy_force_drr = 3,
    dml2_uclk_pstate_change_strategy_force_mall_svp = 4,
    dml2_uclk_pstate_change_strategy_force_mall_full_frame = 5,
    dml2_uclk_pstate_change_strategy_force_alternate = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_svp_mode_override {
    dml2_svp_mode_override_auto = 0,
    dml2_svp_mode_override_main_pipe = 1,
    dml2_svp_mode_override_phantom_pipe = 2, //does not need to be defined explicitly, main overrides result in implicit phantom additions
    dml2_svp_mode_override_phantom_pipe_no_data_return = 3,
    dml2_svp_mode_override_imall = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_refresh_from_mall_mode_override {
    dml2_refresh_from_mall_mode_override_auto = 0,
    dml2_refresh_from_mall_mode_override_force_disable = 1,
    dml2_refresh_from_mall_mode_override_force_enable = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_odm_mode {
    dml2_odm_mode_auto = 0,
    dml2_odm_mode_bypass,
    dml2_odm_mode_combine_2to1,
    dml2_odm_mode_combine_3to1,
    dml2_odm_mode_combine_4to1,
    dml2_odm_mode_split_1to2,
    dml2_odm_mode_mso_1to2,
    dml2_odm_mode_mso_1to4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_scaling_transform {
    dml2_scaling_transform_explicit = 0,
    dml2_scaling_transform_fullscreen,
    dml2_scaling_transform_aspect_ratio,
    dml2_scaling_transform_centered
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_dsc_enable_option {
    dml2_dsc_disable = 0,
    dml2_dsc_enable = 1,
    dml2_dsc_enable_if_necessary = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_tdlut_addressing_mode {
    dml2_tdlut_sw_linear = 0,
    dml2_tdlut_simple_linear = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_tdlut_width_mode {
    dml2_tdlut_width_17_cube = 0,
    dml2_tdlut_width_33_cube = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_twait_budgeting_setting {
    dml2_twait_budgeting_setting_ignore = 0,// Ignore this budget in twait

    dml2_twait_budgeting_setting_if_needed,         // Budget for it only if needed
// (i.e. UCLK/FCLK DPM cannot be supported in active)

    dml2_twait_budgeting_setting_try,	   // Budget for it as long as there is an SoC state that
// can support it
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_get_cursor_dlg_reg {
    pub cursor_x_position: c_uint,
    pub cursor_hotspot_x: c_uint,
    pub cursor_primary_offset: c_uint,
    pub cursor_secondary_offset: c_uint,
    pub cursor_stereo_en: bool,
    pub cursor_2x_magnify: bool,
    pub hratio: double,
    pub pixel_rate_mhz: double,
    pub dlg_refclk_mhz: double,
}

// @brief Surface Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_surface_cfg {
    pub tiling: dml2_swizzle_mode,
    pub format: unsigned long pitch; // In elements, two pixels per element in 422 packed,
    pub width: c_ulong,
    pub height: c_ulong,
    pub plane0: },
    pub pitch: c_ulong,
    pub width: c_ulong,
    pub height: c_ulong,
    pub plane1: },
    pub enable: bool,
    pub pitch: c_ulong,
    pub plane0: },
    pub pitch: c_ulong,
    pub plane1: },
    pub dcc_rate_plane0: double,
    pub dcc_rate_plane1: double,
    pub fraction_of_zero_size_request_plane0: double,
    pub fraction_of_zero_size_request_plane1: double,
    pub informative: },
    pub dcc: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_composition_cfg {
    pub rotation_angle: dml2_rotation_angle,
    pub mirrored: bool,
    pub scaling_transform: dml2_scaling_transform,
    pub rect_out_height_spans_vactive: bool,
    pub stationary: bool,
    pub width: c_ulong,
    pub height: c_ulong,
    pub x_start: c_ulong,
    pub y_start: c_ulong,
    pub plane0: },
    pub width: c_ulong,
    pub height: c_ulong,
    pub x_start: c_ulong,
    pub y_start: c_ulong,
    pub plane1: },
    pub viewport: },
    pub enabled: bool,
    pub easf_enabled: bool,
    pub isharp_enabled: bool,
    pub upsp_enabled: bool,
    pub upsp_sample_positioning: dml2_sample_positioning,
    pub upsp_vtaps: c_uint,
    pub h_ratio: double,
    pub v_ratio: double,
    pub h_taps: c_uint,
    pub v_taps: c_uint,
    pub plane0: },
    pub h_ratio: double,
    pub v_ratio: double,
    pub h_taps: c_uint,
    pub v_taps: c_uint,
    pub plane1: },
    pub rect_out_width: c_ulong,
    pub scaler_info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_timing_cfg {
    pub h_total: c_ulong,
    pub v_total: c_ulong,
    pub h_blank_end: c_ulong,
    pub v_blank_end: c_ulong,
    pub h_front_porch: c_ulong,
    pub v_front_porch: c_ulong,
    pub h_sync_width: c_ulong,
    pub pixel_clock_khz: c_ulong,
    pub h_active: c_ulong,
    pub v_active: c_ulong,
    pub Jun: unsigned int bpc; //FIXME: review with,
    pub enable: dml2_dsc_enable_option,
    pub dsc_compressed_bpp_x16: c_uint,
// for dv to specify num dsc slices to use
    pub num_slices: c_uint,
    pub overrides: },
    pub dsc: },
    pub interlaced: bool,
// static
    pub enabled: bool,
    pub min_refresh_uhz: c_ulong,
    pub max_instant_vtotal_delta: c_uint,
// dynamic
    pub disallowed: bool,
    pub drr_active_variable: bool,
    pub drr_active_fixed: bool,
    pub drr_config: },
    pub vblank_nom: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_link_output_cfg {
    pub output_format: dml2_output_format_class,
    pub output_encoder: dml2_output_encoder_class,
    pub output_dp_lane_count: c_uint,
    pub output_dp_link_rate: dml2_output_link_dp_rate,
    pub audio_sample_rate: c_ulong,
    pub audio_sample_layout: c_ulong,
    pub physical: bool output_disabled; // The stream does not go to a backend for output to a,
// connector (e.g. writeback only, phantom pipe) goes to writeback
    pub stream.: bool validate_output; // Do not validate the link configuration for this display,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_writeback_info {
    pub pixel_format: dml2_source_format_class,
    pub input_width: c_ulong,
    pub input_height: c_ulong,
    pub output_width: c_ulong,
    pub output_height: c_ulong,
    pub v_taps: c_ulong,
    pub h_taps: c_ulong,
    pub v_taps_chroma: c_ulong,
    pub h_taps_chroma: c_ulong,
    pub h_ratio: double,
    pub v_ratio: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_writeback_cfg {
    pub active_writebacks_per_stream: c_uint,
    pub writeback_stream: [dml2_writeback_info; DML2_MAX_WRITEBACK],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_plane_parameters {
    pub composed: unsigned int stream_index; // Identifies which plane will be,
    pub pixel_format: dml2_source_format_class,
//
// The surface and composition structures use
// the terms plane0 and plane1.  These planes
// are expected to hold the following data based
// on the pixel format.
//
// RGB or YUV Non-Planar Types:
// dml2_444_8
// dml2_444_16
// dml2_444_32
// dml2_444_64
// dml2_rgbe
//
// plane0 = argb or rgbe
// plane1 = not used
//
// YUV Planar-Types:
// dml2_420_8
// dml2_420_10
// dml2_420_12
//
// plane0 = luma
// plane1 = chroma
//
// RGB Planar Types:
// dml2_rgbe_alpha
//
// plane0 = rgbe
// plane1 = alpha
//
// Mono Non-Planar Types:
// dml2_mono_8
// dml2_mono_16
//
// plane0 = luma
// plane1 = not used
//
    pub surface: dml2_surface_cfg,
    pub composition: dml2_composition_cfg,
    pub enable: bool,
    pub lines_before_active_required: c_ulong,
    pub transmitted_bytes: c_ulong,
    pub dynamic_meta_data: },
    pub num_cursors: c_uint,
    pub cursor_width: c_ulong,
    pub cursor_bpp: c_ulong,
    pub cursor: },
// For TDLUT, SW would assume TDLUT is setup and enable all the time and
// budget for worst case addressing/width mode
    pub setup_for_tdlut: bool,
    pub tdlut_addressing_mode: dml2_tdlut_addressing_mode,
    pub tdlut_width_mode: dml2_tdlut_width_mode,
    pub tdlut_mpc_width_flag: bool,
    pub tdlut: },
    pub immediate_flip: bool,
// Logical overrides to power management policies (usually)
    pub uclk_pstate_change_strategy: dml2_uclk_pstate_change_strategy,
    pub refresh_from_mall: dml2_refresh_from_mall_mode_override,
    pub det_size_override_kb: c_uint,
    pub mpcc_combine_factor: c_uint,
// reserved_vblank_time_ns is the minimum time to reserve in vblank for Twait
// The actual reserved vblank time used for the corresponding stream in mode_programming would be at least as much as this per-plane override.
    pub reserved_vblank_time_ns: c_long,
    pub delay: unsigned int max_vactive_det_fill_delay_us[dml2_pstate_type_count]; // 0 = no reserved time, +ve = explicit max,
    pub gpuvm_min_page_size_kbytes: c_uint,
    pub hostvm_min_page_size_kbytes: c_uint,
    pub svp_config: dml2_svp_mode_override legacy_svp_config; //TODO remove in favor of,
    pub use_max_lsw: bool,
// HW specific overrides, there's almost no reason to mess with these
// generally used for debugging or simulation
    pub force_one_row_for_frame: bool,
    pub enable: bool,
    pub value: bool,
    pub force_pte_buffer_mode: },
    pub dppclk_mhz: double,
    pub hw: },
    pub overrides: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_stream_parameters {
    pub timing: dml2_timing_cfg,
    pub output: dml2_link_output_cfg,
    pub writeback: dml2_writeback_cfg,
    pub odm_mode: dml2_odm_mode,
    pub disable_dynamic_odm: bool,
    pub disable_subvp: bool,
    pub minimum_vblank_idle_requirement_us: c_int,
    pub uclk_pstate: dml2_twait_budgeting_setting,
    pub fclk_pstate: dml2_twait_budgeting_setting,
    pub stutter_enter_exit: dml2_twait_budgeting_setting,
    pub twait_budgeting: },
    pub hw: },
    pub overrides: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_display_cfg {
    pub gpuvm_enable: bool,
    pub ffbm_enable: bool,
    pub hostvm_enable: bool,
// Allocate DET proportionally between streams based on pixel rate
// and then allocate proportionally between planes.
    pub minimize_det_reallocation: bool,
    pub gpuvm_max_page_table_levels: c_uint,
    pub hostvm_max_non_cached_page_table_levels: c_uint,
    pub plane_descriptors: [dml2_plane_parameters; DML2_MAX_PLANES],
    pub stream_descriptors: [dml2_stream_parameters; DML2_MAX_PLANES],
    pub num_planes: c_uint,
    pub num_streams: c_uint,
// HW specific overrides, there's almost no reason to mess with these
// generally used for debugging or simulation
    pub enable: bool,
    pub value: bool,
    pub force_unbounded_requesting: },
    pub enable: bool,
    pub value: bool,
    pub force_nom_det_size_kbytes: },
    pub 0: bool enable; // So copy time can be forced to,
    pub copy_time_us: c_uint,
    pub force_alt_chan_copy_time: },
    pub 0: bool enable; // So fw delay can be forced to,
    pub fw_delay_us: c_uint,
    pub force_alt_chan_fw_delay: },
    pub mode_support_check_disable: bool,
    pub mcache_admissibility_check_disable: bool,
    pub surface_viewport_size_check_disable: bool,
    pub dlg_ref_clk_mhz: double,
    pub dispclk_mhz: double,
    pub dcfclk_mhz: double,
    pub mode: bool optimize_tdlut_scheduling; // TBD: for DV, will set this to 1, to ensure tdlut schedule is calculated based on address/width,
    pub hw: },
    pub uclk_pstate_change_disable: bool,
    pub fclk_pstate_change_disable: bool,
    pub g6_temp_read_pstate_disable: bool,
    pub g7_ppt_pstate_disable: bool,
    pub power_management: },
    pub enhanced_prefetch_schedule_acceleration: bool,
    pub dcc_programming_assumes_scan_direction_unknown: bool,
    pub synchronize_timings: bool,
    pub synchronize_ddr_displays_for_uclk_pstate_change: bool,
    pub max_outstanding_when_urgent_expected_disable: bool,
    pub programming: bool enable_subvp_implicit_pmo; //enables PMO to switch pipe uclk strategy to subvp, and generate phantom,
    pub all_streams_blanked: bool,
    pub overrides: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pipe_configuration_descriptor {
    pub viewport_x_start: c_uint,
    pub viewport_width: c_uint,
    pub plane0: },
    pub viewport_x_start: c_uint,
    pub viewport_width: c_uint,
    pub plane1: },
    pub plane1_enabled: bool,
    pub imall_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_plane_mcache_configuration_descriptor {
    pub plane_descriptor: *const dml2_plane_parameters,
    pub mcache_allocation: *const dml2_mcache_surface_allocation,
    pub pipe_configurations: [dml2_pipe_configuration_descriptor; DML2_MAX_DCN_PIPES],
    pub num_pipes: c_char,
}
