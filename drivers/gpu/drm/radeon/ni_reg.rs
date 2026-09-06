//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/ni_reg.h
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
// Copyright 2010 Advanced Micro Devices, Inc.
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
// Authors: Alex Deucher
//
// northern islands - DCE5
pub const NI_INPUT_GAMMA_CONTROL: c_uint = 0x6840;

pub const NI_PRESCALE_GRPH_CONTROL: c_uint = 0x68b4;

pub const NI_PRESCALE_OVL_CONTROL: c_uint = 0x68c4;

pub const NI_INPUT_CSC_CONTROL: c_uint = 0x68d4;

pub const NI_OUTPUT_CSC_CONTROL: c_uint = 0x68f0;

pub const NI_DEGAMMA_CONTROL: c_uint = 0x6960;

pub const NI_GAMUT_REMAP_CONTROL: c_uint = 0x6964;

pub const NI_REGAMMA_CONTROL: c_uint = 0x6a80;

pub const NI_DP_MSE_LINK_TIMING: c_uint = 0x73a0;

pub const NI_DP_MSE_MISC_CNTL: c_uint = 0x736c;

pub const NI_DP_MSE_RATE_CNTL: c_uint = 0x7384;

pub const NI_DP_MSE_RATE_UPDATE: c_uint = 0x738c;
pub const NI_DP_MSE_SAT0: c_uint = 0x7390;

pub const NI_DP_MSE_SAT1: c_uint = 0x7394;
pub const NI_DP_MSE_SAT2: c_uint = 0x7398;
pub const NI_DP_MSE_SAT_UPDATE: c_uint = 0x739c;

pub const NI_DIG_BE_CNTL: c_uint = 0x7140;

pub const NI_DIG_FE_CNTL: c_uint = 0x7000;

