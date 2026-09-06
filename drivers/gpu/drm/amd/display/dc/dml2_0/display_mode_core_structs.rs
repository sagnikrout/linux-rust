//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/display_mode_core_structs.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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
pub enum dml_project_id {
    dml_project_invalid = 0,
    dml_project_default = 1,
    dml_project_dcn32 = dml_project_default,
    dml_project_dcn321 = 2,
    dml_project_dcn35 = 3,
    dml_project_dcn351 = 4,
    dml_project_dcn401 = 5,
    dml_project_dcn36 = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_prefetch_modes {
    dml_prefetch_support_uclk_fclk_and_stutter_if_possible = 0,
    dml_prefetch_support_uclk_fclk_and_stutter = 1,
    dml_prefetch_support_fclk_and_stutter = 2,
    dml_prefetch_support_stutter = 3,
    dml_prefetch_support_none = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_use_mall_for_pstate_change_mode {
    dml_use_mall_pstate_change_disable = 0,
    dml_use_mall_pstate_change_full_frame = 1,
    dml_use_mall_pstate_change_sub_viewport = 2,
    dml_use_mall_pstate_change_phantom_pipe = 3,
    dml_use_mall_pstate_change_phantom_pipe_no_data_return = 4,
    dml_use_mall_pstate_change_imall = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_use_mall_for_static_screen_mode {
    dml_use_mall_static_screen_disable = 0,
    dml_use_mall_static_screen_enable = 1,
    dml_use_mall_static_screen_optimize = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_encoder_class {
    dml_dp = 0,
    dml_edp = 1,
    dml_dp2p0 = 2,
    dml_hdmi = 3,
    dml_hdmifrl = 4,
    dml_none = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_link_dp_rate {
    dml_dp_rate_na = 0,
    dml_dp_rate_hbr = 1,
    dml_dp_rate_hbr2 = 2,
    dml_dp_rate_hbr3 = 3,
    dml_dp_rate_uhbr10 = 4,
    dml_dp_rate_uhbr13p5 = 5,
    dml_dp_rate_uhbr20 = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_type_and_rate__type {
    dml_output_type_unknown = 0,
    dml_output_type_dp = 1,
    dml_output_type_edp = 2,
    dml_output_type_dp2p0 = 3,
    dml_output_type_hdmi = 4,
    dml_output_type_hdmifrl = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_type_and_rate__rate {
    dml_output_rate_unknown = 0,
    dml_output_rate_dp_rate_hbr = 1,
    dml_output_rate_dp_rate_hbr2 = 2,
    dml_output_rate_dp_rate_hbr3 = 3,
    dml_output_rate_dp_rate_uhbr10 = 4,
    dml_output_rate_dp_rate_uhbr13p5 = 5,
    dml_output_rate_dp_rate_uhbr20 = 6,
    dml_output_rate_hdmi_rate_3x3 = 7,
    dml_output_rate_hdmi_rate_6x3 = 8,
    dml_output_rate_hdmi_rate_6x4 = 9,
    dml_output_rate_hdmi_rate_8x4 = 10,
    dml_output_rate_hdmi_rate_10x4 = 11,
    dml_output_rate_hdmi_rate_12x4 = 12
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_format_class {
    dml_444 = 0,
    dml_s422 = 1,
    dml_n422 = 2,
    dml_420 = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_source_format_class {
    dml_444_8 = 0,
    dml_444_16 = 1,
    dml_444_32 = 2,
    dml_444_64 = 3,
    dml_420_8 = 4,
    dml_420_10 = 5,
    dml_420_12 = 6,
    dml_422_8 = 7,
    dml_422_10 = 8,
    dml_rgbe_alpha = 9,
    dml_rgbe = 10,
    dml_mono_8 = 11,
    dml_mono_16 = 12
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_bpc_class {
    dml_out_6 = 0,
    dml_out_8 = 1,
    dml_out_10 = 2,
    dml_out_12 = 3,
    dml_out_16 = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_output_standard_class {
    dml_std_cvt = 0,
    dml_std_cea = 1,
    dml_std_cvtr2 = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_rotation_angle {
    dml_rotation_0 = 0,
    dml_rotation_90 = 1,
    dml_rotation_180 = 2,
    dml_rotation_270 = 3,
    dml_rotation_0m = 4,
    dml_rotation_90m = 5,
    dml_rotation_180m = 6,
    dml_rotation_270m = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_swizzle_mode {
    dml_sw_linear = 0,
    dml_sw_256b_s = 1,
    dml_sw_256b_d = 2,
    dml_sw_256b_r = 3,
    dml_sw_4kb_z = 4,
    dml_sw_4kb_s = 5,
    dml_sw_4kb_d = 6,
    dml_sw_4kb_r = 7,
    dml_sw_64kb_z = 8,
    dml_sw_64kb_s = 9,
    dml_sw_64kb_d = 10,
    dml_sw_64kb_r = 11,
    dml_sw_256kb_z = 12,
    dml_sw_256kb_s = 13,
    dml_sw_256kb_d = 14,
    dml_sw_256kb_r = 15,
    dml_sw_64kb_z_t = 16,
    dml_sw_64kb_s_t = 17,
    dml_sw_64kb_d_t = 18,
    dml_sw_64kb_r_t = 19,
    dml_sw_4kb_z_x = 20,
    dml_sw_4kb_s_x = 21,
    dml_sw_4kb_d_x = 22,
    dml_sw_4kb_r_x = 23,
    dml_sw_64kb_z_x = 24,
    dml_sw_64kb_s_x = 25,
    dml_sw_64kb_d_x = 26,
    dml_sw_64kb_r_x = 27,
    dml_sw_256kb_z_x = 28,
    dml_sw_256kb_s_x = 29,
    dml_sw_256kb_d_x = 30,
    dml_sw_256kb_r_x = 31,
    dml_sw_256b_2d = 32,
    dml_sw_4kb_2d = 33,
    dml_sw_64kb_2d = 34,
    dml_sw_256kb_2d = 35
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_lb_depth {
    dml_lb_6 = 0,
    dml_lb_8 = 1,
    dml_lb_10 = 2,
    dml_lb_12 = 3,
    dml_lb_16 = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_voltage_state {
    dml_vmin_lv = 0,
    dml_vmin = 1,
    dml_vmid = 2,
    dml_vnom = 3,
    dml_vmax = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_source_macro_tile_size {
    dml_4k_tile = 0,
    dml_64k_tile = 1,
    dml_256k_tile = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_cursor_bpp {
    dml_cur_2bit = 0,
    dml_cur_32bit = 1,
    dml_cur_64bit = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_dram_clock_change_support {
    dml_dram_clock_change_vactive = 0,
    dml_dram_clock_change_vblank = 1,
    dml_dram_clock_change_vblank_drr = 2,
    dml_dram_clock_change_vactive_w_mall_full_frame = 3,
    dml_dram_clock_change_vactive_w_mall_sub_vp = 4,
    dml_dram_clock_change_vblank_w_mall_full_frame = 5,
    dml_dram_clock_change_vblank_drr_w_mall_full_frame = 6,
    dml_dram_clock_change_vblank_w_mall_sub_vp = 7,
    dml_dram_clock_change_vblank_drr_w_mall_sub_vp = 8,
    dml_dram_clock_change_unsupported = 9
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_fclock_change_support {
    dml_fclock_change_vactive = 0,
    dml_fclock_change_vblank = 1,
    dml_fclock_change_unsupported = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_dsc_enable {
    dml_dsc_disable = 0,
    dml_dsc_enable = 1,
    dml_dsc_enable_if_necessary = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_mpc_use_policy {
    dml_mpc_disabled = 0,
    dml_mpc_as_possible = 1,
    dml_mpc_as_needed_for_voltage = 2,
    dml_mpc_as_needed_for_pstate_and_voltage = 3,
    dml_mpc_as_needed = 4,
    dml_mpc_2to1 = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_odm_use_policy {
    dml_odm_use_policy_bypass = 0,
    dml_odm_use_policy_combine_as_needed = 1,
    dml_odm_use_policy_combine_2to1 = 2,
    dml_odm_use_policy_combine_3to1 = 3,
    dml_odm_use_policy_combine_4to1 = 4,
    dml_odm_use_policy_split_1to2 = 5,
    dml_odm_use_policy_mso_1to2 = 6,
    dml_odm_use_policy_mso_1to4 = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_odm_mode {
    dml_odm_mode_bypass = 0,
    dml_odm_mode_combine_2to1 = 1,
    dml_odm_mode_combine_3to1 = 2,
    dml_odm_mode_combine_4to1 = 3,
    dml_odm_mode_split_1to2 = 4,
    dml_odm_mode_mso_1to2 = 5,
    dml_odm_mode_mso_1to4 = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_writeback_configuration {
    dml_whole_buffer_for_single_stream_no_interleave = 0,
    dml_whole_buffer_for_single_stream_interleave = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_immediate_flip_requirement {
    dml_immediate_flip_not_required = 0,
    dml_immediate_flip_required = 1,
    dml_immediate_flip_if_possible = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_unbounded_requesting_policy {
    dml_unbounded_requesting_enable = 0,
    dml_unbounded_requesting_edp_only = 1,
    dml_unbounded_requesting_disable = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_clk_cfg_policy {
    dml_use_required_freq = 0,
    dml_use_override_freq = 1,
    dml_use_state_freq = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_state_bounding_box_st {
    pub socclk_mhz: dml_float_t,
    pub dscclk_mhz: dml_float_t,
    pub phyclk_mhz: dml_float_t,
    pub phyclk_d18_mhz: dml_float_t,
    pub phyclk_d32_mhz: dml_float_t,
    pub dtbclk_mhz: dml_float_t,
    pub fabricclk_mhz: dml_float_t,
    pub dcfclk_mhz: dml_float_t,
    pub dispclk_mhz: dml_float_t,
    pub dppclk_mhz: dml_float_t,
    pub dram_speed_mts: dml_float_t,
    pub urgent_latency_pixel_data_only_us: dml_float_t,
    pub urgent_latency_pixel_mixed_with_vm_data_us: dml_float_t,
    pub urgent_latency_vm_data_only_us: dml_float_t,
    pub writeback_latency_us: dml_float_t,
    pub urgent_latency_adjustment_fabric_clock_component_us: dml_float_t,
    pub urgent_latency_adjustment_fabric_clock_reference_mhz: dml_float_t,
    pub sr_exit_time_us: dml_float_t,
    pub sr_enter_plus_exit_time_us: dml_float_t,
    pub sr_exit_z8_time_us: dml_float_t,
    pub sr_enter_plus_exit_z8_time_us: dml_float_t,
    pub dram_clock_change_latency_us: dml_float_t,
    pub fclk_change_latency_us: dml_float_t,
    pub usr_retraining_latency_us: dml_float_t,
    pub use_ideal_dram_bw_strobe: dml_bool_t,
    pub g6_temp_read_blackout_us: dml_float_t,
    pub urgent_ramp_uclk_cycles: dml_uint_t,
    pub trip_to_memory_uclk_cycles: dml_uint_t,
    pub meta_trip_to_memory_uclk_cycles: dml_uint_t,
    pub maximum_latency_when_urgent_uclk_cycles: dml_uint_t,
    pub average_latency_when_urgent_uclk_cycles: dml_uint_t,
    pub maximum_latency_when_non_urgent_uclk_cycles: dml_uint_t,
    pub average_latency_when_non_urgent_uclk_cycles: dml_uint_t,
    pub dml_dcn401_uclk_dpm_dependent_soc_qos_params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_bounding_box_st {
    pub dprefclk_mhz: dml_float_t,
    pub xtalclk_mhz: dml_float_t,
    pub pcierefclk_mhz: dml_float_t,
    pub refclk_mhz: dml_float_t,
    pub amclk_mhz: dml_float_t,
    pub max_outstanding_reqs: dml_uint_t,
    pub pct_ideal_sdp_bw_after_urgent: dml_float_t,
    pub pct_ideal_fabric_bw_after_urgent: dml_float_t,
    pub pct_ideal_dram_bw_after_urgent_pixel_only: dml_float_t,
    pub pct_ideal_dram_bw_after_urgent_pixel_and_vm: dml_float_t,
    pub pct_ideal_dram_bw_after_urgent_vm_only: dml_float_t,
    pub pct_ideal_dram_bw_after_urgent_strobe: dml_float_t,
    pub max_avg_sdp_bw_use_normal_percent: dml_float_t,
    pub max_avg_fabric_bw_use_normal_percent: dml_float_t,
    pub max_avg_dram_bw_use_normal_percent: dml_float_t,
    pub max_avg_dram_bw_use_normal_strobe_percent: dml_float_t,
    pub svp_prefetch_pct_ideal_sdp_bw_after_urgent: dml_float_t,
    pub svp_prefetch_pct_ideal_fabric_bw_after_urgent: dml_float_t,
    pub svp_prefetch_pct_ideal_dram_bw_after_urgent_pixel_only: dml_float_t,
    pub svp_prefetch_pct_ideal_dram_bw_after_urgent_pixel_and_vm: dml_float_t,
    pub svp_prefetch_pct_ideal_dram_bw_after_urgent_vm_only: dml_float_t,
    pub svp_prefetch_max_avg_sdp_bw_use_normal_percent: dml_float_t,
    pub svp_prefetch_max_avg_fabric_bw_use_normal_percent: dml_float_t,
    pub svp_prefetch_max_avg_dram_bw_use_normal_percent: dml_float_t,
    pub round_trip_ping_latency_dcfclk_cycles: dml_uint_t,
    pub urgent_out_of_order_return_per_channel_pixel_only_bytes: dml_uint_t,
    pub urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: dml_uint_t,
    pub urgent_out_of_order_return_per_channel_vm_only_bytes: dml_uint_t,
    pub num_chans: dml_uint_t,
    pub return_bus_width_bytes: dml_uint_t,
    pub dram_channel_width_bytes: dml_uint_t,
    pub fabric_datapath_to_dcn_data_return_bytes: dml_uint_t,
    pub hostvm_min_page_size_kbytes: dml_uint_t,
    pub gpuvm_min_page_size_kbytes: dml_uint_t,
    pub phy_downspread_percent: dml_float_t,
    pub dcn_downspread_percent: dml_float_t,
    pub smn_latency_us: dml_float_t,
    pub mall_allocated_for_dcn_mbytes: dml_uint_t,
    pub dispclk_dppclk_vco_speed_mhz: dml_float_t,
    pub do_urgent_latency_adjustment: dml_bool_t,
    pub mem_word_bytes: dml_uint_t,
    pub num_dcc_mcaches: dml_uint_t,
    pub mcache_size_bytes: dml_uint_t,
    pub mcache_line_size_bytes: dml_uint_t,
    pub UseNewDCN401SOCParameters: dml_bool_t,
    pub df_qos_response_time_fclk_cycles: dml_uint_t,
    pub max_round_trip_to_furthest_cs_fclk_cycles: dml_uint_t,
    pub mall_overhead_fclk_cycles: dml_uint_t,
    pub meta_trip_adder_fclk_cycles: dml_uint_t,
    pub average_transport_distance_fclk_cycles: dml_uint_t,
    pub umc_urgent_ramp_latency_margin: dml_float_t,
    pub umc_max_latency_margin: dml_float_t,
    pub umc_average_latency_margin: dml_float_t,
    pub fabric_max_transport_latency_margin: dml_float_t,
    pub fabric_average_transport_latency_margin: dml_float_t,
    pub dml_dcn401_soc_qos_params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_params_st {
    pub vblank_nom_default_us: dml_uint_t,
    pub rob_buffer_size_kbytes: dml_uint_t,
    pub config_return_buffer_size_in_kbytes: dml_uint_t,
    pub config_return_buffer_segment_size_in_kbytes: dml_uint_t,
    pub compressed_buffer_segment_size_in_kbytes: dml_uint_t,
    pub meta_fifo_size_in_kentries: dml_uint_t,
    pub zero_size_buffer_entries: dml_uint_t,
    pub dpte_buffer_size_in_pte_reqs_luma: dml_uint_t,
    pub dpte_buffer_size_in_pte_reqs_chroma: dml_uint_t,
    pub dcc_meta_buffer_size_bytes: dml_uint_t,
    pub gpuvm_enable: dml_bool_t,
    pub hostvm_enable: dml_bool_t,
    pub gpuvm_max_page_table_levels: dml_uint_t,
    pub hostvm_max_page_table_levels: dml_uint_t,
    pub pixel_chunk_size_kbytes: dml_uint_t,
    pub alpha_pixel_chunk_size_kbytes: dml_uint_t,
    pub min_pixel_chunk_size_bytes: dml_uint_t,
    pub meta_chunk_size_kbytes: dml_uint_t,
    pub min_meta_chunk_size_bytes: dml_uint_t,
    pub writeback_chunk_size_kbytes: dml_uint_t,
    pub line_buffer_size_bits: dml_uint_t,
    pub max_line_buffer_lines: dml_uint_t,
    pub writeback_interface_buffer_size_kbytes: dml_uint_t,
    pub max_num_dpp: dml_uint_t,
    pub max_num_otg: dml_uint_t,
    pub max_num_wb: dml_uint_t,
    pub max_dchub_pscl_bw_pix_per_clk: dml_uint_t,
    pub max_pscl_lb_bw_pix_per_clk: dml_uint_t,
    pub max_lb_vscl_bw_pix_per_clk: dml_uint_t,
    pub max_vscl_hscl_bw_pix_per_clk: dml_uint_t,
    pub max_hscl_ratio: dml_float_t,
    pub max_vscl_ratio: dml_float_t,
    pub max_hscl_taps: dml_uint_t,
    pub max_vscl_taps: dml_uint_t,
    pub num_dsc: dml_uint_t,
    pub maximum_dsc_bits_per_component: dml_uint_t,
    pub maximum_pixels_per_line_per_dsc_unit: dml_uint_t,
    pub dsc422_native_support: dml_bool_t,
    pub cursor_64bpp_support: dml_bool_t,
    pub dispclk_ramp_margin_percent: dml_float_t,
    pub dppclk_delay_subtotal: dml_uint_t,
    pub dppclk_delay_scl: dml_uint_t,
    pub dppclk_delay_scl_lb_only: dml_uint_t,
    pub dppclk_delay_cnvc_formatter: dml_uint_t,
    pub dppclk_delay_cnvc_cursor: dml_uint_t,
    pub cursor_buffer_size: dml_uint_t,
    pub cursor_chunk_size: dml_uint_t,
    pub dispclk_delay_subtotal: dml_uint_t,
    pub dynamic_metadata_vm_enabled: dml_bool_t,
    pub max_inter_dcn_tile_repeaters: dml_uint_t,
    pub max_num_hdmi_frl_outputs: dml_uint_t,
    pub max_num_dp2p0_outputs: dml_uint_t,
    pub max_num_dp2p0_streams: dml_uint_t,
    pub dcc_supported: dml_bool_t,
    pub ptoi_supported: dml_bool_t,
    pub writeback_max_hscl_ratio: dml_float_t,
    pub writeback_max_vscl_ratio: dml_float_t,
    pub writeback_min_hscl_ratio: dml_float_t,
    pub writeback_min_vscl_ratio: dml_float_t,
    pub writeback_max_hscl_taps: dml_uint_t,
    pub writeback_max_vscl_taps: dml_uint_t,
    pub writeback_line_buffer_buffer_size: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DmlPipe {
    pub Dppclk: dml_float_t,
    pub Dispclk: dml_float_t,
    pub PixelClock: dml_float_t,
    pub DCFClkDeepSleep: dml_float_t,
    pub DPPPerSurface: dml_uint_t,
    pub ScalerEnabled: dml_bool_t,
    pub SourceScan: dml_rotation_angle,
    pub ViewportHeight: dml_uint_t,
    pub ViewportHeightChroma: dml_uint_t,
    pub BlockWidth256BytesY: dml_uint_t,
    pub BlockHeight256BytesY: dml_uint_t,
    pub BlockWidth256BytesC: dml_uint_t,
    pub BlockHeight256BytesC: dml_uint_t,
    pub BlockWidthY: dml_uint_t,
    pub BlockHeightY: dml_uint_t,
    pub BlockWidthC: dml_uint_t,
    pub BlockHeightC: dml_uint_t,
    pub InterlaceEnable: dml_uint_t,
    pub NumberOfCursors: dml_uint_t,
    pub VBlank: dml_uint_t,
    pub HTotal: dml_uint_t,
    pub HActive: dml_uint_t,
    pub DCCEnable: dml_bool_t,
    pub ODMMode: dml_odm_mode,
    pub SourcePixelFormat: dml_source_format_class,
    pub SurfaceTiling: dml_swizzle_mode,
    pub BytePerPixelY: dml_uint_t,
    pub BytePerPixelC: dml_uint_t,
    pub ProgressiveToInterlaceUnitInOPP: dml_bool_t,
    pub VRatio: dml_float_t,
    pub VRatioChroma: dml_float_t,
    pub VTaps: dml_uint_t,
    pub VTapsChroma: dml_uint_t,
    pub PitchY: dml_uint_t,
    pub DCCMetaPitchY: dml_uint_t,
    pub PitchC: dml_uint_t,
    pub DCCMetaPitchC: dml_uint_t,
    pub ViewportStationary: dml_bool_t,
    pub ViewportXStart: dml_uint_t,
    pub ViewportYStart: dml_uint_t,
    pub ViewportXStartC: dml_uint_t,
    pub ViewportYStartC: dml_uint_t,
    pub FORCE_ONE_ROW_FOR_FRAME: dml_bool_t,
    pub SwathHeightY: dml_uint_t,
    pub SwathHeightC: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Watermarks {
    pub UrgentWatermark: dml_float_t,
    pub WritebackUrgentWatermark: dml_float_t,
    pub DRAMClockChangeWatermark: dml_float_t,
    pub FCLKChangeWatermark: dml_float_t,
    pub WritebackDRAMClockChangeWatermark: dml_float_t,
    pub WritebackFCLKChangeWatermark: dml_float_t,
    pub StutterExitWatermark: dml_float_t,
    pub StutterEnterPlusExitWatermark: dml_float_t,
    pub Z8StutterExitWatermark: dml_float_t,
    pub Z8StutterEnterPlusExitWatermark: dml_float_t,
    pub USRRetrainingWatermark: dml_float_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SOCParametersList {
    pub UrgentLatency: dml_float_t,
    pub ExtraLatency: dml_float_t,
    pub WritebackLatency: dml_float_t,
    pub DRAMClockChangeLatency: dml_float_t,
    pub FCLKChangeLatency: dml_float_t,
    pub SRExitTime: dml_float_t,
    pub SREnterPlusExitTime: dml_float_t,
    pub SRExitZ8Time: dml_float_t,
    pub SREnterPlusExitZ8Time: dml_float_t,
    pub USRRetrainingLatency: dml_float_t,
    pub SMNLatency: dml_float_t,
}

// @brief Struct that represent Plane configration of a display cfg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_plane_cfg_st {
//
// Pipe/Surface Parameters
//
    pub enable: dml_bool_t GPUVMEnable; /// <brief Set if any pipe has GPUVM,
    pub enable: dml_bool_t HostVMEnable; /// <brief Set if any pipe has HostVM,
    pub pipes': dml_uint_t GPUVMMaxPageTableLevels; /// <brief GPUVM level; max of all,
    pub level: dml_uint_t HostVMMaxPageTableLevels; /// <brief HostVM level; max of all pipes'; that is the number of non-cache HVM,
    pub GPUVMMinPageSizeKBytes: [dml_uint_t; __DML_NUM_PLANES__],
    pub ForceOneRowForFrame: [dml_bool_t; __DML_NUM_PLANES__],
    pub is: dml_bool_t PTEBufferModeOverrideEn[__DML_NUM_PLANES__]; //< brief when override enable; the DML will only check the given pte buffer and will use the pte buffer mode as,
    pub PTEBufferMode: [dml_bool_t; __DML_NUM_PLANES__],
    pub ViewportWidth: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportHeight: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportWidthChroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportHeightChroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportXStart: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportXStartC: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportYStart: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportYStartC: [dml_uint_t; __DML_NUM_PLANES__],
    pub ViewportStationary: [dml_bool_t; __DML_NUM_PLANES__],
    pub ScalerEnabled: [dml_bool_t; __DML_NUM_PLANES__],
    pub HRatio: [dml_float_t; __DML_NUM_PLANES__],
    pub VRatio: [dml_float_t; __DML_NUM_PLANES__],
    pub HRatioChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub VRatioChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub HTaps: [dml_uint_t; __DML_NUM_PLANES__],
    pub VTaps: [dml_uint_t; __DML_NUM_PLANES__],
    pub HTapsChroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub VTapsChroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub LBBitPerPixel: [dml_uint_t; __DML_NUM_PLANES__],
    pub SourceScan: [dml_rotation_angle; __DML_NUM_PLANES__],
    pub ScalerRecoutWidth: [dml_uint_t; __DML_NUM_PLANES__],
    pub DynamicMetadataEnable: [dml_bool_t; __DML_NUM_PLANES__],
    pub DynamicMetadataLinesBeforeActiveRequired: [dml_uint_t; __DML_NUM_PLANES__],
    pub DynamicMetadataTransmittedBytes: [dml_uint_t; __DML_NUM_PLANES__],
    pub per-plane: dml_uint_t DETSizeOverride[__DML_NUM_PLANES__]; /// <brief user can specify the desire DET buffer usage,
    pub NumberOfCursors: [dml_uint_t; __DML_NUM_PLANES__],
    pub CursorWidth: [dml_uint_t; __DML_NUM_PLANES__],
    pub CursorBPP: [dml_uint_t; __DML_NUM_PLANES__],
    pub setup_for_tdlut: [dml_bool_t; __DML_NUM_PLANES__],
    pub tdlut_addressing_mode: [dml2_tdlut_addressing_mode; __DML_NUM_PLANES__],
    pub tdlut_width_mode: [dml2_tdlut_width_mode; __DML_NUM_PLANES__],
    pub UseMALLForStaticScreen: [dml_use_mall_for_static_screen_mode; __DML_NUM_PLANES__],
    pub UseMALLForPStateChange: [dml_use_mall_for_pstate_change_mode; __DML_NUM_PLANES__],
    pub etc.: dml_uint_t BlendingAndTiming[__DML_NUM_PLANES__]; /// <brief From which timing group (like OTG) that this plane is getting its timing from. Mode check also need this info for example to check num OTG; encoder; dsc,
}

// @brief Surface Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_surface_cfg_st {
    pub SurfaceTiling: [dml_swizzle_mode; __DML_NUM_PLANES__],
    pub SourcePixelFormat: [dml_source_format_class; __DML_NUM_PLANES__],
    pub PitchY: [dml_uint_t; __DML_NUM_PLANES__],
    pub SurfaceWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub SurfaceHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PitchC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SurfaceWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SurfaceHeightC: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCEnable: [dml_bool_t; __DML_NUM_PLANES__],
    pub DCCMetaPitchY: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCMetaPitchC: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCRateLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub DCCRateChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub DCCFractionOfZeroSizeRequestsLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub DCCFractionOfZeroSizeRequestsChroma: [dml_float_t; __DML_NUM_PLANES__],
}

// @brief structure that represents the timing configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_timing_cfg_st {
    pub HTotal: [dml_uint_t; __DML_NUM_PLANES__],
    pub VTotal: [dml_uint_t; __DML_NUM_PLANES__],
    pub HBlankEnd: [dml_uint_t; __DML_NUM_PLANES__],
    pub VBlankEnd: [dml_uint_t; __DML_NUM_PLANES__],
    pub RefreshRate: [dml_uint_t; __DML_NUM_PLANES__],
    pub VFrontPorch: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelClock: [dml_float_t; __DML_NUM_PLANES__],
    pub HActive: [dml_uint_t; __DML_NUM_PLANES__],
    pub VActive: [dml_uint_t; __DML_NUM_PLANES__],
    pub Interlace: [dml_bool_t; __DML_NUM_PLANES__],
    pub DRRDisplay: [dml_bool_t; __DML_NUM_PLANES__],
    pub VBlankNom: [dml_uint_t; __DML_NUM_PLANES__],
}

// @brief structure that represents the output stream
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_output_cfg_st {
// Output Setting
    pub DSCInputBitPerComponent: [dml_uint_t; __DML_NUM_PLANES__],
    pub OutputFormat: [dml_output_format_class; __DML_NUM_PLANES__],
    pub OutputEncoder: [dml_output_encoder_class; __DML_NUM_PLANES__],
    pub OutputMultistreamId: [dml_uint_t; __DML_NUM_PLANES__],
    pub OutputMultistreamEn: [dml_bool_t; __DML_NUM_PLANES__],
    pub (support.OutputBpp): dml_float_t OutputBpp[__DML_NUM_PLANES__]; //< brief Use by mode_programming to specify a output bpp; user can use the output from mode_support,
    pub PixelClockBackEnd: [dml_float_t; __DML_NUM_PLANES__],
    pub required: dml_dsc_enable DSCEnable[__DML_NUM_PLANES__]; //< brief for mode support check; use to determine if dsc is,
    pub OutputLinkDPLanes: [dml_uint_t; __DML_NUM_PLANES__],
    pub OutputLinkDPRate: [dml_output_link_dp_rate; __DML_NUM_PLANES__],
    pub ForcedOutputLinkBPP: [dml_float_t; __DML_NUM_PLANES__],
    pub AudioSampleRate: [dml_uint_t; __DML_NUM_PLANES__],
    pub AudioSampleLayout: [dml_uint_t; __DML_NUM_PLANES__],
    pub OutputDisabled: [dml_bool_t; __DML_NUM_PLANES__],
    pub DSCSlices: [dml_uint_t; __DML_NUM_PLANES__],
}

// @brief Writeback Setting
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_writeback_cfg_st {
    pub WritebackPixelFormat: [dml_source_format_class; __DML_NUM_PLANES__],
    pub WritebackEnable: [dml_bool_t; __DML_NUM_PLANES__],
    pub ActiveWritebacksPerSurface: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackDestinationWidth: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackDestinationHeight: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackSourceWidth: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackSourceHeight: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackHTaps: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackVTaps: [dml_uint_t; __DML_NUM_PLANES__],
    pub WritebackHRatio: [dml_float_t; __DML_NUM_PLANES__],
    pub WritebackVRatio: [dml_float_t; __DML_NUM_PLANES__],
}

// @brief Hardware resource specific; mainly used by mode_programming when test/sw wants to do some specific setting
// which are not the same as what the mode support stage derive.  When call mode_support with mode_programm; the hw-specific
// resource will be set to what the mode_support layer recommends
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_hw_resource_st {
    pub stage: dml_odm_mode ODMMode[__DML_NUM_PLANES__]; /// <brief ODM mode that is chosen in the mode check stage and will be used in mode programming,
    pub 4.: dml_uint_t DPPPerSurface[__DML_NUM_PLANES__]; /// <brief How many DPPs are needed drive the surface to output. If MPCC or ODMC could be 2 or,
    pub mode_programming: dml_bool_t DSCEnabled[__DML_NUM_PLANES__]; /// <brief Indicate if the DSC is enabled; used in,
    pub mode: dml_uint_t NumberOfDSCSlices[__DML_NUM_PLANES__]; /// <brief Indicate how many slices needed to support the given,
    pub timer: dml_float_t DLGRefClkFreqMHz; /// <brief DLG Global Reference,
}

// @brief To control the clk usage for model programming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_clk_cfg_st {
    pub freq: dml_clk_cfg_policy dcfclk_option; ///< brief Use for mode_program; user can select between use the min require clk req as calculated by DML or use the test-specific,
    pub freq: dml_clk_cfg_policy dispclk_option; ///< brief Use for mode_program; user can select between use the min require clk req as calculated by DML or use the test-specific,
    pub dppclk_option: [dml_clk_cfg_policy; __DML_NUM_PLANES__],
    pub dcfclk_mhz: dml_float_t,
    pub dispclk_mhz: dml_float_t,
    pub dppclk_mhz: [dml_float_t; __DML_NUM_PLANES__],
}

// @brief DML display configuration.
// Describe how to display a surface in multi-plane setup and output to different output and writeback using the specified timgin
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_display_cfg_st {
    pub surface: dml_surface_cfg_st,
    pub plane: dml_plane_cfg_st,
    pub timing: dml_timing_cfg_st,
    pub output: dml_output_cfg_st,
    pub writeback: dml_writeback_cfg_st,
    pub num_surfaces: c_uint,
    pub num_timings: c_uint,
    pub programming: dml_hw_resource_st hw; //< brief for mode,
    pub override: dml_clk_cfg_st clk_overrides; //< brief for mode programming clk,
}

// @brief DML mode evaluation and programming policy
// Those knobs that affect mode support and mode programming
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_mode_eval_policy_st {
// -------------------
// Policy
// -------------------
    pub stage: dml_mpc_use_policy MPCCombineUse[__DML_NUM_PLANES__]; /// <brief MPC Combine mode as selected by the user; used in mode check,
    pub stage: dml_odm_use_policy ODMUse[__DML_NUM_PLANES__]; /// <brief ODM mode as selected by the user; used in mode check,
    pub preference: dml_unbounded_requesting_policy UseUnboundedRequesting; ///< brief Unbounded request mode,
    pub regardless: dml_immediate_flip_requirement ImmediateFlipRequirement[__DML_NUM_PLANES__]; /// <brief Is immediate flip a requirement for this plane. When host vm is present iflip is needed,
    pub vblank: dml_prefetch_modes AllowForPStateChangeOrStutterInVBlank[__DML_NUM_PLANES__]; /// <brief To specify if the DML should calculate the values for support different pwr saving features (cstate; pstate; etc.) during,
    pub AllowForPStateChangeOrStutterInVBlankFinal: dml_prefetch_modes,
    pub UseOnlyMaxPrefetchModes: bool,
    pub programming.: dml_bool_t UseMinimumRequiredDCFCLK; //<brief When set the mode_check stage will figure the min DCFCLK freq to support the given display configuration. User can tell use the output DCFCLK for mode,
    pub DRAMClockChangeRequirementFinal: dml_bool_t,
    pub FCLKChangeRequirementFinal: dml_bool_t,
    pub USRRetrainingRequiredFinal: dml_bool_t,
    pub EnhancedPrefetchScheduleAccelerationFinal: dml_bool_t,
    pub size: dml_bool_t NomDETInKByteOverrideEnable; //<brief Nomimal DET buffer size for a pipe. If this size fit the required 2 swathes; DML will use this DET,
    pub NomDETInKByteOverrideValue: dml_uint_t,
    pub DCCProgrammingAssumesScanDirectionUnknownFinal: dml_bool_t,
    pub SynchronizeTimingsFinal: dml_bool_t,
    pub SynchronizeDRRDisplaysForUCLKPStateChangeFinal: dml_bool_t,
    pub state): dml_bool_t AssumeModeSupportAtMaxPwrStateEvenDRAMClockChangeNotSupported; //<brief if set; the mode support will say mode is supported even though the DRAM clock change is not support (assuming the soc will be stay in max power,
    pub state: dml_bool_t AssumeModeSupportAtMaxPwrStateEvenFClockChangeNotSupported; //<brief if set; the mode support will say mode is supported even though the Fabric clock change is not support (assuming the soc will be stay in max power,
}

// @brief Contains important information after the mode support steps. Also why a mode is not supported.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_mode_support_info_st {
// -----------------
// Mode Support Information
// -----------------
    pub setting: dml_bool_t ModeIsSupported; //<brief Is the mode support any voltage and combine,
    pub programming: dml_bool_t ImmediateFlipSupport; //<brief Means mode support immediate flip at the max combine setting; determine in mode support and used in mode,
    pub 1: dml_uint_t MaximumMPCCombine; //<brief If using MPC combine helps the power saving support; then this will be set to,
    pub UnboundedRequestEnabled: dml_bool_t,
    pub CompressedBufferSizeInkByte: dml_uint_t,
// Mode Support Reason
    pub WritebackLatencySupport: dml_bool_t,
    pub ScaleRatioAndTapsSupport: dml_bool_t,
    pub SourceFormatPixelAndScanSupport: dml_bool_t,
    pub MPCCombineMethodIncompatible: dml_bool_t,
    pub P2IWith420: dml_bool_t,
    pub DSCOnlyIfNecessaryWithBPP: dml_bool_t,
    pub DSC422NativeNotSupported: dml_bool_t,
    pub LinkRateDoesNotMatchDPVersion: dml_bool_t,
    pub LinkRateForMultistreamNotIndicated: dml_bool_t,
    pub BPPForMultistreamNotIndicated: dml_bool_t,
    pub MultistreamWithHDMIOreDP: dml_bool_t,
    pub MSOOrODMSplitWithNonDPLink: dml_bool_t,
    pub NotEnoughLanesForMSO: dml_bool_t,
    pub NumberOfOTGSupport: dml_bool_t,
    pub NumberOfHDMIFRLSupport: dml_bool_t,
    pub NumberOfDP2p0Support: dml_bool_t,
    pub NonsupportedDSCInputBPC: dml_bool_t,
    pub WritebackScaleRatioAndTapsSupport: dml_bool_t,
    pub CursorSupport: dml_bool_t,
    pub PitchSupport: dml_bool_t,
    pub ViewportExceedsSurface: dml_bool_t,
    pub ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified: dml_bool_t,
    pub ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe: dml_bool_t,
    pub InvalidCombinationOfMALLUseForPStateAndStaticScreen: dml_bool_t,
    pub InvalidCombinationOfMALLUseForPState: dml_bool_t,
    pub ExceededMALLSize: dml_bool_t,
    pub EnoughWritebackUnits: dml_bool_t,
    pub ExceededMultistreamSlots: dml_bool_t,
    pub ODMCombineTwoToOneSupportCheckOK: dml_bool_t,
    pub ODMCombineFourToOneSupportCheckOK: dml_bool_t,
    pub NotEnoughDSCUnits: dml_bool_t,
    pub NotEnoughDSCSlices: dml_bool_t,
    pub PixelsPerLinePerDSCUnitSupport: dml_bool_t,
    pub DSCCLKRequiredMoreThanSupported: dml_bool_t,
    pub DTBCLKRequiredMoreThanSupported: dml_bool_t,
    pub LinkCapacitySupport: dml_bool_t,
    pub ROBSupport: [dml_bool_t; 2],
    pub PTEBufferSizeNotExceeded: [dml_bool_t; 2],
    pub DCCMetaBufferSizeNotExceeded: [dml_bool_t; 2],
    pub TotalVerticalActiveBandwidthSupport: [dml_bool_t; 2],
    pub DRAMClockChangeSupport: [dml_dram_clock_change_support; 2],
    pub ActiveDRAMClockChangeLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
    pub SubViewportLinesNeededInMALL: [dml_uint_t; __DML_NUM_PLANES__],
    pub FCLKChangeSupport: [dml_fclock_change_support; 2],
    pub USRRetrainingSupport: [dml_bool_t; 2],
    pub VActiveBandwithSupport: [dml_bool_t; 2],
    pub PrefetchSupported: [dml_bool_t; 2],
    pub DynamicMetadataSupported: [dml_bool_t; 2],
    pub VRatioInPrefetchSupported: [dml_bool_t; 2],
    pub DISPCLK_DPPCLK_Support: [dml_bool_t; 2],
    pub TotalAvailablePipesSupport: [dml_bool_t; 2],
    pub ModeSupport: [dml_bool_t; 2],
    pub ViewportSizeSupport: [dml_bool_t; 2],
    pub ImmediateFlipSupportedForState: [dml_bool_t; 2],
    pub NoTimeForPrefetch: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub NoTimeForDynamicMetadata: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub setting: dml_bool_t MPCCombineEnable[__DML_NUM_PLANES__]; /// <brief Indicate if the MPC Combine enable in the given state and optimize mpc combine,
    pub stage: dml_odm_mode ODMMode[__DML_NUM_PLANES__]; /// <brief ODM mode that is chosen in the mode check stage and will be used in mode programming,
    pub 4.: dml_uint_t DPPPerSurface[__DML_NUM_PLANES__]; /// <brief How many DPPs are needed drive the surface to output. If MPCC or ODMC could be 2 or,
    pub mode_programming: dml_bool_t DSCEnabled[__DML_NUM_PLANES__]; /// <brief Indicate if the DSC is actually required; used in,
    pub required: dml_bool_t FECEnabled[__DML_NUM_PLANES__]; /// <brief Indicate if the FEC is actually,
    pub mode: dml_uint_t NumberOfDSCSlices[__DML_NUM_PLANES__]; /// <brief Indicate how many slices needed to support the given,
    pub OutputBpp: [dml_float_t; __DML_NUM_PLANES__],
    pub OutputType: [dml_output_type_and_rate__type; __DML_NUM_PLANES__],
    pub OutputRate: [dml_output_type_and_rate__rate; __DML_NUM_PLANES__],
    pub setting: dml_float_t AlignedDCCMetaPitchY[__DML_NUM_PLANES__]; /// <brief Pitch value that is aligned to tiling,
    pub AlignedDCCMetaPitchC: [dml_float_t; __DML_NUM_PLANES__],
    pub AlignedYPitch: [dml_float_t; __DML_NUM_PLANES__],
    pub AlignedCPitch: [dml_float_t; __DML_NUM_PLANES__],
    pub display: dml_float_t MaxTotalVerticalActiveAvailableBandwidth[2]; /// <brief nominal bw available for,
}

// @brief Treat this as the intermediate values and outputs of mode check function. User can query the content of the struct to know more about the result of mode evaluation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_support_st {
    pub ip: ip_params_st,
    pub soc: soc_bounding_box_st,
    pub compute: soc_state_bounding_box_st state; //<brief Per-state bbox values; only 1 state per,
    pub policy: dml_mode_eval_policy_st,
    pub computation: dml_uint_t state_idx; //<brief The power state idx for the power state under this,
    pub idx: dml_uint_t max_state_idx; //<brief The MAX power state,
    pub if: soc_state_bounding_box_st max_state; //<brief The MAX power state; some algo needs to know the max state info to determine,
    pub consideration: dml_display_cfg_st cache_display_cfg; // <brief A copy of the current display cfg in,
// Physical info; only using for programming
    pub display_cfg: dml_uint_t num_active_planes; // <brief As determined by either e2e_pipe_param or,
// Calculated Clocks
    pub etc.: dml_float_t RequiredDISPCLK[2]; /// <brief Required DISPCLK; depends on pixel rate; odm mode,
    pub RequiredDPPCLKThisState: [dml_float_t; __DML_NUM_PLANES__],
    pub support: dml_float_t DCFCLKState[2]; /// <brief recommended DCFCLK freq; calculated by DML. If UseMinimumRequiredDCFCLK is not set; then it will be just the state DCFCLK; else it will min DCFCLK for,
    pub RequiredDISPCLKPerSurface: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub RequiredDPPCLKPerSurface: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub state: dml_float_t FabricClock; /// <brief Basically just the clock freq at the min (or given),
    pub state: dml_float_t DRAMSpeed; /// <brief Basically just the clock freq at the min (or given),
    pub state: dml_float_t SOCCLK; /// <brief Basically just the clock freq at the min (or given),
    pub setting: dml_float_t DCFCLK; /// <brief Basically just the clock freq at the min (or given) state and max combine,
    pub pipes: dml_float_t GlobalDPPCLK; /// <brief the Max DPPCLK freq out of all,
// ----------------------------------
// Mode Support Info and fail reason
// ----------------------------------
    pub support: dml_mode_support_info_st,
// These are calculated before the ModeSupport and ModeProgram step
// They represent the bound for the return buffer sizing
    pub MaxTotalDETInKByte: dml_uint_t,
    pub NomDETInKByte: dml_uint_t,
    pub MinCompressedBufferSizeInKByte: dml_uint_t,
// Info obtained at the end of mode support calculations
// The reported info is at the "optimal" state and combine setting
    pub ReturnBW: dml_float_t,
    pub ReturnDRAMBW: dml_float_t,
    pub value.: dml_uint_t DETBufferSizeInKByte[__DML_NUM_PLANES__]; // <brief Recommended DET size configuration for this plane. All pipes under this plane should program the DET buffer size to the calculated,
    pub DETBufferSizeY: [dml_uint_t; __DML_NUM_PLANES__],
    pub DETBufferSizeC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathHeightC: [dml_uint_t; __DML_NUM_PLANES__],
// ----------------------------------
// Intermediates/Informational
// ----------------------------------
    pub TotImmediateFlipBytes: dml_uint_t,
    pub DCCEnabledInAnySurface: dml_bool_t,
    pub WritebackRequiredDISPCLK: dml_float_t,
    pub TimeCalc: dml_float_t,
    pub TWait: dml_float_t,
    pub SwathWidthYAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub SwathWidthCAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub SwathHeightYAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub SwathHeightCAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub SwathWidthYThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathWidthCThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathHeightYThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathHeightCThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub DETBufferSizeInKByteAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub DETBufferSizeYAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub DETBufferSizeCAllStates: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub UnboundedRequestEnabledAllStates: [dml_bool_t; 2],
    pub CompressedBufferSizeInkByteAllStates: [dml_uint_t; 2],
    pub UnboundedRequestEnabledThisState: dml_bool_t,
    pub CompressedBufferSizeInkByteThisState: dml_uint_t,
    pub DETBufferSizeInKByteThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub DETBufferSizeYThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub DETBufferSizeCThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub VRatioPreY: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub VRatioPreC: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub swath_width_luma_ub_all_states: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub swath_width_chroma_ub_all_states: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub swath_width_luma_ub_this_state: [dml_uint_t; __DML_NUM_PLANES__],
    pub swath_width_chroma_ub_this_state: [dml_uint_t; __DML_NUM_PLANES__],
    pub RequiredSlots: [dml_uint_t; __DML_NUM_PLANES__],
    pub PDEAndMetaPTEBytesPerFrame: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub MetaRowBytes: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub DPTEBytesPerRow: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub PrefetchLinesY: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub PrefetchLinesC: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub prefetch: dml_uint_t MaxNumSwY[__DML_NUM_PLANES__]; /// <brief Max number of swath for,
    pub prefetch: dml_uint_t MaxNumSwC[__DML_NUM_PLANES__]; /// <brief Max number of swath for,
    pub PrefillY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PrefillC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PrefetchLinesYThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub PrefetchLinesCThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub DPTEBytesPerRowThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub PDEAndMetaPTEBytesPerFrameThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub MetaRowBytesThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub use_one_row_for_frame: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub use_one_row_for_frame_flip: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub use_one_row_for_frame_this_state: [dml_bool_t; __DML_NUM_PLANES__],
    pub use_one_row_for_frame_flip_this_state: [dml_bool_t; __DML_NUM_PLANES__],
    pub LineTimesForPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub LinesForMetaPTE: [dml_float_t; __DML_NUM_PLANES__],
    pub LinesForMetaAndDPTERow: [dml_float_t; __DML_NUM_PLANES__],
    pub SwathWidthYSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
    pub SwathWidthCSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
    pub BytePerPixelY: [dml_uint_t; __DML_NUM_PLANES__],
    pub BytePerPixelC: [dml_uint_t; __DML_NUM_PLANES__],
    pub BytePerPixelInDETY: [dml_float_t; __DML_NUM_PLANES__],
    pub BytePerPixelInDETC: [dml_float_t; __DML_NUM_PLANES__],
    pub Read256BlockHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub Read256BlockWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub Read256BlockHeightC: [dml_uint_t; __DML_NUM_PLANES__],
    pub Read256BlockWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub MacroTileHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub MacroTileHeightC: [dml_uint_t; __DML_NUM_PLANES__],
    pub MacroTileWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub MacroTileWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PSCL_FACTOR: [dml_float_t; __DML_NUM_PLANES__],
    pub PSCL_FACTOR_CHROMA: [dml_float_t; __DML_NUM_PLANES__],
    pub MaximumSwathWidthLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub MaximumSwathWidthChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub Tno_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub DestinationLinesToRequestVMInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub DestinationLinesToRequestRowInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub WritebackDelayTime: [dml_float_t; __DML_NUM_PLANES__],
    pub dpte_group_bytes: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_height: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub UrgLatency: dml_float_t,
    pub UrgentBurstFactorCursor: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub UrgentBurstFactorCursorPre: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgentBurstFactorLuma: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub UrgentBurstFactorLumaPre: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgentBurstFactorChroma: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub UrgentBurstFactorChromaPre: [dml_float_t; __DML_NUM_PLANES__],
    pub MaximumSwathWidthInLineBufferLuma: dml_float_t,
    pub MaximumSwathWidthInLineBufferChroma: dml_float_t,
    pub ExtraLatency: dml_float_t,
// Backend
    pub RequiresDSC: [dml_bool_t; __DML_NUM_PLANES__],
    pub RequiresFEC: [dml_bool_t; __DML_NUM_PLANES__],
    pub OutputBppPerState: [dml_float_t; __DML_NUM_PLANES__],
    pub DSCDelayPerState: [dml_uint_t; __DML_NUM_PLANES__],
    pub OutputTypePerState: [dml_output_type_and_rate__type; __DML_NUM_PLANES__],
    pub OutputRatePerState: [dml_output_type_and_rate__rate; __DML_NUM_PLANES__],
// Bandwidth Related Info
    pub BandwidthAvailableForImmediateFlip: dml_float_t,
    pub ReadBandwidthLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub ReadBandwidthChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub WriteBandwidth: [dml_float_t; __DML_NUM_PLANES__],
    pub RequiredPrefetchPixelDataBWLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub RequiredPrefetchPixelDataBWChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub cursor_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub cursor_bw_pre: [dml_float_t; __DML_NUM_PLANES__],
    pub prefetch_vmrow_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub final_flip_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub meta_row_bandwidth_this_state: [dml_float_t; __DML_NUM_PLANES__],
    pub dpte_row_bandwidth_this_state: [dml_float_t; __DML_NUM_PLANES__],
    pub ReturnBWPerState: [dml_float_t; 2],
    pub ReturnDRAMBWPerState: [dml_float_t; 2],
    pub meta_row_bandwidth: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub dpte_row_bandwidth: [dml_float_t; 2][__DML_NUM_PLANES__],
// Something that should be feedback to caller
    pub ODMModePerState: [dml_odm_mode; __DML_NUM_PLANES__],
    pub ODMModeThisState: [dml_odm_mode; __DML_NUM_PLANES__],
    pub SurfaceSizeInMALL: [dml_uint_t; __DML_NUM_PLANES__],
    pub NoOfDPP: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub NoOfDPPThisState: [dml_uint_t; __DML_NUM_PLANES__],
    pub MPCCombine: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub MPCCombineThisState: [dml_bool_t; __DML_NUM_PLANES__],
    pub ProjectedDCFCLKDeepSleep: [dml_float_t; 2],
    pub MinDPPCLKUsingSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
    pub SingleDPPViewportSizeSupportPerSurface: [dml_bool_t; __DML_NUM_PLANES__],
    pub ImmediateFlipSupportedForPipe: [dml_bool_t; __DML_NUM_PLANES__],
    pub NotUrgentLatencyHiding: [dml_bool_t; __DML_NUM_PLANES__],
    pub NotUrgentLatencyHidingPre: [dml_bool_t; __DML_NUM_PLANES__],
    pub PTEBufferSizeNotExceededPerState: [dml_bool_t; __DML_NUM_PLANES__],
    pub DCCMetaBufferSizeNotExceededPerState: [dml_bool_t; __DML_NUM_PLANES__],
    pub PrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub TotalNumberOfActiveDPP: [dml_uint_t; 2],
    pub TotalNumberOfSingleDPPSurfaces: [dml_uint_t; 2],
    pub TotalNumberOfDCCActiveDPP: [dml_uint_t; 2],
    pub SubViewportLinesNeededInMALL: [dml_uint_t; __DML_NUM_PLANES__],
}

// @brief A mega structure that houses various info for model programming step.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_program_st {
// -------------
// Intermediate/Informational
// -------------
    pub UrgentLatency: dml_float_t,
    pub UrgentLatencyWithUSRRetraining: dml_float_t,
    pub VInitPreFillY: [dml_uint_t; __DML_NUM_PLANES__],
    pub VInitPreFillC: [dml_uint_t; __DML_NUM_PLANES__],
    pub MaxNumSwathY: [dml_uint_t; __DML_NUM_PLANES__],
    pub MaxNumSwathC: [dml_uint_t; __DML_NUM_PLANES__],
    pub BytePerPixelDETY: [dml_float_t; __DML_NUM_PLANES__],
    pub BytePerPixelDETC: [dml_float_t; __DML_NUM_PLANES__],
    pub BytePerPixelY: [dml_uint_t; __DML_NUM_PLANES__],
    pub BytePerPixelC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathWidthSingleDPPY: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathWidthSingleDPPC: [dml_uint_t; __DML_NUM_PLANES__],
    pub ReadBandwidthSurfaceLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub ReadBandwidthSurfaceChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRow: [dml_uint_t; __DML_NUM_PLANES__],
    pub PDEAndMetaPTEBytesFrame: [dml_uint_t; __DML_NUM_PLANES__],
    pub MetaRowByte: [dml_uint_t; __DML_NUM_PLANES__],
    pub PrefetchSourceLinesY: [dml_uint_t; __DML_NUM_PLANES__],
    pub RequiredPrefetchPixDataBWLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub RequiredPrefetchPixDataBWChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub PrefetchSourceLinesC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PSCL_THROUGHPUT: [dml_float_t; __DML_NUM_PLANES__],
    pub PSCL_THROUGHPUT_CHROMA: [dml_float_t; __DML_NUM_PLANES__],
    pub DSCDelay: [dml_uint_t; __DML_NUM_PLANES__],
    pub DPPCLKUsingSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
    pub MacroTileWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub MacroTileWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockHeight256BytesY: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockHeight256BytesC: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockWidth256BytesY: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockWidth256BytesC: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockHeightC: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub BlockWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SurfaceSizeInTheMALL: [dml_uint_t; __DML_NUM_PLANES__],
    pub VRatioPrefetchY: [dml_float_t; __DML_NUM_PLANES__],
    pub VRatioPrefetchC: [dml_float_t; __DML_NUM_PLANES__],
    pub Tno_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub final_flip_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub prefetch_vmrow_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub cursor_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub cursor_bw_pre: [dml_float_t; __DML_NUM_PLANES__],
    pub WritebackDelay: [dml_float_t; __DML_NUM_PLANES__],
    pub dpte_row_height: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height_linear: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_req_width: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_req_height: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_width: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_height: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_width_luma_ub: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_width_chroma_ub: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height_linear_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_req_width_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_req_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_width_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub vm_group_bytes: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_group_bytes: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_row_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub dpte_row_bw: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgBurstFactorCursor: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgBurstFactorCursorPre: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgBurstFactorLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgBurstFactorLumaPre: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgBurstFactorChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub UrgBurstFactorChromaPre: [dml_float_t; __DML_NUM_PLANES__],
    pub swath_width_luma_ub: [dml_uint_t; __DML_NUM_PLANES__],
    pub swath_width_chroma_ub: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEReqWidthY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEReqHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PTERequestSizeY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEReqWidthC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEReqHeightC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PTERequestSizeC: [dml_uint_t; __DML_NUM_PLANES__],
    pub Tdmdl_vm: [dml_float_t; __DML_NUM_PLANES__],
    pub Tdmdl: [dml_float_t; __DML_NUM_PLANES__],
    pub TSetup: [dml_float_t; __DML_NUM_PLANES__],
    pub dpde0_bytes_per_frame_ub_l: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_pte_bytes_per_frame_ub_l: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpde0_bytes_per_frame_ub_c: [dml_uint_t; __DML_NUM_PLANES__],
    pub meta_pte_bytes_per_frame_ub_c: [dml_uint_t; __DML_NUM_PLANES__],
    pub UnboundedRequestEnabled: dml_bool_t,
    pub compbuf_reserved_space_64b: dml_uint_t,
    pub compbuf_reserved_space_zs: dml_uint_t,
    pub CompressedBufferSizeInkByte: dml_uint_t,
    pub NoUrgentLatencyHiding: [dml_bool_t; __DML_NUM_PLANES__],
    pub NoUrgentLatencyHidingPre: [dml_bool_t; __DML_NUM_PLANES__],
    pub UrgentExtraLatency: dml_float_t,
    pub PrefetchAndImmediateFlipSupported: dml_bool_t,
    pub TotalDataReadBandwidth: dml_float_t,
    pub BandwidthAvailableForImmediateFlip: dml_float_t,
    pub NotEnoughTimeForDynamicMetadata: [dml_bool_t; __DML_NUM_PLANES__],
    pub ReadBandwidthLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub ReadBandwidthChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub total_dcn_read_bw_with_flip: dml_float_t,
    pub total_dcn_read_bw_with_flip_no_urgent_burst: dml_float_t,
    pub TotalDataReadBandwidthNotIncludingMALLPrefetch: dml_float_t,
    pub total_dcn_read_bw_with_flip_not_including_MALL_prefetch: dml_float_t,
    pub non_urgent_total_dcn_read_bw_with_flip: dml_float_t,
    pub non_urgent_total_dcn_read_bw_with_flip_not_including_MALL_prefetch: dml_float_t,
    pub use_one_row_for_frame: [dml_bool_t; __DML_NUM_PLANES__],
    pub use_one_row_for_frame_flip: [dml_bool_t; __DML_NUM_PLANES__],
    pub TCalc: dml_float_t,
    pub TotImmediateFlipBytes: dml_uint_t,
// -------------------
// Output
// -------------------
    pub pipe: dml_uint_t pipe_plane[__DML_NUM_PLANES__]; // <brief used mainly by dv to map the pipe inst to plane index within DML core; the plane idx of a,
    pub num_active_pipes: dml_uint_t,
    pub result: dml_bool_t NoTimeToPrefetch[__DML_NUM_PLANES__]; /// <brief Prefetch schedule calculation,
// Support
    pub step: dml_uint_t PrefetchMode[__DML_NUM_PLANES__]; /// <brief prefetch mode used for prefetch support check in mode programming,
    pub supported: dml_bool_t PrefetchModeSupported; /// <brief Is the prefetch mode (bandwidth and latency),
    pub ImmediateFlipSupported: dml_bool_t,
    pub ImmediateFlipSupportedForPipe: [dml_bool_t; __DML_NUM_PLANES__],
// Clock
    pub Dcfclk: dml_float_t,
    pub programming: dml_float_t Dispclk; /// <brief dispclk being used in mode,
    pub programming: dml_float_t Dppclk[__DML_NUM_PLANES__]; /// <brief dppclk being used in mode,
    pub WritebackDISPCLK: dml_float_t,
    pub GlobalDPPCLK: dml_float_t,
// @ brief These "calculated" dispclk and dppclk clocks are calculated in the mode programming step.
// Depends on the dml_clk_cfg_st option; these calculated values may not used in subsequent calculation.
// Possible DV usage: Calculated values fetched by test once after mode_programming step and then possibly
// use the values as min and adjust the actual freq used for the 2nd pass
    pub Dispclk_calculated: dml_float_t,
    pub Dppclk_calculated: [dml_float_t; __DML_NUM_PLANES__],
    pub now: dml_float_t DSCCLK_calculated[__DML_NUM_PLANES__]; //< brief Required DSCCLK freq. Backend; not used in any subsequent calculations for,
    pub DCFCLKDeepSleep: dml_float_t,
// ARB reg
    pub DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE: dml_bool_t,
    pub Watermark: Watermarks,
// DCC compression control
    pub DCCYMaxUncompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCYMaxCompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCYIndependentBlock: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCCMaxUncompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCCMaxCompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
    pub DCCCIndependentBlock: [dml_uint_t; __DML_NUM_PLANES__],
// Stutter Efficiency
    pub StutterEfficiency: dml_float_t,
    pub StutterEfficiencyNotIncludingVBlank: dml_float_t,
    pub NumberOfStutterBurstsPerFrame: dml_uint_t,
    pub Z8StutterEfficiency: dml_float_t,
    pub Z8NumberOfStutterBurstsPerFrame: dml_uint_t,
    pub Z8StutterEfficiencyNotIncludingVBlank: dml_float_t,
    pub StutterPeriod: dml_float_t,
    pub Z8StutterEfficiencyBestCase: dml_float_t,
    pub Z8NumberOfStutterBurstsPerFrameBestCase: dml_uint_t,
    pub Z8StutterEfficiencyNotIncludingVBlankBestCase: dml_float_t,
    pub StutterPeriodBestCase: dml_float_t,
// DLG TTU reg
    pub MIN_DST_Y_NEXT_START: [dml_float_t; __DML_NUM_PLANES__],
    pub VREADY_AT_OR_AFTER_VSYNC: [dml_bool_t; __DML_NUM_PLANES__],
    pub DSTYAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
    pub DSTXAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
    pub DestinationLinesForPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub DestinationLinesToRequestVMInVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub DestinationLinesToRequestRowInVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub DestinationLinesToRequestVMInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub DestinationLinesToRequestRowInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub MinTTUVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeLineDeliveryTimeLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeLineDeliveryTimeChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeLineDeliveryTimeLumaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeLineDeliveryTimeChromaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeRequestDeliveryTimeLuma: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeRequestDeliveryTimeChroma: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeRequestDeliveryTimeLumaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub DisplayPipeRequestDeliveryTimeChromaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub CursorRequestDeliveryTime: [dml_float_t; __DML_NUM_PLANES__],
    pub CursorRequestDeliveryTimePrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub DST_Y_PER_PTE_ROW_NOM_L: [dml_float_t; __DML_NUM_PLANES__],
    pub DST_Y_PER_PTE_ROW_NOM_C: [dml_float_t; __DML_NUM_PLANES__],
    pub DST_Y_PER_META_ROW_NOM_L: [dml_float_t; __DML_NUM_PLANES__],
    pub DST_Y_PER_META_ROW_NOM_C: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerMetaChunkNominal: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerChromaMetaChunkNominal: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerMetaChunkVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerChromaMetaChunkVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerMetaChunkFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerChromaMetaChunkFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub time_per_pte_group_nom_luma: [dml_float_t; __DML_NUM_PLANES__],
    pub time_per_pte_group_nom_chroma: [dml_float_t; __DML_NUM_PLANES__],
    pub time_per_pte_group_vblank_luma: [dml_float_t; __DML_NUM_PLANES__],
    pub time_per_pte_group_vblank_chroma: [dml_float_t; __DML_NUM_PLANES__],
    pub time_per_pte_group_flip_luma: [dml_float_t; __DML_NUM_PLANES__],
    pub time_per_pte_group_flip_chroma: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerVMGroupVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerVMGroupFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerVMRequestVBlank: [dml_float_t; __DML_NUM_PLANES__],
    pub TimePerVMRequestFlip: [dml_float_t; __DML_NUM_PLANES__],
    pub FractionOfUrgentBandwidth: dml_float_t,
    pub FractionOfUrgentBandwidthImmediateFlip: dml_float_t,
// RQ registers
    pub PTE_BUFFER_MODE: [dml_bool_t; __DML_NUM_PLANES__],
    pub BIGK_FRAGMENT_SIZE: [dml_uint_t; __DML_NUM_PLANES__],
    pub SubViewportLinesNeededInMALL: [dml_uint_t; __DML_NUM_PLANES__],
    pub UsesMALLForStaticScreen: [dml_bool_t; __DML_NUM_PLANES__],
// OTG
    pub pos.: dml_uint_t VStartupMin[__DML_NUM_PLANES__]; /// <brief Minimum vstartup to meet the prefetch schedule (i.e. the prefetch solution can be found at this vstartup time); not the actual global sync vstartup,
    pub vblank)): dml_uint_t VStartup[__DML_NUM_PLANES__]; /// <brief The vstartup value for OTG programming (will set to max vstartup; but now bounded by min(vblank_nom. actual,
    pub VUpdateOffsetPix: [dml_uint_t; __DML_NUM_PLANES__],
    pub VUpdateWidthPix: [dml_uint_t; __DML_NUM_PLANES__],
    pub VReadyOffsetPix: [dml_uint_t; __DML_NUM_PLANES__],
// Latency and Support
    pub MaxActiveFCLKChangeLatencySupported: dml_float_t,
    pub USRRetrainingSupport: dml_bool_t,
    pub FCLKChangeSupport: dml_fclock_change_support,
    pub DRAMClockChangeSupport: dml_dram_clock_change_support,
    pub MaxActiveDRAMClockChangeLatencySupported: [dml_float_t; __DML_NUM_PLANES__],
    pub WritebackAllowFCLKChangeEndPosition: [dml_float_t; __DML_NUM_PLANES__],
    pub WritebackAllowDRAMClockChangeEndPosition: [dml_float_t; __DML_NUM_PLANES__],
// buffer sizing
    pub value.: dml_uint_t DETBufferSizeInKByte[__DML_NUM_PLANES__]; // <brief Recommended DET size configuration for this plane. All pipes under this plane should program the DET buffer size to the calculated,
    pub DETBufferSizeY: [dml_uint_t; __DML_NUM_PLANES__],
    pub DETBufferSizeC: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathHeightY: [dml_uint_t; __DML_NUM_PLANES__],
    pub SwathHeightC: [dml_uint_t; __DML_NUM_PLANES__],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_states_st {
    pub states: dml_uint_t num_states; /// <brief num of soc pwr,
    pub struct: soc_state_bounding_box_st state_array[__DML_MAX_STATE_ARRAY_SIZE__]; /// <brief fixed size array that holds states,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UseMinimumDCFCLK_params_st {
    pub UseMALLForPStateChange: *mut dml_use_mall_for_pstate_change_mode,
    pub DRRDisplay: *mut dml_bool_t,
    pub SynchronizeDRRDisplaysForUCLKPStateChangeFinal: dml_bool_t,
    pub MaxInterDCNTileRepeaters: dml_uint_t,
    pub MaxPrefetchMode: dml_uint_t,
    pub DRAMClockChangeLatencyFinal: dml_float_t,
    pub FCLKChangeLatency: dml_float_t,
    pub SREnterPlusExitTime: dml_float_t,
    pub ReturnBusWidth: dml_uint_t,
    pub RoundTripPingLatencyCycles: dml_uint_t,
    pub ReorderingBytes: dml_uint_t,
    pub PixelChunkSizeInKByte: dml_uint_t,
    pub MetaChunkSize: dml_uint_t,
    pub GPUVMEnable: dml_bool_t,
    pub GPUVMMaxPageTableLevels: dml_uint_t,
    pub HostVMEnable: dml_bool_t,
    pub NumberOfActiveSurfaces: dml_uint_t,
    pub HostVMMinPageSize: dml_uint_t,
    pub HostVMMaxNonCachedPageTableLevels: dml_uint_t,
    pub DynamicMetadataVMEnabled: dml_bool_t,
    pub ImmediateFlipRequirement: dml_bool_t,
    pub ProgressiveToInterlaceUnitInOPP: dml_bool_t,
    pub MaxAveragePercentOfIdealSDPPortBWDisplayCanUseInNormalSystemOperation: dml_float_t,
    pub PercentOfIdealSDPPortBWReceivedAfterUrgLatency: dml_float_t,
    pub VTotal: *mut dml_uint_t,
    pub VActive: *mut dml_uint_t,
    pub DynamicMetadataTransmittedBytes: *mut dml_uint_t,
    pub DynamicMetadataLinesBeforeActiveRequired: *mut dml_uint_t,
    pub Interlace: *mut dml_bool_t,
    pub (*RequiredDPPCLKPerSurface)[__DML_NUM_PLANES__]: *mut dml_float_t,
    pub RequiredDISPCLK: *mut dml_float_t,
    pub UrgLatency: dml_float_t,
    pub (*NoOfDPP)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub ProjectedDCFCLKDeepSleep: *mut dml_float_t,
    pub (*MaximumVStartup)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub TotalNumberOfActiveDPP: *mut dml_uint_t,
    pub TotalNumberOfDCCActiveDPP: *mut dml_uint_t,
    pub dpte_group_bytes: *mut dml_uint_t,
    pub (*PrefetchLinesY)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub (*PrefetchLinesC)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub (*swath_width_luma_ub_all_states)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub (*swath_width_chroma_ub_all_states)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub BytePerPixelY: *mut dml_uint_t,
    pub BytePerPixelC: *mut dml_uint_t,
    pub HTotal: *mut dml_uint_t,
    pub PixelClock: *mut dml_float_t,
    pub (*PDEAndMetaPTEBytesPerFrame)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub (*DPTEBytesPerRow)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub (*MetaRowBytes)[__DML_NUM_PLANES__]: *mut dml_uint_t,
    pub DynamicMetadataEnable: *mut dml_bool_t,
    pub ReadBandwidthLuma: *mut dml_float_t,
    pub ReadBandwidthChroma: *mut dml_float_t,
    pub DCFCLKPerState: dml_float_t,
    pub DCFCLKState: *mut dml_float_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params_st {
    pub USRRetrainingRequiredFinal: dml_bool_t,
    pub UseMALLForPStateChange: *mut dml_use_mall_for_pstate_change_mode,
    pub PrefetchMode: *mut dml_uint_t,
    pub NumberOfActiveSurfaces: dml_uint_t,
    pub MaxLineBufferLines: dml_uint_t,
    pub LineBufferSize: dml_uint_t,
    pub WritebackInterfaceBufferSize: dml_uint_t,
    pub DCFCLK: dml_float_t,
    pub ReturnBW: dml_float_t,
    pub SynchronizeTimingsFinal: dml_bool_t,
    pub SynchronizeDRRDisplaysForUCLKPStateChangeFinal: dml_bool_t,
    pub DRRDisplay: *mut dml_bool_t,
    pub dpte_group_bytes: *mut dml_uint_t,
    pub meta_row_height: *mut dml_uint_t,
    pub meta_row_height_chroma: *mut dml_uint_t,
    pub mmSOCParameters: SOCParametersList,
    pub WritebackChunkSize: dml_uint_t,
    pub SOCCLK: dml_float_t,
    pub DCFClkDeepSleep: dml_float_t,
    pub DETBufferSizeY: *mut dml_uint_t,
    pub DETBufferSizeC: *mut dml_uint_t,
    pub SwathHeightY: *mut dml_uint_t,
    pub SwathHeightC: *mut dml_uint_t,
    pub LBBitPerPixel: *mut dml_uint_t,
    pub SwathWidthY: *mut dml_uint_t,
    pub SwathWidthC: *mut dml_uint_t,
    pub HRatio: *mut dml_float_t,
    pub HRatioChroma: *mut dml_float_t,
    pub VTaps: *mut dml_uint_t,
    pub VTapsChroma: *mut dml_uint_t,
    pub VRatio: *mut dml_float_t,
    pub VRatioChroma: *mut dml_float_t,
    pub HTotal: *mut dml_uint_t,
    pub VTotal: *mut dml_uint_t,
    pub VActive: *mut dml_uint_t,
    pub PixelClock: *mut dml_float_t,
    pub BlendingAndTiming: *mut dml_uint_t,
    pub DPPPerSurface: *mut dml_uint_t,
    pub BytePerPixelDETY: *mut dml_float_t,
    pub BytePerPixelDETC: *mut dml_float_t,
    pub DSTXAfterScaler: *mut dml_uint_t,
    pub DSTYAfterScaler: *mut dml_uint_t,
    pub WritebackEnable: *mut dml_bool_t,
    pub WritebackPixelFormat: *mut dml_source_format_class,
    pub WritebackDestinationWidth: *mut dml_uint_t,
    pub WritebackDestinationHeight: *mut dml_uint_t,
    pub WritebackSourceHeight: *mut dml_uint_t,
    pub UnboundedRequestEnabled: dml_bool_t,
    pub CompressedBufferSizeInkByte: dml_uint_t,
// Output
    pub Watermark: *mut Watermarks,
    pub DRAMClockChangeSupport: *mut dml_dram_clock_change_support,
    pub MaxActiveDRAMClockChangeLatencySupported: *mut dml_float_t,
    pub SubViewportLinesNeededInMALL: *mut dml_uint_t,
    pub FCLKChangeSupport: *mut dml_fclock_change_support,
    pub MaxActiveFCLKChangeLatencySupported: *mut dml_float_t,
    pub USRRetrainingSupport: *mut dml_bool_t,
    pub ActiveDRAMClockChangeLatencyMargin: *mut dml_float_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculateVMRowAndSwath_params_st {
    pub NumberOfActiveSurfaces: dml_uint_t,
    pub myPipe: *mut DmlPipe,
    pub SurfaceSizeInMALL: *mut dml_uint_t,
    pub PTEBufferSizeInRequestsLuma: dml_uint_t,
    pub PTEBufferSizeInRequestsChroma: dml_uint_t,
    pub DCCMetaBufferSizeBytes: dml_uint_t,
    pub UseMALLForStaticScreen: *mut dml_use_mall_for_static_screen_mode,
    pub UseMALLForPStateChange: *mut dml_use_mall_for_pstate_change_mode,
    pub MALLAllocatedForDCN: dml_uint_t,
    pub SwathWidthY: *mut dml_uint_t,
    pub SwathWidthC: *mut dml_uint_t,
    pub GPUVMEnable: dml_bool_t,
    pub HostVMEnable: dml_bool_t,
    pub HostVMMaxNonCachedPageTableLevels: dml_uint_t,
    pub GPUVMMaxPageTableLevels: dml_uint_t,
    pub GPUVMMinPageSizeKBytes: *mut dml_uint_t,
    pub HostVMMinPageSize: dml_uint_t,
    pub PTEBufferModeOverrideEn: *mut dml_bool_t,
    pub PTEBufferModeOverrideVal: *mut dml_bool_t,
// Output
    pub PTEBufferSizeNotExceeded: *mut dml_bool_t,
    pub DCCMetaBufferSizeNotExceeded: *mut dml_bool_t,
    pub dpte_row_width_luma_ub: *mut dml_uint_t,
    pub dpte_row_width_chroma_ub: *mut dml_uint_t,
    pub dpte_row_height_luma: *mut dml_uint_t,
    pub dpte_row_height_chroma: *mut dml_uint_t,
    pub VBA_DELTA: *mut *mut dml_uint_t dpte_row_height_linear_luma; //,
    pub VBA_DELTA: *mut *mut dml_uint_t dpte_row_height_linear_chroma; //,
    pub meta_req_width: *mut dml_uint_t,
    pub meta_req_width_chroma: *mut dml_uint_t,
    pub meta_req_height: *mut dml_uint_t,
    pub meta_req_height_chroma: *mut dml_uint_t,
    pub meta_row_width: *mut dml_uint_t,
    pub meta_row_width_chroma: *mut dml_uint_t,
    pub meta_row_height: *mut dml_uint_t,
    pub meta_row_height_chroma: *mut dml_uint_t,
    pub vm_group_bytes: *mut dml_uint_t,
    pub dpte_group_bytes: *mut dml_uint_t,
    pub PixelPTEReqWidthY: *mut dml_uint_t,
    pub PixelPTEReqHeightY: *mut dml_uint_t,
    pub PTERequestSizeY: *mut dml_uint_t,
    pub PixelPTEReqWidthC: *mut dml_uint_t,
    pub PixelPTEReqHeightC: *mut dml_uint_t,
    pub PTERequestSizeC: *mut dml_uint_t,
    pub dpde0_bytes_per_frame_ub_l: *mut dml_uint_t,
    pub meta_pte_bytes_per_frame_ub_l: *mut dml_uint_t,
    pub dpde0_bytes_per_frame_ub_c: *mut dml_uint_t,
    pub meta_pte_bytes_per_frame_ub_c: *mut dml_uint_t,
    pub PrefetchSourceLinesY: *mut dml_uint_t,
    pub PrefetchSourceLinesC: *mut dml_uint_t,
    pub VInitPreFillY: *mut dml_uint_t,
    pub VInitPreFillC: *mut dml_uint_t,
    pub MaxNumSwathY: *mut dml_uint_t,
    pub MaxNumSwathC: *mut dml_uint_t,
    pub meta_row_bw: *mut dml_float_t,
    pub dpte_row_bw: *mut dml_float_t,
    pub PixelPTEBytesPerRow: *mut dml_uint_t,
    pub PDEAndMetaPTEBytesFrame: *mut dml_uint_t,
    pub MetaRowByte: *mut dml_uint_t,
    pub use_one_row_for_frame: *mut dml_bool_t,
    pub use_one_row_for_frame_flip: *mut dml_bool_t,
    pub UsesMALLForStaticScreen: *mut dml_bool_t,
    pub PTE_BUFFER_MODE: *mut dml_bool_t,
    pub BIGK_FRAGMENT_SIZE: *mut dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculateSwathAndDETConfiguration_params_st {
    pub DETSizeOverride: *mut dml_uint_t,
    pub UseMALLForPStateChange: *mut dml_use_mall_for_pstate_change_mode,
    pub ConfigReturnBufferSizeInKByte: dml_uint_t,
    pub ROBBufferSizeInKByte: dml_uint_t,
    pub MaxTotalDETInKByte: dml_uint_t,
    pub MinCompressedBufferSizeInKByte: dml_uint_t,
    pub PixelChunkSizeInKByte: dml_uint_t,
    pub ForceSingleDPP: dml_bool_t,
    pub NumberOfActiveSurfaces: dml_uint_t,
    pub nomDETInKByte: dml_uint_t,
    pub UseUnboundedRequestingFinal: dml_unbounded_requesting_policy,
    pub ConfigReturnBufferSegmentSizeInkByte: dml_uint_t,
    pub CompressedBufferSegmentSizeInkByteFinal: dml_uint_t,
    pub Output: *mut dml_output_encoder_class,
    pub ReadBandwidthLuma: *mut dml_float_t,
    pub ReadBandwidthChroma: *mut dml_float_t,
    pub MaximumSwathWidthLuma: *mut dml_float_t,
    pub MaximumSwathWidthChroma: *mut dml_float_t,
    pub SourceScan: *mut dml_rotation_angle,
    pub ViewportStationary: *mut dml_bool_t,
    pub SourcePixelFormat: *mut dml_source_format_class,
    pub SurfaceTiling: *mut dml_swizzle_mode,
    pub ViewportWidth: *mut dml_uint_t,
    pub ViewportHeight: *mut dml_uint_t,
    pub ViewportXStart: *mut dml_uint_t,
    pub ViewportYStart: *mut dml_uint_t,
    pub ViewportXStartC: *mut dml_uint_t,
    pub ViewportYStartC: *mut dml_uint_t,
    pub SurfaceWidthY: *mut dml_uint_t,
    pub SurfaceWidthC: *mut dml_uint_t,
    pub SurfaceHeightY: *mut dml_uint_t,
    pub SurfaceHeightC: *mut dml_uint_t,
    pub Read256BytesBlockHeightY: *mut dml_uint_t,
    pub Read256BytesBlockHeightC: *mut dml_uint_t,
    pub Read256BytesBlockWidthY: *mut dml_uint_t,
    pub Read256BytesBlockWidthC: *mut dml_uint_t,
    pub ODMMode: *mut dml_odm_mode,
    pub BlendingAndTiming: *mut dml_uint_t,
    pub BytePerPixY: *mut dml_uint_t,
    pub BytePerPixC: *mut dml_uint_t,
    pub BytePerPixDETY: *mut dml_float_t,
    pub BytePerPixDETC: *mut dml_float_t,
    pub HActive: *mut dml_uint_t,
    pub HRatio: *mut dml_float_t,
    pub HRatioChroma: *mut dml_float_t,
    pub DPPPerSurface: *mut dml_uint_t,
    pub swath_width_luma_ub: *mut dml_uint_t,
    pub swath_width_chroma_ub: *mut dml_uint_t,
    pub SwathWidth: *mut dml_uint_t,
    pub SwathWidthChroma: *mut dml_uint_t,
    pub SwathHeightY: *mut dml_uint_t,
    pub SwathHeightC: *mut dml_uint_t,
    pub DETBufferSizeInKByte: *mut dml_uint_t,
    pub DETBufferSizeY: *mut dml_uint_t,
    pub DETBufferSizeC: *mut dml_uint_t,
    pub UnboundedRequestEnabled: *mut dml_bool_t,
    pub compbuf_reserved_space_64b: *mut dml_uint_t,
    pub compbuf_reserved_space_zs: *mut dml_uint_t,
    pub CompressedBufferSizeInkByte: *mut dml_uint_t,
    pub ViewportSizeSupportPerSurface: *mut dml_bool_t,
    pub ViewportSizeSupport: *mut dml_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculateStutterEfficiency_params_st {
    pub CompressedBufferSizeInkByte: dml_uint_t,
    pub UseMALLForPStateChange: *mut dml_use_mall_for_pstate_change_mode,
    pub UnboundedRequestEnabled: dml_bool_t,
    pub MetaFIFOSizeInKEntries: dml_uint_t,
    pub ZeroSizeBufferEntries: dml_uint_t,
    pub PixelChunkSizeInKByte: dml_uint_t,
    pub NumberOfActiveSurfaces: dml_uint_t,
    pub ROBBufferSizeInKByte: dml_uint_t,
    pub TotalDataReadBandwidth: dml_float_t,
    pub DCFCLK: dml_float_t,
    pub ReturnBW: dml_float_t,
    pub CompbufReservedSpace64B: dml_uint_t,
    pub CompbufReservedSpaceZs: dml_uint_t,
    pub SRExitTime: dml_float_t,
    pub SRExitZ8Time: dml_float_t,
    pub SynchronizeTimingsFinal: dml_bool_t,
    pub BlendingAndTiming: *mut dml_uint_t,
    pub StutterEnterPlusExitWatermark: dml_float_t,
    pub Z8StutterEnterPlusExitWatermark: dml_float_t,
    pub ProgressiveToInterlaceUnitInOPP: dml_bool_t,
    pub Interlace: *mut dml_bool_t,
    pub MinTTUVBlank: *mut dml_float_t,
    pub DPPPerSurface: *mut dml_uint_t,
    pub DETBufferSizeY: *mut dml_uint_t,
    pub BytePerPixelY: *mut dml_uint_t,
    pub BytePerPixelDETY: *mut dml_float_t,
    pub SwathWidthY: *mut dml_uint_t,
    pub SwathHeightY: *mut dml_uint_t,
    pub SwathHeightC: *mut dml_uint_t,
    pub NetDCCRateLuma: *mut dml_float_t,
    pub NetDCCRateChroma: *mut dml_float_t,
    pub DCCFractionOfZeroSizeRequestsLuma: *mut dml_float_t,
    pub DCCFractionOfZeroSizeRequestsChroma: *mut dml_float_t,
    pub HTotal: *mut dml_uint_t,
    pub VTotal: *mut dml_uint_t,
    pub PixelClock: *mut dml_float_t,
    pub VRatio: *mut dml_float_t,
    pub SourceScan: *mut dml_rotation_angle,
    pub BlockHeight256BytesY: *mut dml_uint_t,
    pub BlockWidth256BytesY: *mut dml_uint_t,
    pub BlockHeight256BytesC: *mut dml_uint_t,
    pub BlockWidth256BytesC: *mut dml_uint_t,
    pub DCCYMaxUncompressedBlock: *mut dml_uint_t,
    pub DCCCMaxUncompressedBlock: *mut dml_uint_t,
    pub VActive: *mut dml_uint_t,
    pub DCCEnable: *mut dml_bool_t,
    pub WritebackEnable: *mut dml_bool_t,
    pub ReadBandwidthSurfaceLuma: *mut dml_float_t,
    pub ReadBandwidthSurfaceChroma: *mut dml_float_t,
    pub meta_row_bw: *mut dml_float_t,
    pub dpte_row_bw: *mut dml_float_t,
    pub StutterEfficiencyNotIncludingVBlank: *mut dml_float_t,
    pub StutterEfficiency: *mut dml_float_t,
    pub NumberOfStutterBurstsPerFrame: *mut dml_uint_t,
    pub Z8StutterEfficiencyNotIncludingVBlank: *mut dml_float_t,
    pub Z8StutterEfficiency: *mut dml_float_t,
    pub Z8NumberOfStutterBurstsPerFrame: *mut dml_uint_t,
    pub StutterPeriod: *mut dml_float_t,
    pub DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE: *mut dml_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculatePrefetchSchedule_params_st {
    pub EnhancedPrefetchScheduleAccelerationFinal: dml_bool_t,
    pub HostVMInefficiencyFactor: dml_float_t,
    pub myPipe: *mut DmlPipe,
    pub DSCDelay: dml_uint_t,
    pub DPPCLKDelaySubtotalPlusCNVCFormater: dml_float_t,
    pub DPPCLKDelaySCL: dml_float_t,
    pub DPPCLKDelaySCLLBOnly: dml_float_t,
    pub DPPCLKDelayCNVCCursor: dml_float_t,
    pub DISPCLKDelaySubtotal: dml_float_t,
    pub DPP_RECOUT_WIDTH: dml_uint_t,
    pub OutputFormat: dml_output_format_class,
    pub MaxInterDCNTileRepeaters: dml_uint_t,
    pub VStartup: dml_uint_t,
    pub MaxVStartup: dml_uint_t,
    pub GPUVMPageTableLevels: dml_uint_t,
    pub GPUVMEnable: dml_bool_t,
    pub HostVMEnable: dml_bool_t,
    pub HostVMMaxNonCachedPageTableLevels: dml_uint_t,
    pub HostVMMinPageSize: dml_uint_t,
    pub DynamicMetadataEnable: dml_bool_t,
    pub DynamicMetadataVMEnabled: dml_bool_t,
    pub DynamicMetadataLinesBeforeActiveRequired: c_int,
    pub DynamicMetadataTransmittedBytes: dml_uint_t,
    pub UrgentLatency: dml_float_t,
    pub UrgentExtraLatency: dml_float_t,
    pub TCalc: dml_float_t,
    pub PDEAndMetaPTEBytesFrame: dml_uint_t,
    pub MetaRowByte: dml_uint_t,
    pub PixelPTEBytesPerRow: dml_uint_t,
    pub PrefetchSourceLinesY: dml_float_t,
    pub VInitPreFillY: dml_uint_t,
    pub MaxNumSwathY: dml_uint_t,
    pub PrefetchSourceLinesC: dml_float_t,
    pub VInitPreFillC: dml_uint_t,
    pub MaxNumSwathC: dml_uint_t,
    pub swath_width_luma_ub: dml_uint_t,
    pub swath_width_chroma_ub: dml_uint_t,
    pub SwathHeightY: dml_uint_t,
    pub SwathHeightC: dml_uint_t,
    pub TWait: dml_float_t,
    pub DSTXAfterScaler: *mut dml_uint_t,
    pub DSTYAfterScaler: *mut dml_uint_t,
    pub DestinationLinesForPrefetch: *mut dml_float_t,
    pub DestinationLinesToRequestVMInVBlank: *mut dml_float_t,
    pub DestinationLinesToRequestRowInVBlank: *mut dml_float_t,
    pub VRatioPrefetchY: *mut dml_float_t,
    pub VRatioPrefetchC: *mut dml_float_t,
    pub RequiredPrefetchPixDataBWLuma: *mut dml_float_t,
    pub RequiredPrefetchPixDataBWChroma: *mut dml_float_t,
    pub NotEnoughTimeForDynamicMetadata: *mut dml_bool_t,
    pub Tno_bw: *mut dml_float_t,
    pub prefetch_vmrow_bw: *mut dml_float_t,
    pub Tdmdl_vm: *mut dml_float_t,
    pub Tdmdl: *mut dml_float_t,
    pub TSetup: *mut dml_float_t,
    pub VUpdateOffsetPix: *mut dml_uint_t,
    pub VUpdateWidthPix: *mut dml_uint_t,
    pub VReadyOffsetPix: *mut dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_core_mode_support_locals_st {
    pub dummy_boolean: [dml_bool_t; 2],
    pub dummy_integer: [dml_uint_t; 3],
    pub dummy_integer_array: [dml_uint_t; 22][__DML_NUM_PLANES__],
    pub dummy_odm_mode: [dml_odm_mode; __DML_NUM_PLANES__],
    pub dummy_boolean_array: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub MaxVStartupAllPlanes: [dml_uint_t; 2],
    pub MaximumVStartup: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub DSTYAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
    pub DSTXAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
    pub NextPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub MinPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub MaxPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub dummy_single: [dml_float_t; 3],
    pub dummy_single_array: [dml_float_t; __DML_NUM_PLANES__],
    pub dummy_watermark: Watermarks,
    pub mSOCParameters: SOCParametersList,
    pub myPipe: DmlPipe,
    pub SurfParameters: [DmlPipe; __DML_NUM_PLANES__],
    pub TotalNumberOfActiveWriteback: dml_uint_t,
    pub MaximumSwathWidthSupportLuma: dml_uint_t,
    pub MaximumSwathWidthSupportChroma: dml_uint_t,
    pub MPCCombineMethodAsNeededForPStateChangeAndVoltage: dml_bool_t,
    pub MPCCombineMethodAsPossible: dml_bool_t,
    pub TotalAvailablePipesSupportNoDSC: dml_bool_t,
    pub NumberOfDPPNoDSC: dml_uint_t,
    pub ODMModeNoDSC: dml_odm_mode,
    pub RequiredDISPCLKPerSurfaceNoDSC: dml_float_t,
    pub TotalAvailablePipesSupportDSC: dml_bool_t,
    pub NumberOfDPPDSC: dml_uint_t,
    pub ODMModeDSC: dml_odm_mode,
    pub RequiredDISPCLKPerSurfaceDSC: dml_float_t,
    pub NoChromaOrLinear: dml_bool_t,
    pub BWOfNonCombinedSurfaceOfMaximumBandwidth: dml_float_t,
    pub NumberOfNonCombinedSurfaceOfMaximumBandwidth: dml_uint_t,
    pub TotalNumberOfActiveOTG: dml_uint_t,
    pub TotalNumberOfActiveHDMIFRL: dml_uint_t,
    pub TotalNumberOfActiveDP2p0: dml_uint_t,
    pub TotalNumberOfActiveDP2p0Outputs: dml_uint_t,
    pub TotalSlots: dml_uint_t,
    pub DSCFormatFactor: dml_uint_t,
    pub TotalDSCUnitsRequired: dml_uint_t,
    pub ReorderingBytes: dml_uint_t,
    pub ImmediateFlipRequiredFinal: dml_bool_t,
    pub FullFrameMALLPStateMethod: dml_bool_t,
    pub SubViewportMALLPStateMethod: dml_bool_t,
    pub PhantomPipeMALLPStateMethod: dml_bool_t,
    pub SubViewportMALLRefreshGreaterThan120Hz: dml_bool_t,
    pub MaxTotalVActiveRDBandwidth: dml_float_t,
    pub VMDataOnlyReturnBWPerState: dml_float_t,
    pub HostVMInefficiencyFactor: dml_float_t,
    pub NextMaxVStartup: dml_uint_t,
    pub MaxVStartup: dml_uint_t,
    pub AllPrefetchModeTested: dml_bool_t,
    pub AnyLinesForVMOrRowTooLarge: dml_bool_t,
    pub is_max_pwr_state: dml_bool_t,
    pub is_max_dram_pwr_state: dml_bool_t,
    pub dram_clock_change_support: dml_bool_t,
    pub f_clock_change_support: dml_bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_core_mode_programming_locals_st {
    pub DSCFormatFactor: dml_uint_t,
    pub dummy_integer_array: [dml_uint_t; 2][__DML_NUM_PLANES__],
    pub dummy_output_encoder_array: [dml_output_encoder_class; __DML_NUM_PLANES__],
    pub dummy_single_array: [dml_float_t; 2][__DML_NUM_PLANES__],
    pub dummy_long_array: [dml_uint_t; 4][__DML_NUM_PLANES__],
    pub dummy_boolean_array: [dml_bool_t; 2][__DML_NUM_PLANES__],
    pub dummy_boolean: [dml_bool_t; 1],
    pub SurfaceParameters: [DmlPipe; __DML_NUM_PLANES__],
    pub ReorderBytes: dml_uint_t,
    pub VMDataOnlyReturnBW: dml_float_t,
    pub HostVMInefficiencyFactor: dml_float_t,
    pub TotalDCCActiveDPP: dml_uint_t,
    pub TotalActiveDPP: dml_uint_t,
    pub VStartupLines: dml_uint_t,
    pub OTG: dml_uint_t MaxVStartupLines[__DML_NUM_PLANES__]; /// <brief more like vblank for the plane's,
    pub MaxVStartupAllPlanes: dml_uint_t,
    pub ImmediateFlipRequirementFinal: dml_bool_t,
    pub iteration: c_int,
    pub MaxTotalRDBandwidth: dml_float_t,
    pub MaxTotalRDBandwidthNoUrgentBurst: dml_float_t,
    pub DestinationLineTimesForPrefetchLessThan2: dml_bool_t,
    pub VRatioPrefetchMoreThanMax: dml_bool_t,
    pub MaxTotalRDBandwidthNotIncludingMALLPrefetch: dml_float_t,
    pub NextPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub MinPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub MaxPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
    pub AllPrefetchModeTested: dml_bool_t,
    pub dummy_unit_vector: [dml_float_t; __DML_NUM_PLANES__],
    pub NonUrgentMaxTotalRDBandwidth: dml_float_t,
    pub NonUrgentMaxTotalRDBandwidthNotIncludingMALLPrefetch: dml_float_t,
    pub dummy_single: [dml_float_t; 2],
    pub mmSOCParameters: SOCParametersList,
    pub Tvstartup_margin: dml_float_t,
    pub dlg_vblank_start: dml_float_t,
    pub LSetup: dml_float_t,
    pub blank_lines_remaining: dml_float_t,
    pub old_MIN_DST_Y_NEXT_START: dml_float_t,
    pub TotalWRBandwidth: dml_float_t,
    pub WRBandwidth: dml_float_t,
    pub dummy_watermark: Watermarks,
    pub myPipe: DmlPipe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals_st {
    pub ActiveDRAMClockChangeLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
    pub ActiveFCLKChangeLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
    pub USRRetrainingLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
    pub SynchronizedSurfaces: [dml_bool_t; __DML_NUM_PLANES__][__DML_NUM_PLANES__],
    pub EffectiveLBLatencyHidingY: dml_float_t,
    pub EffectiveLBLatencyHidingC: dml_float_t,
    pub LinesInDETY: [dml_float_t; __DML_NUM_PLANES__],
    pub LinesInDETC: [dml_float_t; __DML_NUM_PLANES__],
    pub LinesInDETYRoundedDownToSwath: [dml_uint_t; __DML_NUM_PLANES__],
    pub LinesInDETCRoundedDownToSwath: [dml_uint_t; __DML_NUM_PLANES__],
    pub FullDETBufferingTimeY: dml_float_t,
    pub FullDETBufferingTimeC: dml_float_t,
    pub WritebackDRAMClockChangeLatencyMargin: dml_float_t,
    pub WritebackFCLKChangeLatencyMargin: dml_float_t,
    pub WritebackLatencyHiding: dml_float_t,
    pub TotalActiveWriteback: dml_uint_t,
    pub LBLatencyHidingSourceLinesY: [dml_uint_t; __DML_NUM_PLANES__],
    pub LBLatencyHidingSourceLinesC: [dml_uint_t; __DML_NUM_PLANES__],
    pub TotalPixelBW: dml_float_t,
    pub EffectiveDETBufferSizeY: dml_float_t,
    pub ActiveClockChangeLatencyHidingY: dml_float_t,
    pub ActiveClockChangeLatencyHidingC: dml_float_t,
    pub ActiveClockChangeLatencyHiding: dml_float_t,
    pub FoundCriticalSurface: dml_bool_t,
    pub LastSurfaceWithoutMargin: dml_uint_t,
    pub FCLKChangeSupportNumber: dml_uint_t,
    pub DRAMClockChangeMethod: dml_uint_t,
    pub DRAMClockChangeSupportNumber: dml_uint_t,
    pub dst_y_pstate: dml_uint_t,
    pub src_y_pstate_l: dml_uint_t,
    pub src_y_pstate_c: dml_uint_t,
    pub src_y_ahead_l: dml_uint_t,
    pub src_y_ahead_c: dml_uint_t,
    pub sub_vp_lines_l: dml_uint_t,
    pub sub_vp_lines_c: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculateVMRowAndSwath_locals_st {
    pub PTEBufferSizeInRequestsForLuma: [dml_uint_t; __DML_NUM_PLANES__],
    pub PTEBufferSizeInRequestsForChroma: [dml_uint_t; __DML_NUM_PLANES__],
    pub PDEAndMetaPTEBytesFrameY: dml_uint_t,
    pub PDEAndMetaPTEBytesFrameC: dml_uint_t,
    pub MetaRowByteY: [dml_uint_t; __DML_NUM_PLANES__],
    pub MetaRowByteC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRowY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRowC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRowStorageY: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRowStorageC: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRowY_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
    pub PixelPTEBytesPerRowC_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_width_luma_ub_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height_luma_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_width_chroma_ub_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
    pub dpte_row_height_chroma_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
    pub one_row_per_frame_fits_in_buffer: [dml_bool_t; __DML_NUM_PLANES__],
    pub HostVMDynamicLevels: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UseMinimumDCFCLK_locals_st {
    pub dummy1: dml_uint_t,
    pub dummy2: dml_uint_t,
    pub dummy3: dml_uint_t,
    pub NormalEfficiency: dml_float_t,
    pub TotalMaxPrefetchFlipDPTERowBandwidth: [dml_float_t; 2],
    pub PixelDCFCLKCyclesRequiredInPrefetch: [dml_float_t; __DML_NUM_PLANES__],
    pub PrefetchPixelLinesTime: [dml_float_t; __DML_NUM_PLANES__],
    pub DCFCLKRequiredForPeakBandwidthPerSurface: [dml_float_t; __DML_NUM_PLANES__],
    pub DynamicMetadataVMExtraLatency: [dml_float_t; __DML_NUM_PLANES__],
    pub MinimumTWait: dml_float_t,
    pub DPTEBandwidth: dml_float_t,
    pub DCFCLKRequiredForAverageBandwidth: dml_float_t,
    pub ExtraLatencyBytes: dml_uint_t,
    pub ExtraLatencyCycles: dml_float_t,
    pub DCFCLKRequiredForPeakBandwidth: dml_float_t,
    pub NoOfDPPState: [dml_uint_t; __DML_NUM_PLANES__],
    pub MinimumTvmPlus2Tr0: dml_float_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CalculatePrefetchSchedule_locals_st {
    pub MyError: dml_bool_t,
    pub DPPCycles: dml_uint_t,
    pub DISPCLKCycles: dml_uint_t,
    pub DSTTotalPixelsAfterScaler: dml_float_t,
    pub LineTime: dml_float_t,
    pub dst_y_prefetch_equ: dml_float_t,
    pub prefetch_bw_oto: dml_float_t,
    pub Tvm_oto: dml_float_t,
    pub Tr0_oto: dml_float_t,
    pub Tvm_oto_lines: dml_float_t,
    pub Tr0_oto_lines: dml_float_t,
    pub dst_y_prefetch_oto: dml_float_t,
    pub TimeForFetchingMetaPTE: dml_float_t,
    pub TimeForFetchingRowInVBlank: dml_float_t,
    pub LinesToRequestPrefetchPixelData: dml_float_t,
    pub HostVMDynamicLevelsTrips: dml_uint_t,
    pub trip_to_mem: dml_float_t,
    pub Tvm_trips: dml_float_t,
    pub Tr0_trips: dml_float_t,
    pub Tvm_trips_rounded: dml_float_t,
    pub Tr0_trips_rounded: dml_float_t,
    pub max_Tsw: dml_float_t,
    pub Lsw_oto: dml_float_t,
    pub Tpre_rounded: dml_float_t,
    pub prefetch_bw_equ: dml_float_t,
    pub Tvm_equ: dml_float_t,
    pub Tr0_equ: dml_float_t,
    pub Tdmbf: dml_float_t,
    pub Tdmec: dml_float_t,
    pub Tdmsks: dml_float_t,
    pub prefetch_sw_bytes: dml_float_t,
    pub prefetch_bw_pr: dml_float_t,
    pub bytes_pp: dml_float_t,
    pub dep_bytes: dml_float_t,
    pub min_Lsw_oto: dml_float_t,
    pub Tsw_est1: dml_float_t,
    pub Tsw_est3: dml_float_t,
    pub PrefetchBandwidth1: dml_float_t,
    pub PrefetchBandwidth2: dml_float_t,
    pub PrefetchBandwidth3: dml_float_t,
    pub PrefetchBandwidth4: dml_float_t,
}

// @brief To minimize stack usage; function locals are instead placed into this scratch structure which is allocated per context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_mode_lib_scratch_st {
// Scratch space for function locals
    pub dml_core_mode_support_locals: dml_core_mode_support_locals_st,
    pub dml_core_mode_programming_locals: dml_core_mode_programming_locals_st,
    pub CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals: CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals_st,
    pub CalculateVMRowAndSwath_locals: CalculateVMRowAndSwath_locals_st,
    pub UseMinimumDCFCLK_locals: UseMinimumDCFCLK_locals_st,
    pub CalculatePrefetchSchedule_locals: CalculatePrefetchSchedule_locals_st,
// Scratch space for function params
    pub CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params: CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params_st,
    pub CalculateVMRowAndSwath_params: CalculateVMRowAndSwath_params_st,
    pub UseMinimumDCFCLK_params: UseMinimumDCFCLK_params_st,
    pub CalculateSwathAndDETConfiguration_params: CalculateSwathAndDETConfiguration_params_st,
    pub CalculateStutterEfficiency_params: CalculateStutterEfficiency_params_st,
    pub CalculatePrefetchSchedule_params: CalculatePrefetchSchedule_params_st,
}

// @brief Represent the overall soc/ip environment. It contains data structure represent the soc/ip characteristic and also structures that hold calculation output
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_mode_lib_st {
    pub project: dml_uint_t,
// @brief Mode evaluation and programming policy
    pub policy: dml_mode_eval_policy_st,
// @brief IP/SOC characteristic
    pub ip: ip_params_st,
    pub soc: soc_bounding_box_st,
    pub states: soc_states_st,
// @brief Mode Support and Mode programming struct
// Used to hold input; intermediate and output of the calculations
    pub support: mode_support_st ms; // for mode,
    pub programming: mode_program_st mp; // for mode,
    pub scratch: display_mode_lib_scratch_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml_mode_support_ex_params_st {
    pub mode_lib: *mut display_mode_lib_st,
    pub in_display_cfg: *const dml_display_cfg_st,
    pub in_start_state_idx: dml_uint_t,
    pub out_lowest_state_idx: dml_uint_t,
    pub out_evaluation_info: *mut dml_mode_support_info_st,
}

pub type dml_display_rq_regs_st = _vcs_dpi_dml_display_rq_regs_st;
pub type dml_display_dlg_regs_st = _vcs_dpi_dml_display_dlg_regs_st;
pub type dml_display_ttu_regs_st = _vcs_dpi_dml_display_ttu_regs_st;
pub type dml_display_arb_params_st = _vcs_dpi_dml_display_arb_params_st;
pub type dml_display_plane_rq_regs_st = _vcs_dpi_dml_display_plane_rq_regs_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_dml_display_dlg_regs_st {
    pub refcyc_h_blank_end: dml_uint_t,
    pub dlg_vblank_end: dml_uint_t,
    pub min_dst_y_next_start: dml_uint_t,
    pub refcyc_per_htotal: dml_uint_t,
    pub refcyc_x_after_scaler: dml_uint_t,
    pub dst_y_after_scaler: dml_uint_t,
    pub dst_y_prefetch: dml_uint_t,
    pub dst_y_per_vm_vblank: dml_uint_t,
    pub dst_y_per_row_vblank: dml_uint_t,
    pub dst_y_per_vm_flip: dml_uint_t,
    pub dst_y_per_row_flip: dml_uint_t,
    pub ref_freq_to_pix_freq: dml_uint_t,
    pub vratio_prefetch: dml_uint_t,
    pub vratio_prefetch_c: dml_uint_t,
    pub refcyc_per_pte_group_vblank_l: dml_uint_t,
    pub refcyc_per_pte_group_vblank_c: dml_uint_t,
    pub refcyc_per_meta_chunk_vblank_l: dml_uint_t,
    pub refcyc_per_meta_chunk_vblank_c: dml_uint_t,
    pub refcyc_per_pte_group_flip_l: dml_uint_t,
    pub refcyc_per_pte_group_flip_c: dml_uint_t,
    pub refcyc_per_meta_chunk_flip_l: dml_uint_t,
    pub refcyc_per_meta_chunk_flip_c: dml_uint_t,
    pub dst_y_per_pte_row_nom_l: dml_uint_t,
    pub dst_y_per_pte_row_nom_c: dml_uint_t,
    pub refcyc_per_pte_group_nom_l: dml_uint_t,
    pub refcyc_per_pte_group_nom_c: dml_uint_t,
    pub dst_y_per_meta_row_nom_l: dml_uint_t,
    pub dst_y_per_meta_row_nom_c: dml_uint_t,
    pub refcyc_per_meta_chunk_nom_l: dml_uint_t,
    pub refcyc_per_meta_chunk_nom_c: dml_uint_t,
    pub refcyc_per_line_delivery_pre_l: dml_uint_t,
    pub refcyc_per_line_delivery_pre_c: dml_uint_t,
    pub refcyc_per_line_delivery_l: dml_uint_t,
    pub refcyc_per_line_delivery_c: dml_uint_t,
    pub refcyc_per_vm_group_vblank: dml_uint_t,
    pub refcyc_per_vm_group_flip: dml_uint_t,
    pub refcyc_per_vm_req_vblank: dml_uint_t,
    pub refcyc_per_vm_req_flip: dml_uint_t,
    pub dst_y_offset_cur0: dml_uint_t,
    pub chunk_hdl_adjust_cur0: dml_uint_t,
    pub dst_y_offset_cur1: dml_uint_t,
    pub chunk_hdl_adjust_cur1: dml_uint_t,
    pub vready_after_vcount0: dml_uint_t,
    pub dst_y_delta_drq_limit: dml_uint_t,
    pub refcyc_per_vm_dmdata: dml_uint_t,
    pub dmdata_dl_delta: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_dml_display_ttu_regs_st {
    pub qos_level_low_wm: dml_uint_t,
    pub qos_level_high_wm: dml_uint_t,
    pub min_ttu_vblank: dml_uint_t,
    pub qos_level_flip: dml_uint_t,
    pub refcyc_per_req_delivery_l: dml_uint_t,
    pub refcyc_per_req_delivery_c: dml_uint_t,
    pub refcyc_per_req_delivery_cur0: dml_uint_t,
    pub refcyc_per_req_delivery_cur1: dml_uint_t,
    pub refcyc_per_req_delivery_pre_l: dml_uint_t,
    pub refcyc_per_req_delivery_pre_c: dml_uint_t,
    pub refcyc_per_req_delivery_pre_cur0: dml_uint_t,
    pub refcyc_per_req_delivery_pre_cur1: dml_uint_t,
    pub qos_level_fixed_l: dml_uint_t,
    pub qos_level_fixed_c: dml_uint_t,
    pub qos_level_fixed_cur0: dml_uint_t,
    pub qos_level_fixed_cur1: dml_uint_t,
    pub qos_ramp_disable_l: dml_uint_t,
    pub qos_ramp_disable_c: dml_uint_t,
    pub qos_ramp_disable_cur0: dml_uint_t,
    pub qos_ramp_disable_cur1: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_dml_display_arb_params_st {
    pub max_req_outstanding: dml_uint_t,
    pub min_req_outstanding: dml_uint_t,
    pub sat_level_us: dml_uint_t,
    pub hvm_max_qos_commit_threshold: dml_uint_t,
    pub hvm_min_req_outstand_commit_threshold: dml_uint_t,
    pub compbuf_reserved_space_kbytes: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_dml_display_plane_rq_regs_st {
    pub chunk_size: dml_uint_t,
    pub min_chunk_size: dml_uint_t,
    pub meta_chunk_size: dml_uint_t,
    pub min_meta_chunk_size: dml_uint_t,
    pub dpte_group_size: dml_uint_t,
    pub mpte_group_size: dml_uint_t,
    pub swath_height: dml_uint_t,
    pub pte_row_height_linear: dml_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_dml_display_rq_regs_st {
    pub rq_regs_l: dml_display_plane_rq_regs_st,
    pub rq_regs_c: dml_display_plane_rq_regs_st,
    pub drq_expansion_mode: dml_uint_t,
    pub prq_expansion_mode: dml_uint_t,
    pub mrq_expansion_mode: dml_uint_t,
    pub crq_expansion_mode: dml_uint_t,
    pub plane1_base_address: dml_uint_t,
}
