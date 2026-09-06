//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml2_wrapper.h
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

pub const DML2_MAX_NUM_DPM_LVL: c_int = 30;
// Configuration of the MALL on the SoC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_soc_mall_info {
// Cache line size of 0 means MALL is not enabled/present
    pub cache_line_size_bytes: c_uint,
    pub cache_num_ways: c_uint,
    pub max_cab_allocation_bytes: c_uint,
    pub mblk_width_pixels: c_uint,
    pub mblk_size_bytes: c_uint,
    pub mblk_height_4bpe_pixels: c_uint,
    pub mblk_height_8bpe_pixels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_soc_alt_ch_info {
    pub region_size_bytes: [c_uint; 2],
// bits 47:16 of the base address
    pub region_base_addr_47_16: [c_uint; 2],
}

// Output of DML2 for clock requirements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dcn_clocks {
    pub dispclk_khz: c_uint,
    pub dcfclk_khz: c_uint,
    pub fclk_khz: c_uint,
    pub uclk_mts: c_uint,
    pub phyclk_khz: c_uint,
    pub socclk_khz: c_uint,
    pub ref_dtbclk_khz: c_uint,
    pub p_state_supported: bool,
    pub cab_num_ways_required: c_uint,
    pub dcfclk_khz_ds: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dc_callbacks {
    pub dc: *mut dc,
    pub pipe_ctx): *mut *mut bool (build_scaling_params)(struct pipe_ctx,
    pub otg_master): *mut *mut *mut void (build_test_pattern_params)(struct resource_context res_ctx, struct pipe_ctx,
    pub context): *mut *mut *mut bool (can_support_mclk_switch_using_fw_based_vblank_stretch)(struct dc dc, struct dc_state,
    pub odm): *const *const *const *const *const *const bool (acquire_secondary_pipe_for_mpc_odm)(struct dc dc, struct dc_state state, struct pipe_ctx pri_pipe, struct pipe_ctx sec_pipe, bool,
    pub new_slice_count): c_int,
    pub slice_count): c_int,
    pub opp_head): *const *const int (get_odm_slice_index)(struct pipe_ctx,
    pub opp_head): *const *const int (get_odm_slice_count)(struct pipe_ctx,
    pub dpp_pipe): *const *const int (get_mpc_slice_index)(struct pipe_ctx,
    pub dpp_pipe): *const *const int (get_mpc_slice_count)(struct pipe_ctx,
    pub pipe_ctx): *const *const *const pipe_ctx (get_opp_head)(pipe_ctx,
    pub stream): *const dc_stream_state,
    pub opp_heads[MAX_PIPES]): *mut pipe_ctx,
    pub dpp_pipes[MAX_PIPES]): *mut pipe_ctx,
    pub stream): *const dc_stream_state,
    pub id): *const *const *const *const dc_stream_state (get_stream_from_id)(dc_state state, unsigned int,
    pub is_gaming): bool,
    pub mcache_params): *const *const *const bool (allocate_mcache)(struct dc_state context, struct dc_mcache_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dc_svp_callbacks {
    pub dc: *mut dc,
    pub pipe_ctx): *mut *mut bool (build_scaling_params)(struct pipe_ctx,
    pub main_stream): *mut dc_stream_state,
    pub main_plane): *mut dc_plane_state,
    pub main_stream): *mut dc_stream_state,
    pub context): *const *const *const *const *const bool (add_phantom_plane)(struct dc dc, struct dc_stream_state stream, struct dc_plane_state plane_state, struct dc_state,
    pub context): *mut dc_state,
    pub stream): *mut dc_stream_state,
    pub plane): *mut dc_plane_state,
    pub stream): *mut dc_stream_state,
    pub dsc): *const *const *const *const void (release_dsc)(struct resource_context res_ctx, struct resource_pool pool, struct display_stream_compressor,
    pub pipe_ctx): *const *const *const mall_stream_type (get_pipe_subvp_type)(struct dc_state state, struct pipe_ctx,
    pub stream): *const *const *const mall_stream_type (get_stream_subvp_type)(struct dc_state state, struct dc_stream_state,
    pub stream): *const *const *const *const dc_stream_state (get_paired_subvp_stream)(dc_state state, dc_stream_state,
    pub state): *mut dc_state,
    pub state): *mut dc_state,
    pub total_size_in_mall_bytes): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_clks_table_entry {
    pub dcfclk_mhz: c_uint,
    pub fclk_mhz: c_uint,
    pub memclk_mhz: c_uint,
    pub socclk_mhz: c_uint,
    pub dtbclk_mhz: c_uint,
    pub dispclk_mhz: c_uint,
    pub dppclk_mhz: c_uint,
    pub wck_ratio*/: *mut *mut unsigned int dram_speed_mts; /which is based on,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_clks_num_entries {
    pub num_dcfclk_levels: c_uint,
    pub num_fclk_levels: c_uint,
    pub num_memclk_levels: c_uint,
    pub num_socclk_levels: c_uint,
    pub num_dtbclk_levels: c_uint,
    pub num_dispclk_levels: c_uint,
    pub num_dppclk_levels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_clks_limit_table {
    pub clk_entries: [dml2_clks_table_entry; DML2_MAX_NUM_DPM_LVL],
    pub num_entries_per_clk: dml2_clks_num_entries,
    pub num_states: c_uint,
}

// Various overrides, per ASIC or per SKU specific, or for debugging purpose when/if available
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_soc_bbox_overrides {
    pub xtalclk_mhz: double,
    pub dchub_refclk_mhz: double,
    pub dprefclk_mhz: double,
    pub disp_pll_vco_speed_mhz: double,
    pub urgent_latency_us: double,
    pub sr_exit_latency_us: double,
    pub sr_enter_plus_exit_latency_us: double,
    pub sr_exit_z8_time_us: double,
    pub sr_enter_plus_exit_z8_time_us: double,
    pub dram_clock_change_latency_us: double,
    pub fclk_change_latency_us: double,
    pub dram_num_chan: c_uint,
    pub dram_chanel_width_bytes: c_uint,
    pub clks_table: dml2_clks_limit_table,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_force_pstate_methods {
    dml2_force_pstate_method_auto = 0,
    dml2_force_pstate_method_vactive,
    dml2_force_pstate_method_vblank,
    dml2_force_pstate_method_drr,
    dml2_force_pstate_method_subvp,
    dml2_force_pstate_method_alternate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_configuration_options {
    pub dcn_pipe_count: c_int,
    pub use_native_pstate_optimization: bool,
    pub enable_windowed_mpo_odm: bool,
    pub use_native_soc_bb_construction: bool,
    pub skip_hw_state_mapping: bool,
    pub optimize_odm_4to1: bool,
    pub minimize_dispclk_using_odm: bool,
    pub override_det_buffer_size_kbytes: bool,
    pub callbacks: dml2_dc_callbacks,
    pub force_disable_subvp: bool,
    pub force_enable_subvp: bool,
    pub subvp_fw_processing_delay_us: c_uint,
    pub subvp_pstate_allow_width_us: c_uint,
    pub subvp_prefetch_end_to_mall_start_us: c_uint,
    pub subvp_swath_height_margin_lines: c_uint,
    pub callbacks: dml2_dc_svp_callbacks,
    pub svp_pstate: },
    pub mall_cfg: dml2_soc_mall_info,
    pub alt_ch_cfg: dml2_soc_alt_ch_info,
    pub bbox_overrides: dml2_soc_bbox_overrides,
    pub max_segments_per_hubp: c_uint,
    pub det_segment_size: c_uint,
// Only for debugging purposes when initializing SOCBB params via tool for DML21.
    pub external_socbb_ip_params: *mut socbb_ip_params_external,
    pub force_mandatory_uclk_pstate_support: bool,
    pub force_pstate_method_enable: bool,
    pub force_pstate_method_values: [dml2_force_pstate_methods; MAX_PIPES],
    pub pmo: },
    pub map_dc_pipes_with_callbacks: bool,
    pub use_clock_dc_limits: bool,
    pub gpuvm_enable: bool,
    pub hostvm_enable: bool,
    pub force_tdlut_enable: bool,
    pub bb_from_dmub: *mut c_void,
}

//
// dml2_create - Creates dml2_context.
// @in_dc: dc.
// @config: dml2 configuration options.
// @dml2: Created dml2 context.
//
// Create and destroy of DML2 is done as part of dc_state creation
// and dc_state_free. DML2 IP, SOC and STATES are initialized at
// creation time.
//
// Return: True if dml2 is successfully created, false otherwise.
//
extern "C" {
    pub fn dml2_destroy(dml2: *mut dml2_context);
}
//
// dml2_validate - Determines if a display configuration is supported or not.
// @in_dc: dc.
// @context: dc_state to be validated.
// @validate_mode: DC_VALIDATE_MODE_ONLY and DC_VALIDATE_MODE_AND_STATE_INDEX will not populate context.res_ctx.
//
// DML1.0 compatible interface for validation.
//
// Based on fast_validate option internally would call:
//
// -dml2_validate_and_build_resource - for non fast_validate option
// Calculates if dc_state can be supported on the SOC, and attempts to
// optimize the power management feature supports versus minimum clocks.
// If supported, also builds out_new_hw_state to represent the hw programming
// for the new dc state.
//
// -dml2_validate_only - for fast_validate option
// Calculates if dc_state can be supported on the SOC (i.e. at maximum
// clocks) with all mandatory power features enabled.
// Context: Two threads may not invoke this function concurrently unless they reference
// separate dc_states for validation.
// Return: True if mode is supported, false otherwise.
//
// dml2_extract_dram_and_fclk_change_support - Extracts the FCLK and UCLK change support info.
// @dml2: input dml2 context pointer.
// @fclk_change_support: output pointer holding the fclk change support info (vactive, vblank, unsupported).
// @dram_clk_change_support: output pointer holding the uclk change support info (vactive, vblank, unsupported).
//
extern "C" {
    pub fn dml2_prepare_mcache_programming(in_dc: *mut dc, context: *mut dc_state, dml2: *mut dml2_context);
}
extern "C" {
    pub fn dml2_apply_debug_options(dc: *const dc, dml2: *mut dml2_context);
}
extern "C" {
    pub fn dml2_validate_only(context: *mut dc_state, validate_mode: dc_validate_mode) -> bool;
}
