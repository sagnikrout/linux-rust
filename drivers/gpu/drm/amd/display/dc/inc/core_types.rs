//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/core_types.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

pub const MAX_CLOCK_SOURCES: c_int = 7;
pub const MAX_SVP_PHANTOM_STREAMS: c_int = 2;
pub const MAX_SVP_PHANTOM_PLANES: c_int = 2;

// DAL Core

pub const MAX_RMCM_INST: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_funcs {
    pub dpia_index): *mut *mut engine_id (get_preferred_eng_id_dpia)(unsigned int,
    pub pool): *mut *mut void (destroy)(struct resource_pool,
    pub link): *mut *mut void (link_init)(struct dc_link,
    pub panel_cntl_init_data): *const panel_cntl_init_data,
    pub init): *const encoder_init_data,
// Create a minimal link encoder object with no dc_link object
// associated with it.
    pub eng_id): *mut *mut *mut *mut link_encoder (link_enc_create_minimal)(dc_context ctx, enum engine_id,
    pub ctx): *mut dc_context,
    pub validate_mode): dc_validate_mode,
    pub vlevel): c_int,
    pub context): *mut *mut dc dc, dc_state,
    pub total_size_in_mall_bytes): c_uint,
    pub context): *mut dc_state,
//
// @populate_dml_pipes - Populate pipe data struct
//
// Returns:
// Total of pipes available in the specific ASIC.
//
    pub validate_mode): dc_validate_mode,
//
// Algorithm for assigning available link encoders to links.
//
// Update link_enc_assignments table and link_enc_avail list accordingly in
// struct resource_context.
//
    pub stream_count): u8,
//
// Unassign a link encoder from a stream.
//
// Update link_enc_assignments table and link_enc_avail list accordingly in
// struct resource_context.
//
    pub stream): *mut dc_stream_state,
    pub context): *mut dc_state,
    pub opp_head_pipe): *const pipe_ctx,
    pub otg_master): *const pipe_ctx,
    pub pool): *const resource_pool,
    pub caps): *mut dc_caps,
    pub dc_stream): *mut dc_stream_state,
    pub stream): *mut dc_stream_state,
    pub plane_state): *mut dc_plane_state,
    pub stream): *mut dc_stream_state,
    pub pipes): *mut display_e2e_pipe_params_st,
    pub pipe_cnt): c_int,
    pub bw_params): *mut clk_bw_params,
    pub shaper): *mut dc_transfer_func,
    pub shaper): *mut dc_transfer_func,
    pub stream): *mut dc_stream_state,
    pub index): c_uint,
    pub panel_config): *mut *mut void (get_panel_config_defaults)(struct dc_panel_config,
    pub tiling_info): *mut *mut void (get_default_tiling_info)(struct dc_tiling_info,
    pub pipe_ctx): *mut *mut void (build_pipe_pix_clk_params)(struct pipe_ctx,
//
// Get indicator of power from a context that went through full validation
//
    pub context): *const *const int (get_power_profile)(struct dc_state,
    pub context): *const *const unsigned int (get_det_buffer_size)(struct dc_state,
    pub pipe_ctx): *mut *mut unsigned int (get_vstartup_for_pipe)(struct pipe_ctx,
    pub stream): *const dc_stream_state,
    pub mcache_params): *const dc_mcache_params,
    pub audio_output): *mut audio_output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_support {
    pub dp_audio: bool,
    pub hdmi_audio_on_dongle: bool,
    pub hdmi_audio_native: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_pool {
    pub mis: [*mut mem_input; MAX_PIPES],
    pub hubps: [*mut hubp; MAX_PIPES],
    pub ipps: [*mut input_pixel_processor; MAX_PIPES],
    pub transforms: [*mut transform; MAX_PIPES],
    pub dpps: [*mut dpp; MAX_PIPES],
    pub opps: [*mut output_pixel_processor; MAX_PIPES],
    pub timing_generators: [*mut timing_generator; MAX_PIPES],
    pub 2]: *mut *mut *mut stream_encoder stream_enc[MAX_PIPES,
    pub hubbub: *mut hubbub,
    pub dio: *mut dio,
    pub mpc: *mut mpc,
    pub pp_smu: *mut pp_smu_funcs,
    pub engines: [*mut dce_aux; MAX_PIPES],
    pub hw_i2cs: [*mut dce_i2c_hw; MAX_PIPES],
    pub sw_i2cs: [*mut dce_i2c_sw; MAX_PIPES],
    pub i2c_hw_buffer_in_use: bool,
    pub dwbc: [*mut dwbc; MAX_DWB_PIPES],
    pub mcif_wb: [*mut mcif_wb; MAX_DWB_PIPES],
    pub gsl_0:1: c_uint,
    pub gsl_1:1: c_uint,
    pub gsl_2:1: c_uint,
    pub gsl_groups: },
    pub dscs: [*mut display_stream_compressor; MAX_PIPES],
    pub pipe_count: c_uint,
    pub underlay_pipe_index: c_uint,
    pub stream_enc_count: c_uint,
// An array for accessing the link encoder objects that have been created.
// Index in array corresponds to engine ID - viz. 0: ENGINE_ID_DIGA
//
    pub link_encoders: [*mut link_encoder; MAX_LINK_ENCODERS],
// Number of DIG link encoder objects created - i.e. number of valid
// entries in link_encoders array.
//
    pub dig_link_enc_count: c_uint,
// Number of USB4 DPIA (DisplayPort Input Adapter) link objects created.
    pub usb4_dpia_count: c_uint,
    pub hpo_frl_stream_enc_count: c_uint,
    pub hpo_frl_stream_enc: [*mut hpo_frl_stream_encoder; MAX_HDMI_FRL_ENCODERS],
    pub hpo_frl_link_enc_count: c_uint,
    pub hpo_frl_link_enc: [*mut hpo_frl_link_encoder; MAX_HDMI_FRL_ENCODERS],
    pub hpo_dp_stream_enc_count: c_uint,
    pub hpo_dp_stream_enc: [*mut hpo_dp_stream_encoder; MAX_HPO_DP2_ENCODERS],
    pub hpo_dp_link_enc_count: c_uint,
    pub hpo_dp_link_enc: [*mut hpo_dp_link_encoder; MAX_HPO_DP2_LINK_ENCODERS],
    pub mpc_lut: [*mut dc_3dlut; MAX_PIPES],
    pub mpc_shaper: [*mut dc_transfer_func; MAX_PIPES],
    pub rmcm_3dlut: [dc_rmcm_3dlut; MAX_RMCM_INST],
    pub xtalin_clock_inKhz: c_uint,
    pub dccg_ref_clock_inKhz: c_uint,
    pub dchub_ref_clock_inKhz: c_uint,
    pub ref_clocks: },
    pub timing_generator_count: c_uint,
    pub mpcc_count: c_uint,
    pub writeback_pipe_count: c_uint,
//
// reserved clock source for DP
//
    pub dp_clock_source: *mut clock_source,
    pub clock_sources: [*mut clock_source; MAX_CLOCK_SOURCES],
    pub clk_src_count: c_uint,
    pub audios: [*mut audio; MAX_AUDIOS],
    pub audio_count: c_uint,
    pub audio_support: audio_support,
    pub dccg: *mut dccg,
    pub pg_cntl: *mut pg_cntl,
    pub irqs: *mut irq_service,
    pub abm: *mut abm,
    pub dmcu: *mut dmcu,
    pub psr: *mut dmub_psr,
    pub replay: *mut dmub_replay,
    pub multiple_abms: [*mut abm; MAX_PIPES],
    pub funcs: *const resource_funcs,
    pub res_cap: *const resource_caps,
    pub oem_device: *mut ddc_service,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_fe_bandwidth {
    pub dppclk_khz: c_int,
}

// Parameters needed to call set_disp_pattern_generator
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_pattern_params {
    pub test_pattern: controller_dp_test_pattern,
    pub color_space: controller_dp_color_space,
    pub color_depth: dc_color_depth,
    pub width: c_int,
    pub height: c_int,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_resource {
    pub opp: *mut output_pixel_processor,
    pub dsc: *mut display_stream_compressor,
    pub tg: *mut timing_generator,
    pub stream_enc: *mut stream_encoder,
    pub hpo_frl_stream_enc: *mut hpo_frl_stream_encoder,
    pub hpo_dp_stream_enc: *mut hpo_dp_stream_encoder,
    pub audio: *mut audio,
    pub pix_clk_params: pixel_clk_params,
    pub encoder_info_frame: encoder_info_frame,
    pub abm: *mut abm,
// There are only (num_pipes+1)/2 groups. 0 means unassigned,
// otherwise it's using group number 'gsl_group-1'
//
    pub gsl_group: u8,
    pub test_pattern_params: test_pattern_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plane_resource {
// scl_data is scratch space required to program a plane
    pub scl_data: scaler_data,
// Below pointers to hw objects are required to enable the plane
// spl_in and spl_out are the input and output structures for SPL
// which are required when using Scaler Programming Library
// these are scratch spaces needed when programming a plane
//
    pub spl_in: spl_in,
    pub spl_out: spl_out,
// Below pointers to hw objects are required to enable the plane
    pub hubp: *mut hubp,
    pub mi: *mut mem_input,
    pub ipp: *mut input_pixel_processor,
    pub xfm: *mut transform,
    pub dpp: *mut dpp,
    pub mpcc_inst: u8,
    pub bw: dcn_fe_bandwidth,
}

pub const LINK_RES_HPO_DP_REC_MAP__MASK: c_uint = 0xFFFF;
pub const LINK_RES_HPO_DP_REC_MAP__SHIFT: c_int = 0;
// all mappable hardware resources used to enable a link
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_resource {
    pub dio_link_enc: *mut link_encoder,
    pub hpo_dp_link_enc: *mut hpo_dp_link_encoder,
    pub hpo_frl_link_enc: *mut hpo_frl_link_encoder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_config {
    pub dp_link_settings: dc_link_settings,
    pub dp_tunnel_settings: dc_tunnel_settings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe_update_flags {
    pub 1: uint32_t enable :,
    pub 1: uint32_t disable :,
    pub 1: uint32_t odm :,
    pub 1: uint32_t global_sync :,
    pub 1: uint32_t opp_changed :,
    pub 1: uint32_t tg_changed :,
    pub 1: uint32_t mpcc :,
    pub 1: uint32_t dppclk :,
    pub 1: uint32_t hubp_interdependent :,
    pub 1: uint32_t hubp_rq_dlg_ttu :,
    pub 1: uint32_t gamut_remap :,
    pub 1: uint32_t scaler :,
    pub 1: uint32_t viewport :,
    pub 1: uint32_t plane_changed :,
    pub 1: uint32_t det_size :,
    pub 1: uint32_t unbounded_req :,
    pub 1: uint32_t test_pattern_changed :,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_rate_divider {
    pub div_factor1: u32,
    pub div_factor2: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p_state_switch_method {
    P_STATE_UNKNOWN						= 0,
    P_STATE_V_BLANK						= 1,
    P_STATE_FPO,
    P_STATE_V_ACTIVE,
    P_STATE_SUB_VP,
    P_STATE_DRR_SUB_VP,
    P_STATE_V_BLANK_SUB_VP,
    P_STATE_ALT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_padding_params {
// pixels borrowed from hblank to hactive
    pub dsc_hactive_padding: u8,
    pub dsc_htotal_padding: u32,
    pub dsc_pix_clk_100hz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_ctx {
    pub plane_state: *mut dc_plane_state,
    pub stream: *mut dc_stream_state,
    pub plane_res: plane_resource,
//
// @stream_res: Reference to DCN resource components such OPP and DSC.
//
    pub stream_res: stream_resource,
    pub link_res: link_resource,
    pub clock_source: *mut clock_source,
    pub pll_settings: pll_settings,
//
// @link_config:
//
// link config records software decision for what link config should be
// enabled given current link capability and stream during hw resource
// mapping. This is to decouple the dependency on link capability during
// dc commit or update.
//
    pub link_config: link_config,
    pub pipe_idx: u8,
    pub pipe_idx_syncd: u8,
    pub top_pipe: *mut pipe_ctx,
    pub bottom_pipe: *mut pipe_ctx,
    pub next_odm_pipe: *mut pipe_ctx,
    pub prev_odm_pipe: *mut pipe_ctx,
    pub dlg_regs: _vcs_dpi_display_dlg_regs_st,
    pub ttu_regs: _vcs_dpi_display_ttu_regs_st,
    pub rq_regs: _vcs_dpi_display_rq_regs_st,
    pub pipe_dlg_param: _vcs_dpi_display_pipe_dest_params_st,
    pub dml_rq_param: _vcs_dpi_display_rq_params_st,
    pub dml_dlg_sys_param: _vcs_dpi_display_dlg_sys_params_st,
    pub dml_input: _vcs_dpi_display_e2e_pipe_params_st,
    pub det_buffer_size_kb: c_int,
    pub unbounded_req: bool,
    pub surface_size_in_mall_bytes: c_uint,
    pub hubp_regs: dml2_dchub_per_pipe_register_set,
    pub mcache_regs: dml2_hubp_pipe_mcache_regs,
    pub global_sync: dml2_global_sync_programming,
    pub dwbc: *mut dwbc,
    pub mcif_wb: *mut mcif_wb,
    pub update_flags: pipe_update_flags,
    pub p_state_type: p_state_switch_method,
    pub visual_confirm_color: tg_color,
    pub has_vactive_margin: bool,
// subvp_index: only valid if the pipe is a SUBVP_MAIN
    pub subvp_index: u8,
    pub pixel_rate_divider: pixel_rate_divider,
    pub dsc_padding_params: dsc_padding_params,
// next vupdate
    pub next_vupdate: u32,
    pub wait_frame_count: u32,
    pub wait_is_required: bool,
}

// Data used for dynamic link encoder assignment.
// Tracks current and future assignments; available link encoders;
// and mode of operation (whether to use current or future assignments).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_enc_cfg_context {
    pub mode: link_enc_cfg_mode,
    pub link_enc_assignments: [link_enc_assignment; MAX_PIPES],
    pub link_enc_avail: [engine_id; MAX_LINK_ENCODERS],
    pub transient_assignments: [link_enc_assignment; MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_context {
    pub pipe_ctx: [pipe_ctx; MAX_PIPES],
    pub 2]: *mut *mut bool is_stream_enc_acquired[MAX_PIPES,
    pub is_audio_acquired: [bool; MAX_PIPES],
    pub clock_source_ref_count: [u8; MAX_CLOCK_SOURCES],
    pub dp_clock_source_ref_count: u8,
    pub is_dsc_acquired: [bool; MAX_PIPES],
    pub link_enc_cfg_ctx: link_enc_cfg_context,
    pub dio_link_enc_to_link_idx: [c_uint; MAX_LINK_ENCODERS],
    pub dio_link_enc_ref_cnts: [c_int; MAX_LINK_ENCODERS],
    pub is_hpo_frl_stream_enc_acquired: [bool; MAX_HDMI_FRL_ENCODERS],
    pub hpo_frl_link_enc_to_link_idx: [c_uint; MAX_HDMI_FRL_ENCODERS],
    pub hpo_frl_link_enc_ref_cnts: [c_int; MAX_HDMI_FRL_ENCODERS],
    pub is_hpo_dp_stream_enc_acquired: [bool; MAX_HPO_DP2_ENCODERS],
    pub hpo_dp_link_enc_to_link_idx: [c_uint; MAX_HPO_DP2_LINK_ENCODERS],
    pub hpo_dp_link_enc_ref_cnts: [c_int; MAX_HPO_DP2_LINK_ENCODERS],
    pub is_mpc_3dlut_acquired: [bool; MAX_PIPES],
// used to build scalar data in dml2 and for edp backlight programming
    pub temp_pipe: pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_bw_output {
    pub cpuc_state_change_enable: bool,
    pub cpup_state_change_enable: bool,
    pub stutter_mode_enable: bool,
    pub nbp_state_change_enable: bool,
    pub all_displays_in_sync: bool,
    pub urgent_wm_ns: [dce_watermarks; MAX_PIPES],
    pub stutter_exit_wm_ns: [dce_watermarks; MAX_PIPES],
    pub stutter_entry_wm_ns: [dce_watermarks; MAX_PIPES],
    pub nbp_state_change_wm_ns: [dce_watermarks; MAX_PIPES],
    pub sclk_khz: c_int,
    pub sclk_deep_sleep_khz: c_int,
    pub yclk_khz: c_int,
    pub dispclk_khz: c_int,
    pub blackout_recovery_time_us: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_bw_writeback {
    pub mcif_wb_arb: [mcif_arb_params; MAX_DWB_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_bw_output {
    pub clk: dc_clocks,
    pub watermarks: dcn_watermark_set,
    pub bw_writeback: dcn_bw_writeback,
    pub compbuf_size_kb: c_int,
    pub mall_ss_size_bytes: c_uint,
    pub mall_ss_psr_active_size_bytes: c_uint,
    pub mall_subvp_size_bytes: c_uint,
    pub legacy_svp_drr_stream_index: c_uint,
    pub legacy_svp_drr_stream_index_valid: bool,
    pub mcache_allocations: [dml2_mcache_surface_allocation; DML2_MAX_PLANES],
    pub fams2_global_config: dmub_cmd_fams2_global_config,
    pub fams2_stream_base_params: [dmub_cmd_fams2_config; DML2_MAX_PLANES],
    pub fams2_stream_sub_params: [dmub_cmd_fams2_config; DML2_MAX_PLANES],
    pub fams2_stream_sub_params_v2: [dmub_fams2_stream_static_sub_state_v2; DML2_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bw_output {
    pub dcn: dcn_bw_output,
    pub dce: dce_bw_output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_context {
    pub bw: bw_output,
    pub dml: display_mode_lib,
    pub dml2: *mut dml2_context,
    pub dml2_dc_power_source: *mut dml2_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dmub_cmd {
    pub dmub_cmd: dmub_rb_cmd,
    pub wait_type: dm_dmub_wait_type,
}

//
// struct dc_state - The full description of a state requested by users
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_state {
//
// @streams: Stream state properties
//
    pub streams: [*mut dc_stream_state; MAX_PIPES],
//
// @stream_status: Planes status on a given stream
//
    pub stream_status: [dc_stream_status; MAX_PIPES],
//
// @phantom_streams: Stream state properties for phantoms
//
    pub phantom_streams: [*mut dc_stream_state; MAX_PHANTOM_PIPES],
//
// @phantom_planes: Planes state properties for phantoms
//
    pub phantom_planes: [*mut dc_plane_state; MAX_PHANTOM_PIPES],
//
// @stream_count: Total of streams in use
//
    pub stream_count: u8,
    pub stream_mask: u8,
//
// @stream_count: Total phantom streams in use
//
    pub phantom_stream_count: u8,
//
// @stream_count: Total phantom planes in use
//
    pub phantom_plane_count: u8,
//
// @probes: Committed absolute set of probe descriptors.
//
    pub probes: [dc_probe_state; MAX_PROBES],
//
// @probe_status: Committed absolute set of probe results.
//
    pub probe_status: [dc_probe_status; MAX_PROBES],
//
// @probe_count: Number of valid entries in @probes.
//
    pub probe_count: c_int,
//
// @res_ctx: Persistent state of resources
//
    pub res_ctx: resource_context,
//
// @pp_display_cfg: PowerPlay clocks and settings
// Note: this is a big struct, do *not* put on stack!
//
    pub pp_display_cfg: dm_pp_display_configuration,
//
// @dcn_bw_vars: non-stack memory to support bandwidth calculations
// Note: this is a big struct, do *not* put on stack!
//
    pub dcn_bw_vars: dcn_bw_internal_vars,
    pub clk_mgr: *mut clk_mgr,
//
// @bw_ctx: The output from bandwidth and watermark calculations and the DML
//
// Each context must have its own instance of VBA, and in order to
// initialize and obtain IP and SOC, the base DML instance from DC is
// initially copied into every context.
//
    pub bw_ctx: bw_context,
    pub block_sequence: [block_sequence; MAX_HWSS_BLOCK_SEQUENCE_SIZE],
    pub block_sequence_steps: c_uint,
    pub dc_dmub_cmd: [dc_dmub_cmd; 10],
    pub dmub_cmd_count: c_uint,
//
// @refcount: refcount reference
//
// Notice that dc_state is used around the code to capture the current
// context, so we need to pass it everywhere. That's why we want to use
// kref in this struct.
//
    pub refcount: kref,
    pub stutter_period_us: c_uint,
    pub perf_params: },
    pub power_source: dc_power_source_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct replay_context {
// ddc line
    pub aux_inst: channel_id,
// Transmitter id
    pub digbe_inst: transmitter,
// Engine Id is used for Dig Be source select
    pub digfe_inst: engine_id,
// Controller Id used for Dig Fe source select
    pub controllerId: controller_id,
    pub line_time_in_ns: c_uint,
    pub os_request_force_ffu: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_replay_enable {
    DC_REPLAY_DISABLE			= 0,
    DC_REPLAY_ENABLE			= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_bounding_box_max_clk {
    pub max_dcfclk_mhz: c_int,
    pub max_dispclk_mhz: c_int,
    pub max_dppclk_mhz: c_int,
    pub max_phyclk_mhz: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_measured_memory_qos {
    pub peak_bw_mbps: u32,
    pub avg_bw_mbps: u32,
    pub max_latency_ns: u32,
    pub min_latency_ns: u32,
    pub avg_latency_ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_requested_memory_qos {
    pub bandwidth_lb_in_mbps: u32,
    pub calculated_avg_bw_in_mbps: u32,
    pub max_latency_ub_in_ns: u32,
    pub avg_latency_ub_in_ns: u32,
    pub max_bw_budget_in_mbps: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum update_v3_flow {
    UPDATE_V3_FLOW_INVALID,
    UPDATE_V3_FLOW_NO_NEW_CONTEXT_CONTEXT_FAST,
    UPDATE_V3_FLOW_NO_NEW_CONTEXT_CONTEXT_FULL,
    UPDATE_V3_FLOW_NEW_CONTEXT_SEAMLESS,
    UPDATE_V3_FLOW_NEW_CONTEXT_MINIMAL_NEW,
    UPDATE_V3_FLOW_NEW_CONTEXT_MINIMAL_CURRENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_split_policy_backup {
    pub dynamic_odm_policy: bool,
    pub subvp_policy: bool,
    pub mpc_policy: pipe_split_policy,
    pub force_odm: [c_char; MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_update_scratch_space {
    pub dc: *mut dc,
    pub surface_updates: *mut dc_surface_update,
    pub surface_count: c_int,
    pub stream: *mut dc_stream_state,
    pub stream_update: *mut dc_stream_update,
    pub probe_updates: *const dc_probe_updates,
    pub update_v3: bool,
    pub do_clear_update_bits: bool,
    pub update_type: dc_update_type,
    pub new_context: *mut dc_state,
    pub flow: update_v3_flow,
    pub backup_context: *mut dc_state,
    pub intermediate_context: *mut dc_state,
    pub intermediate_policy: pipe_split_policy_backup,
    pub intermediate_updates: [dc_surface_update; MAX_SURFACES],
    pub intermediate_count: c_int,
}
