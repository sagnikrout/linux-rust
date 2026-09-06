//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/inc/cz_ppsmc.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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

// Fan control algorithm:
pub const FDO_MODE_HARDWARE: c_int = 0;
pub const FDO_MODE_PIECE_WISE_LINEAR: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FAN_CONTROL {
    FAN_CONTROL_FUZZY,
    FAN_CONTROL_TABLE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DPM_ARRAY {
    DPM_ARRAY_HARD_MAX,
    DPM_ARRAY_HARD_MIN,
    DPM_ARRAY_SOFT_MAX,
    DPM_ARRAY_SOFT_MIN
}

//
// Return codes for driver to SMC communication.
// Leave these #define-s, enums might not be exactly 8-bits on the microcontroller.
//

//
// Supported driver messages
//

// REMOVE LATER

// Feature Enable Masks
pub const NB_DPM_MASK: c_uint = 0x00000800;
pub const VDDGFX_MASK: c_uint = 0x00800000;
pub const VCE_DPM_MASK: c_uint = 0x00400000;
pub const ACP_DPM_MASK: c_uint = 0x00040000;
pub const UVD_DPM_MASK: c_uint = 0x00010000;
pub const GFX_CU_PG_MASK: c_uint = 0x00004000;
pub const SCLK_DPM_MASK: c_uint = 0x00080000;

