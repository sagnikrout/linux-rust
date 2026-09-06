//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/wave5/wave5-vpu.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Wave5 series multi-standard codec IP - basic types
//
// Copyright (C) 2021-2023 CHIPS&MEDIA INC
//

pub const VPU_BUF_SYNC_TO_DEVICE: c_int = 0;
pub const VPU_BUF_SYNC_FROM_DEVICE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_src_buffer {
    pub v4l2_m2m_buf: v4l2_m2m_buffer,
    pub consumed: bool,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_dst_buffer {
    pub v4l2_m2m_buf: v4l2_m2m_buffer,
    pub display: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_fmt_type {
    VPU_FMT_TYPE_CODEC = 0,
    VPU_FMT_TYPE_RAW   = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_format {
    pub v4l2_pix_fmt: c_uint,
    pub v4l2_frmsize: *const v4l2_frmsize_stepwise,
}

extern "C" {
    pub fn container_of(_arg: vfh, vpu_instance: struct, _arg: v4l2_fh) -> return;
}
extern "C" {
    pub fn wave5_to_vpu_inst(_arg: file_to_v4l2_fh(filp)) -> return;
}
extern "C" {
    pub fn container_of(_arg: vctrl->handler, vpu_instance: struct, _arg: v4l2_ctrl_hdl) -> return;
}
extern "C" {
    pub fn container_of(_arg: vbuf, vpu_src_buffer: struct, _arg: v4l2_m2m_buf.vb) -> return;
}
extern "C" {
    pub fn container_of(_arg: vbuf, vpu_dst_buffer: struct, _arg: v4l2_m2m_buf.vb) -> return;
}
extern "C" {
    pub fn wave5_vpu_wait_interrupt(inst: *mut vpu_instance, timeout: c_uint) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_register_device(dev: *mut vpu_device) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_dec_unregister_device(dev: *mut vpu_device);
}
extern "C" {
    pub fn wave5_vpu_enc_register_device(dev: *mut vpu_device) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_enc_unregister_device(dev: *mut vpu_device);
}
extern "C" {
    pub fn vb2_is_streaming(vb2_is_streaming(vq_out: vq_cap) &&) -> return;
}
