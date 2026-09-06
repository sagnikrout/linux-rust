//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dcn31/dcn31_apg.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
// Macro flag: #define DCN31_APG_FROM_APG(apg)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_apg_registers {
    pub APG_CONTROL: u32,
    pub APG_CONTROL2: u32,
    pub APG_MEM_PWR: u32,
    pub APG_DBG_GEN_CONTROL: u32,
}

// Macro flag: #define DCN31_APG_MASK_SH_LIST(mask_sh)\

// APG0_APG_DBG_GEN_CONTROL
pub const APG0_APG_DBG_GEN_CONTROL__APG_DBG_AUDIO_CHANNEL_ENABLE__SHIFT: c_uint = 0x8;
pub const APG0_APG_DBG_GEN_CONTROL__APG_DBG_AUDIO_CHANNEL_ENABLE_MASK: c_uint = 0x0000FF00L;
// Not in DCN42B: APG_DBG_GEN_CONTROL, APG0_APG_DBG_60958
// Macro flag: #define DCN42B_APG_MASK_SH_LIST(mask_sh)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_apg_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_apg_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apg {
    pub funcs: *const apg_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apg_funcs {
    pub apg): *mut apg,
    pub audio_info): *mut audio_info,
    pub apg): *mut apg,
    pub apg): *mut apg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_apg {
    pub base: apg,
    pub regs: *const dcn31_apg_registers,
    pub apg_shift: *const dcn31_apg_shift,
    pub apg_mask: *const dcn31_apg_mask,
}
