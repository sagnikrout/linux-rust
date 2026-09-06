//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/vce/vce_2_0_sh_mask.h
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
pub const VCE_STATUS__JOB_BUSY_MASK: c_uint = 0x1;
pub const VCE_STATUS__JOB_BUSY__SHIFT: c_uint = 0x0;
pub const VCE_STATUS__VCPU_REPORT_MASK: c_uint = 0xfe;
pub const VCE_STATUS__VCPU_REPORT__SHIFT: c_uint = 0x1;
pub const VCE_STATUS__UENC_BUSY_MASK: c_uint = 0x100;
pub const VCE_STATUS__UENC_BUSY__SHIFT: c_uint = 0x8;
pub const VCE_VCPU_CNTL__CLK_EN_MASK: c_uint = 0x1;
pub const VCE_VCPU_CNTL__CLK_EN__SHIFT: c_uint = 0x0;
pub const VCE_VCPU_CNTL__RBBM_SOFT_RESET_MASK: c_uint = 0x40000;
pub const VCE_VCPU_CNTL__RBBM_SOFT_RESET__SHIFT: c_uint = 0x12;
pub const VCE_VCPU_CACHE_OFFSET0__OFFSET_MASK: c_uint = 0xfffffff;
pub const VCE_VCPU_CACHE_OFFSET0__OFFSET__SHIFT: c_uint = 0x0;
pub const VCE_VCPU_CACHE_SIZE0__SIZE_MASK: c_uint = 0xffffff;
pub const VCE_VCPU_CACHE_SIZE0__SIZE__SHIFT: c_uint = 0x0;
pub const VCE_VCPU_CACHE_OFFSET1__OFFSET_MASK: c_uint = 0xfffffff;
pub const VCE_VCPU_CACHE_OFFSET1__OFFSET__SHIFT: c_uint = 0x0;
pub const VCE_VCPU_CACHE_SIZE1__SIZE_MASK: c_uint = 0xffffff;
pub const VCE_VCPU_CACHE_SIZE1__SIZE__SHIFT: c_uint = 0x0;
pub const VCE_VCPU_CACHE_OFFSET2__OFFSET_MASK: c_uint = 0xfffffff;
pub const VCE_VCPU_CACHE_OFFSET2__OFFSET__SHIFT: c_uint = 0x0;
pub const VCE_VCPU_CACHE_SIZE2__SIZE_MASK: c_uint = 0xffffff;
pub const VCE_VCPU_CACHE_SIZE2__SIZE__SHIFT: c_uint = 0x0;
pub const VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK: c_uint = 0x1;
pub const VCE_SOFT_RESET__ECPU_SOFT_RESET__SHIFT: c_uint = 0x0;
pub const VCE_RB_BASE_LO2__RB_BASE_LO_MASK: c_uint = 0xffffffc0;
pub const VCE_RB_BASE_LO2__RB_BASE_LO__SHIFT: c_uint = 0x6;
pub const VCE_RB_BASE_HI2__RB_BASE_HI_MASK: c_uint = 0xffffffff;
pub const VCE_RB_BASE_HI2__RB_BASE_HI__SHIFT: c_uint = 0x0;
pub const VCE_RB_SIZE2__RB_SIZE_MASK: c_uint = 0x7ffff0;
pub const VCE_RB_SIZE2__RB_SIZE__SHIFT: c_uint = 0x4;
pub const VCE_RB_RPTR2__RB_RPTR_MASK: c_uint = 0x7ffff0;
pub const VCE_RB_RPTR2__RB_RPTR__SHIFT: c_uint = 0x4;
pub const VCE_RB_WPTR2__RB_WPTR_MASK: c_uint = 0x7ffff0;
pub const VCE_RB_WPTR2__RB_WPTR__SHIFT: c_uint = 0x4;
pub const VCE_RB_BASE_LO__RB_BASE_LO_MASK: c_uint = 0xffffffc0;
pub const VCE_RB_BASE_LO__RB_BASE_LO__SHIFT: c_uint = 0x6;
pub const VCE_RB_BASE_HI__RB_BASE_HI_MASK: c_uint = 0xffffffff;
pub const VCE_RB_BASE_HI__RB_BASE_HI__SHIFT: c_uint = 0x0;
pub const VCE_RB_SIZE__RB_SIZE_MASK: c_uint = 0x7ffff0;
pub const VCE_RB_SIZE__RB_SIZE__SHIFT: c_uint = 0x4;
pub const VCE_RB_RPTR__RB_RPTR_MASK: c_uint = 0x7ffff0;
pub const VCE_RB_RPTR__RB_RPTR__SHIFT: c_uint = 0x4;
pub const VCE_RB_WPTR__RB_WPTR_MASK: c_uint = 0x7ffff0;
pub const VCE_RB_WPTR__RB_WPTR__SHIFT: c_uint = 0x4;
pub const VCE_UENC_DMA_DCLK_CTRL__WRDMCLK_FORCEON_MASK: c_uint = 0x1;
pub const VCE_UENC_DMA_DCLK_CTRL__WRDMCLK_FORCEON__SHIFT: c_uint = 0x0;
pub const VCE_UENC_DMA_DCLK_CTRL__RDDMCLK_FORCEON_MASK: c_uint = 0x2;
pub const VCE_UENC_DMA_DCLK_CTRL__RDDMCLK_FORCEON__SHIFT: c_uint = 0x1;
pub const VCE_UENC_DMA_DCLK_CTRL__REGCLK_FORCEON_MASK: c_uint = 0x4;
pub const VCE_UENC_DMA_DCLK_CTRL__REGCLK_FORCEON__SHIFT: c_uint = 0x2;
pub const VCE_SYS_INT_EN__VCE_SYS_INT_TRAP_INTERRUPT_EN_MASK: c_uint = 0x8;
pub const VCE_SYS_INT_EN__VCE_SYS_INT_TRAP_INTERRUPT_EN__SHIFT: c_uint = 0x3;
pub const VCE_SYS_INT_STATUS__VCE_SYS_INT_TRAP_INTERRUPT_INT_MASK: c_uint = 0x8;
pub const VCE_SYS_INT_STATUS__VCE_SYS_INT_TRAP_INTERRUPT_INT__SHIFT: c_uint = 0x3;
pub const VCE_SYS_INT_ACK__VCE_SYS_INT_TRAP_INTERRUPT_ACK_MASK: c_uint = 0x8;
pub const VCE_SYS_INT_ACK__VCE_SYS_INT_TRAP_INTERRUPT_ACK__SHIFT: c_uint = 0x3;
pub const VCE_LMI_VCPU_CACHE_40BIT_BAR__BAR_MASK: c_uint = 0xffffffff;
pub const VCE_LMI_VCPU_CACHE_40BIT_BAR__BAR__SHIFT: c_uint = 0x0;
pub const VCE_LMI_CTRL2__STALL_ARB_UMC_MASK: c_uint = 0x100;
pub const VCE_LMI_CTRL2__STALL_ARB_UMC__SHIFT: c_uint = 0x8;
pub const VCE_LMI_SWAP_CNTL3__RD_MC_CID_SWAP_MASK: c_uint = 0x3;
pub const VCE_LMI_SWAP_CNTL3__RD_MC_CID_SWAP__SHIFT: c_uint = 0x0;
pub const VCE_LMI_CTRL__VCPU_DATA_COHERENCY_EN_MASK: c_uint = 0x200000;
pub const VCE_LMI_CTRL__VCPU_DATA_COHERENCY_EN__SHIFT: c_uint = 0x15;
pub const VCE_LMI_SWAP_CNTL__VCPU_W_MC_SWAP_MASK: c_uint = 0x3;
pub const VCE_LMI_SWAP_CNTL__VCPU_W_MC_SWAP__SHIFT: c_uint = 0x0;
pub const VCE_LMI_SWAP_CNTL__WR_MC_CID_SWAP_MASK: c_uint = 0x3ffc;
pub const VCE_LMI_SWAP_CNTL__WR_MC_CID_SWAP__SHIFT: c_uint = 0x2;
pub const VCE_LMI_SWAP_CNTL1__VCPU_R_MC_SWAP_MASK: c_uint = 0x3;
pub const VCE_LMI_SWAP_CNTL1__VCPU_R_MC_SWAP__SHIFT: c_uint = 0x0;
pub const VCE_LMI_SWAP_CNTL1__RD_MC_CID_SWAP_MASK: c_uint = 0x3ffc;
pub const VCE_LMI_SWAP_CNTL1__RD_MC_CID_SWAP__SHIFT: c_uint = 0x2;
pub const VCE_LMI_SWAP_CNTL2__WR_MC_CID_SWAP_MASK: c_uint = 0xff;
pub const VCE_LMI_SWAP_CNTL2__WR_MC_CID_SWAP__SHIFT: c_uint = 0x0;
pub const VCE_LMI_CACHE_CTRL__VCPU_EN_MASK: c_uint = 0x1;
pub const VCE_LMI_CACHE_CTRL__VCPU_EN__SHIFT: c_uint = 0x0;
