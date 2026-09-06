//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dcn10/dcn10_dwb.h
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


// Copyright 2012-17 Advanced Micro Devices, Inc.
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
// DCN

// Macro flag: #define SR(reg_name)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_dwbc_registers {
    pub WB_ENABLE: u32,
    pub WB_EC_CONFIG: u32,
    pub CNV_MODE: u32,
    pub WB_SOFT_RESET: u32,
    pub MCIF_WB_BUFMGR_SW_CONTROL: u32,
    pub MCIF_WB_BUF_PITCH: u32,
    pub MCIF_WB_ARBITRATION_CONTROL: u32,
    pub MCIF_WB_SCLK_CHANGE: u32,
    pub MCIF_WB_BUF_1_ADDR_Y: u32,
    pub MCIF_WB_BUF_1_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_1_ADDR_C: u32,
    pub MCIF_WB_BUF_1_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_2_ADDR_Y: u32,
    pub MCIF_WB_BUF_2_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_2_ADDR_C: u32,
    pub MCIF_WB_BUF_2_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_3_ADDR_Y: u32,
    pub MCIF_WB_BUF_3_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_3_ADDR_C: u32,
    pub MCIF_WB_BUF_3_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUF_4_ADDR_Y: u32,
    pub MCIF_WB_BUF_4_ADDR_Y_OFFSET: u32,
    pub MCIF_WB_BUF_4_ADDR_C: u32,
    pub MCIF_WB_BUF_4_ADDR_C_OFFSET: u32,
    pub MCIF_WB_BUFMGR_VCE_CONTROL: u32,
    pub MCIF_WB_NB_PSTATE_LATENCY_WATERMARK: u32,
    pub MCIF_WB_NB_PSTATE_CONTROL: u32,
    pub MCIF_WB_WATERMARK: u32,
    pub MCIF_WB_WARM_UP_CNTL: u32,
    pub MCIF_WB_BUF_LUMA_SIZE: u32,
    pub MCIF_WB_BUF_CHROMA_SIZE: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_dwbc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_dwbc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_dwbc {
    pub base: dwbc,
    pub dwbc_regs: *const dcn10_dwbc_registers,
    pub dwbc_shift: *const dcn10_dwbc_shift,
    pub dwbc_mask: *const dcn10_dwbc_mask,
}
