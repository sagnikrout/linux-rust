//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/helpers.h
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//

extern "C" {
    pub fn venus_helper_check_codec(inst: *mut venus_inst, v4l2_pixfmt: u32) -> bool;
}
extern "C" {
    pub fn venus_helper_vb2_buf_init(vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn venus_helper_vb2_buf_prepare(vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn venus_helper_vb2_buf_queue(vb: *mut vb2_buffer);
}
extern "C" {
    pub fn venus_helper_vb2_stop_streaming(q: *mut vb2_queue);
}
extern "C" {
    pub fn venus_helper_vb2_start_streaming(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_vb2_queue_error(inst: *mut venus_inst);
}
extern "C" {
    pub fn venus_helper_m2m_device_run(priv: *mut c_void);
}
extern "C" {
    pub fn venus_helper_m2m_job_abort(priv: *mut c_void);
}
extern "C" {
    pub fn venus_helper_get_framesz_raw(hfi_fmt: u32, width: u32, height: u32) -> u32;
}
extern "C" {
    pub fn venus_helper_get_framesz(v4l2_fmt: u32, width: u32, height: u32) -> u32;
}
extern "C" {
    pub fn venus_helper_set_work_mode(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_set_format_constraints(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_set_color_format(inst: *mut venus_inst, fmt: u32) -> c_int;
}
extern "C" {
    pub fn venus_helper_set_dyn_bufmode(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_set_bufsize(inst: *mut venus_inst, bufsize: u32, buftype: u32) -> c_int;
}
extern "C" {
    pub fn venus_helper_get_opb_size(inst: *mut venus_inst) -> c_uint;
}
extern "C" {
    pub fn venus_helper_acquire_buf_ref(vbuf: *mut vb2_v4l2_buffer);
}
extern "C" {
    pub fn venus_helper_release_buf_ref(inst: *mut venus_inst, idx: c_uint);
}
extern "C" {
    pub fn venus_helper_init_instance(inst: *mut venus_inst);
}
extern "C" {
    pub fn venus_helper_session_init(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_check_format(inst: *mut venus_inst, v4l2_pixfmt: u32) -> bool;
}
extern "C" {
    pub fn venus_helper_alloc_dpb_bufs(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_free_dpb_bufs(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_intbufs_alloc(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_intbufs_free(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_intbufs_realloc(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_queue_dpb_bufs(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_unregister_bufs(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_process_initial_cap_bufs(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_process_initial_out_bufs(inst: *mut venus_inst) -> c_int;
}
extern "C" {
    pub fn venus_helper_get_profile_level(inst: *mut venus_inst, profile: *mut u32, level: *mut u32) -> c_int;
}
extern "C" {
    pub fn venus_helper_set_profile_level(inst: *mut venus_inst, profile: u32, level: u32) -> c_int;
}
