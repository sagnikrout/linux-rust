//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml2_utils.h
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

extern "C" {
    pub fn dml2_util_copy_dml_timing(dml_timing_array: *mut dml_timing_cfg_st, dst_index: c_uint, src_index: c_uint);
}
extern "C" {
    pub fn dml2_util_copy_dml_plane(dml_plane_array: *mut dml_plane_cfg_st, dst_index: c_uint, src_index: c_uint);
}
extern "C" {
    pub fn dml2_util_copy_dml_surface(dml_surface_array: *mut dml_surface_cfg_st, dst_index: c_uint, src_index: c_uint);
}
extern "C" {
    pub fn dml2_util_copy_dml_output(dml_output_array: *mut dml_output_cfg_st, dst_index: c_uint, src_index: c_uint);
}
extern "C" {
    pub fn dml2_util_get_maximum_odm_combine_for_output(force_odm_4to1: bool, encoder: dml_output_encoder_class, dsc_enabled: bool) -> c_uint;
}
extern "C" {
    pub fn dml2_copy_clocks_to_dc_state(out_clks: *mut dml2_dcn_clocks, context: *mut dc_state);
}
extern "C" {
    pub fn dml2_extract_watermark_set(watermark: *mut dcn_watermarks, dml_core_ctx: *mut display_mode_lib_st);
}
extern "C" {
    pub fn dml2_extract_writeback_wm(context: *mut dc_state, dml_core_ctx: *mut display_mode_lib_st);
}
extern "C" {
    pub fn dml2_helper_find_dml_pipe_idx_by_stream_id(ctx: *mut dml2_context, stream_id: c_uint) -> c_int;
}
extern "C" {
    pub fn is_dtbclk_required(dc: *const dc, context: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dml2_is_stereo_timing(stream: *const dc_stream_state) -> bool;
}
//
// dml2_dc_construct_pipes - This function will determine if we need additional pipes based
// on the DML calculated outputs for MPC, ODM and allocate them as necessary. This function
// could be called after in dml_validate_build_resource after dml_mode_pragramming like :
// {
// ...
// map_hw_resources(&s->cur_display_config, &s->mode_support_info);
// result = dml_mode_programming(&in_ctx->dml_core_ctx, s->mode_support_params.out_lowest_state_idx, &s->cur_display_config, true);
// dml2_dc_construct_pipes(in_display_state, s->mode_support_info, out_hw_context);
// ...
// }
//
// @context: To obtain res_ctx and read other information like stream ID etc.
// @dml_mode_support_st : To get the ODM, MPC outputs as determined by the DML.
// @out_hw_context : Handle to the new hardware context.
//
// Return: None.
//
// dml2_predict_pipe_split - This function is the dml2 version of predict split pipe. It predicts a
// if pipe split is required or not and returns the output as a bool.
// @context : dc_state.
// @pipe : old_index is the index of the pipe as derived from pipe_idx.
// @index : index of the pipe
//
// Return: Returns the result in boolean.
//
extern "C" {
    pub fn dml2_predict_pipe_split(context: *mut dc_state, pipe: display_pipe_params_st, index: c_int) -> bool;
}
//
// dml2_build_mapped_resource - This function is the dml2 version of build_mapped_resource.
// In case of ODM, we need to build pipe hardware params again as done in dcn20_build_mapped_resource.
// @dc : struct dc
// @context : struct dc_state.
// @stream : stream whoose corresponding pipe params need to be modified.
//
// Return: Returns DC_OK if successful.
//
extern "C" {
    pub fn dml2_build_mapped_resource(dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
}
//
// dml2_extract_rq_regs - This function will extract information needed for struct _vcs_dpi_display_rq_regs_st
// and populate it.
// @context: To obtain and populate the res_ctx->pipe_ctx->rq_regs with DML outputs.
// @support : This structure has the DML intermediate outputs required to populate rq_regs.
//
// Return: None.
//
// dml2_calculate_rq_and_dlg_params - This function will call into DML2 functions needed
// for populating rq, ttu and dlg param structures and populate it.
// @dc : struct dc
// @context : dc_state provides a handle to selectively populate pipe_ctx
// @out_new_hw_state: To obtain and populate the rq, dlg and ttu regs in
// out_new_hw_state->pipe_ctx with DML outputs.
// @in_ctx : This structure has the pointer to display_mode_lib_st.
// @pipe_cnt : DML functions to obtain RQ, TTu and DLG params need a pipe_index.
// This helps provide pipe_index in the pipe_cnt loop.
//
// Return: None.
//
extern "C" {
    pub fn dml2_calculate_rq_and_dlg_params(dc: *const dc, context: *mut dc_state, out_new_hw_state: *mut resource_context, in_ctx: *mut dml2_context, pipe_cnt: c_uint);
}
//
// dml2_apply_det_buffer_allocation_policy - This function will determine the DET Buffer size
// and return the number of streams.
// @dml2 : Handle for dml2 context
// @dml_dispcfg : dml_dispcfg is the DML2 struct representing the current display config
// Return : None.
//
extern "C" {
    pub fn dml2_apply_det_buffer_allocation_policy(in_ctx: *mut dml2_context, dml_dispcfg: *mut dml_display_cfg_st);
}
//
// dml2_verify_det_buffer_configuration - This function will verify if the allocated DET buffer exceeds
// the total available DET size available and outputs a boolean to indicate if recalulation is needed.
// @dml2 : Handle for dml2 context
// @dml_dispcfg : dml_dispcfg is the DML2 struct representing the current display config
// @struct dml2_helper_det_policy_scratch : Pointer to DET helper scratch
// Return : returns true if recalculation is required, false otherwise.
//
extern "C" {
    pub fn dml2_verify_det_buffer_configuration(in_ctx: *mut dml2_context, display_state: *mut dc_state, det_scratch: *mut dml2_helper_det_policy_scratch) -> bool;
}
//
// dml2_initialize_det_scratch - This function will initialize the DET scratch space as per requirements.
// @dml2 : Handle for dml2 context
// Return : None
//
extern "C" {
    pub fn dml2_initialize_det_scratch(in_ctx: *mut dml2_context);
}
