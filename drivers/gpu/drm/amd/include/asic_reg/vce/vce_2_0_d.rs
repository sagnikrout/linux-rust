//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/vce/vce_2_0_d.h
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
// VCE_2_0 Register documentation
//
// Copyright (C) 2014  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
pub const mmVCE_STATUS: c_uint = 0x8001;
pub const mmVCE_VCPU_CNTL: c_uint = 0x8005;
pub const mmVCE_VCPU_CACHE_OFFSET0: c_uint = 0x8009;
pub const mmVCE_VCPU_CACHE_SIZE0: c_uint = 0x800a;
pub const mmVCE_VCPU_CACHE_OFFSET1: c_uint = 0x800b;
pub const mmVCE_VCPU_CACHE_SIZE1: c_uint = 0x800c;
pub const mmVCE_VCPU_CACHE_OFFSET2: c_uint = 0x800d;
pub const mmVCE_VCPU_CACHE_SIZE2: c_uint = 0x800e;
pub const mmVCE_SOFT_RESET: c_uint = 0x8048;
pub const mmVCE_RB_BASE_LO2: c_uint = 0x805b;
pub const mmVCE_RB_BASE_HI2: c_uint = 0x805c;
pub const mmVCE_RB_SIZE2: c_uint = 0x805d;
pub const mmVCE_RB_RPTR2: c_uint = 0x805e;
pub const mmVCE_RB_WPTR2: c_uint = 0x805f;
pub const mmVCE_RB_BASE_LO: c_uint = 0x8060;
pub const mmVCE_RB_BASE_HI: c_uint = 0x8061;
pub const mmVCE_RB_SIZE: c_uint = 0x8062;
pub const mmVCE_RB_RPTR: c_uint = 0x8063;
pub const mmVCE_RB_WPTR: c_uint = 0x8064;
pub const mmVCE_RB_ARB_CTRL: c_uint = 0x809f;
pub const mmVCE_CLOCK_GATING_A: c_uint = 0x80be;
pub const mmVCE_CLOCK_GATING_B: c_uint = 0x80bf;
pub const mmVCE_UENC_DMA_DCLK_CTRL: c_uint = 0x8390;
pub const mmVCE_CGTT_CLK_OVERRIDE: c_uint = 0x81e8;
pub const mmVCE_UENC_CLOCK_GATING: c_uint = 0x81ef;
pub const mmVCE_UENC_REG_CLOCK_GATING: c_uint = 0x81f0;
pub const mmVCE_SYS_INT_EN: c_uint = 0x84c0;
pub const mmVCE_SYS_INT_STATUS: c_uint = 0x84c1;
pub const mmVCE_SYS_INT_ACK: c_uint = 0x84c1;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR: c_uint = 0x8517;
pub const mmVCE_LMI_CTRL2: c_uint = 0x851d;
pub const mmVCE_LMI_SWAP_CNTL3: c_uint = 0x851e;
pub const mmVCE_LMI_CTRL: c_uint = 0x8526;
pub const mmVCE_LMI_STATUS: c_uint = 0x8527;
pub const mmVCE_LMI_VM_CTRL: c_uint = 0x8528;
pub const mmVCE_LMI_SWAP_CNTL: c_uint = 0x852d;
pub const mmVCE_LMI_SWAP_CNTL1: c_uint = 0x852e;
pub const mmVCE_LMI_SWAP_CNTL2: c_uint = 0x8533;
pub const mmVCE_LMI_MISC_CTRL: c_uint = 0x8535;
pub const mmVCE_LMI_CACHE_CTRL: c_uint = 0x853d;
