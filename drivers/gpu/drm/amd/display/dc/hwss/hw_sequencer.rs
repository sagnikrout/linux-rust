//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/hw_sequencer.h
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
// Copyright 2015-2026 Advanced Micro Devices, Inc.
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
#[derive(Copy, Clone)]
pub struct subvp_pipe_control_lock_fast_params {
    pub dc: *mut dc,
    pub lock: bool,
    pub subvp_immediate_flip: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_control_lock_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub lock: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_flip_control_gsl_params {
    pub hubp: *mut hubp,
    pub flip_immediate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_triplebuffer_params {
    pub dc: *const dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub enableTripleBuffer: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_plane_addr_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_input_transfer_func_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub plane_state: *mut dc_plane_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_gamut_remap_params {
    pub xfm: *mut transform,
    pub dpp: *mut dpp,
    pub mpc: *mut mpc,
    pub mpcc_id: c_int,
    pub stream: *const dc_stream_state,
    pub plane: *const dc_plane_state,
    pub is_top_pipe: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_enable_3dlut_fl_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_setup_vertical_interrupt0_params {
    pub tg: *mut timing_generator,
    pub start_line: u32,
    pub end_line: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_info_frame_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_manual_trigger_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_dmcub_cmd_params {
    pub ctx: *mut dc_context,
    pub cmd: *mut dmub_rb_cmd,
    pub wait_type: dm_dmub_wait_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct setup_dpp_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_bias_and_scale_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_output_transfer_func_params {
    pub xfm: *mut transform,
    pub dpp: *mut dpp,
    pub mpc: *mut mpc,
    pub mpcc_id: c_int,
    pub is_top_pipe: bool,
    pub stream: *const dc_stream_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_upsp_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_visual_confirm_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub mpcc_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_on_mpc_mem_pwr_params {
    pub mpc: *mut mpc,
    pub mpcc_id: c_int,
    pub power_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_output_csc_params {
    pub mpc: *mut mpc,
    pub opp_id: c_int,
    pub regval: *const u16,
    pub ocsc_mode: mpc_output_csc_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_ocsc_default_params {
    pub mpc: *mut mpc,
    pub opp_id: c_int,
    pub color_space: dc_color_space,
    pub ocsc_mode: mpc_output_csc_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subvp_save_surf_addr {
    pub dc_dmub_srv: *mut dc_dmub_srv,
    pub addr: *const dc_plane_address,
    pub subvp_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_for_dcc_meta_propagation_params {
    pub dc: *const dc,
    pub top_pipe_to_program: *const pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_hw_control_lock_fast_params {
    pub dc: *mut dc,
    pub is_required: bool,
    pub lock: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_surface_config_params {
    pub hubp: *mut hubp,
    pub format: surface_pixel_format,
    pub tiling_info: *mut dc_tiling_info,
    pub plane_size: plane_size,
    pub rotation: dc_rotation_angle,
    pub dcc: *mut dc_plane_dcc_param,
    pub horizontal_mirror: bool,
    pub compat_level: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_mcache_id_and_split_coordinate {
    pub hubp: *mut hubp,
    pub mcache_regs: *mut dml2_hubp_pipe_mcache_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct control_cm_hist_params {
    pub dpp: *mut dpp,
    pub cm_hist_control: cm_hist_control,
    pub color_space: dc_color_space,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_cursor_update_now_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_wait_pipe_read_start_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apply_update_flags_for_phantom_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_phantom_vp_position_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_odm_combine_params {
    pub tg: *mut timing_generator,
    pub opp_inst: [c_int; MAX_PIPES],
    pub opp_head_count: c_int,
    pub odm_slice_width: c_int,
    pub last_odm_slice_width: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_odm_bypass_params {
    pub tg: *mut timing_generator,
    pub timing: *const dc_crtc_timing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_pipe_clock_control_params {
    pub opp: *mut output_pixel_processor,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_program_left_edge_extra_pixel_params {
    pub opp: *mut output_pixel_processor,
    pub pixel_encoding: dc_pixel_encoding,
    pub is_otg_master: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_set_dto_dscclk_params {
    pub dccg: *mut dccg,
    pub inst: c_int,
    pub num_slices_h: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_set_config_params {
    pub dsc: *mut display_stream_compressor,
    pub dsc_cfg: *mut dsc_config,
    pub dsc_optc_cfg: *mut dsc_optc_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_enable_params {
    pub dsc: *mut display_stream_compressor,
    pub opp_inst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_set_dsc_config_params {
    pub tg: *mut timing_generator,
    pub dsc_optc_cfg: *mut dsc_optc_config,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_disconnect_params {
    pub dsc: *mut display_stream_compressor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_read_state_params {
    pub dsc: *mut display_stream_compressor,
    pub dsc_state: *mut dcn_dsc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_calculate_and_set_config_params {
    pub pipe_ctx: *mut pipe_ctx,
    pub dsc_optc_cfg: dsc_optc_config,
    pub enable: bool,
    pub opp_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_enable_with_opp_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_tg_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_program_global_sync_params {
    pub tg: *mut timing_generator,
    pub vready_offset: c_int,
    pub vstartup_lines: c_uint,
    pub vupdate_offset_pixels: c_uint,
    pub vupdate_vupdate_width_pixels: c_uint,
    pub pstate_keepout_start_lines: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_wait_for_state_params {
    pub tg: *mut timing_generator,
    pub state: crtc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_set_vtg_params_params {
    pub tg: *mut timing_generator,
    pub timing: *mut dc_crtc_timing,
    pub program_fp2: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_set_gsl_params {
    pub tg: *mut timing_generator,
    pub gsl: gsl_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_set_gsl_source_select_params {
    pub tg: *mut timing_generator,
    pub group_idx: c_int,
    pub gsl_ready_signal: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct setup_vupdate_interrupt_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_setup_vertical_interrupt2_params {
    pub tg: *mut timing_generator,
    pub start_line: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_set_hdr_multiplier_params {
    pub dpp: *mut dpp,
    pub hw_mult: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_det_size_params {
    pub hubbub: *mut hubbub,
    pub hubp_inst: c_uint,
    pub det_buffer_size_kb: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_det_segments_params {
    pub hubbub: *mut hubbub,
    pub hubp_inst: c_uint,
    pub det_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_dchubp_dpp_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_set_dyn_expansion_params {
    pub opp: *mut output_pixel_processor,
    pub color_space: dc_color_space,
    pub color_depth: dc_color_depth,
    pub signal: signal_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_program_fmt_params {
    pub opp: *mut output_pixel_processor,
    pub fmt_bit_depth: *mut bit_depth_reduction_params,
    pub clamping: *mut clamping_and_pixel_encoding_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_program_bit_depth_reduction_params {
    pub opp: *mut output_pixel_processor,
    pub use_default_params: bool,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_set_disp_pattern_generator_params {
    pub opp: *mut output_pixel_processor,
    pub test_pattern: controller_dp_test_pattern,
    pub color_space: controller_dp_color_space,
    pub color_depth: dc_color_depth,
    pub solid_color: tg_color,
    pub use_solid_color: bool,
    pub width: c_int,
    pub height: c_int,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_abm_pipe_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_abm_level_params {
    pub abm: *mut abm,
    pub abm_level: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_abm_immediate_disable_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_disp_pattern_generator_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub test_pattern: controller_dp_test_pattern,
    pub color_space: controller_dp_color_space,
    pub color_depth: dc_color_depth,
    pub solid_color: *const tg_color,
    pub width: c_int,
    pub height: c_int,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_update_blending_params {
    pub mpc: *mut mpc,
    pub blnd_cfg: mpcc_blnd_cfg,
    pub mpcc_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_assert_idle_mpcc_params {
    pub mpc: *mut mpc,
    pub mpcc_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_insert_plane_params {
    pub mpc: *mut mpc,
    pub mpc_tree_params: *mut mpc_tree,
    pub blnd_cfg: mpcc_blnd_cfg,
    pub sm_cfg: *mut mpcc_sm_cfg,
    pub insert_above_mpcc: *mut mpcc,
    pub dpp_id: c_int,
    pub mpcc_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_remove_mpcc_params {
    pub mpc: *mut mpc,
    pub mpc_tree_params: *mut mpc_tree,
    pub mpcc_to_remove: *mut mpcc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opp_set_mpcc_disconnect_pending_params {
    pub opp: *mut output_pixel_processor,
    pub mpcc_inst: c_int,
    pub pending: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_set_optimized_required_params {
    pub dc: *mut dc,
    pub optimized_required: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_disconnect_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_force_pstate_change_control_params {
    pub hubbub: *mut hubbub,
    pub enable: bool,
    pub wait: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_enable_crtc_params {
    pub tg: *mut timing_generator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_wait_flip_pending_params {
    pub hubp: *mut hubp,
    pub timeout_us: c_uint,
    pub polling_interval_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_wait_double_buffer_pending_params {
    pub tg: *mut timing_generator,
    pub timeout_us: c_uint,
    pub polling_interval_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_force_pstate_params {
    pub dc: *mut dc,
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_apply_dedcn21_147_wa_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_allow_self_refresh_control_params {
    pub hubbub: *mut hubbub,
    pub allow: bool,
    pub disallow_self_refresh_applied: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tg_get_frame_count_params {
    pub tg: *mut timing_generator,
    pub frame_count: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_set_dwb_mux_params {
    pub mpc: *mut mpc,
    pub dwb_id: c_int,
    pub mpcc_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_disable_dwb_mux_params {
    pub mpc: *mut mpc,
    pub dwb_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb_config_buf_params {
    pub mcif_wb: *mut mcif_wb,
    pub mcif_buf_params: *mut mcif_buf_params,
    pub dest_height: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb_config_arb_params {
    pub mcif_wb: *mut mcif_wb,
    pub mcif_arb_params: *mut mcif_arb_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb_enable_params {
    pub mcif_wb: *mut mcif_wb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb_disable_params {
    pub mcif_wb: *mut mcif_wb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwbc_enable_params {
    pub dwb: *mut dwbc,
    pub dwb_params: *mut dc_dwb_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwbc_disable_params {
    pub dwb: *mut dwbc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwbc_update_params {
    pub dwb: *mut dwbc,
    pub dwb_params: *mut dc_dwb_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_update_mall_sel_params {
    pub hubp: *mut hubp,
    pub mall_sel: u32,
    pub cache_cursor: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_prepare_subvp_buffering_params {
    pub hubp: *mut hubp,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_set_blank_en_params {
    pub hubp: *mut hubp,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_disable_control_params {
    pub hubp: *mut hubp,
    pub disable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_soft_reset_params {
    pub hubbub: *mut hubbub,
    pub reset): *mut *mut *mut void (hubbub_soft_reset)(struct hubbub hubbub, bool,
    pub reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_reset_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_arm_out_of_order_bw_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_start_out_of_order_bw_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_start_in_order_bw_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_start_memory_latencies_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_start_urgent_assertion_count_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_start_urgent_ramp_latency_params {
    pub hubbub: *mut hubbub,
    pub latency_params: hubbub_urgent_latency_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_start_prefetch_data_size_params {
    pub hubbub: *mut hubbub,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_get_out_of_order_bw_params {
    pub hubbub: *mut hubbub,
    pub refclk_mhz: u32,
    pub bandwidth_mbps: *mut u32,
    pub duration_ns: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_get_in_order_bw_params {
    pub hubbub: *mut hubbub,
    pub refclk_mhz: u32,
    pub min_duration_ns: u32,
    pub bandwidth_mbps: *mut u32,
    pub duration_ns: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_get_memory_latencies_params {
    pub hubbub: *mut hubbub,
    pub refclk_mhz: u32,
    pub result: *mut dc_probe_latencies,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_get_urgent_assertion_count_params {
    pub hubbub: *mut hubbub,
    pub refclk_mhz: u32,
    pub assertion_count: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_get_prefetch_data_size_params {
    pub hubbub: *mut hubbub,
    pub prefetch_data_size: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_perfmon_get_urgent_ramp_latency_params {
    pub hubbub: *mut hubbub,
    pub refclk_mhz: u32,
    pub latency_ns: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_clk_cntl_params {
    pub hubp: *mut hubp,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_init_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_set_vm_system_aperture_settings_params {
    pub hubp: *mut hubp,
// struct vm_system_aperture_param apt;
    pub sys_default: PHYSICAL_ADDRESS_LOC,
    pub sys_low: PHYSICAL_ADDRESS_LOC,
    pub sys_high: PHYSICAL_ADDRESS_LOC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_set_flip_int_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_dppclk_control_params {
    pub dpp: *mut dpp,
    pub dppclk_div: bool,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disable_phantom_crtc_params {
    pub tg: *mut timing_generator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_pg_control_params {
    pub hws: *mut dce_hwseq,
    pub dpp_inst: c_uint,
    pub power_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_pg_control_params {
    pub hws: *mut dce_hwseq,
    pub hubp_inst: c_uint,
    pub power_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_reset_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_reset_params {
    pub dpp: *mut dpp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_root_clock_control_params {
    pub hws: *mut dce_hwseq,
    pub dpp_inst: c_uint,
    pub clock_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_ip_request_cntl_params {
    pub dc: *mut dc,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_pg_status_params {
    pub hws: *mut dce_hwseq,
    pub dsc_inst: c_int,
    pub is_ungated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_wait_disconnect_pending_clear_params {
    pub dsc: *mut display_stream_compressor,
    pub is_ungated: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_disable_params {
    pub dsc: *mut display_stream_compressor,
    pub is_ungated: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_set_ref_dscclk_params {
    pub dccg: *mut dccg,
    pub dsc_inst: c_int,
    pub is_ungated: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dccg_update_dpp_dto_params {
    pub dccg: *mut dccg,
    pub dpp_inst: c_int,
    pub dppclk_khz: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_vtg_sel_params {
    pub hubp: *mut hubp,
    pub otg_inst: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_setup2_params {
    pub hubp: *mut hubp,
    pub hubp_regs: *mut dml2_dchub_per_pipe_register_set,
    pub global_sync: *mut dml2_global_sync_programming,
    pub timing: *mut dc_crtc_timing,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_setup_params {
    pub hubp: *mut hubp,
    pub dlg_regs: *mut _vcs_dpi_display_dlg_regs_st,
    pub ttu_regs: *mut _vcs_dpi_display_ttu_regs_st,
    pub rq_regs: *mut _vcs_dpi_display_rq_regs_st,
    pub pipe_dest: *mut _vcs_dpi_display_pipe_dest_params_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_set_unbounded_requesting_params {
    pub hubp: *mut hubp,
    pub unbounded_req: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_setup_interdependent2_params {
    pub hubp: *mut hubp,
    pub hubp_regs: *mut dml2_dchub_per_pipe_register_set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_setup_interdependent_params {
    pub hubp: *mut hubp,
    pub dlg_regs: *mut _vcs_dpi_display_dlg_regs_st,
    pub ttu_regs: *mut _vcs_dpi_display_ttu_regs_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_set_cursor_matrix_params {
    pub dpp: *mut dpp,
    pub color_space: dc_color_space,
    pub cursor_csc_color_matrix: *mut dc_csc_transform,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_update_mpcc_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_set_scaler_params {
    pub dpp: *mut dpp,
    pub scl_data: *const scaler_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_mem_program_viewport_params {
    pub hubp: *mut hubp,
    pub viewport: *const rect,
    pub viewport_c: *const rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_program_mcache_id_and_split_coordinate_params {
    pub hubp: *mut hubp,
    pub mcache_regs: *mut mcache_regs_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct abort_cursor_offload_update_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cursor_lock_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub lock: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct setup_periodic_interrupt_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_cursor_info_to_dmu_params {
    pub pipe_ctx: *mut pipe_ctx,
    pub pipe_idx: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_cursor_attribute_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_set_cursor_attributes_params {
    pub hubp: *mut hubp,
    pub attributes: *const dc_cursor_attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_set_cursor_attributes_params {
    pub dpp: *mut dpp,
    pub attributes: *mut dc_cursor_attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_cursor_position_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_cursor_sdr_white_level_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct program_output_csc_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
    pub colorspace: dc_color_space,
    pub matrix: *mut u16,
    pub opp_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubp_set_blank_params {
    pub hubp: *mut hubp,
    pub blank: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phantom_hubp_post_enable_params {
    pub hubp: *mut hubp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct begin_cursor_offload_update_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_cursor_offload_pipe_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct commit_cursor_offload_update_params {
    pub dc: *mut dc,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_enc_update_hdmi_info_packets_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_stream_enc_update_hdmi_info_packets_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_stream_enc_update_dp_info_packets_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_enc_update_dp_info_packets_sdp_line_num_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_enc_update_dp_info_packets_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_set_config_simple_params {
    pub dsc: *mut display_stream_compressor,
    pub dsc_cfg: dsc_config,
    pub dsc_optc_cfg: dsc_optc_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_enc_dp_set_dsc_config_params {
    pub stream_enc: *mut stream_encoder,
    pub dsc_optc_cfg: *const dsc_optc_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_stream_enc_dp_set_dsc_pps_info_packet_params {
    pub hpo_dp_stream_enc: *mut hpo_dp_stream_encoder,
    pub immediate_update: bool,
    pub dsc_packed_pps: *mut u8,
    pub pps_sdp_stream: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_enc_dp_set_dsc_pps_info_packet_params {
    pub stream_enc: *mut stream_encoder,
    pub immediate_update: bool,
    pub dsc_packed_pps: *mut u8,
    pub pps_sdp_stream: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_stream_enc_set_dsc_config_params {
    pub hpo_frl_stream_enc: *mut hpo_frl_stream_encoder,
    pub timing: *const dc_crtc_timing,
    pub dsc_packed_pps: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_trace_source_sequence_params {
    pub link: *mut dc_link,
    pub source: dpcd_source_sequence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_dmdata_attributes_params {
    pub hubp: *mut hubp,
    pub attr: dc_dmdata_attributes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_increase_mst_payload_params {
    pub pipe_ctx: *mut pipe_ctx,
    pub mst_stream_bw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_reduce_mst_payload_params {
    pub pipe_ctx: *mut pipe_ctx,
    pub mst_stream_bw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_set_test_pattern_params {
    pub link: *mut dc_link,
    pub test_pattern: dp_test_pattern,
    pub test_pattern_color_space: dp_test_pattern_color_space,
    pub p_link_settings: *const link_training_settings,
    pub p_custom_pattern: *const c_uchar,
    pub cust_pattern_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_set_dpms_off_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disable_audio_stream_params {
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_set_max_memclk_params {
    pub clk_mgr: *mut clk_mgr,
    pub memclk_mhz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mgr_update_clocks_params {
    pub clk_mgr: *mut clk_mgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_program_watermarks_params {
    pub dc: *mut dc,
    pub hubbub: *mut hubbub,
    pub watermarks: *mut dcn_watermark_set,
    pub refclk_mhz: c_uint,
    pub safe_to_lower: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_program_arbiter_params {
    pub dc: *mut dc,
    pub hubbub: *mut hubbub,
    pub arb_regs: *mut dml2_display_arb_regs,
    pub safe_to_lower: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hubbub_program_compbuf_segments_params {
    pub hubbub: *mut hubbub,
    pub compbuf_size: c_uint,
    pub safe_to_lower: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prepare_bandwidth_params {
    pub dc: *mut dc,
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_set_dpms_on_params {
    pub state: *mut dc_state,
    pub pipe_ctx: *mut pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union block_sequence_params {
    pub update_plane_addr_params: update_plane_addr_params,
    pub subvp_pipe_control_lock_fast_params: subvp_pipe_control_lock_fast_params,
    pub pipe_control_lock_params: pipe_control_lock_params,
    pub set_flip_control_gsl_params: set_flip_control_gsl_params,
    pub program_triplebuffer_params: program_triplebuffer_params,
    pub set_input_transfer_func_params: set_input_transfer_func_params,
    pub program_gamut_remap_params: program_gamut_remap_params,
    pub hubp_enable_3dlut_fl_params: hubp_enable_3dlut_fl_params,
    pub tg_setup_vertical_interrupt0_params: tg_setup_vertical_interrupt0_params,
    pub update_info_frame_params: update_info_frame_params,
    pub program_manual_trigger_params: program_manual_trigger_params,
    pub send_dmcub_cmd_params: send_dmcub_cmd_params,
    pub setup_dpp_params: setup_dpp_params,
    pub program_bias_and_scale_params: program_bias_and_scale_params,
    pub set_output_transfer_func_params: set_output_transfer_func_params,
    pub program_upsp_params: program_upsp_params,
    pub update_visual_confirm_params: update_visual_confirm_params,
    pub power_on_mpc_mem_pwr_params: power_on_mpc_mem_pwr_params,
    pub set_output_csc_params: set_output_csc_params,
    pub set_ocsc_default_params: set_ocsc_default_params,
    pub subvp_save_surf_addr: subvp_save_surf_addr,
    pub wait_for_dcc_meta_propagation_params: wait_for_dcc_meta_propagation_params,
    pub dmub_hw_control_lock_fast_params: dmub_hw_control_lock_fast_params,
    pub program_surface_config_params: program_surface_config_params,
    pub program_mcache_id_and_split_coordinate: program_mcache_id_and_split_coordinate,
    pub control_cm_hist_params: control_cm_hist_params,
    pub program_cursor_update_now_params: program_cursor_update_now_params,
    pub hubp_wait_pipe_read_start_params: hubp_wait_pipe_read_start_params,
    pub apply_update_flags_for_phantom_params: apply_update_flags_for_phantom_params,
    pub update_phantom_vp_position_params: update_phantom_vp_position_params,
    pub set_odm_combine_params: set_odm_combine_params,
    pub set_odm_bypass_params: set_odm_bypass_params,
    pub opp_pipe_clock_control_params: opp_pipe_clock_control_params,
    pub opp_program_left_edge_extra_pixel_params: opp_program_left_edge_extra_pixel_params,
    pub dccg_set_dto_dscclk_params: dccg_set_dto_dscclk_params,
    pub dsc_set_config_params: dsc_set_config_params,
    pub dsc_enable_params: dsc_enable_params,
    pub tg_set_dsc_config_params: tg_set_dsc_config_params,
    pub dsc_disconnect_params: dsc_disconnect_params,
    pub dsc_read_state_params: dsc_read_state_params,
    pub dsc_calculate_and_set_config_params: dsc_calculate_and_set_config_params,
    pub dsc_enable_with_opp_params: dsc_enable_with_opp_params,
    pub program_tg_params: program_tg_params,
    pub tg_program_global_sync_params: tg_program_global_sync_params,
    pub tg_wait_for_state_params: tg_wait_for_state_params,
    pub tg_set_vtg_params_params: tg_set_vtg_params_params,
    pub tg_setup_vertical_interrupt2_params: tg_setup_vertical_interrupt2_params,
    pub dpp_set_hdr_multiplier_params: dpp_set_hdr_multiplier_params,
    pub tg_set_gsl_params: tg_set_gsl_params,
    pub tg_set_gsl_source_select_params: tg_set_gsl_source_select_params,
    pub setup_vupdate_interrupt_params: setup_vupdate_interrupt_params,
    pub program_det_size_params: program_det_size_params,
    pub program_det_segments_params: program_det_segments_params,
    pub update_dchubp_dpp_params: update_dchubp_dpp_params,
    pub opp_set_dyn_expansion_params: opp_set_dyn_expansion_params,
    pub opp_program_fmt_params: opp_program_fmt_params,
    pub opp_program_bit_depth_reduction_params: opp_program_bit_depth_reduction_params,
    pub opp_set_disp_pattern_generator_params: opp_set_disp_pattern_generator_params,
    pub set_abm_pipe_params: set_abm_pipe_params,
    pub set_abm_level_params: set_abm_level_params,
    pub set_abm_immediate_disable_params: set_abm_immediate_disable_params,
    pub set_disp_pattern_generator_params: set_disp_pattern_generator_params,
    pub mpc_remove_mpcc_params: mpc_remove_mpcc_params,
    pub opp_set_mpcc_disconnect_pending_params: opp_set_mpcc_disconnect_pending_params,
    pub dc_set_optimized_required_params: dc_set_optimized_required_params,
    pub hubp_disconnect_params: hubp_disconnect_params,
    pub hubbub_force_pstate_change_control_params: hubbub_force_pstate_change_control_params,
    pub tg_enable_crtc_params: tg_enable_crtc_params,
    pub hubp_wait_flip_pending_params: hubp_wait_flip_pending_params,
    pub tg_wait_double_buffer_pending_params: tg_wait_double_buffer_pending_params,
    pub update_force_pstate_params: update_force_pstate_params,
    pub hubbub_apply_dedcn21_147_wa_params: hubbub_apply_dedcn21_147_wa_params,
    pub hubbub_allow_self_refresh_control_params: hubbub_allow_self_refresh_control_params,
    pub tg_get_frame_count_params: tg_get_frame_count_params,
    pub mpc_set_dwb_mux_params: mpc_set_dwb_mux_params,
    pub mpc_disable_dwb_mux_params: mpc_disable_dwb_mux_params,
    pub mcif_wb_config_buf_params: mcif_wb_config_buf_params,
    pub mcif_wb_config_arb_params: mcif_wb_config_arb_params,
    pub mcif_wb_enable_params: mcif_wb_enable_params,
    pub mcif_wb_disable_params: mcif_wb_disable_params,
    pub dwbc_enable_params: dwbc_enable_params,
    pub dwbc_disable_params: dwbc_disable_params,
    pub dwbc_update_params: dwbc_update_params,
    pub hubp_update_mall_sel_params: hubp_update_mall_sel_params,
    pub hubp_prepare_subvp_buffering_params: hubp_prepare_subvp_buffering_params,
    pub hubp_set_blank_en_params: hubp_set_blank_en_params,
    pub hubp_disable_control_params: hubp_disable_control_params,
    pub hubbub_soft_reset_params: hubbub_soft_reset_params,
    pub hubbub_perfmon_reset_params: hubbub_perfmon_reset_params,
    pub hubbub_perfmon_arm_out_of_order_bw_params: hubbub_perfmon_arm_out_of_order_bw_params,
    pub hubbub_perfmon_start_out_of_order_bw_params: hubbub_perfmon_start_out_of_order_bw_params,
    pub hubbub_perfmon_start_in_order_bw_params: hubbub_perfmon_start_in_order_bw_params,
    pub hubbub_perfmon_start_memory_latencies_params: hubbub_perfmon_start_memory_latencies_params,
    pub hubbub_perfmon_start_urgent_assertion_count_params: hubbub_perfmon_start_urgent_assertion_count_params,
    pub hubbub_perfmon_start_urgent_ramp_latency_params: hubbub_perfmon_start_urgent_ramp_latency_params,
    pub hubbub_perfmon_start_prefetch_data_size_params: hubbub_perfmon_start_prefetch_data_size_params,
    pub hubbub_perfmon_get_out_of_order_bw_params: hubbub_perfmon_get_out_of_order_bw_params,
    pub hubbub_perfmon_get_in_order_bw_params: hubbub_perfmon_get_in_order_bw_params,
    pub hubbub_perfmon_get_memory_latencies_params: hubbub_perfmon_get_memory_latencies_params,
    pub hubbub_perfmon_get_urgent_assertion_count_params: hubbub_perfmon_get_urgent_assertion_count_params,
    pub hubbub_perfmon_get_prefetch_data_size_params: hubbub_perfmon_get_prefetch_data_size_params,
    pub hubbub_perfmon_get_urgent_ramp_latency_params: hubbub_perfmon_get_urgent_ramp_latency_params,
    pub hubp_clk_cntl_params: hubp_clk_cntl_params,
    pub hubp_init_params: hubp_init_params,
    pub hubp_set_vm_system_aperture_settings_params: hubp_set_vm_system_aperture_settings_params,
    pub hubp_set_flip_int_params: hubp_set_flip_int_params,
    pub dpp_dppclk_control_params: dpp_dppclk_control_params,
    pub disable_phantom_crtc_params: disable_phantom_crtc_params,
    pub dpp_pg_control_params: dpp_pg_control_params,
    pub hubp_pg_control_params: hubp_pg_control_params,
    pub hubp_reset_params: hubp_reset_params,
    pub dpp_reset_params: dpp_reset_params,
    pub dpp_root_clock_control_params: dpp_root_clock_control_params,
    pub dc_ip_request_cntl_params: dc_ip_request_cntl_params,
    pub dsc_pg_status_params: dsc_pg_status_params,
    pub dsc_wait_disconnect_pending_clear_params: dsc_wait_disconnect_pending_clear_params,
    pub dsc_disable_params: dsc_disable_params,
    pub dccg_set_ref_dscclk_params: dccg_set_ref_dscclk_params,
    pub dccg_update_dpp_dto_params: dccg_update_dpp_dto_params,
    pub hubp_vtg_sel_params: hubp_vtg_sel_params,
    pub hubp_setup2_params: hubp_setup2_params,
    pub hubp_setup_params: hubp_setup_params,
    pub hubp_set_unbounded_requesting_params: hubp_set_unbounded_requesting_params,
    pub hubp_setup_interdependent2_params: hubp_setup_interdependent2_params,
    pub hubp_setup_interdependent_params: hubp_setup_interdependent_params,
    pub dpp_set_cursor_matrix_params: dpp_set_cursor_matrix_params,
    pub mpc_update_mpcc_params: mpc_update_mpcc_params,
    pub mpc_update_blending_params: mpc_update_blending_params,
    pub mpc_assert_idle_mpcc_params: mpc_assert_idle_mpcc_params,
    pub mpc_insert_plane_params: mpc_insert_plane_params,
    pub dpp_set_scaler_params: dpp_set_scaler_params,
    pub hubp_mem_program_viewport_params: hubp_mem_program_viewport_params,
    pub abort_cursor_offload_update_params: abort_cursor_offload_update_params,
    pub cursor_lock_params: cursor_lock_params,
    pub setup_periodic_interrupt_params: setup_periodic_interrupt_params,
    pub send_cursor_info_to_dmu_params: send_cursor_info_to_dmu_params,
    pub set_cursor_attribute_params: set_cursor_attribute_params,
    pub hubp_set_cursor_attributes_params: hubp_set_cursor_attributes_params,
    pub dpp_set_cursor_attributes_params: dpp_set_cursor_attributes_params,
    pub set_cursor_position_params: set_cursor_position_params,
    pub set_cursor_sdr_white_level_params: set_cursor_sdr_white_level_params,
    pub program_output_csc_params: program_output_csc_params,
    pub hubp_set_blank_params: hubp_set_blank_params,
    pub phantom_hubp_post_enable_params: phantom_hubp_post_enable_params,
    pub begin_cursor_offload_update_params: begin_cursor_offload_update_params,
    pub update_cursor_offload_pipe_params: update_cursor_offload_pipe_params,
    pub commit_cursor_offload_update_params: commit_cursor_offload_update_params,
    pub stream_enc_update_hdmi_info_packets_params: stream_enc_update_hdmi_info_packets_params,
    pub hpo_frl_stream_enc_update_hdmi_info_packets_params: hpo_frl_stream_enc_update_hdmi_info_packets_params,
    pub hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num_params: hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num_params,
    pub hpo_dp_stream_enc_update_dp_info_packets_params: hpo_dp_stream_enc_update_dp_info_packets_params,
    pub stream_enc_update_dp_info_packets_sdp_line_num_params: stream_enc_update_dp_info_packets_sdp_line_num_params,
    pub stream_enc_update_dp_info_packets_params: stream_enc_update_dp_info_packets_params,
    pub dsc_set_config_simple_params: dsc_set_config_simple_params,
    pub stream_enc_dp_set_dsc_config_params: stream_enc_dp_set_dsc_config_params,
    pub hpo_dp_stream_enc_dp_set_dsc_pps_info_packet_params: hpo_dp_stream_enc_dp_set_dsc_pps_info_packet_params,
    pub stream_enc_dp_set_dsc_pps_info_packet_params: stream_enc_dp_set_dsc_pps_info_packet_params,
    pub hpo_frl_stream_enc_set_dsc_config_params: hpo_frl_stream_enc_set_dsc_config_params,
    pub dp_trace_source_sequence_params: dp_trace_source_sequence_params,
    pub set_dmdata_attributes_params: set_dmdata_attributes_params,
    pub link_increase_mst_payload_params: link_increase_mst_payload_params,
    pub link_reduce_mst_payload_params: link_reduce_mst_payload_params,
    pub dp_set_test_pattern_params: dp_set_test_pattern_params,
    pub link_set_dpms_off_params: link_set_dpms_off_params,
    pub disable_audio_stream_params: disable_audio_stream_params,
    pub prepare_bandwidth_params: prepare_bandwidth_params,
    pub link_set_dpms_on_params: link_set_dpms_on_params,
    pub clk_mgr_set_max_memclk_params: clk_mgr_set_max_memclk_params,
    pub clk_mgr_update_clocks_params: clk_mgr_update_clocks_params,
    pub hubbub_program_watermarks_params: hubbub_program_watermarks_params,
    pub hubbub_program_arbiter_params: hubbub_program_arbiter_params,
    pub hubbub_program_compbuf_segments_params: hubbub_program_compbuf_segments_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum block_sequence_func {
    DMUB_SUBVP_PIPE_CONTROL_LOCK_FAST = 0,
    OPTC_PIPE_CONTROL_LOCK,
    HUBP_SET_FLIP_CONTROL_GSL,
    HUBP_PROGRAM_TRIPLEBUFFER,
    HUBP_UPDATE_PLANE_ADDR,
    DPP_SET_INPUT_TRANSFER_FUNC,
    DPP_PROGRAM_GAMUT_REMAP,
    HUBP_ENABLE_3DLUT_FL,
    OTG_SETUP_VERTICAL_INTERRUPT,
    HWSS_SETUP_PERIODIC_INTERRUPT,
    HWSS_UPDATE_INFO_FRAME,
    HUBP_SET_DMDATA_ATTRIBUTES,
    OPTC_PROGRAM_MANUAL_TRIGGER,
    DMUB_SEND_DMCUB_CMD,
    DPP_SETUP_DPP,
    DPP_PROGRAM_BIAS_AND_SCALE,
    DPP_SET_OUTPUT_TRANSFER_FUNC,
    DPP_SET_HDR_MULTIPLIER,
    DPP_PROGRAM_UPSP,
    MPC_UPDATE_VISUAL_CONFIRM,
    MPC_POWER_ON_MPC_MEM_PWR,
    MPC_SET_OUTPUT_CSC,
    MPC_SET_OCSC_DEFAULT,
    DMUB_SUBVP_SAVE_SURF_ADDR,
    HUBP_WAIT_FOR_DCC_META_PROP,
    DMUB_HW_CONTROL_LOCK_FAST,
    HUBP_PROGRAM_SURFACE_CONFIG,
    HUBP_PROGRAM_MCACHE_ID,
    DPP_PROGRAM_CM_HIST,
    PROGRAM_CURSOR_UPDATE_NOW,
    HUBP_WAIT_PIPE_READ_START,
    HWS_APPLY_UPDATE_FLAGS_FOR_PHANTOM,
    HWS_UPDATE_PHANTOM_VP_POSITION,
    OPTC_SET_ODM_COMBINE,
    OPTC_SET_ODM_BYPASS,
    OPP_PIPE_CLOCK_CONTROL,
    OPP_PROGRAM_LEFT_EDGE_EXTRA_PIXEL,
    DCCG_SET_DTO_DSCCLK,
    DSC_SET_CONFIG,
    DSC_ENABLE,
    TG_SET_DSC_CONFIG,
    DSC_DISCONNECT,
    DSC_READ_STATE,
    DSC_CALCULATE_AND_SET_CONFIG,
    DSC_ENABLE_WITH_OPP,
    TG_PROGRAM_GLOBAL_SYNC,
    TG_WAIT_FOR_STATE,
    TG_SET_VTG_PARAMS,
    TG_SETUP_VERTICAL_INTERRUPT2,
    HUBP_PROGRAM_DET_SIZE,
    HUBP_PROGRAM_DET_SEGMENTS,
    OPP_SET_DYN_EXPANSION,
    OPP_PROGRAM_FMT,
    OPP_PROGRAM_BIT_DEPTH_REDUCTION,
    OPP_SET_DISP_PATTERN_GENERATOR,
    ABM_SET_PIPE,
    ABM_SET_LEVEL,
    ABM_SET_IMMEDIATE_DISABLE,
    MPC_REMOVE_MPCC,
    OPP_SET_MPCC_DISCONNECT_PENDING,
    DC_SET_OPTIMIZED_REQUIRED,
    HUBP_DISCONNECT,
    HUBBUB_FORCE_PSTATE_CHANGE_CONTROL,
    TG_ENABLE_CRTC,
    TG_SET_GSL,
    TG_SET_GSL_SOURCE_SELECT,
    HUBP_WAIT_FLIP_PENDING,
    TG_WAIT_DOUBLE_BUFFER_PENDING,
    UPDATE_FORCE_PSTATE,
    PROGRAM_MALL_PIPE_CONFIG,
    HUBBUB_APPLY_DEDCN21_147_WA,
    HUBBUB_ALLOW_SELF_REFRESH_CONTROL,
    TG_GET_FRAME_COUNT,
    MPC_SET_DWB_MUX,
    MPC_DISABLE_DWB_MUX,
    MCIF_WB_CONFIG_BUF,
    MCIF_WB_CONFIG_ARB,
    MCIF_WB_ENABLE,
    MCIF_WB_DISABLE,
    DWBC_ENABLE,
    DWBC_DISABLE,
    DWBC_UPDATE,
    HUBP_UPDATE_MALL_SEL,
    HUBP_PREPARE_SUBVP_BUFFERING,
    HUBP_SET_BLANK_EN,
    HUBP_DISABLE_CONTROL,
    HUBBUB_SOFT_RESET,
    HUBP_CLK_CNTL,
    HUBP_INIT,
    HUBP_SET_VM_SYSTEM_APERTURE_SETTINGS,
    HUBP_SET_FLIP_INT,
    DPP_DPPCLK_CONTROL,
    DISABLE_PHANTOM_CRTC,
    DSC_PG_STATUS,
    DSC_WAIT_DISCONNECT_PENDING_CLEAR,
    DSC_DISABLE,
    DCCG_SET_REF_DSCCLK,
    DPP_PG_CONTROL,
    HUBP_PG_CONTROL,
    HUBP_RESET,
    DPP_RESET,
    DPP_ROOT_CLOCK_CONTROL,
    DC_IP_REQUEST_CNTL,
    DCCG_UPDATE_DPP_DTO,
    HUBP_VTG_SEL,
    HUBP_SETUP2,
    HUBP_SETUP,
    HUBP_SET_UNBOUNDED_REQUESTING,
    HUBP_SETUP_INTERDEPENDENT2,
    HUBP_SETUP_INTERDEPENDENT,
    DPP_SET_CURSOR_MATRIX,
    MPC_UPDATE_BLENDING,
    MPC_ASSERT_IDLE_MPCC,
    MPC_INSERT_PLANE,
    DPP_SET_SCALER,
    HUBP_MEM_PROGRAM_VIEWPORT,
    ABORT_CURSOR_OFFLOAD_UPDATE,
    HWSS_CURSOR_LOCK,
    HWSS_BEGIN_CURSOR_OFFLOAD_UPDATE,
    HWSS_COMMIT_CURSOR_OFFLOAD_UPDATE,
    HWSS_UPDATE_CURSOR_OFFLOAD_PIPE,
    DC_SEND_CURSOR_INFO_TO_DMU,
    SET_CURSOR_ATTRIBUTE,
    HUBP_SET_CURSOR_ATTRIBUTES,
    DPP_SET_CURSOR_ATTRIBUTES,
    SET_CURSOR_POSITION,
    SET_CURSOR_SDR_WHITE_LEVEL,
    PROGRAM_OUTPUT_CSC,
    HUBP_SET_LEGACY_TILING_COMPAT_LEVEL,
    HUBP_SET_BLANK,
    PHANTOM_HUBP_POST_ENABLE,
    STREAM_ENC_UPDATE_HDMI_INFO_PACKETS,
    HPO_FRL_STREAM_ENC_UPDATE_HDMI_INFO_PACKETS,
    HPO_DP_STREAM_ENC_UPDATE_DP_INFO_PACKETS_SDP_LINE_NUM,
    HPO_DP_STREAM_ENC_UPDATE_DP_INFO_PACKETS,
    STREAM_ENC_UPDATE_DP_INFO_PACKETS_SDP_LINE_NUM,
    STREAM_ENC_UPDATE_DP_INFO_PACKETS,
    DSC_SET_CONFIG_SIMPLE,
    STREAM_ENC_DP_SET_DSC_CONFIG,
    HPO_DP_STREAM_ENC_DP_SET_DSC_PPS_INFO_PACKET,
    STREAM_ENC_DP_SET_DSC_PPS_INFO_PACKET,
    HPO_FRL_STREAM_ENC_SET_DSC_CONFIG,
    LINK_INCREASE_MST_PAYLOAD,
    LINK_REDUCE_MST_PAYLOAD,
    DP_TRACE_SOURCE_SEQUENCE,
    DP_SET_TEST_PATTERN,
    LINK_SET_DPMS_OFF,
    DISABLE_AUDIO_STREAM,
    PREPARE_BANDWIDTH,
    LINK_SET_DPMS_ON,
    CLK_MGR_SET_MAX_MEMCLK,
    CLK_MGR_UPDATE_CLOCKS,
    HUBBUB_PROGRAM_WATERMARKS,
    HUBBUB_PROGRAM_ARBITER,
    HUBBUB_PROGRAM_COMPBUF_SEGMENTS,
    HUBBUB_PERFMON_RESET,
    HUBBUB_PERFMON_ARM_OUT_OF_ORDER_BW,
    HUBBUB_PERFMON_START_OUT_OF_ORDER_BW,
    HUBBUB_PERFMON_START_IN_ORDER_BW,
    HUBBUB_PERFMON_START_MEMORY_LATENCIES,
    HUBBUB_PERFMON_START_URGENT_ASSERTION_COUNT,
    HUBBUB_PERFMON_START_URGENT_RAMP_LATENCY,
    HUBBUB_PERFMON_START_PREFETCH_DATA_SIZE,
    HUBBUB_PERFMON_GET_OUT_OF_ORDER_BW,
    HUBBUB_PERFMON_GET_IN_ORDER_BW,
    HUBBUB_PERFMON_GET_MEMORY_LATENCIES,
    HUBBUB_PERFMON_GET_URGENT_ASSERTION_COUNT,
    HUBBUB_PERFMON_GET_PREFETCH_DATA_SIZE,
    HUBBUB_PERFMON_GET_URGENT_RAMP_LATENCY,
// This must be the last value in this enum, add new ones above
    HWSS_BLOCK_SEQUENCE_FUNC_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_sequence {
    pub params: block_sequence_params,
    pub func: block_sequence_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_sequence_state {
    pub steps: *mut block_sequence,
    pub num_steps: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_sequencer_funcs {
    pub dc): *mut *mut void (hardware_release)(struct dc,
// Embedded Display Related
    pub enable): *mut *mut *mut void (edp_power_control)(struct dc_link link, bool,
    pub power_up): *mut *mut *mut void (edp_wait_for_hpd_ready)(struct dc_link link, bool,
    pub link): *mut *mut void (edp_wait_for_T12)(struct dc_link,
// Pipe Programming Related
    pub dc): *mut *mut void (init_hw)(struct dc,
    pub dc): *mut *mut void (power_down_on_boot)(struct dc,
    pub context): *mut dc_state,
    pub context): *mut dc_state,
    pub pipe_ctx): *mut *mut *mut *mut void (disable_plane)(struct dc dc, struct dc_state state, struct pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub blank): *mut *mut *mut *mut void (disable_pixel_data)(struct dc dc, struct pipe_ctx pipe_ctx, bool,
    pub context): *mut int num_planes, struct dc_state,
    pub context): *mut dc_state,
    pub context): *mut dc_state,
    pub context): *mut dc_state,
    pub pipe_ctx): *mut pipe_ctx,
    pub dh_data): *mut dchub_init_data,
    pub pipe_ctx): *mut pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub enable): bool,
    pub enableTripleBuffer): *mut *mut pipe_ctx pipe_ctx, bool,
    pub pipe_ctx): *mut *mut void (update_pending_status)(struct pipe_ctx,
    pub safe_to_disable): *mut *mut *mut *mut void (update_dsc_pg)(struct dc dc, struct dc_state context, bool,
    pub clear_tiling): *mut *mut *mut *mut void (clear_surface_dcc_and_tiling)(struct pipe_ctx pipe_ctx, struct dc_plane_state plane_state, bool,
// Pipe Lock Related
    pub lock): *mut *mut pipe_ctx pipe, bool,
    pub lock): *mut *mut dc_state context, bool,
    pub flip_immediate): bool,
    pub lock): *mut *mut *mut *mut void (cursor_lock)(struct dc dc, struct pipe_ctx pipe, bool,
// Timing Related
    pub position): *mut crtc_position,
    pub pipe_ctx): *mut *mut int (get_vupdate_offset_from_vsync)(struct pipe_ctx,
    pub end_line): *mut u32,
    pub grouped_pipes[]): *mut int group_size, struct pipe_ctx,
    pub grouped_pipes[]): *mut pipe_ctx,
    pub grouped_pipes[]): *mut pipe_ctx,
    pub pipe_ctx): *mut pipe_ctx,
    pub adjust): dc_crtc_timing_adjust,
    pub events): *const dc_static_screen_params,
// Stream Related
    pub pipe_ctx): *mut *mut void (enable_stream)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (disable_stream)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (blank_stream)(struct pipe_ctx,
    pub link_settings): *mut dc_link_settings,
// Bandwidth Related
    pub context): *mut *mut *mut void (prepare_bandwidth)(struct dc dc, struct dc_state,
    pub seq_state): *mut block_sequence_state,
    pub context): *mut *mut *mut bool (update_bandwidth)(struct dc dc, struct dc_state,
    pub context): *mut *mut *mut void (optimize_bandwidth)(struct dc dc, struct dc_state,
    pub seq_state): *mut block_sequence_state,
// Infopacket Related
    pub enable): *mut *mut *mut void (set_avmute)(struct pipe_ctx pipe_ctx, bool,
    pub sdp_message_size): c_uint,
    pub pipe_ctx): *mut *mut void (update_info_frame)(struct pipe_ctx,
    pub pipe): *mut *mut void (set_dmdata_attributes)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (program_dmdata_engine)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut bool (dmdata_status_done)(struct pipe_ctx,
// Cursor Related
    pub pipe): *mut *mut void (set_cursor_position)(struct pipe_ctx,
    pub pipe): *mut *mut void (set_cursor_attribute)(struct pipe_ctx,
    pub pipe): *mut *mut void (set_cursor_sdr_white_level)(struct pipe_ctx,
    pub pipe): *const *const *const void (abort_cursor_offload_update)(struct dc dc, struct pipe_ctx,
    pub pipe): *const *const *const void (begin_cursor_offload_update)(struct dc dc, struct pipe_ctx,
    pub pipe): *const *const *const void (commit_cursor_offload_update)(struct dc dc, struct pipe_ctx,
    pub pipe): *const *const *const void (update_cursor_offload_pipe)(struct dc dc, struct pipe_ctx,
    pub stream): *const dc_stream_state,
    pub pipe): *const *const *const void (program_cursor_offload_now)(struct dc dc, struct pipe_ctx,
// Colour Related
    pub params): *mut *mut void (program_gamut_remap)(struct program_gamut_remap_params,
    pub opp_id): *mut *mut uint16_t matrix, int,
    pub pipe_ctx): *mut *mut void (trigger_3dlut_dma_load)(struct pipe_ctx,
// VM Related
    pub pa_config): *mut dc_phy_addr_space_config,
    pub vmid): c_int,
// Writeback Related
    pub context): *mut dc_state,
    pub context): *mut dc_state,
    pub dwb_pipe_inst): c_uint,
// Clock Related
    pub stepping): uint32_t clk_khz, uint32_t,
    pub clock_cfg): *mut dc_clock_config,
    pub context): *mut dc_state,
    pub context): *mut dc_state,
    pub stream): *const dc_stream_state,
// Audio Related
    pub pipe_ctx): *mut *mut void (enable_audio_stream)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (disable_audio_stream)(struct pipe_ctx,
// Stereo 3D Related
    pub dc): *mut *mut *mut void (setup_stereo)(struct pipe_ctx pipe_ctx, struct dc,
// HW State Logging Related
    pub log_ctx): *mut *mut *mut void (log_hw_state)(struct dc dc, struct dc_log_buffer_ctx,
    pub log_ctx): *mut dc_log_buffer_ctx,
    pub mask): unsigned int bufSize, unsigned int,
    pub mask): *mut *mut *mut void (clear_status_bits)(struct dc dc, unsigned int,
    pub params): *mut set_backlight_level_params,
    pub pipe_ctx): *mut *mut void (set_abm_immediate_disable)(struct pipe_ctx,
    pub pipe_ctx): *mut *mut void (set_pipe)(struct pipe_ctx,
    pub link_settings): *const dc_link_settings,
    pub pixel_clock): u32,
    pub pixel_clock): u32,
    pub pixel_clock): u32,
    pub signal): signal_type,
    pub link): *mut *mut bool (dac_load_detect)(struct dc_link,
    pub link): *mut *mut void (prepare_ddc)(struct dc_link,
    pub dcc_en_bits): *mut *mut *mut void (get_dcc_en_bits)(struct dc dc, int,
    pub frl_phy_clock_source_id): clock_source_id,
    pub context): *mut dc_state,
// Idle Optimization Related
    pub enable): *mut *mut *mut bool (apply_idle_power_optimizations)(struct dc dc, bool,
    pub cursor_attr): *mut dc_cursor_attributes,
    pub context): *mut *mut *mut void (commit_subvp_config)(struct dc dc, struct dc_state,
    pub context): *mut *mut *mut void (enable_phantom_streams)(struct dc dc, struct dc_state,
    pub context): *mut *mut *mut void (disable_phantom_streams)(struct dc dc, struct dc_state,
    pub subvp_prev_use): bool,
    pub params): *mut *mut void (subvp_pipe_control_lock_fast)(union block_sequence_params,
    pub dc): *const *const void (z10_restore)(struct dc,
    pub dc): *mut *mut void (z10_save_init)(struct dc,
    pub stream): *mut *mut dc_state context, dc_stream_state,
    pub offset): int width, int height, int,
    pub height): c_int,
    pub mpcc_id): c_int,
    pub phantom_pipe): *mut pipe_ctx,
    pub phantom_pipe): *mut *mut void (apply_update_flags_for_phantom)(struct pipe_ctx,
    pub update_state): *mut pg_block_update,
    pub update_state): *mut pg_block_update,
    pub update_state): *mut pg_block_update,
    pub update_state): *mut pg_block_update,
    pub power_on): *mut *mut pg_block_update update_state, bool,
    pub new_ctx): *const dc_state,
    pub top_pipe_to_program): *const pipe_ctx,
    pub lock): bool,
    pub enable): bool,
    pub params): *mut *mut void (dmub_hw_control_lock_fast)(union block_sequence_params,
    pub v_total_max): *mut *mut *mut *mut void (set_long_vtotal)(struct pipe_ctx pipe_ctx, int num_pipes, uint32_t v_total_min, uint32_t,
    pub context): *mut dc_state,
    pub enable): *const *const *const void (setup_hpo_hw_control)(struct dce_hwseq hws, bool,
    pub pipe_ctx): *const *const void (wait_for_all_pending_updates)(struct pipe_ctx,
    pub new_pipe): *mut pipe_ctx,
    pub context): *mut dc_state,
    pub seq_state): *mut block_sequence_state,
    pub context): *mut dc_state,
    pub seq_state): *mut block_sequence_state,
    pub opp_head): *mut pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub out_data): *mut dc_underflow_debug_data,
//
// program_perfmon - Program/transition perfmon probes for a commit.
// @dc:      DC structure
// @context: target state; probes, probe_count, and probe_status are
// read from and written to this object
//
// Invoked during the execute phase of dc_update_state. The hook resolves
// each probe's transition by diffing @context against dc->current_state
// and latches MEASURED results into @context->probe_status.
//
    pub context): *mut *mut *mut void (program_perfmon)(struct dc dc, struct dc_state,
}

extern "C" {
    pub fn hwss_send_dmcub_cmd(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_manual_trigger(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_setup_dpp(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_bias_and_scale(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_upsp(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_power_on_mpc_mem_pwr(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_output_csc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_ocsc_default(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_subvp_save_surf_addr(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_surface_config(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_mcache_id_and_split_coordinate(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_cm_hist(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_odm_combine(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_odm_bypass(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_pipe_clock_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_program_left_edge_extra_pixel(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_blank_pixel_data(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dccg_set_dto_dscclk(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_set_config(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_enable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_set_dsc_config(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_disconnect(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_read_state(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_calculate_and_set_config(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_enable_with_opp(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_set_config_simple(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_stream_enc_update_hdmi_info_packets(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hpo_frl_stream_enc_update_hdmi_info_packets(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hpo_dp_stream_enc_update_dp_info_packets_sdp_line_num(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hpo_dp_stream_enc_update_dp_info_packets(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_stream_enc_update_dp_info_packets_sdp_line_num(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_stream_enc_update_dp_info_packets(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_stream_enc_dp_set_dsc_config(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hpo_dp_stream_enc_dp_set_dsc_pps_info_packet(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_stream_enc_dp_set_dsc_pps_info_packet(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hpo_frl_stream_enc_set_dsc_config(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_dmdata_attributes(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dp_trace_source_sequence(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_link_increase_mst_payload(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_link_reduce_mst_payload(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dp_set_test_pattern(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_link_set_dpms_off(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_prepare_bandwidth(dc: *mut dc, params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_link_set_dpms_on(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_tg(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_program_global_sync(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_wait_for_state(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_set_vtg_params(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_enable_3dlut_fl(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_update_info_frame(dc: *mut dc, params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_setup_vertical_interrupt0(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_setup_vertical_interrupt2(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_set_hdr_multiplier(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_det_size(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_det_segments(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_set_dyn_expansion(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_program_fmt(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_program_bit_depth_reduction(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_set_disp_pattern_generator(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_abm_pipe(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_abm_level(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_abm_immediate_disable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_remove_mpcc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_opp_set_mpcc_disconnect_pending(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dc_set_optimized_required(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_disconnect(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_force_pstate_change_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_enable_crtc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_set_gsl(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_set_gsl_source_select(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_wait_flip_pending(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_wait_double_buffer_pending(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_update_force_pstate(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_apply_dedcn21_147_wa(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_allow_self_refresh_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_tg_get_frame_count(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_set_dwb_mux(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_disable_dwb_mux(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mcif_wb_config_buf(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mcif_wb_config_arb(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mcif_wb_enable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mcif_wb_disable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dwbc_enable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dwbc_disable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dwbc_update(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_update_mall_sel(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_prepare_subvp_buffering(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_blank_en(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_disable_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_soft_reset(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_reset(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_arm_out_of_order_bw(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_start_out_of_order_bw(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_start_in_order_bw(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_start_memory_latencies(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_start_urgent_assertion_count(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_start_urgent_ramp_latency(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_start_prefetch_data_size(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_get_out_of_order_bw(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_get_in_order_bw(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_get_memory_latencies(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_get_urgent_assertion_count(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_get_prefetch_data_size(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_perfmon_get_urgent_ramp_latency(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_clk_cntl(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_init(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_vm_system_aperture_settings(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_flip_int(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_dppclk_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_disable_phantom_crtc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_pg_status(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_wait_disconnect_pending_clear(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dsc_disable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dccg_set_ref_dscclk(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_pg_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_pg_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_reset(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_reset(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_root_clock_control(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dc_ip_request_cntl(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dccg_update_dpp_dto(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_vtg_sel(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_setup2(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_setup(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_unbounded_requesting(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_setup_interdependent2(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_setup_interdependent(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_set_cursor_matrix(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_update_mpcc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_update_blending(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_assert_idle_mpcc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_mpc_insert_plane(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_set_scaler(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_mem_program_viewport(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_abort_cursor_offload_update(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_send_cursor_info_to_dmu(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_cursor_attribute(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_cursor_attributes(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_dpp_set_cursor_attributes(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_cursor_position(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_cursor_sdr_white_level(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_program_gamut_remap(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn hwss_program_output_csc(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_legacy_tiling_compat_level(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubp_set_blank(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_phantom_hubp_post_enable(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_cursor_lock(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_begin_cursor_offload_update(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_commit_cursor_offload_update(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_update_cursor_offload_pipe(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_setup_periodic_interrupt(dc: *mut dc, params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_disable_audio_stream(dc: *mut dc, params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_set_output_transfer_func(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
}
// Clock manager BLS executor functions
extern "C" {
    pub fn hwss_clk_mgr_set_max_memclk(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_clk_mgr_update_clocks(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_program_watermarks(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_program_arbiter(params: *mut block_sequence_params);
}
extern "C" {
    pub fn hwss_hubbub_program_compbuf_segments(params: *mut block_sequence_params);
}
// Clock manager BLS add-helper functions
