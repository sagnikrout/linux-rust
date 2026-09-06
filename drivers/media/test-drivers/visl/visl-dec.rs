//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/visl/visl-dec.h
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
// Contains the virtual decoder logic. The functions here control the
// tracing/TPG on a per-frame basis
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_fwht_run {
    pub params: *const v4l2_ctrl_fwht_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_mpeg2_run {
    pub seq: *const v4l2_ctrl_mpeg2_sequence,
    pub pic: *const v4l2_ctrl_mpeg2_picture,
    pub quant: *const v4l2_ctrl_mpeg2_quantisation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_vp8_run {
    pub frame: *const v4l2_ctrl_vp8_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_vp9_run {
    pub frame: *const v4l2_ctrl_vp9_frame,
    pub probs: *const v4l2_ctrl_vp9_compressed_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_h264_run {
    pub sps: *const v4l2_ctrl_h264_sps,
    pub pps: *const v4l2_ctrl_h264_pps,
    pub sm: *const v4l2_ctrl_h264_scaling_matrix,
    pub spram: *const v4l2_ctrl_h264_slice_params,
    pub dpram: *const v4l2_ctrl_h264_decode_params,
    pub pwht: *const v4l2_ctrl_h264_pred_weights,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_hevc_run {
    pub sps: *const v4l2_ctrl_hevc_sps,
    pub pps: *const v4l2_ctrl_hevc_pps,
    pub spram: *const v4l2_ctrl_hevc_slice_params,
    pub sm: *const v4l2_ctrl_hevc_scaling_matrix,
    pub dpram: *const v4l2_ctrl_hevc_decode_params,
    pub rps_lt: *const v4l2_ctrl_hevc_ext_sps_lt_rps,
    pub rps_st: *const v4l2_ctrl_hevc_ext_sps_st_rps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_av1_run {
    pub seq: *const v4l2_ctrl_av1_sequence,
    pub frame: *const v4l2_ctrl_av1_frame,
    pub tge: *const v4l2_ctrl_av1_tile_group_entry,
    pub grain: *const v4l2_ctrl_av1_film_grain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visl_run {
    pub src: *mut vb2_v4l2_buffer,
    pub dst: *mut vb2_v4l2_buffer,
    pub fwht: visl_fwht_run,
    pub mpeg2: visl_mpeg2_run,
    pub vp8: visl_vp8_run,
    pub vp9: visl_vp9_run,
    pub h264: visl_h264_run,
    pub hevc: visl_hevc_run,
    pub av1: visl_av1_run,
}

extern "C" {
    pub fn visl_dec_start(ctx: *mut visl_ctx) -> c_int;
}
extern "C" {
    pub fn visl_dec_stop(ctx: *mut visl_ctx) -> c_int;
}
extern "C" {
    pub fn visl_job_ready(priv: *mut c_void) -> c_int;
}
extern "C" {
    pub fn visl_device_run(priv: *mut c_void);
}
