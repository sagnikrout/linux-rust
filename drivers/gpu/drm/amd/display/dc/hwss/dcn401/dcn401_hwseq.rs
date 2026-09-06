//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/dcn401/dcn401_hwseq.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ips_ono_state {
    ONO_ON = 0,
    ONO_ON_IN_PROGRESS = 1,
    ONO_OFF = 2,
    ONO_OFF_IN_PROGRESS = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ips_ono_region_state {
//
// @desire_pwr_state: desired power state based on configured value
//
    pub desire_pwr_state: u32,
//
// @current_pwr_state: current power gate status
//
    pub current_pwr_state: u32,
}

extern "C" {
    pub fn dcn401_program_gamut_remap(params: *mut program_gamut_remap_params);
}
extern "C" {
    pub fn dcn401_init_hw(dc: *mut dc);
}
extern "C" {
    pub fn dcn401_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;
}
extern "C" {
    pub fn dcn401_trigger_3dlut_dma_load(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn401_enable_stream(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn401_setup_hpo_hw_control(hws: *const dce_hwseq, enable: bool);
}
extern "C" {
    pub fn dcn401_set_cursor_position(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn401_apply_idle_power_optimizations(dc: *mut dc, enable: bool) -> bool;
}
extern "C" {
    pub fn dcn401_fams2_update_config(dc: *mut dc, context: *mut dc_state, enable: bool);
}
extern "C" {
    pub fn dcn401_dmub_hw_control_lock_fast(params: *mut block_sequence_params);
}
extern "C" {
    pub fn dcn401_unblank_stream(pipe_ctx: *mut pipe_ctx, link_settings: *mut dc_link_settings);
}
extern "C" {
    pub fn dcn401_hardware_release(dc: *mut dc);
}
extern "C" {
    pub fn adjust_hotspot_between_slices_for_2x_magnify(cursor_width: u32, pos_cpy: *mut dc_cursor_position);
}
extern "C" {
    pub fn dcn401_wait_for_det_buffer_update_under_otg_master(dc: *mut dc, context: *mut dc_state, otg_master: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn401_interdependent_update_lock(dc: *mut dc, context: *mut dc_state, lock: bool);
}
extern "C" {
    pub fn dcn401_program_outstanding_updates(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn401_perform_3dlut_wa_unlock(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn401_program_front_end_for_ctx(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn401_post_unlock_program_front_end(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn401_update_bandwidth(dc: *mut dc, context: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dcn401_initialize_min_clocks(dc: *mut dc);
}
extern "C" {
    pub fn dcn401_update_cursor_offload_pipe(dc: *mut dc, pipe: *const pipe_ctx);
}
extern "C" {
    pub fn dcn401_dc_ip_request_cntl(dc: *mut dc, enable: bool);
}
