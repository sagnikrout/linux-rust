//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/vce/vce_4_0_default.h
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
// Copyright (C) 2017  Advanced Micro Devices, Inc.
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

// Macro flag: #define _vce_4_0_DEFAULT_HEADER
// addressBlock: vce0_vce_dec
pub const mmVCE_STATUS_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CNTL_DEFAULT: c_uint = 0x00200000;
pub const mmVCE_VCPU_CACHE_OFFSET0_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE0_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET1_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE1_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET4_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE4_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET5_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE5_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET6_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE6_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET7_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE7_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_OFFSET8_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_VCPU_CACHE_SIZE8_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_SOFT_RESET_DEFAULT: c_uint = 0x00000001;
pub const mmVCE_RB_BASE_LO2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_BASE_HI2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_SIZE2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_RPTR2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_WPTR2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_BASE_LO_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_BASE_HI_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_RPTR_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_WPTR_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_ARB_CTRL_DEFAULT: c_uint = 0x00010000;
pub const mmVCE_CLOCK_GATING_A_DEFAULT: c_uint = 0x00000040;
pub const mmVCE_CLOCK_GATING_B_DEFAULT: c_uint = 0x01ef0100;
pub const mmVCE_RB_BASE_LO3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_BASE_HI3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_SIZE3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_RPTR3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_RB_WPTR3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_SYS_INT_EN_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_SYS_INT_ACK_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_SYS_INT_STATUS_DEFAULT: c_uint = 0x00000000;
// addressBlock: vce0_ctl_dec
pub const mmVCE_UENC_CLOCK_GATING_DEFAULT: c_uint = 0xffc00040;
pub const mmVCE_UENC_REG_CLOCK_GATING_DEFAULT: c_uint = 0x000007ff;
pub const mmVCE_UENC_CLOCK_GATING_2_DEFAULT: c_uint = 0x00010000;
// addressBlock: vce0_vce_sclk_dec
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_CTRL2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_SWAP_CNTL3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_CTRL_DEFAULT: c_uint = 0x00104000;
pub const mmVCE_LMI_STATUS_DEFAULT: c_uint = 0x00003f7f;
pub const mmVCE_LMI_VM_CTRL_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_SWAP_CNTL_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_SWAP_CNTL1_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_SWAP_CNTL2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_CACHE_CTRL_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR0_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR1_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR4_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR5_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR6_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR7_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR0_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR1_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR2_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR3_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR4_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR5_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR6_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR7_DEFAULT: c_uint = 0x00000000;
// addressBlock: vce0_mmsch_dec
pub const mmVCE_MMSCH_VF_VMID_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_CTX_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_CTX_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_CTX_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_LO_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_HI_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_GPCOM_SIZE_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_MAILBOX_HOST_DEFAULT: c_uint = 0x00000000;
pub const mmVCE_MMSCH_VF_MAILBOX_RESP_DEFAULT: c_uint = 0x00000000;
// addressBlock: vce0_vce_rb_pg_dec
pub const mmVCE_HW_VERSION_DEFAULT: c_uint = 0x00000000;
