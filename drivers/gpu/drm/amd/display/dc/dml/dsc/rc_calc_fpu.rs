//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml/dsc/rc_calc_fpu.h
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


//
// Copyright 2021 Advanced Micro Devices, Inc.
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

pub const QP_SET_SIZE: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rc_params {
    pub rc_quant_incr_limit0: c_int,
    pub rc_quant_incr_limit1: c_int,
    pub initial_fullness_offset: c_int,
    pub initial_xmit_delay: c_int,
    pub first_line_bpg_offset: c_int,
    pub second_line_bpg_offset: c_int,
    pub flatness_min_qp: c_int,
    pub flatness_max_qp: c_int,
    pub flatness_det_thresh: c_int,
    pub qp_min: qp_set,
    pub qp_max: qp_set,
    pub ofs: qp_set,
    pub rc_model_size: c_int,
    pub rc_edge_factor: c_int,
    pub rc_tgt_offset_hi: c_int,
    pub rc_tgt_offset_lo: c_int,
    pub 1]: int rc_buf_thresh[QP_SET_SIZE -,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum colour_mode {
    CM_RGB,   /* 444 RGB */
    CM_444,   /* 444 YUV or simple 422 */
    CM_422,   /* native 422 */
    CM_420    /* native 420 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bits_per_comp {
    BPC_8  =  8,
    BPC_10 = 10,
    BPC_12 = 12
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max_min {
    DAL_MM_MIN = 0,
    DAL_MM_MAX = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qp_entry {
    pub bpp: float,
    pub qps: qp_set,
}
