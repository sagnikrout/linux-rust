//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_core/dml2_core_dcn4_calcs.h
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
// Copyright 2024 Advanced Micro Devices, Inc.

extern "C" {
    pub fn dml2_core_calcs_mode_support_ex(in_out_params: *mut dml2_core_calcs_mode_support_ex) -> c_uint;
}
extern "C" {
    pub fn dml2_core_calcs_mode_programming_ex(in_out_params: *mut dml2_core_calcs_mode_programming_ex) -> bool;
}
extern "C" {
    pub fn dml2_core_calcs_get_watermarks(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_dchub_watermark_regs);
}
extern "C" {
    pub fn dml2_core_calcs_get_mcif_arb_params(mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_mcif_global_register_set);
}
extern "C" {
    pub fn dml2_core_calcs_get_arb_params(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_display_arb_regs);
}
extern "C" {
    pub fn dml2_core_calcs_get_pipe_regs(dml2_display_cfg: *const dml2_display_cfg, mode_lib: *mut dml2_core_internal_display_mode_lib, out: *mut dml2_dchub_per_pipe_register_set, pipe_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_stream_programming(mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_per_stream_programming, pipe_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_global_sync_programming(mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_global_sync_programming, pipe_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_mcache_allocation(mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_mcache_surface_allocation, plane_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_plane_support_info(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut core_plane_support_info, plane_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_informative(mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_display_cfg_programming);
}
extern "C" {
    pub fn dml2_core_calcs_get_stream_support_info(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut core_stream_support_info, plane_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_mall_allocation(mode_lib: *mut dml2_core_internal_display_mode_lib, out: *mut c_uint, pipe_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_stream_fams2_programming(mode_lib: *const dml2_core_internal_display_mode_lib, display_cfg: *const display_configuation_with_meta, fams2_base_programming: *mut dmub_cmd_fams2_config, fams2_sub_programming: *mut dmub_cmd_fams2_config, pstate_method: dml2_pstate_method, plane_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_global_fams2_programming(mode_lib: *const dml2_core_internal_display_mode_lib, display_cfg: *const display_configuation_with_meta, fams2_global_config: *mut dmub_cmd_fams2_global_config);
}
extern "C" {
    pub fn dml2_core_calcs_get_per_dwb_params(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, out: *mut dml2_mcif_per_pipe_register_set, stream_index: c_int, dwb_index: c_int);
}
extern "C" {
    pub fn dml2_core_calcs_get_dpte_row_height(dpte_row_height: *mut c_uint, mode_lib: *mut dml2_core_internal_display_mode_lib, is_plane1: bool, SourcePixelFormat: dml2_source_format_class, SurfaceTiling: dml2_swizzle_mode, ScanDirection: dml2_rotation_angle, pitch: c_uint, GPUVMMinPageSizeKBytes: c_uint);
}
extern "C" {
    pub fn dml2_core_calcs_cursor_dlg_reg(cursor_dlg_regs: *mut dml2_cursor_dlg_regs, p: *const dml2_get_cursor_dlg_reg);
}
