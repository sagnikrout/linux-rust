//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/umc_v12_0.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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

// one piece of normalized address is mapped to 8 pieces of physical address
pub const UMC_V12_0_NA_MAP_PA_NUM: c_int = 8;
// R13 bit shift should be considered, double the number

// column bits in SOC physical address
pub const UMC_V12_0_PA_C2_BIT: c_int = 15;
pub const UMC_V12_0_PA_C3_BIT: c_int = 16;
pub const UMC_V12_0_PA_C4_BIT: c_int = 21;
// row bits in SOC physical address
pub const UMC_V12_0_PA_R0_BIT: c_int = 22;
pub const UMC_V12_0_PA_R10_BIT: c_int = 32;
pub const UMC_V12_0_PA_R11_BIT: c_int = 33;
pub const UMC_V12_0_PA_R12_BIT: c_int = 34;
pub const UMC_V12_0_PA_R13_BIT: c_int = 35;
// channel bit in SOC physical address
pub const UMC_V12_0_PA_CH4_BIT: c_int = 12;
pub const UMC_V12_0_PA_CH5_BIT: c_int = 13;
// bank bit in SOC physical address
pub const UMC_V12_0_PA_B0_BIT: c_int = 19;
pub const UMC_V12_0_PA_B1_BIT: c_int = 20;
// row bits in MCA address
pub const UMC_V12_0_MA_R0_BIT: c_int = 10;

extern "C" {
    pub fn umc_v12_0_is_uncorrectable_error(adev: *mut amdgpu_device, mc_umc_status: u64) -> bool;
}
extern "C" {
    pub fn umc_v12_0_is_correctable_error(adev: *mut amdgpu_device, mc_umc_status: u64) -> bool;
}
