//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/umc/umc_6_1_2_sh_mask.h
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
// Copyright (C) 2020  Advanced Micro Devices, Inc.
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

// Macro flag: #define _umc_6_1_2_SH_MASK_HEADER
// UMCCH0_0_EccErrCntSel_ARCT
pub const UMCCH0_0_EccErrCntSel_ARCT__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_EccErrCntSel_ARCT__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH0_0_EccErrCntSel_ARCT__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH0_0_EccErrCntSel_ARCT__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_0_EccErrCntSel_ARCT__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH0_0_EccErrCntSel_ARCT__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH0_0_EccErrCnt_ARCT
pub const UMCCH0_0_EccErrCnt_ARCT__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_EccErrCnt_ARCT__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// MCA_UMC_UMC0_MCUMC_STATUST0_ARCT
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrorCode__SHIFT: c_uint = 0x0;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrorCodeExt__SHIFT: c_uint = 0x10;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV0__SHIFT: c_uint = 0x16;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrCoreId__SHIFT: c_uint = 0x20;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV1__SHIFT: c_uint = 0x26;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Scrub__SHIFT: c_uint = 0x28;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV2__SHIFT: c_uint = 0x29;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Poison__SHIFT: c_uint = 0x2b;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Deferred__SHIFT: c_uint = 0x2c;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__UECC__SHIFT: c_uint = 0x2d;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__CECC__SHIFT: c_uint = 0x2e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV3__SHIFT: c_uint = 0x2f;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Transparent__SHIFT: c_uint = 0x34;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__SyndV__SHIFT: c_uint = 0x35;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV4__SHIFT: c_uint = 0x36;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__TCC__SHIFT: c_uint = 0x37;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrCoreIdVal__SHIFT: c_uint = 0x38;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__PCC__SHIFT: c_uint = 0x39;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__AddrV__SHIFT: c_uint = 0x3a;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__MiscV__SHIFT: c_uint = 0x3b;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__En__SHIFT: c_uint = 0x3c;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__UC__SHIFT: c_uint = 0x3d;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Overflow__SHIFT: c_uint = 0x3e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Val__SHIFT: c_uint = 0x3f;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrorCode_MASK: c_uint = 0x000000000000FFFFL;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrorCodeExt_MASK: c_uint = 0x00000000003F0000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV0_MASK: c_uint = 0x00000000FFC00000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrCoreId_MASK: c_uint = 0x0000003F00000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV1_MASK: c_uint = 0x000000C000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Scrub_MASK: c_uint = 0x0000010000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV2_MASK: c_uint = 0x0000060000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Poison_MASK: c_uint = 0x0000080000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Deferred_MASK: c_uint = 0x0000100000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__UECC_MASK: c_uint = 0x0000200000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__CECC_MASK: c_uint = 0x0000400000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV3_MASK: c_uint = 0x000F800000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Transparent_MASK: c_uint = 0x0010000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__SyndV_MASK: c_uint = 0x0020000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__RESERV4_MASK: c_uint = 0x0040000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__TCC_MASK: c_uint = 0x0080000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__ErrCoreIdVal_MASK: c_uint = 0x0100000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__PCC_MASK: c_uint = 0x0200000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__AddrV_MASK: c_uint = 0x0400000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__MiscV_MASK: c_uint = 0x0800000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__En_MASK: c_uint = 0x1000000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__UC_MASK: c_uint = 0x2000000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Overflow_MASK: c_uint = 0x4000000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0_ARCT__Val_MASK: c_uint = 0x8000000000000000L;
// MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT
pub const MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT__ErrorAddr__SHIFT: c_uint = 0x0;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT__LSB__SHIFT: c_uint = 0x38;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT__Reserved__SHIFT: c_uint = 0x3e;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT__ErrorAddr_MASK: c_uint = 0x00FFFFFFFFFFFFFFL;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT__LSB_MASK: c_uint = 0x3F00000000000000L;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0_ARCT__Reserved_MASK: c_uint = 0xC000000000000000L;
