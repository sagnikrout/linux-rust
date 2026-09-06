//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hubp/dcn10/dcn10_hubp.h
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


// Copyright 2012-15 Advanced Micro Devices, Inc.
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

// Macro flag: #define TO_DCN10_HUBP(hubp)\
// Register address initialization macro for all ASICs (including those with reduced functionality)
// Macro flag: #define HUBP_REG_LIST_DCN(id)\
// Register address initialization macro for ASICs with VM
// Macro flag: #define HUBP_REG_LIST_DCN_VM(id)\
// Macro flag: #define HUBP_REG_LIST_DCN10(id)\

// Mask/shift struct generation macro for all ASICs (including those with reduced functionality)
// 1.x, 2.x, and 3.x
// Macro flag: #define HUBP_MASK_SH_LIST_DCN_SHARE_COMMON(mask_sh)\
// 2.x and 1.x only
// Macro flag: #define HUBP_MASK_SH_LIST_DCN_COMMON(mask_sh)\
// 2.x and 1.x only
// Macro flag: #define HUBP_MASK_SH_LIST_DCN(mask_sh)\
// Mask/shift struct generation macro for ASICs with VM
// Macro flag: #define HUBP_MASK_SH_LIST_DCN_VM(mask_sh)\
// Macro flag: #define HUBP_MASK_SH_LIST_DCN10(mask_sh)\

// todo:  get these from GVM instead of reading registers ourselves */\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_mi_registers {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_mi_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_mi_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_fl_regs_st {
    pub lut_enable: u32,
    pub lut_done: u32,
    pub lut_addr_mode: u32,
    pub lut_width: u32,
    pub lut_mpc_width: u32,
    pub lut_tmz: u32,
    pub lut_crossbar_sel_r: u32,
    pub lut_crossbar_sel_g: u32,
    pub lut_crossbar_sel_b: u32,
    pub lut_addr_hi: u32,
    pub lut_addr_lo: u32,
    pub refcyc_3dlut_group: u32,
    pub lut_fl_bias: u32,
    pub lut_fl_scale: u32,
    pub lut_fl_mode: u32,
    pub lut_fl_format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubp_reg_state {
    pub hubp_cntl: u32,
    pub mall_config: u32,
    pub mall_sub_vp: u32,
    pub hubp_req_size_config: u32,
    pub hubp_req_size_config_c: u32,
    pub vmpg_config: u32,
    pub addr_config: u32,
    pub pri_viewport_dimension: u32,
    pub pri_viewport_dimension_c: u32,
    pub pri_viewport_start: u32,
    pub pri_viewport_start_c: u32,
    pub sec_viewport_dimension: u32,
    pub sec_viewport_dimension_c: u32,
    pub sec_viewport_start: u32,
    pub sec_viewport_start_c: u32,
    pub surface_config: u32,
    pub tiling_config: u32,
    pub clk_cntl: u32,
    pub mall_status: u32,
    pub measure_win_ctrl_dcfclk: u32,
    pub measure_win_ctrl_dppclk: u32,
    pub blank_offset_0: u32,
    pub blank_offset_1: u32,
    pub cursor_settings: u32,
    pub dcn_cur0_ttu_cntl0: u32,
    pub dcn_cur0_ttu_cntl1: u32,
    pub dcn_cur1_ttu_cntl0: u32,
    pub dcn_cur1_ttu_cntl1: u32,
    pub dcn_dmdat_vm_cntl: u32,
    pub dcn_expansion_mode: u32,
    pub dcn_global_ttu_cntl: u32,
    pub dcn_surf0_ttu_cntl0: u32,
    pub dcn_surf0_ttu_cntl1: u32,
    pub dcn_surf1_ttu_cntl0: u32,
    pub dcn_surf1_ttu_cntl1: u32,
    pub dcn_ttu_qos_wm: u32,
    pub dcn_vm_mx_l1_tlb_cntl: u32,
    pub dcn_vm_system_aperture_high_addr: u32,
    pub dcn_vm_system_aperture_low_addr: u32,
    pub dcsurf_flip_control: u32,
    pub dcsurf_flip_control2: u32,
    pub dcsurf_primary_meta_surface_address: u32,
    pub dcsurf_primary_meta_surface_address_c: u32,
    pub dcsurf_primary_meta_surface_address_high: u32,
    pub dcsurf_primary_meta_surface_address_high_c: u32,
    pub dcsurf_primary_surface_address: u32,
    pub dcsurf_primary_surface_address_c: u32,
    pub dcsurf_primary_surface_address_high: u32,
    pub dcsurf_primary_surface_address_high_c: u32,
    pub dcsurf_secondary_meta_surface_address: u32,
    pub dcsurf_secondary_meta_surface_address_c: u32,
    pub dcsurf_secondary_meta_surface_address_high: u32,
    pub dcsurf_secondary_meta_surface_address_high_c: u32,
    pub dcsurf_secondary_surface_address: u32,
    pub dcsurf_secondary_surface_address_c: u32,
    pub dcsurf_secondary_surface_address_high: u32,
    pub dcsurf_secondary_surface_address_high_c: u32,
    pub dcsurf_surface_control: u32,
    pub dcsurf_surface_earliest_inuse: u32,
    pub dcsurf_surface_earliest_inuse_c: u32,
    pub dcsurf_surface_earliest_inuse_high: u32,
    pub dcsurf_surface_earliest_inuse_high_c: u32,
    pub dcsurf_surface_flip_interrupt: u32,
    pub dcsurf_surface_inuse: u32,
    pub dcsurf_surface_inuse_c: u32,
    pub dcsurf_surface_inuse_high: u32,
    pub dcsurf_surface_inuse_high_c: u32,
    pub dcsurf_surface_pitch: u32,
    pub dcsurf_surface_pitch_c: u32,
    pub dst_after_scaler: u32,
    pub dst_dimensions: u32,
    pub dst_y_delta_drq_limit: u32,
    pub flip_parameters_0: u32,
    pub flip_parameters_1: u32,
    pub flip_parameters_2: u32,
    pub flip_parameters_3: u32,
    pub flip_parameters_4: u32,
    pub flip_parameters_5: u32,
    pub flip_parameters_6: u32,
    pub hubpreq_mem_pwr_ctrl: u32,
    pub hubpreq_mem_pwr_status: u32,
    pub nom_parameters_0: u32,
    pub nom_parameters_1: u32,
    pub nom_parameters_2: u32,
    pub nom_parameters_3: u32,
    pub nom_parameters_4: u32,
    pub nom_parameters_5: u32,
    pub nom_parameters_6: u32,
    pub nom_parameters_7: u32,
    pub per_line_delivery: u32,
    pub per_line_delivery_pre: u32,
    pub prefetch_settings: u32,
    pub prefetch_settings_c: u32,
    pub ref_freq_to_pix_freq: u32,
    pub uclk_pstate_force: u32,
    pub vblank_parameters_0: u32,
    pub vblank_parameters_1: u32,
    pub vblank_parameters_2: u32,
    pub vblank_parameters_3: u32,
    pub vblank_parameters_4: u32,
    pub vblank_parameters_5: u32,
    pub vblank_parameters_6: u32,
    pub vmid_settings_0: u32,
    pub hubpret_control: u32,
    pub hubpret_interrupt: u32,
    pub hubpret_mem_pwr_ctrl: u32,
    pub hubpret_mem_pwr_status: u32,
    pub hubpret_read_line_ctrl0: u32,
    pub hubpret_read_line_ctrl1: u32,
    pub hubpret_read_line_status: u32,
    pub hubpret_read_line_value: u32,
    pub hubpret_read_line0: u32,
    pub hubpret_read_line1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubp_state {
    pub dlg_attr: _vcs_dpi_display_dlg_regs_st,
    pub ttu_attr: _vcs_dpi_display_ttu_regs_st,
    pub rq_regs: _vcs_dpi_display_rq_regs_st,
    pub fl_regs: dcn_fl_regs_st,
    pub pixel_format: u32,
    pub inuse_addr_hi: u32,
    pub inuse_addr_lo: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub rotation_angle: u32,
    pub h_mirror_en: u32,
    pub sw_mode: u32,
    pub dcc_en: u32,
    pub blank_en: u32,
    pub clock_en: u32,
    pub underflow_status: u32,
    pub ttu_disable: u32,
    pub min_ttu_vblank: u32,
    pub qos_level_low_wm: u32,
    pub qos_level_high_wm: u32,
    pub primary_surface_addr_lo: u32,
    pub primary_surface_addr_hi: u32,
    pub primary_meta_addr_lo: u32,
    pub primary_meta_addr_hi: u32,
    pub uclk_pstate_force: u32,
    pub hubp_cntl: u32,
    pub flip_control: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_hubp {
    pub base: hubp,
    pub state: dcn_hubp_state,
    pub hubp_regs: *const dcn_mi_registers,
    pub hubp_shift: *const dcn_mi_shift,
    pub hubp_mask: *const dcn_mi_mask,
}

extern "C" {
    pub fn hubp_reset(hubp: *mut hubp);
}
extern "C" {
    pub fn hubp1_is_flip_pending(hubp: *mut hubp) -> bool;
}
extern "C" {
    pub fn hubp1_set_blank(hubp: *mut hubp, blank: bool);
}
extern "C" {
    pub fn hubp1_clk_cntl(hubp: *mut hubp, enable: bool);
}
extern "C" {
    pub fn hubp1_vtg_sel(hubp: *mut hubp, otg_inst: u32);
}
extern "C" {
    pub fn hubp1_read_state(hubp: *mut hubp);
}
extern "C" {
    pub fn hubp1_clear_underflow(hubp: *mut hubp);
}
extern "C" {
    pub fn hubp1_get_cursor_pitch(pitch: c_uint) -> cursor_pitch;
}
extern "C" {
    pub fn hubp1_init(hubp: *mut hubp);
}
extern "C" {
    pub fn hubp1_read_state_common(hubp: *mut hubp);
}
extern "C" {
    pub fn hubp1_in_blank(hubp: *mut hubp) -> bool;
}
extern "C" {
    pub fn hubp1_soft_reset(hubp: *mut hubp, reset: bool);
}
extern "C" {
    pub fn hubp1_set_flip_int(hubp: *mut hubp);
}
extern "C" {
    pub fn hubp1_clear_tiling(hubp: *mut hubp);
}
