//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml/display_mode_structs.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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

//
// DOC: overview
//
// Most of the DML code is automatically generated and tested via hardware
// description language. Usually, we use the reference _vcs_dpi in the code
// where VCS means "Verilog Compiled Simulator" and DPI stands for "Direct
// Programmer Interface". In other words, those structs can be used to
// interface with Verilog with other languages such as C.
//
pub type voltage_scaling_st = _vcs_dpi_voltage_scaling_st;
pub type soc_bounding_box_st = _vcs_dpi_soc_bounding_box_st;
pub type ip_params_st = _vcs_dpi_ip_params_st;
pub type display_pipe_source_params_st = _vcs_dpi_display_pipe_source_params_st;
pub type display_output_params_st = _vcs_dpi_display_output_params_st;
pub type scaler_ratio_depth_st = _vcs_dpi_scaler_ratio_depth_st;
pub type scaler_taps_st = _vcs_dpi_scaler_taps_st;
pub type display_pipe_dest_params_st = _vcs_dpi_display_pipe_dest_params_st;
pub type display_pipe_params_st = _vcs_dpi_display_pipe_params_st;
pub type display_clocks_and_cfg_st = _vcs_dpi_display_clocks_and_cfg_st;
pub type display_e2e_pipe_params_st = _vcs_dpi_display_e2e_pipe_params_st;
pub type display_data_rq_misc_params_st = _vcs_dpi_display_data_rq_misc_params_st;
pub type display_data_rq_sizing_params_st = _vcs_dpi_display_data_rq_sizing_params_st;
pub type display_data_rq_dlg_params_st = _vcs_dpi_display_data_rq_dlg_params_st;
pub type display_rq_dlg_params_st = _vcs_dpi_display_rq_dlg_params_st;
pub type display_rq_sizing_params_st = _vcs_dpi_display_rq_sizing_params_st;
pub type display_rq_misc_params_st = _vcs_dpi_display_rq_misc_params_st;
pub type display_rq_params_st = _vcs_dpi_display_rq_params_st;
pub type display_dlg_regs_st = _vcs_dpi_display_dlg_regs_st;
pub type display_ttu_regs_st = _vcs_dpi_display_ttu_regs_st;
pub type display_data_rq_regs_st = _vcs_dpi_display_data_rq_regs_st;
pub type display_rq_regs_st = _vcs_dpi_display_rq_regs_st;
pub type display_dlg_sys_params_st = _vcs_dpi_display_dlg_sys_params_st;
pub type display_arb_params_st = _vcs_dpi_display_arb_params_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_voltage_scaling_st {
    pub state: c_int,
    pub dscclk_mhz: double,
    pub dcfclk_mhz: double,
    pub socclk_mhz: double,
    pub phyclk_d18_mhz: double,
    pub phyclk_d32_mhz: double,
    pub dram_speed_mts: double,
    pub fabricclk_mhz: double,
    pub dispclk_mhz: double,
    pub dram_bw_per_chan_gbps: double,
    pub phyclk_mhz: double,
    pub dppclk_mhz: double,
    pub dtbclk_mhz: double,
    pub net_bw_in_kbytes_sec: float,
}

//
// _vcs_dpi_soc_bounding_box_st: SOC definitions
//
// This struct maintains the SOC Bounding Box information for the ASIC; it
// defines things such as clock, voltage, performance, etc. Usually, we load
// these values from VBIOS; if something goes wrong, we use some hard-coded
// values, which will enable the ASIC to light up with limitations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_soc_bounding_box_st {
    pub clock_limits: [_vcs_dpi_voltage_scaling_st; DC__VOLTAGE_STATES],
//
// @num_states: It represents the total of Display Power Management
// (DPM) supported by the specific ASIC.
//
    pub num_states: c_uint,
    pub sr_exit_time_us: double,
    pub sr_enter_plus_exit_time_us: double,
    pub sr_exit_z8_time_us: double,
    pub sr_enter_plus_exit_z8_time_us: double,
    pub urgent_latency_us: double,
    pub urgent_latency_pixel_data_only_us: double,
    pub urgent_latency_pixel_mixed_with_vm_data_us: double,
    pub urgent_latency_vm_data_only_us: double,
    pub usr_retraining_latency_us: double,
    pub smn_latency_us: double,
    pub fclk_change_latency_us: double,
    pub mall_allocated_for_dcn_mbytes: double,
    pub pct_ideal_fabric_bw_after_urgent: double,
    pub pct_ideal_dram_bw_after_urgent_strobe: double,
    pub max_avg_fabric_bw_use_normal_percent: double,
    pub max_avg_dram_bw_use_normal_strobe_percent: double,
    pub allow_for_pstate_or_stutter_in_vblank_final: dm_prefetch_modes,
    pub dram_clock_change_requirement_final: bool,
    pub writeback_latency_us: double,
    pub ideal_dram_bw_after_urgent_percent: double,
    pub PercentOfIdealDRAMFabricAndSDPPortBWReceivedAfterUrgLatencyPixelDataOnly: double pct_ideal_dram_sdp_bw_after_urgent_pixel_only; //,
    pub pct_ideal_dram_sdp_bw_after_urgent_pixel_and_vm: double,
    pub pct_ideal_dram_sdp_bw_after_urgent_vm_only: double,
    pub pct_ideal_sdp_bw_after_urgent: double,
    pub max_avg_sdp_bw_use_normal_percent: double,
    pub max_avg_dram_bw_use_normal_percent: double,
    pub max_request_size_bytes: c_uint,
    pub downspread_percent: double,
    pub dram_page_open_time_ns: double,
    pub dram_rw_turnaround_time_ns: double,
    pub dram_return_buffer_per_channel_bytes: double,
    pub dram_channel_width_bytes: double,
    pub fabric_datapath_to_dcn_data_return_bytes: double,
    pub dcn_downspread_percent: double,
    pub dispclk_dppclk_vco_speed_mhz: double,
    pub dfs_vco_period_ps: double,
    pub urgent_out_of_order_return_per_channel_pixel_only_bytes: c_uint,
    pub urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: c_uint,
    pub urgent_out_of_order_return_per_channel_vm_only_bytes: c_uint,
    pub round_trip_ping_latency_dcfclk_cycles: c_uint,
    pub urgent_out_of_order_return_per_channel_bytes: c_uint,
    pub channel_interleave_bytes: c_uint,
    pub num_banks: c_uint,
    pub num_chans: c_uint,
    pub vmm_page_size_bytes: c_uint,
    pub hostvm_min_page_size_bytes: c_uint,
    pub gpuvm_min_page_size_bytes: c_uint,
    pub dram_clock_change_latency_us: double,
    pub dummy_pstate_latency_us: double,
    pub writeback_dram_clock_change_latency_us: double,
    pub return_bus_width_bytes: c_uint,
    pub voltage_override: c_uint,
    pub xfc_bus_transport_time_us: double,
    pub xfc_xbuf_latency_tolerance_us: double,
    pub use_urgent_burst_bw: c_int,
    pub min_dcfclk: double,
    pub do_urgent_latency_adjustment: bool,
    pub urgent_latency_adjustment_fabric_clock_component_us: double,
    pub urgent_latency_adjustment_fabric_clock_reference_mhz: double,
    pub disable_dram_clock_change_vactive_support: bool,
    pub allow_dram_clock_one_display_vactive: bool,
    pub allow_dram_self_refresh_or_dram_clock_change_in_vblank: self_refresh_affinity,
    pub max_vratio_pre: double,
}

//
// @_vcs_dpi_ip_params_st: IP configuraion for DCN blocks
//
// In this struct you can find the DCN configuration associated to the specific
// ASIC. For example, here we can save how many DPPs the ASIC is using and it
// is available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_ip_params_st {
    pub use_min_dcfclk: bool,
    pub clamp_min_dcfclk: bool,
    pub gpuvm_enable: bool,
    pub hostvm_enable: bool,
    pub dsc422_native_support: bool,
    pub gpuvm_max_page_table_levels: c_uint,
    pub hostvm_max_page_table_levels: c_uint,
    pub hostvm_cached_page_table_levels: c_uint,
    pub pte_group_size_bytes: c_uint,
    pub max_inter_dcn_tile_repeaters: c_uint,
    pub num_dsc: c_uint,
    pub odm_capable: c_uint,
    pub rob_buffer_size_kbytes: c_uint,
    pub det_buffer_size_kbytes: c_uint,
    pub min_comp_buffer_size_kbytes: c_uint,
    pub dpte_buffer_size_in_pte_reqs_luma: c_uint,
    pub dpte_buffer_size_in_pte_reqs_chroma: c_uint,
    pub pde_proc_buffer_size_64k_reqs: c_uint,
    pub dpp_output_buffer_pixels: c_uint,
    pub opp_output_buffer_lines: c_uint,
    pub pixel_chunk_size_kbytes: c_uint,
    pub alpha_pixel_chunk_size_kbytes: c_uint,
    pub min_pixel_chunk_size_bytes: c_uint,
    pub dcc_meta_buffer_size_bytes: c_uint,
    pub pte_enable: c_uchar,
    pub pte_chunk_size_kbytes: c_uint,
    pub meta_chunk_size_kbytes: c_uint,
    pub min_meta_chunk_size_bytes: c_uint,
    pub writeback_chunk_size_kbytes: c_uint,
    pub line_buffer_size_bits: c_uint,
    pub max_line_buffer_lines: c_uint,
    pub writeback_luma_buffer_size_kbytes: c_uint,
    pub writeback_chroma_buffer_size_kbytes: c_uint,
    pub writeback_chroma_line_buffer_width_pixels: c_uint,
    pub writeback_interface_buffer_size_kbytes: c_uint,
    pub writeback_line_buffer_buffer_size: c_uint,
    pub writeback_10bpc420_supported: c_uint,
    pub writeback_max_hscl_ratio: double,
    pub writeback_max_vscl_ratio: double,
    pub writeback_min_hscl_ratio: double,
    pub writeback_min_vscl_ratio: double,
    pub maximum_dsc_bits_per_component: c_uint,
    pub maximum_pixels_per_line_per_dsc_unit: c_uint,
    pub writeback_max_hscl_taps: c_uint,
    pub writeback_max_vscl_taps: c_uint,
    pub writeback_line_buffer_luma_buffer_size: c_uint,
    pub writeback_line_buffer_chroma_buffer_size: c_uint,
    pub max_page_table_levels: c_uint,
//
// @max_num_dpp: Maximum number of DPP supported in the target ASIC.
//
    pub max_num_dpp: c_uint,
    pub max_num_otg: c_uint,
    pub cursor_chunk_size: c_uint,
    pub cursor_buffer_size: c_uint,
    pub max_num_wb: c_uint,
    pub max_dchub_pscl_bw_pix_per_clk: c_uint,
    pub max_pscl_lb_bw_pix_per_clk: c_uint,
    pub max_lb_vscl_bw_pix_per_clk: c_uint,
    pub max_vscl_hscl_bw_pix_per_clk: c_uint,
    pub max_hscl_ratio: double,
    pub max_vscl_ratio: double,
    pub hscl_mults: c_uint,
    pub vscl_mults: c_uint,
    pub max_hscl_taps: c_uint,
    pub max_vscl_taps: c_uint,
    pub xfc_supported: c_uint,
    pub ptoi_supported: c_uint,
    pub gfx7_compat_tiling_supported: c_uint,
    pub odm_combine_4to1_supported: bool,
    pub dynamic_metadata_vm_enabled: bool,
    pub max_num_hdmi_frl_outputs: c_uint,
    pub xfc_fill_constant_bytes: c_uint,
    pub dispclk_ramp_margin_percent: double,
    pub xfc_fill_bw_overhead_percent: double,
    pub underscan_factor: double,
    pub min_vblank_lines: c_uint,
    pub dppclk_delay_subtotal: c_uint,
    pub dispclk_delay_subtotal: c_uint,
    pub dcfclk_cstate_latency: double,
    pub dppclk_delay_scl: c_uint,
    pub dppclk_delay_scl_lb_only: c_uint,
    pub dppclk_delay_cnvc_formatter: c_uint,
    pub dppclk_delay_cnvc_cursor: c_uint,
    pub is_line_buffer_bpp_fixed: c_uint,
    pub line_buffer_fixed_bpp: c_uint,
    pub dcc_supported: c_uint,
    pub config_return_buffer_size_in_kbytes: c_uint,
    pub compressed_buffer_segment_size_in_kbytes: c_uint,
    pub meta_fifo_size_in_kentries: c_uint,
    pub zero_size_buffer_entries: c_uint,
    pub compbuf_reserved_space_64b: c_uint,
    pub compbuf_reserved_space_zs: c_uint,
    pub IsLineBufferBppFixed: c_uint,
    pub LineBufferFixedBpp: c_uint,
    pub can_vstartup_lines_exceed_vsync_plus_back_porch_lines_minus_one: c_uint,
    pub bug_forcing_LC_req_same_size_fixed: c_uint,
    pub number_of_cursors: c_uint,
    pub max_num_dp2p0_outputs: c_uint,
    pub max_num_dp2p0_streams: c_uint,
    pub VBlankNomDefaultUS: c_uint,
// DM workarounds
    pub fix: double dsc_delay_factor_wa; // TODO: Remove after implementing root cause,
    pub min_prefetch_in_strobe_us: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_xfc_params_st {
    pub xfc_tslv_vready_offset_us: double,
    pub xfc_tslv_vupdate_width_us: double,
    pub xfc_tslv_vupdate_offset_us: double,
    pub xfc_slv_chunk_size_bytes: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_pipe_source_params_st {
    pub source_format: c_int,
    pub dcc_fraction_of_zs_req_luma: double,
    pub dcc_fraction_of_zs_req_chroma: double,
    pub dcc: c_uchar,
    pub dcc_rate: c_uint,
    pub dcc_rate_chroma: c_uint,
    pub dcc_use_global: c_uchar,
    pub vm: c_uchar,
    pub unbounded_req_mode: bool,
    pub enabled: bool gpuvm; // gpuvm,
    pub enabled: bool hostvm; // hostvm,
    pub gpuvm_levels_force_en: bool,
    pub gpuvm_levels_force: c_uint,
    pub hostvm_levels_force_en: bool,
    pub hostvm_levels_force: c_uint,
    pub source_scan: c_int,
    pub dml32: int source_rotation; // new in,
    pub struct: unsigned int det_size_override; // use to populate DETSizeOverride in vba,
    pub sw_mode: c_int,
    pub macro_tile_size: c_int,
    pub surface_width_y: c_uint,
    pub surface_height_y: c_uint,
    pub surface_width_c: c_uint,
    pub surface_height_c: c_uint,
    pub viewport_width: c_uint,
    pub viewport_height: c_uint,
    pub viewport_y_y: c_uint,
    pub viewport_y_c: c_uint,
    pub viewport_width_c: c_uint,
    pub viewport_height_c: c_uint,
    pub viewport_width_max: c_uint,
    pub viewport_height_max: c_uint,
    pub viewport_x_y: c_uint,
    pub viewport_x_c: c_uint,
    pub viewport_stationary: bool,
    pub dcc_rate_luma: c_uint,
    pub gpuvm_min_page_size_kbytes: c_uint,
    pub use_mall_for_pstate_change: c_uint,
    pub use_mall_for_static_screen: c_uint,
    pub force_one_row_for_frame: bool,
    pub pte_buffer_mode: bool,
    pub data_pitch: c_uint,
    pub data_pitch_c: c_uint,
    pub meta_pitch: c_uint,
    pub meta_pitch_c: c_uint,
    pub cur0_src_width: c_uint,
    pub cur0_bpp: c_int,
    pub cur1_src_width: c_uint,
    pub cur1_bpp: c_int,
    pub num_cursors: c_int,
    pub is_hsplit: c_uchar,
    pub dynamic_metadata_enable: c_uchar,
    pub dynamic_metadata_lines_before_active: c_uint,
    pub dynamic_metadata_xmit_bytes: c_uint,
    pub hsplit_grp: c_uint,
    pub xfc_enable: c_uchar,
    pub xfc_slave: c_uchar,
    pub immediate_flip: c_uchar,
    pub xfc_params: _vcs_dpi_display_xfc_params_st,
// for vstartuplines calculation freesync
    pub v_total_min: c_uchar,
    pub v_total_max: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct writeback_st {
    pub wb_src_height: c_int,
    pub wb_src_width: c_int,
    pub wb_dst_width: c_int,
    pub wb_dst_height: c_int,
    pub wb_pixel_format: c_int,
    pub wb_htaps_luma: c_int,
    pub wb_vtaps_luma: c_int,
    pub wb_htaps_chroma: c_int,
    pub wb_vtaps_chroma: c_int,
    pub wb_htaps: c_uint,
    pub wb_vtaps: c_uint,
    pub wb_hratio: double,
    pub wb_vratio: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_audio_params_st {
    pub audio_sample_rate_khz: c_uint,
    pub audio_sample_layout: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_output_params_st {
    pub dp_lanes: c_int,
    pub output_bpp: double,
    pub dsc_input_bpc: c_uint,
    pub dsc_enable: c_int,
    pub wb_enable: c_int,
    pub num_active_wb: c_int,
    pub output_type: c_int,
    pub is_virtual: c_int,
    pub output_format: c_int,
    pub dsc_slices: c_int,
    pub max_audio_sample_rate: c_int,
    pub wb: writeback_st,
    pub audio: display_audio_params_st,
    pub output_bpc: c_uint,
    pub dp_rate: c_int,
    pub dp_multistream_id: c_uint,
    pub dp_multistream_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_scaler_ratio_depth_st {
    pub hscl_ratio: double,
    pub vscl_ratio: double,
    pub hscl_ratio_c: double,
    pub vscl_ratio_c: double,
    pub vinit: double,
    pub vinit_c: double,
    pub vinit_bot: double,
    pub vinit_bot_c: double,
    pub lb_depth: c_int,
    pub scl_enable: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_scaler_taps_st {
    pub htaps: c_uint,
    pub vtaps: c_uint,
    pub htaps_c: c_uint,
    pub vtaps_c: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_pipe_dest_params_st {
    pub recout_width: c_uint,
    pub recout_height: c_uint,
    pub full_recout_width: c_uint,
    pub full_recout_height: c_uint,
    pub hblank_start: c_uint,
    pub hblank_end: c_uint,
    pub vblank_start: c_uint,
    pub vblank_end: c_uint,
    pub htotal: c_uint,
    pub vtotal: c_uint,
    pub vfront_porch: c_uint,
    pub vblank_nom: c_uint,
    pub vactive: c_uint,
    pub hactive: c_uint,
    pub vstartup_start: c_uint,
    pub vupdate_offset: c_uint,
    pub vupdate_width: c_uint,
    pub vready_offset: c_uint,
    pub pstate_keepout: c_uint,
    pub interlaced: c_uchar,
    pub pixel_rate_mhz: double,
    pub synchronized_vblank_all_planes: c_uchar,
    pub otg_inst: c_uchar,
    pub odm_combine: c_uint,
    pub use_maximum_vstartup: c_uchar,
    pub vtotal_max: c_uint,
    pub vtotal_min: c_uint,
    pub refresh_rate: c_uint,
    pub synchronize_timings: bool,
    pub odm_combine_policy: c_uint,
    pub drr_display: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_pipe_params_st {
    pub src: display_pipe_source_params_st,
    pub dest: display_pipe_dest_params_st,
    pub scale_ratio_depth: scaler_ratio_depth_st,
    pub scale_taps: scaler_taps_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_clocks_and_cfg_st {
    pub voltage: c_int,
    pub dppclk_mhz: double,
    pub refclk_mhz: double,
    pub dispclk_mhz: double,
    pub dcfclk_mhz: double,
    pub socclk_mhz: double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_e2e_pipe_params_st {
    pub pipe: display_pipe_params_st,
    pub dout: display_output_params_st,
    pub clks_cfg: display_clocks_and_cfg_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_data_rq_misc_params_st {
    pub full_swath_bytes: c_uint,
    pub stored_swath_bytes: c_uint,
    pub blk256_height: c_uint,
    pub blk256_width: c_uint,
    pub req_height: c_uint,
    pub req_width: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_data_rq_sizing_params_st {
    pub chunk_bytes: c_uint,
    pub min_chunk_bytes: c_uint,
    pub meta_chunk_bytes: c_uint,
    pub min_meta_chunk_bytes: c_uint,
    pub mpte_group_bytes: c_uint,
    pub dpte_group_bytes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_data_rq_dlg_params_st {
    pub swath_width_ub: c_uint,
    pub swath_height: c_uint,
    pub req_per_swath_ub: c_uint,
    pub meta_pte_bytes_per_frame_ub: c_uint,
    pub dpte_req_per_row_ub: c_uint,
    pub dpte_groups_per_row_ub: c_uint,
    pub dpte_row_height: c_uint,
    pub dpte_bytes_per_row_ub: c_uint,
    pub meta_chunks_per_row_ub: c_uint,
    pub meta_req_per_row_ub: c_uint,
    pub meta_row_height: c_uint,
    pub meta_bytes_per_row_ub: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_rq_dlg_params_st {
    pub rq_l: display_data_rq_dlg_params_st,
    pub rq_c: display_data_rq_dlg_params_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_rq_sizing_params_st {
    pub rq_l: display_data_rq_sizing_params_st,
    pub rq_c: display_data_rq_sizing_params_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_rq_misc_params_st {
    pub rq_l: display_data_rq_misc_params_st,
    pub rq_c: display_data_rq_misc_params_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_rq_params_st {
    pub yuv420: c_uchar,
    pub yuv420_10bpc: c_uchar,
    pub rgbe_alpha: c_uchar,
    pub misc: display_rq_misc_params_st,
    pub sizing: display_rq_sizing_params_st,
    pub dlg: display_rq_dlg_params_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_dlg_regs_st {
    pub refcyc_h_blank_end: c_uint,
    pub dlg_vblank_end: c_uint,
    pub min_dst_y_next_start: c_uint,
    pub min_dst_y_next_start_us: c_uint,
    pub refcyc_per_htotal: c_uint,
    pub refcyc_x_after_scaler: c_uint,
    pub dst_y_after_scaler: c_uint,
    pub dst_y_prefetch: c_uint,
    pub dst_y_per_vm_vblank: c_uint,
    pub dst_y_per_row_vblank: c_uint,
    pub dst_y_per_vm_flip: c_uint,
    pub dst_y_per_row_flip: c_uint,
    pub ref_freq_to_pix_freq: c_uint,
    pub vratio_prefetch: c_uint,
    pub vratio_prefetch_c: c_uint,
    pub refcyc_per_tdlut_group: c_uint,
    pub refcyc_per_pte_group_vblank_l: c_uint,
    pub refcyc_per_pte_group_vblank_c: c_uint,
    pub refcyc_per_meta_chunk_vblank_l: c_uint,
    pub refcyc_per_meta_chunk_vblank_c: c_uint,
    pub refcyc_per_pte_group_flip_l: c_uint,
    pub refcyc_per_pte_group_flip_c: c_uint,
    pub refcyc_per_meta_chunk_flip_l: c_uint,
    pub refcyc_per_meta_chunk_flip_c: c_uint,
    pub dst_y_per_pte_row_nom_l: c_uint,
    pub dst_y_per_pte_row_nom_c: c_uint,
    pub refcyc_per_pte_group_nom_l: c_uint,
    pub refcyc_per_pte_group_nom_c: c_uint,
    pub dst_y_per_meta_row_nom_l: c_uint,
    pub dst_y_per_meta_row_nom_c: c_uint,
    pub refcyc_per_meta_chunk_nom_l: c_uint,
    pub refcyc_per_meta_chunk_nom_c: c_uint,
    pub refcyc_per_line_delivery_pre_l: c_uint,
    pub refcyc_per_line_delivery_pre_c: c_uint,
    pub refcyc_per_line_delivery_l: c_uint,
    pub refcyc_per_line_delivery_c: c_uint,
    pub chunk_hdl_adjust_cur0: c_uint,
    pub chunk_hdl_adjust_cur1: c_uint,
    pub vready_after_vcount0: c_uint,
    pub dst_y_offset_cur0: c_uint,
    pub dst_y_offset_cur1: c_uint,
    pub xfc_reg_transfer_delay: c_uint,
    pub xfc_reg_precharge_delay: c_uint,
    pub xfc_reg_remote_surface_flip_latency: c_uint,
    pub xfc_reg_prefetch_margin: c_uint,
    pub dst_y_delta_drq_limit: c_uint,
    pub refcyc_per_vm_group_vblank: c_uint,
    pub refcyc_per_vm_group_flip: c_uint,
    pub refcyc_per_vm_req_vblank: c_uint,
    pub refcyc_per_vm_req_flip: c_uint,
    pub refcyc_per_vm_dmdata: c_uint,
    pub dmdata_dl_delta: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_ttu_regs_st {
    pub qos_level_low_wm: c_uint,
    pub qos_level_high_wm: c_uint,
    pub min_ttu_vblank: c_uint,
    pub qos_level_flip: c_uint,
    pub refcyc_per_req_delivery_l: c_uint,
    pub refcyc_per_req_delivery_c: c_uint,
    pub refcyc_per_req_delivery_cur0: c_uint,
    pub refcyc_per_req_delivery_cur1: c_uint,
    pub refcyc_per_req_delivery_pre_l: c_uint,
    pub refcyc_per_req_delivery_pre_c: c_uint,
    pub refcyc_per_req_delivery_pre_cur0: c_uint,
    pub refcyc_per_req_delivery_pre_cur1: c_uint,
    pub qos_level_fixed_l: c_uint,
    pub qos_level_fixed_c: c_uint,
    pub qos_level_fixed_cur0: c_uint,
    pub qos_level_fixed_cur1: c_uint,
    pub qos_ramp_disable_l: c_uint,
    pub qos_ramp_disable_c: c_uint,
    pub qos_ramp_disable_cur0: c_uint,
    pub qos_ramp_disable_cur1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_data_rq_regs_st {
    pub chunk_size: c_uint,
    pub min_chunk_size: c_uint,
    pub meta_chunk_size: c_uint,
    pub min_meta_chunk_size: c_uint,
    pub dpte_group_size: c_uint,
    pub mpte_group_size: c_uint,
    pub swath_height: c_uint,
    pub pte_row_height_linear: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_rq_regs_st {
    pub rq_regs_l: display_data_rq_regs_st,
    pub rq_regs_c: display_data_rq_regs_st,
    pub drq_expansion_mode: c_uint,
    pub prq_expansion_mode: c_uint,
    pub mrq_expansion_mode: c_uint,
    pub crq_expansion_mode: c_uint,
    pub plane1_base_address: c_uint,
    pub [47:18]: unsigned int aperture_low_addr; // bits,
    pub [47:18]: unsigned int aperture_high_addr; // bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_dlg_sys_params_st {
    pub t_mclk_wm_us: double,
    pub t_urg_wm_us: double,
    pub t_sr_wm_us: double,
    pub t_extra_us: double,
    pub mem_trip_us: double,
    pub deepsleep_dcfclk_mhz: double,
    pub total_flip_bw: double,
    pub total_flip_bytes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _vcs_dpi_display_arb_params_st {
    pub max_req_outstanding: c_int,
    pub min_req_outstanding: c_int,
    pub sat_level_us: c_int,
    pub hvm_min_req_outstand_commit_threshold: c_int,
    pub hvm_max_qos_commit_threshold: c_int,
    pub compbuf_reserved_space_kbytes: c_int,
}
