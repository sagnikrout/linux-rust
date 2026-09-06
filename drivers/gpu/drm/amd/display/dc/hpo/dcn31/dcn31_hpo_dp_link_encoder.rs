//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hpo/dcn31/dcn31_hpo_dp_link_encoder.h
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
// Copyright 2019 Advanced Micro Devices, Inc.
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

// Macro flag: #define DCN3_1_HPO_DP_LINK_ENC_FROM_HPO_LINK_ENC(hpo_dp_link_encoder)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_hpo_dp_link_encoder_registers {
    pub RDPCSTX_PHY_CNTL6: [u32; 5],
}

// Macro flag: #define DCN3_1_HPO_DP_LINK_ENC_RDPCSTX_MASK_SH_LIST(mask_sh)\
// Macro flag: #define DCN3_1_HPO_DP_LINK_ENC_COMMON_MASK_SH_LIST(mask_sh)\
// Macro flag: #define DCN3_1_HPO_DP_LINK_ENC_MASK_SH_LIST(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_hpo_dp_link_encoder_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_hpo_dp_link_encoder_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_hpo_dp_link_encoder {
    pub base: hpo_dp_link_encoder,
    pub regs: *const dcn31_hpo_dp_link_encoder_registers,
    pub hpo_le_shift: *const dcn31_hpo_dp_link_encoder_shift,
    pub hpo_le_mask: *const dcn31_hpo_dp_link_encoder_mask,
}
