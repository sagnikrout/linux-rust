//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/umc/umc_8_10_0_sh_mask.h
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
// Copyright (C) 2022  Advanced Micro Devices, Inc.
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

// Macro flag: #define _umc_8_10_0_SH_MASK_HEADER
// UMCCH0_0_GeccErrCntSel
pub const UMCCH0_0_GeccErrCntSel__GeccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH0_0_GeccErrCntSel__GeccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH0_0_GeccErrCntSel__PoisonCntEn__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_GeccErrCntSel__GeccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH0_0_GeccErrCntSel__GeccErrCntEn_MASK: c_uint = 0x00008000L;
pub const UMCCH0_0_GeccErrCntSel__PoisonCntEn_MASK: c_uint = 0x00030000L;
// UMCCH0_0_GeccErrCnt
pub const UMCCH0_0_GeccErrCnt__GeccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_GeccErrCnt__GeccUnCorrErrCnt__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_GeccErrCnt__GeccErrCnt_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_GeccErrCnt__GeccUnCorrErrCnt_MASK: c_uint = 0xFFFF0000L;
// MCA_UMC_UMC0_MCUMC_STATUST0
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrorCode__SHIFT: c_uint = 0x0;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrorCodeExt__SHIFT: c_uint = 0x10;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV22__SHIFT: c_uint = 0x16;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__AddrLsb__SHIFT: c_uint = 0x18;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV30__SHIFT: c_uint = 0x1e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrCoreId__SHIFT: c_uint = 0x20;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV38__SHIFT: c_uint = 0x26;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Scrub__SHIFT: c_uint = 0x28;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV41__SHIFT: c_uint = 0x29;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Poison__SHIFT: c_uint = 0x2b;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Deferred__SHIFT: c_uint = 0x2c;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__UECC__SHIFT: c_uint = 0x2d;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__CECC__SHIFT: c_uint = 0x2e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV47__SHIFT: c_uint = 0x2f;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Transparent__SHIFT: c_uint = 0x34;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__SyndV__SHIFT: c_uint = 0x35;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV54__SHIFT: c_uint = 0x36;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__TCC__SHIFT: c_uint = 0x37;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrCoreIdVal__SHIFT: c_uint = 0x38;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__PCC__SHIFT: c_uint = 0x39;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__AddrV__SHIFT: c_uint = 0x3a;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__MiscV__SHIFT: c_uint = 0x3b;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__En__SHIFT: c_uint = 0x3c;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__UC__SHIFT: c_uint = 0x3d;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Overflow__SHIFT: c_uint = 0x3e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Val__SHIFT: c_uint = 0x3f;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrorCode_MASK: c_uint = 0x000000000000FFFFL;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrorCodeExt_MASK: c_uint = 0x00000000003F0000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV22_MASK: c_uint = 0x0000000000C00000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__AddrLsb_MASK: c_uint = 0x000000003F000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV30_MASK: c_uint = 0x00000000C0000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrCoreId_MASK: c_uint = 0x0000003F00000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV38_MASK: c_uint = 0x000000C000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Scrub_MASK: c_uint = 0x0000010000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV41_MASK: c_uint = 0x0000060000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Poison_MASK: c_uint = 0x0000080000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Deferred_MASK: c_uint = 0x0000100000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__UECC_MASK: c_uint = 0x0000200000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__CECC_MASK: c_uint = 0x0000400000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV47_MASK: c_uint = 0x000F800000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Transparent_MASK: c_uint = 0x0010000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__SyndV_MASK: c_uint = 0x0020000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV54_MASK: c_uint = 0x0040000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__TCC_MASK: c_uint = 0x0080000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrCoreIdVal_MASK: c_uint = 0x0100000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__PCC_MASK: c_uint = 0x0200000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__AddrV_MASK: c_uint = 0x0400000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__MiscV_MASK: c_uint = 0x0800000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__En_MASK: c_uint = 0x1000000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__UC_MASK: c_uint = 0x2000000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Overflow_MASK: c_uint = 0x4000000000000000L;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Val_MASK: c_uint = 0x8000000000000000L;
// MCA_UMC_UMC0_MCUMC_ADDRT0
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__ErrorAddr__SHIFT: c_uint = 0x0;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__Reserved__SHIFT: c_uint = 0x38;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__ErrorAddr_MASK: c_uint = 0x00FFFFFFFFFFFFFFL;
// UMCCH0_0_GeccCtrl
pub const UMCCH0_0_GeccCtrl__UCFatalEn__SHIFT: c_uint = 0xd;
pub const UMCCH0_0_GeccCtrl__UCFatalEn_MASK: c_uint = 0x00002000L;
