//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/mpc.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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
// DOC: overview
//
// Multiple Pipe/Plane Combiner (MPC) is a component in the hardware pipeline
// that performs blending of multiple planes, using global and per-pixel alpha.
// It also performs post-blending color correction operations according to the
// hardware capabilities, such as color transformation matrix and gamma 1D and
// 3D LUT.
//
// MPC receives output from all DPP pipes and combines them to multiple outputs
// supporting "M MPC inputs -> N MPC outputs" flexible composition
// architecture. It features:
//
// - Programmable blending structure to allow software controlled blending and
// cascading;
// - Programmable window location of each DPP in active region of display;
// - Combining multiple DPP pipes in one active region when a single DPP pipe
// cannot process very large surface;
// - Combining multiple DPP from different SLS with blending;
// - Stereo formats from single DPP in top-bottom or side-by-side modes;
// - Stereo formats from 2 DPPs;
// - Alpha blending of multiple layers from different DPP pipes;
// - Programmable background color;
//

pub const MAX_MPCC: c_int = 6;
pub const MAX_OPP: c_int = 6;
pub const MAX_DWB: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpc_output_csc_mode {
    MPC_OUTPUT_CSC_DISABLE = 0,
    MPC_OUTPUT_CSC_COEF_A,
    MPC_OUTPUT_CSC_COEF_B
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcc_blend_mode {
    MPCC_BLEND_MODE_BYPASS,
    MPCC_BLEND_MODE_TOP_LAYER_PASSTHROUGH,
    MPCC_BLEND_MODE_TOP_LAYER_ONLY,
    MPCC_BLEND_MODE_TOP_BOT_BLENDING
}

//
// enum mpcc_alpha_blend_mode - define the alpha blend mode regarding pixel
// alpha and plane alpha values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcc_alpha_blend_mode {
//
// @MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA: per pixel alpha using DPP
// alpha value
//
    MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA,
//
// @MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN: per
// pixel alpha using DPP alpha value multiplied by a global gain (plane
// alpha)
//
    MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN,
//
// @MPCC_ALPHA_BLEND_MODE_GLOBAL_ALPHA: global alpha value, ignores
// pixel alpha and consider only plane alpha
//
    MPCC_ALPHA_BLEND_MODE_GLOBAL_ALPHA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcc_movable_cm_location {
    MPCC_MOVABLE_CM_LOCATION_BEFORE,
    MPCC_MOVABLE_CM_LOCATION_AFTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCM_LUT_ID {
    MCM_LUT_3DLUT,
    MCM_LUT_1DLUT,
    MCM_LUT_SHAPER
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_fl_3dlut_config {
    pub enabled: bool,
    pub size: dc_cm_lut_size,
    pub select_lut_bank_a: bool,
    pub bit_depth: u16,
    pub hubp_index: c_int,
    pub bias: u16,
    pub scale: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mcm_lut_params {
    pub pwl: *const pwl_params,
    pub lut3d: *const tetrahedral_params,
}

//
// struct mpcc_blnd_cfg - MPCC blending configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpcc_blnd_cfg {
//
// @black_color: background color.
//
    pub black_color: tg_color,
//
// @alpha_mode: alpha blend mode (MPCC_ALPHA_BLND_MODE).
//
    pub alpha_mode: mpcc_alpha_blend_mode,
//
// @pre_multiplied_alpha:
// Whether pixel color values were pre-multiplied by the alpha channel
// (MPCC_ALPHA_MULTIPLIED_MODE).
//
    pub pre_multiplied_alpha: bool,
//
// @global_gain: Used when blend mode considers both pixel alpha and plane.
//
    pub global_gain: c_int,
//
// @global_alpha: Plane alpha value.
//
    pub global_alpha: c_int,
//
// @overlap_only: Whether overlapping of different planes is allowed.
//
    pub overlap_only: bool,
// MPCC top/bottom gain settings
//
// @bottom_gain_mode: Blend mode for bottom gain setting.
//
    pub bottom_gain_mode: c_int,
//
// @background_color_bpc: Background color for bpc.
//
    pub background_color_bpc: c_int,
//
// @top_gain: Top gain setting.
//
    pub top_gain: c_int,
//
// @bottom_inside_gain: Blend mode for bottom inside.
//
    pub bottom_inside_gain: c_int,
//
// @bottom_outside_gain: Blend mode for bottom outside.
//
    pub bottom_outside_gain: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_grph_gamut_adjustment {
    pub temperature_matrix: [fixed31_32; CSC_TEMPERATURE_MATRIX_SIZE],
    pub gamut_adjust_type: graphics_gamut_adjust_type,
    pub mpcc_gamut_remap_block_id: mpcc_gamut_remap_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_rmcm_regs {
    pub rmcm_3dlut_mem_pwr_state: u32,
    pub rmcm_3dlut_mem_pwr_force: u32,
    pub rmcm_3dlut_mem_pwr_dis: u32,
    pub rmcm_3dlut_mem_pwr_mode: u32,
    pub rmcm_3dlut_size: u32,
    pub rmcm_3dlut_mode: u32,
    pub rmcm_3dlut_mode_cur: u32,
    pub rmcm_3dlut_read_sel: u32,
    pub rmcm_3dlut_30bit_en: u32,
    pub rmcm_3dlut_wr_en_mask: u32,
    pub rmcm_3dlut_ram_sel: u32,
    pub rmcm_3dlut_out_norm_factor: u32,
    pub rmcm_3dlut_fl_sel: u32,
    pub rmcm_3dlut_out_offset_r: u32,
    pub rmcm_3dlut_out_scale_r: u32,
    pub rmcm_3dlut_fl_done: u32,
    pub rmcm_3dlut_fl_soft_underflow: u32,
    pub rmcm_3dlut_fl_hard_underflow: u32,
    pub rmcm_cntl: u32,
    pub rmcm_shaper_mem_pwr_state: u32,
    pub rmcm_shaper_mem_pwr_force: u32,
    pub rmcm_shaper_mem_pwr_dis: u32,
    pub rmcm_shaper_mem_pwr_mode: u32,
    pub rmcm_shaper_lut_mode: u32,
    pub rmcm_shaper_mode_cur: u32,
    pub rmcm_shaper_lut_write_en_mask: u32,
    pub rmcm_shaper_lut_write_sel: u32,
    pub rmcm_shaper_offset_b: u32,
    pub rmcm_shaper_scale_b: u32,
    pub rmcm_shaper_rama_exp_region_start_b: u32,
    pub rmcm_shaper_rama_exp_region_start_seg_b: u32,
    pub rmcm_shaper_rama_exp_region_end_b: u32,
    pub rmcm_shaper_rama_exp_region_end_base_b: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpcc_sm_cfg {
    pub enable: bool,
// 0-single plane,2-row subsampling,4-column subsampling,6-checkboard subsampling
    pub sm_mode: c_int,
// 0- disable frame alternate, 1- enable frame alternate
    pub frame_alt: bool,
// 0- disable field alternate, 1- enable field alternate
    pub field_alt: bool,
// 0-no force,2-force frame polarity from top,3-force frame polarity from bottom
    pub force_next_frame_porlarity: c_int,
// 0-no force,2-force field polarity from top,3-force field polarity from bottom
    pub force_next_field_polarity: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_denorm_clamp {
    pub clamp_max_r_cr: c_int,
    pub clamp_min_r_cr: c_int,
    pub clamp_max_g_y: c_int,
    pub clamp_min_g_y: c_int,
    pub clamp_max_b_cb: c_int,
    pub clamp_min_b_cb: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_dwb_flow_control {
    pub flow_ctrl_mode: c_int,
    pub flow_ctrl_cnt0: c_int,
    pub flow_ctrl_cnt1: c_int,
}

//
// struct mpcc - MPCC connection and blending configuration for a single MPCC instance.
//
// This struct is used as a node in an MPC tree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpcc {
//
// @mpcc_id: MPCC physical instance.
//
    pub mpcc_id: c_int,
//
// @dpp_id: DPP input to this MPCC
//
    pub dpp_id: c_int,
//
// @mpcc_bot: Pointer to bottom layer MPCC. NULL when not connected.
//
    pub mpcc_bot: *mut mpcc,
//
// @blnd_cfg: The blending configuration for this MPCC.
//
    pub blnd_cfg: mpcc_blnd_cfg,
//
// @sm_cfg: stereo mix setting for this MPCC
//
    pub sm_cfg: mpcc_sm_cfg,
//
// @shared_bottom:
//
// If MPCC output to both OPP and DWB endpoints, true. Otherwise, false.
//
    pub shared_bottom: bool,
}

//
// struct mpc_tree - MPC tree represents all MPCC connections for a pipe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_tree {
//
// @opp_id: The OPP instance that owns this MPC tree.
//
    pub opp_id: c_int,
//
// @opp_list: the top MPCC layer of the MPC tree that outputs to OPP endpoint
//
    pub opp_list: *mut mpcc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc {
    pub funcs: *const mpc_funcs,
    pub ctx: *mut dc_context,
    pub mpcc_array: [mpcc; MAX_MPCC],
    pub blender_params: pwl_params,
    pub cm_bypass_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpcc_state {
    pub opp_id: u32,
    pub dpp_id: u32,
    pub bot_mpcc_id: u32,
    pub mode: u32,
    pub alpha_mode: u32,
    pub pre_multiplied_alpha: u32,
    pub overlap_only: u32,
    pub idle: u32,
    pub busy: u32,
    pub shaper_lut_mode: u32,
    pub lut3d_mode: u32,
    pub lut3d_bit_depth: u32,
    pub lut3d_size: u32,
    pub rgam_mode: u32,
    pub rgam_lut: u32,
    pub gamut_remap: mpc_grph_gamut_adjustment,
    pub rmcm_regs: mpc_rmcm_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_mpc_reg_state {
    pub mpcc_bot_sel: u32,
    pub mpcc_control: u32,
    pub mpcc_status: u32,
    pub mpcc_top_sel: u32,
    pub mpcc_opp_id: u32,
    pub mpcc_ogam_control: u32,
}

//
// struct mpc_funcs - funcs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_funcs {
//
// @read_mpcc_state:
//
// Read register content from given MPCC physical instance.
//
// Parameters:
//
// - [in/out] mpc - MPC context
// - [in] mpcc_instance - MPC context instance
// - [in] mpcc_state - MPC context state
//
// Return:
//
// void
//
    pub s): *mut mpcc_state,
//
// @mpc_read_reg_state:
//
// Read MPC register state for debugging underflow purposes.
//
// Parameters:
//
// - [in] mpc - MPC context
// - [out] reg_state - MPC register state structure
//
// Return:
//
// void
//
    pub mpc_reg_state): *mut dcn_mpc_reg_state,
//
// @insert_plane:
//
// Insert DPP into MPC tree based on specified blending position.
// Only used for planes that are part of blending chain for OPP output
//
// Parameters:
//
// - [in/out] mpc  - MPC context.
// - [in/out] tree - MPC tree structure that plane will be added to.
// - [in] blnd_cfg - MPCC blending configuration for the new blending layer.
// - [in] sm_cfg   - MPCC stereo mix configuration for the new blending layer.
// stereo mix must disable for the very bottom layer of the tree config.
// - [in] insert_above_mpcc - Insert new plane above this MPCC.
// If NULL, insert as bottom plane.
// - [in] dpp_id  - DPP instance for the plane to be added.
// - [in] mpcc_id - The MPCC physical instance to use for blending.
//
// Return:
//
// struct mpcc* - MPCC that was added.
//
    pub mpcc_id): c_int,
//
// @remove_mpcc:
//
// Remove a specified MPCC from the MPC tree.
//
// Parameters:
//
// - [in/out] mpc   - MPC context.
// - [in/out] tree  - MPC tree structure that plane will be removed from.
// - [in/out] mpcc  - MPCC to be removed from tree.
//
// Return:
//
// void
//
    pub mpcc): *mut mpcc,
//
// @mpc_init:
//
// Reset the MPCC HW status by disconnecting all muxes.
//
// Parameters:
//
// - [in/out] mpc - MPC context.
//
// Return:
//
// void
//
    pub mpc): *mut *mut void (mpc_init)(struct mpc,
//
// @mpc_init_single_inst:
//
// Initialize given MPCC physical instance.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] mpcc_id - The MPCC physical instance to be initialized.
//
    pub mpcc_id): c_uint,
//
// @update_blending:
//
// Update the blending configuration for a specified MPCC.
//
// Parameters:
//
// - [in/out] mpc - MPC context.
// - [in] blnd_cfg - MPCC blending configuration.
// - [in] mpcc_id  - The MPCC physical instance.
//
// Return:
//
// void
//
    pub mpcc_id): c_int,
//
// @cursor_lock:
//
// Lock cursor updates for the specified OPP. OPP defines the set of
// MPCC that are locked together for cursor.
//
// Parameters:
//
// - [in] mpc - MPC context.
// - [in] opp_id  - The OPP to lock cursor updates on
// - [in] lock - lock/unlock the OPP
//
// Return:
//
// void
//
    pub lock): bool,
//
// @insert_plane_to_secondary:
//
// Add DPP into secondary MPC tree based on specified blending
// position.  Only used for planes that are part of blending chain for
// DWB output
//
// Parameters:
//
// - [in/out] mpc  - MPC context.
// - [in/out] tree - MPC tree structure that plane will be added to.
// - [in] blnd_cfg - MPCC blending configuration for the new blending layer.
// - [in] sm_cfg   - MPCC stereo mix configuration for the new blending layer.
// stereo mix must disable for the very bottom layer of the tree config.
// - [in] insert_above_mpcc - Insert new plane above this MPCC.  If
// NULL, insert as bottom plane.
// - [in] dpp_id - DPP instance for the plane to be added.
// - [in] mpcc_id - The MPCC physical instance to use for blending.
//
// Return:
//
// struct mpcc* - MPCC that was added.
//
    pub mpcc_id): c_int,
//
// @remove_mpcc_from_secondary:
//
// Remove a specified DPP from the 'secondary' MPC tree.
//
// Parameters:
//
// - [in/out] mpc  - MPC context.
// - [in/out] tree - MPC tree structure that plane will be removed from.
// - [in]     mpcc - MPCC to be removed from tree.
//
// Return:
//
// void
//
    pub mpcc): *mut mpcc,
//
// @get_mpcc_for_dpp_from_secondary:
//
// Find, if it exists, a MPCC from a given 'secondary' MPC tree that
// is associated with specified plane.
//
// Parameters:
// - [in/out] tree - MPC tree structure to search for plane.
// - [in] dpp_id - DPP to be searched.
//
// Return:
//
// struct mpcc* - pointer to plane or NULL if no plane found.
//
    pub dpp_id): c_int,
//
// @get_mpcc_for_dpp:
//
// Find, if it exists, a MPCC from a given MPC tree that
// is associated with specified plane.
//
// Parameters:
// - [in/out] tree - MPC tree structure to search for plane.
// - [in] dpp_id - DPP to be searched.
//
// Return:
//
// struct mpcc* - pointer to plane or NULL if no plane found.
//
    pub dpp_id): c_int,
//
// @wait_for_idle:
//
// Wait for a MPCC in MPC context to enter idle state.
//
// Parameters:
// - [in/out] mpc - MPC Context.
// - [in] id - MPCC to wait for idle state.
//
// Return:
//
// void
//
    pub id): *mut *mut *mut void (wait_for_idle)(struct mpc mpc, int,
//
// @assert_mpcc_idle_before_connect:
//
// Assert if MPCC in MPC context is in idle state.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] id - MPCC to assert idle state.
//
// Return:
//
// void
//
    pub mpcc_id): *mut *mut *mut void (assert_mpcc_idle_before_connect)(struct mpc mpc, int,
//
// @init_mpcc_list_from_hw:
//
// Iterate through the MPCC array from a given MPC context struct
// and configure each MPCC according to its registers' values.
//
// Parameters:
// - [in/out] mpc - MPC context to initialize MPCC array.
// - [in/out] tree - MPC tree structure containing MPCC contexts to initialize.
//
// Return:
//
// void
//
    pub tree): *mut mpc_tree,
//
// @set_denorm:
//
// Set corresponding OPP DENORM_CONTROL register value to specific denorm_mode
// based on given color depth.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] opp_id - Corresponding OPP to update register.
// - [in] output_depth - Arbitrary color depth to set denorm_mode.
//
// Return:
//
// void
//
    pub output_depth): dc_color_depth,
//
// @set_denorm_clamp:
//
// Set denorm clamp values on corresponding OPP DENORM CONTROL register.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] opp_id - Corresponding OPP to update register.
// - [in] denorm_clamp - Arbitrary denorm clamp to be set.
//
// Return:
//
// void
//
    pub denorm_clamp): mpc_denorm_clamp,
//
// @set_output_csc:
//
// Set the Output Color Space Conversion matrix
// with given values and mode.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] opp_id - Corresponding OPP to update register.
// - [in] regval - Values to set in CSC matrix.
// - [in] ocsc_mode - Mode to set CSC.
//
// Return:
//
// void
//
    pub ocsc_mode): mpc_output_csc_mode,
//
// @set_ocsc_default:
//
// Set the Output Color Space Conversion matrix
// to default values according to color space.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] opp_id - Corresponding OPP to update register.
// - [in] color_space - OCSC color space.
// - [in] ocsc_mode - Mode to set CSC.
//
// Return:
//
// void
//
    pub ocsc_mode): mpc_output_csc_mode,
//
// @set_output_gamma:
//
// Set Output Gamma with given curve parameters.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] mpcc_id - Corresponding MPC to update registers.
// - [in] params - Parameters.
//
// Return:
//
// void
//
    pub params): *const pwl_params,
//
// @power_on_mpc_mem_pwr:
//
// Power on/off memory LUT for given MPCC.
// Powering on enables LUT to be updated.
// Powering off allows entering low power mode.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] mpcc_id - MPCC to power on.
// - [in] power_on
//
// Return:
//
// void
//
    pub power_on): bool,
//
// @set_dwb_mux:
//
// Set corresponding Display Writeback mux
// MPC register field to given MPCC id.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] dwb_id - DWB to be set.
// - [in] mpcc_id - MPCC id to be stored in DWB mux register.
//
// Return:
//
// void
//
    pub mpcc_id): c_int,
//
// @disable_dwb_mux:
//
// Reset corresponding Display Writeback mux
// MPC register field.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] dwb_id - DWB to be set.
//
// Return:
//
// void
//
    pub dwb_id): c_int,
//
// @is_dwb_idle:
//
// Check DWB status on MPC_DWB0_MUX_STATUS register field.
// Return if it is null.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] dwb_id - DWB to be checked.
//
// Return:
//
// bool - wheter DWB is idle or not
//
    pub dwb_id): c_int,
//
// @set_out_rate_control:
//
// Set display output rate control.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] opp_id - OPP to be set.
// - [in] enable
// - [in] rate_2x_mode
// - [in] flow_control
//
// Return:
//
// void
//
    pub flow_control): *mut mpc_dwb_flow_control,
//
// @set_gamut_remap:
//
// Set post-blending CTM for given MPCC.
//
// Parameters:
// - [in] mpc - MPC context.
// - [in] mpcc_id - MPCC to set gamut map.
// - [in] adjust
//
// Return:
//
// void
//
    pub adjust): *const mpc_grph_gamut_adjustment,
//
// @program_1dlut:
//
// Set 1 dimensional Lookup Table.
//
// Parameters:
// - [in/out] mpc - MPC context
// - [in] params - curve parameters for the LUT configuration
// - [in] rmu_idx
//
// bool - wheter LUT was set (set with given parameters) or not (params is NULL and LUT is disabled).
//
    pub rmu_idx): u32,
//
// @program_shaper:
//
// Set shaper.
//
// Parameters:
// - [in/out] mpc - MPC context
// - [in] params - curve parameters to be set
// - [in] rmu_idx
//
// Return:
//
// bool - wheter shaper was set (set with given parameters) or not (params is NULL and LUT is disabled).
//
    pub rmu_idx): u32,
//
// @acquire_rmu:
//
// Set given MPCC to be multiplexed to given RMU unit.
//
// Parameters:
// - [in/out] mpc - MPC context
// - [in] mpcc_id - MPCC
// - [in] rmu_idx - Given RMU unit to set MPCC to be multiplexed to.
//
// Return:
//
// unit32_t - rmu_idx if operation was successful, -1 else.
//
    pub rmu_idx): *mut *mut *mut uint32_t (acquire_rmu)(struct mpc mpc, int mpcc_id, int,
//
// @program_3dlut:
//
// Set 3 dimensional Lookup Table.
//
// Parameters:
// - [in/out] mpc - MPC context
// - [in] params - tetrahedral parameters for the LUT configuration
// - [in] rmu_idx
//
// bool - wheter LUT was set (set with given parameters) or not (params is NULL and LUT is disabled).
//
    pub rmu_idx): c_int,
//
// @release_rmu:
//
// For a given MPCC, release the RMU unit it muliplexes to.
//
// Parameters:
// - [in/out] mpc - MPC context
// - [in] mpcc_id - MPCC
//
// Return:
//
// int - a valid rmu_idx representing released RMU unit or -1 if there was no RMU unit to release.
//
    pub mpcc_id): *mut *mut *mut int (release_rmu)(struct mpc mpc, int,
//
// @get_mpc_out_mux:
//
// Return MPC out mux.
//
// Parameters:
// - [in] mpc - MPC context.
// - [in] opp_id - OPP
//
// Return:
//
// unsigned int - Out Mux
//
    pub opp_id): c_int,
//
// @set_bg_color:
//
// Find corresponding bottommost MPCC and
// set its bg color.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] bg_color - background color to be set.
// - [in] mpcc_id
//
// Return:
//
// void
//
    pub mpcc_id): c_int,
//
// @set_mpc_mem_lp_mode:
//
// Set mpc_mem_lp_mode.
//
// Parameters:
// - [in/out] mpc - MPC context.
//
// Return:
//
// void
//
    pub mpc): *mut *mut void (set_mpc_mem_lp_mode)(struct mpc,
//
// @set_movable_cm_location:
//
// Set Movable CM Location.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] location
// - [in] mpcc_id
//
// Return:
//
// void
//
    pub mpcc_id): *mut *mut *mut void (set_movable_cm_location)(struct mpc mpc, enum mpcc_movable_cm_location location, int,
//
// @update_3dlut_fast_load_select:
//
// Update 3D LUT fast load select.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] mpcc_id
// - [in] hubp_idx
//
// Return:
//
// void
//
    pub hubp_idx): *mut *mut *mut void (update_3dlut_fast_load_select)(struct mpc mpc, int mpcc_id, int,
//
// @get_3dlut_fast_load_status:
//
// Get 3D LUT fast load status and reference them with done, soft_underflow and hard_underflow pointers.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] mpcc_id
// - [in/out] done
// - [in/out] soft_underflow
// - [in/out] hard_underflow
//
// Return:
//
// void
//
    pub hard_underflow): *mut *mut *mut *mut *mut void (get_3dlut_fast_load_status)(struct mpc mpc, int mpcc_id, uint32_t done, uint32_t soft_underflow, uint32_t,
//
// @populate_lut:
//
// Populate LUT with given tetrahedral parameters.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] id
// - [in] params
// - [in] lut_bank_a
// - [in] mpcc_id
//
// Return:
//
// void
//
    pub mpcc_id): c_int,
//
// @program_lut_read_write_control:
//
// Program LUT RW control.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] id
// - [in] lut_bank_a
// - [in] bit_depth
// - [in] mpcc_id
//
// Return:
//
// void
//
    pub mpcc_id): c_int,
//
// @program_lut_mode:
//
// Program LUT mode.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] id
// - [in] enable
// - [in] lut_bank_a
// - [in] size
// - [in] mpcc_id
//
// Return:
//
// void
//
    pub mpcc_id): c_int,
//
// @get_lut_mode:
//
// Obtains enablement and ram bank status.
//
// Parameters:
// - [in/out] mpc - MPC context.
// - [in] id
// - [in] mpcc_id
// - [out] enable
// - [out] lut_bank_a
//
// Return:
//
// void
//
    pub lut_bank_a): *mut bool,
//
// @rmcm:
//
// MPC RMCM new HW sequential programming functions
//
    pub mpcc_id): *mut *mut *mut *mut void (fl_3dlut_configure)(struct mpc mpc, struct mpc_fl_3dlut_config cfg, int,
    pub mpcc_id): *mut *mut *mut void (enable_3dlut_fl)(struct mpc mpc, bool enable, int,
    pub hubp_idx): *mut *mut *mut void (update_3dlut_fast_load_select)(struct mpc mpc, int mpcc_id, int,
    pub mpcc_id): bool lut_bank_a, bool enabled, int,
    pub mpcc_id): c_int,
    pub mpcc_id): *const *const *const void (program_3dlut_size)(struct mpc mpc, enum dc_cm_lut_size size, int,
    pub mpcc_id): *mut *mut *mut void (program_bias_scale)(struct mpc mpc, uint16_t bias, uint16_t scale, int,
    pub mpcc_id): *mut *mut *mut void (program_bit_depth)(struct mpc mpc, uint16_t bit_depth, int,
    pub width): *mut *mut bool (is_config_supported)(uint32_t,
    pub power_on): *mut *mut *mut void (power_on_shaper_3dlut)(struct mpc mpc, uint32_t mpcc_id, bool,
    pub mpcc_id): bool lut_bank_a, int,
    pub lut_bank_a): *mut *mut bool enable, bool,
    pub rmcm: },
}
