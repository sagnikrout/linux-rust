//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_core/dml2_core_dcn5_calcs_dchub.h
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

// Output
// ???
extern "C" {
    pub fn dcn5_adjust_pixel_clock_for_progressive_to_interlace_unit(display_cfg: *const dml2_display_cfg, ptoi_supported: bool, PixelClockBackEnd: *mut double);
}
// Output
// outputs
// output
// Output
// input
// output
extern "C" {
    pub fn dcn5_calculate_prefetch_schedule(scratch: *mut dml2_core_internal_scratch, p: *mut dml2_core_calcs_CalculatePrefetchSchedule_params) -> bool;
}
// ???
// Output
// Input
// Output
// output
extern "C" {
    pub fn dcn5_calculate_meta_and_pte_times(p: *mut dml2_core_shared_CalculateMetaAndPTETimes_params);
}
// Output
extern "C" {
    pub fn dml2_core_dcn5_calcs_cursor_dlg_reg(cursor_dlg_regs: *mut dml2_cursor_dlg_regs, p: *const dml2_get_cursor_dlg_reg);
}
extern "C" {
    pub fn dcn5_calculate_vm_and_row_bytes(p: *mut dml2_core_shared_calculate_vm_and_row_bytes_params) -> c_uint;
}
extern "C" {
    pub fn dcn5_get_arb_params(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, utm_soc_bb: *const dml2_utm_soc_bb, out: *mut dml2_display_arb_regs);
}
extern "C" {
    pub fn dcn5_get_watermarks(display_cfg: *const dml2_display_cfg, mode_lib: *const dml2_core_internal_display_mode_lib, utm_soc_bb: *const dml2_utm_soc_bb, out: *mut dml2_dchub_watermark_regs);
}
