//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/vce/vce_4_0_offset.h
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

// Macro flag: #define _vce_4_0_OFFSET_HEADER
// addressBlock: vce0_vce_dec
// base address: 0x22000
pub const mmVCE_STATUS: c_uint = 0x0a01;
pub const mmVCE_STATUS_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CNTL: c_uint = 0x0a05;
pub const mmVCE_VCPU_CNTL_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET0: c_uint = 0x0a09;
pub const mmVCE_VCPU_CACHE_OFFSET0_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE0: c_uint = 0x0a0a;
pub const mmVCE_VCPU_CACHE_SIZE0_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET1: c_uint = 0x0a0b;
pub const mmVCE_VCPU_CACHE_OFFSET1_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE1: c_uint = 0x0a0c;
pub const mmVCE_VCPU_CACHE_SIZE1_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET2: c_uint = 0x0a0d;
pub const mmVCE_VCPU_CACHE_OFFSET2_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE2: c_uint = 0x0a0e;
pub const mmVCE_VCPU_CACHE_SIZE2_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET3: c_uint = 0x0a0f;
pub const mmVCE_VCPU_CACHE_OFFSET3_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE3: c_uint = 0x0a10;
pub const mmVCE_VCPU_CACHE_SIZE3_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET4: c_uint = 0x0a11;
pub const mmVCE_VCPU_CACHE_OFFSET4_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE4: c_uint = 0x0a12;
pub const mmVCE_VCPU_CACHE_SIZE4_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET5: c_uint = 0x0a13;
pub const mmVCE_VCPU_CACHE_OFFSET5_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE5: c_uint = 0x0a14;
pub const mmVCE_VCPU_CACHE_SIZE5_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET6: c_uint = 0x0a15;
pub const mmVCE_VCPU_CACHE_OFFSET6_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE6: c_uint = 0x0a16;
pub const mmVCE_VCPU_CACHE_SIZE6_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET7: c_uint = 0x0a17;
pub const mmVCE_VCPU_CACHE_OFFSET7_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE7: c_uint = 0x0a18;
pub const mmVCE_VCPU_CACHE_SIZE7_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_OFFSET8: c_uint = 0x0a19;
pub const mmVCE_VCPU_CACHE_OFFSET8_BASE_IDX: c_int = 0;
pub const mmVCE_VCPU_CACHE_SIZE8: c_uint = 0x0a1a;
pub const mmVCE_VCPU_CACHE_SIZE8_BASE_IDX: c_int = 0;
pub const mmVCE_SOFT_RESET: c_uint = 0x0a48;
pub const mmVCE_SOFT_RESET_BASE_IDX: c_int = 0;
pub const mmVCE_RB_BASE_LO2: c_uint = 0x0a5b;
pub const mmVCE_RB_BASE_LO2_BASE_IDX: c_int = 0;
pub const mmVCE_RB_BASE_HI2: c_uint = 0x0a5c;
pub const mmVCE_RB_BASE_HI2_BASE_IDX: c_int = 0;
pub const mmVCE_RB_SIZE2: c_uint = 0x0a5d;
pub const mmVCE_RB_SIZE2_BASE_IDX: c_int = 0;
pub const mmVCE_RB_RPTR2: c_uint = 0x0a5e;
pub const mmVCE_RB_RPTR2_BASE_IDX: c_int = 0;
pub const mmVCE_RB_WPTR2: c_uint = 0x0a5f;
pub const mmVCE_RB_WPTR2_BASE_IDX: c_int = 0;
pub const mmVCE_RB_BASE_LO: c_uint = 0x0a60;
pub const mmVCE_RB_BASE_LO_BASE_IDX: c_int = 0;
pub const mmVCE_RB_BASE_HI: c_uint = 0x0a61;
pub const mmVCE_RB_BASE_HI_BASE_IDX: c_int = 0;
pub const mmVCE_RB_SIZE: c_uint = 0x0a62;
pub const mmVCE_RB_SIZE_BASE_IDX: c_int = 0;
pub const mmVCE_RB_RPTR: c_uint = 0x0a63;
pub const mmVCE_RB_RPTR_BASE_IDX: c_int = 0;
pub const mmVCE_RB_WPTR: c_uint = 0x0a64;
pub const mmVCE_RB_WPTR_BASE_IDX: c_int = 0;
pub const mmVCE_RB_ARB_CTRL: c_uint = 0x0a9f;
pub const mmVCE_RB_ARB_CTRL_BASE_IDX: c_int = 0;
pub const mmVCE_CLOCK_GATING_A: c_uint = 0x0abe;
pub const mmVCE_CLOCK_GATING_A_BASE_IDX: c_int = 0;
pub const mmVCE_CLOCK_GATING_B: c_uint = 0x0abf;
pub const mmVCE_CLOCK_GATING_B_BASE_IDX: c_int = 0;
pub const mmVCE_RB_BASE_LO3: c_uint = 0x0ad4;
pub const mmVCE_RB_BASE_LO3_BASE_IDX: c_int = 0;
pub const mmVCE_RB_BASE_HI3: c_uint = 0x0ad5;
pub const mmVCE_RB_BASE_HI3_BASE_IDX: c_int = 0;
pub const mmVCE_RB_SIZE3: c_uint = 0x0ad6;
pub const mmVCE_RB_SIZE3_BASE_IDX: c_int = 0;
pub const mmVCE_RB_RPTR3: c_uint = 0x0ad7;
pub const mmVCE_RB_RPTR3_BASE_IDX: c_int = 0;
pub const mmVCE_RB_WPTR3: c_uint = 0x0ad8;
pub const mmVCE_RB_WPTR3_BASE_IDX: c_int = 0;
pub const mmVCE_SYS_INT_EN: c_uint = 0x0b00;
pub const mmVCE_SYS_INT_EN_BASE_IDX: c_int = 0;
pub const mmVCE_SYS_INT_ACK: c_uint = 0x0b01;
pub const mmVCE_SYS_INT_ACK_BASE_IDX: c_int = 0;
pub const mmVCE_SYS_INT_STATUS: c_uint = 0x0b01;
pub const mmVCE_SYS_INT_STATUS_BASE_IDX: c_int = 0;
// addressBlock: vce0_ctl_dec
// base address: 0x22780
pub const mmVCE_UENC_CLOCK_GATING: c_uint = 0x0bef;
pub const mmVCE_UENC_CLOCK_GATING_BASE_IDX: c_int = 0;
pub const mmVCE_UENC_REG_CLOCK_GATING: c_uint = 0x0bf0;
pub const mmVCE_UENC_REG_CLOCK_GATING_BASE_IDX: c_int = 0;
pub const mmVCE_UENC_CLOCK_GATING_2: c_uint = 0x0c10;
pub const mmVCE_UENC_CLOCK_GATING_2_BASE_IDX: c_int = 0;
// addressBlock: vce0_vce_sclk_dec
// base address: 0x23700
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR: c_uint = 0x0fcc;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_CTRL2: c_uint = 0x0fcf;
pub const mmVCE_LMI_CTRL2_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_SWAP_CNTL3: c_uint = 0x0fd0;
pub const mmVCE_LMI_SWAP_CNTL3_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_CTRL: c_uint = 0x0fd6;
pub const mmVCE_LMI_CTRL_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_STATUS: c_uint = 0x0fd7;
pub const mmVCE_LMI_STATUS_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VM_CTRL: c_uint = 0x0fd8;
pub const mmVCE_LMI_VM_CTRL_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_SWAP_CNTL: c_uint = 0x0fdd;
pub const mmVCE_LMI_SWAP_CNTL_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_SWAP_CNTL1: c_uint = 0x0fde;
pub const mmVCE_LMI_SWAP_CNTL1_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_SWAP_CNTL2: c_uint = 0x0fe2;
pub const mmVCE_LMI_SWAP_CNTL2_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_CACHE_CTRL: c_uint = 0x0fec;
pub const mmVCE_LMI_CACHE_CTRL_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR0: c_uint = 0x1086;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR0_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR1: c_uint = 0x1087;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR1_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR2: c_uint = 0x1088;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR2_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR3: c_uint = 0x1089;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR3_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR4: c_uint = 0x108a;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR4_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR5: c_uint = 0x108b;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR5_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR6: c_uint = 0x108c;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR6_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR7: c_uint = 0x108d;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR7_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR0: c_uint = 0x1096;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR0_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR1: c_uint = 0x1097;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR1_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR2: c_uint = 0x1098;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR2_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR3: c_uint = 0x1099;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR3_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR4: c_uint = 0x109a;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR4_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR5: c_uint = 0x109b;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR5_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR6: c_uint = 0x109c;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR6_BASE_IDX: c_int = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR7: c_uint = 0x109d;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR7_BASE_IDX: c_int = 0;
// addressBlock: vce0_mmsch_dec
// base address: 0x23b00
pub const mmVCE_MMSCH_VF_VMID: c_uint = 0x10cb;
pub const mmVCE_MMSCH_VF_VMID_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_CTX_ADDR_LO: c_uint = 0x10cc;
pub const mmVCE_MMSCH_VF_CTX_ADDR_LO_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_CTX_ADDR_HI: c_uint = 0x10cd;
pub const mmVCE_MMSCH_VF_CTX_ADDR_HI_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_CTX_SIZE: c_uint = 0x10ce;
pub const mmVCE_MMSCH_VF_CTX_SIZE_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_LO: c_uint = 0x10cf;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_LO_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_HI: c_uint = 0x10d0;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_HI_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_GPCOM_SIZE: c_uint = 0x10d1;
pub const mmVCE_MMSCH_VF_GPCOM_SIZE_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_MAILBOX_HOST: c_uint = 0x10d2;
pub const mmVCE_MMSCH_VF_MAILBOX_HOST_BASE_IDX: c_int = 0;
pub const mmVCE_MMSCH_VF_MAILBOX_RESP: c_uint = 0x10d3;
pub const mmVCE_MMSCH_VF_MAILBOX_RESP_BASE_IDX: c_int = 0;
// addressBlock: vce0_vce_rb_pg_dec
// base address: 0x23fa0
pub const mmVCE_HW_VERSION: c_uint = 0x11e8;
pub const mmVCE_HW_VERSION_BASE_IDX: c_int = 0;
