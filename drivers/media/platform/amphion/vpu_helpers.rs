//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_helpers.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2020-2021 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_pair {
    pub src: u32,
    pub dst: u32,
}

extern "C" {
    pub fn vpu_helper_find_in_array_u8(array: *const u8, size: u32, x: u32) -> c_int;
}
extern "C" {
    pub fn vpu_helper_check_type(inst: *mut vpu_inst, type: u32) -> bool;
}
extern "C" {
    pub fn vpu_helper_match_format(inst: *mut vpu_inst, type: u32, fmta: u32, fmtb: u32) -> bool;
}
extern "C" {
    pub fn vpu_helper_valid_frame_width(inst: *mut vpu_inst, width: u32) -> u32;
}
extern "C" {
    pub fn vpu_helper_valid_frame_height(inst: *mut vpu_inst, height: u32) -> u32;
}
extern "C" {
    pub fn vpu_helper_get_free_space(inst: *mut vpu_inst) -> u32;
}
extern "C" {
    pub fn vpu_helper_get_used_space(inst: *mut vpu_inst) -> u32;
}
extern "C" {
    pub fn vpu_helper_g_volatile_ctrl(ctrl: *mut v4l2_ctrl) -> c_int;
}
extern "C" {
    pub fn vpu_helper_get_kmp_next(pattern: *const u8, next: *mut c_int, size: c_int);
}
extern "C" {
    pub fn vpu_helper_kmp_search(s: *mut u8, s_len: c_int, p: *const u8, p_len: c_int, next: *mut c_int) -> c_int;
}
extern "C" {
    pub fn vpu_color_cvrt_primaries_v2i(primaries: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_primaries_i2v(primaries: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_transfers_v2i(transfers: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_transfers_i2v(transfers: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_matrix_v2i(matrix: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_matrix_i2v(matrix: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_full_range_v2i(full_range: u32) -> u32;
}
extern "C" {
    pub fn vpu_color_cvrt_full_range_i2v(full_range: u32) -> u32;
}
extern "C" {
    pub fn vpu_find_dst_by_src(pairs: *mut vpu_pair, cnt: u32, src: u32) -> c_int;
}
extern "C" {
    pub fn vpu_find_src_by_dst(pairs: *mut vpu_pair, cnt: u32, dst: u32) -> c_int;
}
extern "C" {
    pub fn vpu_get_h264_v4l2_profile(hdr: *mut vpu_dec_codec_info) -> u32;
}
extern "C" {
    pub fn vpu_get_h264_v4l2_level(hdr: *mut vpu_dec_codec_info) -> u32;
}
extern "C" {
    pub fn vpu_get_hevc_v4l2_profile(hdr: *mut vpu_dec_codec_info) -> u32;
}
extern "C" {
    pub fn vpu_get_hevc_v4l2_level(hdr: *mut vpu_dec_codec_info) -> u32;
}
