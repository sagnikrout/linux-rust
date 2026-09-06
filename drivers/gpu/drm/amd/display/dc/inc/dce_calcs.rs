//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/dce_calcs.h
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
// Copyright 2015-2017 Advanced Micro Devices, Inc.
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
// Bandwidth and Watermark calculations interface.
// (Refer to "DCEx_mode_support.xlsm" from Perforce.)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bw_calcs_version {
    BW_CALCS_VERSION_INVALID,
    BW_CALCS_VERSION_CARRIZO,
    BW_CALCS_VERSION_POLARIS10,
    BW_CALCS_VERSION_POLARIS11,
    BW_CALCS_VERSION_POLARIS12,
    BW_CALCS_VERSION_VEGAM,
    BW_CALCS_VERSION_STONEY,
    BW_CALCS_VERSION_VEGA10
}

//
// There are three types of input into Calculations:
// 1. per-DCE static values - these are "hardcoded" properties of the DCEIP
// 2. board-level values - these are generally coming from VBIOS parser
// 3. mode/configuration values - depending Mode, Scaling number of Displays etc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bw_defines {
// Common
    bw_def_no = 0,
    bw_def_none = 0,
    bw_def_yes = 1,
    bw_def_ok = 1,
    bw_def_high = 2,
    bw_def_mid = 1,
    bw_def_low = 0,

// Internal
    bw_defs_start = 255,
    bw_def_underlay422,
    bw_def_underlay420_luma,
    bw_def_underlay420_chroma,
    bw_def_underlay444,
    bw_def_graphics,
    bw_def_display_write_back420_luma,
    bw_def_display_write_back420_chroma,
    bw_def_portrait,
    bw_def_hsr_mtn_4,
    bw_def_hsr_mtn_h_taps,
    bw_def_ceiling__h_taps_div_4___meq_hsr,
    bw_def_invalid_linear_or_stereo_mode,
    bw_def_invalid_rotation_or_bpp_or_stereo,
    bw_def_vsr_mtn_v_taps,
    bw_def_vsr_mtn_4,
    bw_def_auto,
    bw_def_manual,
    bw_def_exceeded_allowed_maximum_sclk,
    bw_def_exceeded_allowed_page_close_open,
    bw_def_exceeded_allowed_outstanding_pte_req_queue_size,
    bw_def_exceeded_allowed_maximum_bw,
    bw_def_landscape,

// Panning and bezel
    bw_def_any_lines,

// Underlay mode
    bw_def_underlay_only,
    bw_def_blended,
    bw_def_blend,

// Stereo mode
    bw_def_mono,
    bw_def_side_by_side,
    bw_def_top_bottom,

// Underlay surface type
    bw_def_420,
    bw_def_422,
    bw_def_444,

// Tiling mode
    bw_def_linear,
    bw_def_tiled,
    bw_def_array_linear_general,
    bw_def_array_linear_aligned,
    bw_def_rotated_micro_tiling,
    bw_def_display_micro_tiling,

// Memory type
    bw_def_gddr5,
    bw_def_hbm,

// Voltage
    bw_def_high_no_nbp_state_change,
    bw_def_0_72,
    bw_def_0_8,
    bw_def_0_9,

    bw_def_notok = -1,
    bw_def_na = -1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_calcs_dceip {
    pub version: bw_calcs_version,
    pub percent_of_ideal_port_bw_received_after_urgent_latency: u32,
    pub max_average_percent_of_ideal_port_bw_display_can_use_in_normal_system_operation: u32,
    pub max_average_percent_of_ideal_drambw_display_can_use_in_normal_system_operation: u32,
    pub large_cursor: bool,
    pub cursor_max_outstanding_group_num: u32,
    pub dmif_pipe_en_fbc_chunk_tracker: bool,
    pub dmif_request_buffer_size: bw_fixed,
    pub lines_interleaved_into_lb: u32,
    pub low_power_tiling_mode: u32,
    pub chunk_width: u32,
    pub number_of_graphics_pipes: u32,
    pub number_of_underlay_pipes: u32,
    pub display_write_back_supported: bool,
    pub argb_compression_support: bool,
    pub underlay_vscaler_efficiency6_bit_per_component: bw_fixed,
    pub underlay_vscaler_efficiency8_bit_per_component: bw_fixed,
    pub underlay_vscaler_efficiency10_bit_per_component: bw_fixed,
    pub underlay_vscaler_efficiency12_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency6_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency8_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency10_bit_per_component: bw_fixed,
    pub graphics_vscaler_efficiency12_bit_per_component: bw_fixed,
    pub alpha_vscaler_efficiency: bw_fixed,
    pub max_dmif_buffer_allocated: u32,
    pub graphics_dmif_size: u32,
    pub underlay_luma_dmif_size: u32,
    pub underlay_chroma_dmif_size: u32,
    pub pre_downscaler_enabled: bool,
    pub underlay_downscale_prefetch_enabled: bool,
    pub lb_write_pixels_per_dispclk: bw_fixed,
    pub lb_size_per_component444: bw_fixed,
    pub graphics_lb_nodownscaling_multi_line_prefetching: bool,
    pub stutter_and_dram_clock_state_change_gated_before_cursor: bw_fixed,
    pub underlay420_luma_lb_size_per_component: bw_fixed,
    pub underlay420_chroma_lb_size_per_component: bw_fixed,
    pub underlay422_lb_size_per_component: bw_fixed,
    pub cursor_chunk_width: bw_fixed,
    pub cursor_dcp_buffer_lines: bw_fixed,
    pub underlay_maximum_width_efficient_for_tiling: bw_fixed,
    pub underlay_maximum_height_efficient_for_tiling: bw_fixed,
    pub peak_pte_request_to_eviction_ratio_limiting_multiple_displays_or_single_rotated_display: bw_fixed,
    pub peak_pte_request_to_eviction_ratio_limiting_single_display_no_rotation: bw_fixed,
    pub minimum_outstanding_pte_request_limit: bw_fixed,
    pub maximum_total_outstanding_pte_requests_allowed_by_saw: bw_fixed,
    pub limit_excessive_outstanding_dmif_requests: bool,
    pub linear_mode_line_request_alternation_slice: bw_fixed,
    pub scatter_gather_lines_of_pte_prefetching_in_linear_mode: u32,
    pub display_write_back420_luma_mcifwr_buffer_size: u32,
    pub display_write_back420_chroma_mcifwr_buffer_size: u32,
    pub request_efficiency: bw_fixed,
    pub dispclk_per_request: bw_fixed,
    pub dispclk_ramping_factor: bw_fixed,
    pub display_pipe_throughput_factor: bw_fixed,
    pub scatter_gather_pte_request_rows_in_tiling_mode: u32,
    pub mcifwr_all_surfaces_burst_time: bw_fixed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_calcs_vbios {
    pub memory_type: bw_defines,
    pub dram_channel_width_in_bits: u32,
    pub number_of_dram_channels: u32,
    pub number_of_dram_banks: u32,
    pub /*m_hz*/: *mut bw_fixed low_yclk;,
    pub /*m_hz*/: *mut bw_fixed mid_yclk;,
    pub /*m_hz*/: *mut bw_fixed high_yclk;,
    pub /*m_hz*/: *mut bw_fixed low_sclk;,
    pub /*m_hz*/: *mut bw_fixed mid1_sclk;,
    pub /*m_hz*/: *mut bw_fixed mid2_sclk;,
    pub /*m_hz*/: *mut bw_fixed mid3_sclk;,
    pub /*m_hz*/: *mut bw_fixed mid4_sclk;,
    pub /*m_hz*/: *mut bw_fixed mid5_sclk;,
    pub /*m_hz*/: *mut bw_fixed mid6_sclk;,
    pub /*m_hz*/: *mut bw_fixed high_sclk;,
    pub /*m_hz*/: *mut bw_fixed low_voltage_max_dispclk;,
    pub /*m_hz*/: *mut bw_fixed mid_voltage_max_dispclk;,
    pub /*m_hz*/: *mut bw_fixed high_voltage_max_dispclk;,
    pub low_voltage_max_phyclk: bw_fixed,
    pub mid_voltage_max_phyclk: bw_fixed,
    pub high_voltage_max_phyclk: bw_fixed,
    pub data_return_bus_width: bw_fixed,
    pub trc: bw_fixed,
    pub dmifmc_urgent_latency: bw_fixed,
    pub stutter_self_refresh_exit_latency: bw_fixed,
    pub stutter_self_refresh_entry_latency: bw_fixed,
    pub nbp_state_change_latency: bw_fixed,
    pub mcifwrmc_urgent_latency: bw_fixed,
    pub scatter_gather_enable: bool,
    pub down_spread_percentage: bw_fixed,
    pub cursor_width: u32,
    pub average_compression_rate: u32,
    pub number_of_request_slots_gmc_reserves_for_dmif_per_channel: u32,
    pub blackout_duration: bw_fixed,
    pub maximum_blackout_recovery_time: bw_fixed,
}

//
// Temporary data structure(s).
//
pub const maximum_number_of_surfaces: c_int = 12;
// Units : MHz, us
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_calcs_data {
// data for all displays
    pub display_synchronization_enabled: bool,
    pub number_of_displays: u32,
    pub underlay_surface_type: bw_defines,
    pub panning_and_bezel_adjustment: bw_defines,
    pub graphics_tiling_mode: bw_defines,
    pub graphics_lb_bpc: u32,
    pub underlay_lb_bpc: u32,
    pub underlay_tiling_mode: bw_defines,
    pub d0_underlay_mode: bw_defines,
    pub d1_display_write_back_dwb_enable: bool,
    pub d1_underlay_mode: bw_defines,
    pub increase_voltage_to_support_mclk_switch: bool,
    pub cpup_state_change_enable: bool,
    pub cpuc_state_change_enable: bool,
    pub nbp_state_change_enable: bool,
    pub stutter_mode_enable: bool,
    pub y_clk_level: u32,
    pub sclk_level: u32,
    pub number_of_underlay_surfaces: u32,
    pub number_of_dram_wrchannels: u32,
    pub chunk_request_delay: u32,
    pub number_of_dram_channels: u32,
    pub underlay_micro_tile_mode: bw_defines,
    pub graphics_micro_tile_mode: bw_defines,
    pub max_phyclk: bw_fixed,
    pub dram_efficiency: bw_fixed,
    pub src_width_after_surface_type: bw_fixed,
    pub src_height_after_surface_type: bw_fixed,
    pub hsr_after_surface_type: bw_fixed,
    pub vsr_after_surface_type: bw_fixed,
    pub src_width_after_rotation: bw_fixed,
    pub src_height_after_rotation: bw_fixed,
    pub hsr_after_rotation: bw_fixed,
    pub vsr_after_rotation: bw_fixed,
    pub source_height_pixels: bw_fixed,
    pub hsr_after_stereo: bw_fixed,
    pub vsr_after_stereo: bw_fixed,
    pub source_width_in_lb: bw_fixed,
    pub lb_line_pitch: bw_fixed,
    pub underlay_maximum_source_efficient_for_tiling: bw_fixed,
    pub num_lines_at_frame_start: bw_fixed,
    pub min_dmif_size_in_time: bw_fixed,
    pub min_mcifwr_size_in_time: bw_fixed,
    pub total_requests_for_dmif_size: bw_fixed,
    pub peak_pte_request_to_eviction_ratio_limiting: bw_fixed,
    pub useful_pte_per_pte_request: bw_fixed,
    pub scatter_gather_pte_request_rows: bw_fixed,
    pub scatter_gather_row_height: bw_fixed,
    pub scatter_gather_pte_requests_in_vblank: bw_fixed,
    pub inefficient_linear_pitch_in_bytes: bw_fixed,
    pub cursor_total_data: bw_fixed,
    pub cursor_total_request_groups: bw_fixed,
    pub scatter_gather_total_pte_requests: bw_fixed,
    pub scatter_gather_total_pte_request_groups: bw_fixed,
    pub tile_width_in_pixels: bw_fixed,
    pub dmif_total_number_of_data_request_page_close_open: bw_fixed,
    pub mcifwr_total_number_of_data_request_page_close_open: bw_fixed,
    pub bytes_per_page_close_open: bw_fixed,
    pub mcifwr_total_page_close_open_time: bw_fixed,
    pub total_requests_for_adjusted_dmif_size: bw_fixed,
    pub total_dmifmc_urgent_trips: bw_fixed,
    pub total_dmifmc_urgent_latency: bw_fixed,
    pub total_display_reads_required_data: bw_fixed,
    pub total_display_reads_required_dram_access_data: bw_fixed,
    pub total_display_writes_required_data: bw_fixed,
    pub total_display_writes_required_dram_access_data: bw_fixed,
    pub display_reads_required_data: bw_fixed,
    pub display_reads_required_dram_access_data: bw_fixed,
    pub dmif_total_page_close_open_time: bw_fixed,
    pub min_cursor_memory_interface_buffer_size_in_time: bw_fixed,
    pub min_read_buffer_size_in_time: bw_fixed,
    pub display_reads_time_for_data_transfer: bw_fixed,
    pub display_writes_time_for_data_transfer: bw_fixed,
    pub dmif_required_dram_bandwidth: bw_fixed,
    pub mcifwr_required_dram_bandwidth: bw_fixed,
    pub required_dmifmc_urgent_latency_for_page_close_open: bw_fixed,
    pub required_mcifmcwr_urgent_latency: bw_fixed,
    pub required_dram_bandwidth_gbyte_per_second: bw_fixed,
    pub dram_bandwidth: bw_fixed,
    pub dmif_required_sclk: bw_fixed,
    pub mcifwr_required_sclk: bw_fixed,
    pub required_sclk: bw_fixed,
    pub downspread_factor: bw_fixed,
    pub v_scaler_efficiency: bw_fixed,
    pub scaler_limits_factor: bw_fixed,
    pub display_pipe_pixel_throughput: bw_fixed,
    pub total_dispclk_required_with_ramping: bw_fixed,
    pub total_dispclk_required_without_ramping: bw_fixed,
    pub total_read_request_bandwidth: bw_fixed,
    pub total_write_request_bandwidth: bw_fixed,
    pub dispclk_required_for_total_read_request_bandwidth: bw_fixed,
    pub total_dispclk_required_with_ramping_with_request_bandwidth: bw_fixed,
    pub total_dispclk_required_without_ramping_with_request_bandwidth: bw_fixed,
    pub dispclk: bw_fixed,
    pub blackout_recovery_time: bw_fixed,
    pub min_pixels_per_data_fifo_entry: bw_fixed,
    pub sclk_deep_sleep: bw_fixed,
    pub chunk_request_time: bw_fixed,
    pub cursor_request_time: bw_fixed,
    pub line_source_pixels_transfer_time: bw_fixed,
    pub dmifdram_access_efficiency: bw_fixed,
    pub mcifwrdram_access_efficiency: bw_fixed,
    pub total_average_bandwidth_no_compression: bw_fixed,
    pub total_average_bandwidth: bw_fixed,
    pub total_stutter_cycle_duration: bw_fixed,
    pub stutter_burst_time: bw_fixed,
    pub time_in_self_refresh: bw_fixed,
    pub stutter_efficiency: bw_fixed,
    pub worst_number_of_trips_to_memory: bw_fixed,
    pub immediate_flip_time: bw_fixed,
    pub latency_for_non_dmif_clients: bw_fixed,
    pub latency_for_non_mcifwr_clients: bw_fixed,
    pub dmifmc_urgent_latency_supported_in_high_sclk_and_yclk: bw_fixed,
    pub nbp_state_dram_speed_change_margin: bw_fixed,
    pub display_reads_time_for_data_transfer_and_urgent_latency: bw_fixed,
    pub dram_speed_change_margin: bw_fixed,
    pub min_vblank_dram_speed_change_margin: bw_fixed,
    pub min_stutter_refresh_duration: bw_fixed,
    pub total_stutter_dmif_buffer_size: u32,
    pub total_bytes_requested: u32,
    pub min_stutter_dmif_buffer_size: u32,
    pub num_stutter_bursts: u32,
    pub v_blank_nbp_state_dram_speed_change_latency_supported: bw_fixed,
    pub nbp_state_dram_speed_change_latency_supported: bw_fixed,
    pub fbc_en: [bool; maximum_number_of_surfaces],
    pub lpt_en: [bool; maximum_number_of_surfaces],
    pub displays_match_flag: [bool; maximum_number_of_surfaces],
    pub use_alpha: [bool; maximum_number_of_surfaces],
    pub orthogonal_rotation: [bool; maximum_number_of_surfaces],
    pub enable: [bool; maximum_number_of_surfaces],
    pub access_one_channel_only: [bool; maximum_number_of_surfaces],
    pub scatter_gather_enable_for_pipe: [bool; maximum_number_of_surfaces],
    pub interlace_mode: [bool; maximum_number_of_surfaces],
    pub display_pstate_change_enable: [bool; maximum_number_of_surfaces],
    pub line_buffer_prefetch: [bool; maximum_number_of_surfaces],
    pub bytes_per_pixel: [u32; maximum_number_of_surfaces],
    pub max_chunks_non_fbc_mode: [u32; maximum_number_of_surfaces],
    pub lb_bpc: [u32; maximum_number_of_surfaces],
    pub output_bpphdmi: [u32; maximum_number_of_surfaces],
    pub output_bppdp4_lane_hbr: [u32; maximum_number_of_surfaces],
    pub output_bppdp4_lane_hbr2: [u32; maximum_number_of_surfaces],
    pub output_bppdp4_lane_hbr3: [u32; maximum_number_of_surfaces],
    pub stereo_mode: [bw_defines; maximum_number_of_surfaces],
    pub dmif_buffer_transfer_time: [bw_fixed; maximum_number_of_surfaces],
    pub displays_with_same_mode: [bw_fixed; maximum_number_of_surfaces],
    pub stutter_dmif_buffer_size: [bw_fixed; maximum_number_of_surfaces],
    pub stutter_refresh_duration: [bw_fixed; maximum_number_of_surfaces],
    pub stutter_exit_watermark: [bw_fixed; maximum_number_of_surfaces],
    pub stutter_entry_watermark: [bw_fixed; maximum_number_of_surfaces],
    pub h_total: [bw_fixed; maximum_number_of_surfaces],
    pub v_total: [bw_fixed; maximum_number_of_surfaces],
    pub pixel_rate: [bw_fixed; maximum_number_of_surfaces],
    pub src_width: [bw_fixed; maximum_number_of_surfaces],
    pub pitch_in_pixels: [bw_fixed; maximum_number_of_surfaces],
    pub pitch_in_pixels_after_surface_type: [bw_fixed; maximum_number_of_surfaces],
    pub src_height: [bw_fixed; maximum_number_of_surfaces],
    pub scale_ratio: [bw_fixed; maximum_number_of_surfaces],
    pub h_taps: [bw_fixed; maximum_number_of_surfaces],
    pub v_taps: [bw_fixed; maximum_number_of_surfaces],
    pub h_scale_ratio: [bw_fixed; maximum_number_of_surfaces],
    pub v_scale_ratio: [bw_fixed; maximum_number_of_surfaces],
    pub rotation_angle: [bw_fixed; maximum_number_of_surfaces],
    pub compression_rate: [bw_fixed; maximum_number_of_surfaces],
    pub hsr: [bw_fixed; maximum_number_of_surfaces],
    pub vsr: [bw_fixed; maximum_number_of_surfaces],
    pub source_width_rounded_up_to_chunks: [bw_fixed; maximum_number_of_surfaces],
    pub source_width_pixels: [bw_fixed; maximum_number_of_surfaces],
    pub source_height_rounded_up_to_chunks: [bw_fixed; maximum_number_of_surfaces],
    pub display_bandwidth: [bw_fixed; maximum_number_of_surfaces],
    pub request_bandwidth: [bw_fixed; maximum_number_of_surfaces],
    pub bytes_per_request: [bw_fixed; maximum_number_of_surfaces],
    pub useful_bytes_per_request: [bw_fixed; maximum_number_of_surfaces],
    pub lines_interleaved_in_mem_access: [bw_fixed; maximum_number_of_surfaces],
    pub latency_hiding_lines: [bw_fixed; maximum_number_of_surfaces],
    pub lb_partitions: [bw_fixed; maximum_number_of_surfaces],
    pub lb_partitions_max: [bw_fixed; maximum_number_of_surfaces],
    pub dispclk_required_with_ramping: [bw_fixed; maximum_number_of_surfaces],
    pub dispclk_required_without_ramping: [bw_fixed; maximum_number_of_surfaces],
    pub data_buffer_size: [bw_fixed; maximum_number_of_surfaces],
    pub outstanding_chunk_request_limit: [bw_fixed; maximum_number_of_surfaces],
    pub urgent_watermark: [bw_fixed; maximum_number_of_surfaces],
    pub nbp_state_change_watermark: [bw_fixed; maximum_number_of_surfaces],
    pub v_filter_init: [bw_fixed; maximum_number_of_surfaces],
    pub stutter_cycle_duration: [bw_fixed; maximum_number_of_surfaces],
    pub average_bandwidth: [bw_fixed; maximum_number_of_surfaces],
    pub average_bandwidth_no_compression: [bw_fixed; maximum_number_of_surfaces],
    pub scatter_gather_pte_request_limit: [bw_fixed; maximum_number_of_surfaces],
    pub lb_size_per_component: [bw_fixed; maximum_number_of_surfaces],
    pub memory_chunk_size_in_bytes: [bw_fixed; maximum_number_of_surfaces],
    pub pipe_chunk_size_in_bytes: [bw_fixed; maximum_number_of_surfaces],
    pub number_of_trips_to_memory_for_getting_apte_row: [bw_fixed; maximum_number_of_surfaces],
    pub adjusted_data_buffer_size: [bw_fixed; maximum_number_of_surfaces],
    pub adjusted_data_buffer_size_in_memory: [bw_fixed; maximum_number_of_surfaces],
    pub pixels_per_data_fifo_entry: [bw_fixed; maximum_number_of_surfaces],
    pub scatter_gather_pte_requests_in_row: [bw_fixed; maximum_number_of_surfaces],
    pub pte_request_per_chunk: [bw_fixed; maximum_number_of_surfaces],
    pub scatter_gather_page_width: [bw_fixed; maximum_number_of_surfaces],
    pub scatter_gather_page_height: [bw_fixed; maximum_number_of_surfaces],
    pub lb_lines_in_per_line_out_in_beginning_of_frame: [bw_fixed; maximum_number_of_surfaces],
    pub lb_lines_in_per_line_out_in_middle_of_frame: [bw_fixed; maximum_number_of_surfaces],
    pub cursor_width_pixels: [bw_fixed; maximum_number_of_surfaces],
    pub minimum_latency_hiding: [bw_fixed; maximum_number_of_surfaces],
    pub maximum_latency_hiding: [bw_fixed; maximum_number_of_surfaces],
    pub minimum_latency_hiding_with_cursor: [bw_fixed; maximum_number_of_surfaces],
    pub maximum_latency_hiding_with_cursor: [bw_fixed; maximum_number_of_surfaces],
    pub src_pixels_for_first_output_pixel: [bw_fixed; maximum_number_of_surfaces],
    pub src_pixels_for_last_output_pixel: [bw_fixed; maximum_number_of_surfaces],
    pub src_data_for_first_output_pixel: [bw_fixed; maximum_number_of_surfaces],
    pub src_data_for_last_output_pixel: [bw_fixed; maximum_number_of_surfaces],
    pub active_time: [bw_fixed; maximum_number_of_surfaces],
    pub horizontal_blank_and_chunk_granularity_factor: [bw_fixed; maximum_number_of_surfaces],
    pub cursor_latency_hiding: [bw_fixed; maximum_number_of_surfaces],
    pub v_blank_dram_speed_change_margin: [bw_fixed; maximum_number_of_surfaces],
    pub num_displays_with_margin: [u32; 3][8],
    pub dmif_burst_time: [bw_fixed; 3][8],
    pub mcifwr_burst_time: [bw_fixed; 3][8],
    pub line_source_transfer_time: [bw_fixed; maximum_number_of_surfaces][3][8],
    pub dram_speed_change_line_source_transfer_time: [bw_fixed; maximum_number_of_surfaces][3][8],
    pub min_dram_speed_change_margin: [bw_fixed; 3][8],
    pub dispclk_required_for_dram_speed_change: [bw_fixed; 3][8],
    pub dispclk_required_for_dram_speed_change_pipe: [bw_fixed; 3][8],
    pub blackout_duration_margin: [bw_fixed; 3][8],
    pub dispclk_required_for_blackout_duration: [bw_fixed; 3][8],
    pub dispclk_required_for_blackout_recovery: [bw_fixed; 3][8],
    pub dmif_required_sclk_for_urgent_latency: [bw_fixed; 6],
}

//
// Initialize structures with data which will NOT change at runtime.
//
// Return:
// true -	Display(s) configuration supported.
// In this case 'calcs_output' contains data for HW programming
// false - Display(s) configuration not supported (not enough bandwidth).
//
