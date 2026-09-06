//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_v4l2.h
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

extern "C" {
    pub fn vpu_inst_lock(inst: *mut vpu_inst);
}
extern "C" {
    pub fn vpu_inst_unlock(inst: *mut vpu_inst);
}
extern "C" {
    pub fn vpu_set_buffer_state(vbuf: *mut vb2_v4l2_buffer, state: c_uint);
}
extern "C" {
    pub fn vpu_get_buffer_state(vbuf: *mut vb2_v4l2_buffer) -> c_uint;
}
extern "C" {
    pub fn vpu_set_buffer_average_qp(vbuf: *mut vb2_v4l2_buffer, qp: u32);
}
extern "C" {
    pub fn vpu_v4l2_open(file: *mut file, inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_v4l2_close(file: *mut file) -> c_int;
}
extern "C" {
    pub fn vpu_get_fmt_plane_size(fmt: *mut vpu_format, plane_no: u32) -> u32;
}
extern "C" {
    pub fn vpu_try_fmt_common(inst: *mut vpu_inst, f: *mut v4l2_format, fmt: *mut vpu_format) -> c_int;
}
extern "C" {
    pub fn vpu_process_output_buffer(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_process_capture_buffer(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_skip_frame(inst: *mut vpu_inst, count: c_int);
}
extern "C" {
    pub fn vpu_v4l2_set_error(inst: *mut vpu_inst);
}
extern "C" {
    pub fn vpu_notify_source_change(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_set_last_buffer_dequeued(inst: *mut vpu_inst, eos: bool) -> c_int;
}
extern "C" {
    pub fn vpu_get_num_buffers(inst: *mut vpu_inst, type: u32) -> c_int;
}
extern "C" {
    pub fn vpu_is_source_empty(inst: *mut vpu_inst) -> bool;
}
extern "C" {
    pub fn vpu_get_vb_phy_addr(vb: *mut vb2_buffer, plane_no: u32) -> dma_addr_t;
}
