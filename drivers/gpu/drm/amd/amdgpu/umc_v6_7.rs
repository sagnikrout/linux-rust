//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/umc_v6_7.h
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

// EccErrCnt max value
pub const UMC_V6_7_CE_CNT_MAX: c_uint = 0xffff;
// umc ce interrupt threshold
pub const UMC_V6_7_CE_INT_THRESHOLD: c_uint = 0xffff;
// umc ce count initial value

pub const UMC_V6_7_INST_DIST: c_uint = 0x40000;
// number of umc channel instance with memory map register access
pub const UMC_V6_7_UMC_INSTANCE_NUM: c_int = 4;
// number of umc instance with memory map register access
pub const UMC_V6_7_CHANNEL_INSTANCE_NUM: c_int = 8;
// total channel instances in one umc block

// one piece of normalizing address is mapped to 8 pieces of physical address
pub const UMC_V6_7_NA_MAP_PA_NUM: c_int = 8;
// R14 bit shift should be considered, double the number

// The CH4 bit in SOC physical address
pub const UMC_V6_7_PA_CH4_BIT: c_int = 12;
// The C2 bit in SOC physical address
pub const UMC_V6_7_PA_C2_BIT: c_int = 17;
// The R14 bit in SOC physical address
pub const UMC_V6_7_PA_R14_BIT: c_int = 34;
// UMC regiser per channel offset
pub const UMC_V6_7_PER_CHANNEL_OFFSET: c_uint = 0x400;
// XOR bit 20, 25, 34 of PA into CH4 bit (bit 12 of PA),
// hash bit is only effective when related setting is enabled
//

