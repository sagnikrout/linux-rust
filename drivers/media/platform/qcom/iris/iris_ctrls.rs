//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_ctrls.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

extern "C" {
    pub fn iris_ctrls_init(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_session_init_caps(core: *mut iris_core);
}
extern "C" {
    pub fn iris_set_u32_enum(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_stage(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_pipe(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_u32(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_profile(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_level(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_profile_level_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_header_mode_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_header_mode_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_bitrate_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_bitrate_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_peak_bitrate(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_bitrate_mode_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_bitrate_mode_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_entropy_mode_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_entropy_mode_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_min_qp(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_max_qp(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_frame_qp(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_qp_range(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_rotation(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_flip(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_ir_period_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_ir_period_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_ltr_count_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_ltr_count_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_use_ltr(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_mark_ltr(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_use_and_mark_ltr(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_intra_period(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_layer_type(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_layer_count_gen1(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_layer_count_gen2(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_layer_bitrate(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_req_sync_frame(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_time_delta_based_rc(inst: *mut iris_inst, cap_id: platform_inst_fw_cap_type) -> c_int;
}
extern "C" {
    pub fn iris_set_properties(inst: *mut iris_inst, plane: u32) -> c_int;
}
