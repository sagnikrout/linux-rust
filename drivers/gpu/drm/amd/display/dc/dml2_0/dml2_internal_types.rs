//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml2_internal_types.h
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
#[derive(Copy, Clone)]
pub struct dml2_wrapper_optimize_configuration_params {
    pub dml_core_ctx: *mut display_mode_lib_st,
    pub config: *mut dml2_configuration_options,
    pub ip_params: *mut ip_params_st,
    pub cur_display_config: *mut dml_display_cfg_st,
    pub new_display_config: *mut dml_display_cfg_st,
    pub cur_mode_support_info: *const dml_mode_support_info_st,
    pub cur_policy: *mut dml_mode_eval_policy_st,
    pub new_policy: *mut dml_mode_eval_policy_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_calculate_lowest_supported_state_for_temp_read_scratch {
    pub evaluation_info: dml_mode_support_info_st,
    pub uclk_change_latencies: [dml_float_t; __DML_MAX_STATE_ARRAY_SIZE__],
    pub cur_display_config: dml_display_cfg_st,
    pub new_display_config: dml_display_cfg_st,
    pub new_policy: dml_mode_eval_policy_st,
    pub cur_policy: dml_mode_eval_policy_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_create_scratch {
    pub build_synthetic_socbb_scratch: dml2_policy_build_synthetic_soc_states_scratch,
    pub in_states: soc_states_st,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_calculate_rq_and_dlg_params_scratch {
    pub rq_regs: _vcs_dpi_dml_display_rq_regs_st,
    pub disp_dlg_regs: _vcs_dpi_dml_display_dlg_regs_st,
    pub disp_ttu_regs: _vcs_dpi_dml_display_ttu_regs_st,
}

pub const __DML2_WRAPPER_MAX_STREAMS_PLANES__: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_dml_to_dc_pipe_mapping {
    pub disp_cfg_to_stream_id: [c_uint; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub disp_cfg_to_stream_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub disp_cfg_to_plane_id: [c_uint; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub disp_cfg_to_plane_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_stream_id: [c_uint; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_stream_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_id: [c_uint; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_id_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_index: [c_uint; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
    pub dml_pipe_idx_to_plane_index_valid: [bool; __DML2_WRAPPER_MAX_STREAMS_PLANES__],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_wrapper_scratch {
    pub cur_display_config: dml_display_cfg_st,
    pub new_display_config: dml_display_cfg_st,
    pub new_policy: dml_mode_eval_policy_st,
    pub cur_policy: dml_mode_eval_policy_st,
    pub mode_support_info: dml_mode_support_info_st,
    pub mode_support_params: dml_mode_support_ex_params_st,
    pub dummy_pstate_table: [dummy_pstate_entry; 4],
    pub create_scratch: dml2_create_scratch,
    pub dml2_calculate_lowest_supported_state_for_temp_read_scratch: dml2_calculate_lowest_supported_state_for_temp_read_scratch,
    pub calculate_rq_and_dlg_params_scratch: dml2_calculate_rq_and_dlg_params_scratch,
    pub optimize_configuration_params: dml2_wrapper_optimize_configuration_params,
    pub build_synthetic_socbb_params: dml2_policy_build_synthetic_soc_states_params,
    pub dml_to_dc_pipe_mapping: dml2_dml_to_dc_pipe_mapping,
    pub enable_flexible_pipe_mapping: bool,
    pub plane_duplicate_exists: bool,
    pub hpo_stream_to_link_encoder_mapping: [c_int; MAX_HPO_DP2_ENCODERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_helper_det_policy_scratch {
    pub dpps_per_surface: [c_int; MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml2_architecture {
    dml2_architecture_20,
    dml2_architecture_21
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prepare_mcache_programming_locals {
    pub build_mcache_programming_params: dml2_build_mcache_programming_in_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml21_wrapper_scratch {
    pub prepare_mcache_locals: prepare_mcache_programming_locals,
    pub temp_pipe: pipe_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pipe_combine_factor {
    pub source: c_uint,
    pub target: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_pipe_combine_scratch {
    pub odm_factors: [dml2_pipe_combine_factor; MAX_PIPES],
    pub mpc_factors: [dml2_pipe_combine_factor; MAX_PIPES][MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_context {
    pub architecture: dml2_architecture,
    pub config: dml2_configuration_options,
    pub det_helper_scratch: dml2_helper_det_policy_scratch,
    pub pipe_combine_scratch: dml2_pipe_combine_scratch,
    pub dml_core_ctx: display_mode_lib_st,
    pub scratch: dml2_wrapper_scratch,
    pub g6_temp_read_watermark_set: dcn_watermarks,
    pub v20: },
    pub scratch: dml21_wrapper_scratch,
    pub dml_init: dml2_initialize_instance_in_out,
    pub display_config: dml2_display_cfg,
    pub mode_support: dml2_check_mode_supported_in_out,
    pub mode_programming: dml2_build_mode_programming_in_out,
    pub dml_to_dc_pipe_mapping: dml2_dml_to_dc_pipe_mapping,
    pub v21: },
}
