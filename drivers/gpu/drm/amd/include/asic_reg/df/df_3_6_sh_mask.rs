//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/df/df_3_6_sh_mask.h
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
// Copyright (C) 2018  Advanced Micro Devices, Inc.
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

// Macro flag: #define _df_3_6_SH_MASK_HEADER
// FabricConfigAccessControl
pub const FabricConfigAccessControl__CfgRegInstAccEn__SHIFT: c_uint = 0x0;
pub const FabricConfigAccessControl__CfgRegInstAccRegLock__SHIFT: c_uint = 0x1;
pub const FabricConfigAccessControl__CfgRegInstID__SHIFT: c_uint = 0x10;
pub const FabricConfigAccessControl__CfgRegInstAccEn_MASK: c_uint = 0x00000001L;
pub const FabricConfigAccessControl__CfgRegInstAccRegLock_MASK: c_uint = 0x00000002L;
pub const FabricConfigAccessControl__CfgRegInstID_MASK: c_uint = 0x00FF0000L;
// DF_PIE_AON0_DfGlobalClkGater
pub const DF_PIE_AON0_DfGlobalClkGater__MGCGMode__SHIFT: c_uint = 0x0;
pub const DF_PIE_AON0_DfGlobalClkGater__MGCGMode_MASK: c_uint = 0x0000000FL;
// DF_CS_UMC_AON0_DfGlobalCtrl
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl64K__SHIFT: c_uint = 0x14;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl2M__SHIFT: c_uint = 0x15;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl1G__SHIFT: c_uint = 0x16;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl64K_MASK: c_uint = 0x00100000L;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl2M_MASK: c_uint = 0x00200000L;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl1G_MASK: c_uint = 0x00400000L;
// DF_CS_AON0_DramBaseAddress0
pub const DF_CS_UMC_AON0_DramBaseAddress0__AddrRngVal__SHIFT: c_uint = 0x0;
pub const DF_CS_UMC_AON0_DramBaseAddress0__LgcyMmioHoleEn__SHIFT: c_uint = 0x1;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan__SHIFT: c_uint = 0x2;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvAddrSel__SHIFT: c_uint = 0x9;
pub const DF_CS_UMC_AON0_DramBaseAddress0__DramBaseAddr__SHIFT: c_uint = 0xc;
pub const DF_CS_UMC_AON0_DramBaseAddress0__AddrRngVal_MASK: c_uint = 0x00000001L;
pub const DF_CS_UMC_AON0_DramBaseAddress0__LgcyMmioHoleEn_MASK: c_uint = 0x00000002L;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan_MASK: c_uint = 0x0000003CL;
pub const ALDEBARAN_DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan_MASK: c_uint = 0x0000007CL;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvAddrSel_MASK: c_uint = 0x00000E00L;
pub const DF_CS_UMC_AON0_DramBaseAddress0__DramBaseAddr_MASK: c_uint = 0xFFFFF000L;
// DF_CS_UMC_AON0_DramLimitAddress0
pub const DF_CS_UMC_AON0_DramLimitAddress0__DstFabricID__SHIFT: c_uint = 0x0;
pub const DF_CS_UMC_AON0_DramLimitAddress0__AllowReqIO__SHIFT: c_uint = 0xa;
pub const DF_CS_UMC_AON0_DramLimitAddress0__DramLimitAddr__SHIFT: c_uint = 0xc;
pub const DF_CS_UMC_AON0_DramLimitAddress0__DstFabricID_MASK: c_uint = 0x000003FFL;
pub const DF_CS_UMC_AON0_DramLimitAddress0__AllowReqIO_MASK: c_uint = 0x00000400L;
pub const DF_CS_UMC_AON0_DramLimitAddress0__DramLimitAddr_MASK: c_uint = 0xFFFFF000L;
// DF_CS_UMC_AON0_HardwareAssertMaskLow
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk0__SHIFT: c_uint = 0x0;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk1__SHIFT: c_uint = 0x1;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk2__SHIFT: c_uint = 0x2;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk3__SHIFT: c_uint = 0x3;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk4__SHIFT: c_uint = 0x4;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk5__SHIFT: c_uint = 0x5;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk6__SHIFT: c_uint = 0x6;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk7__SHIFT: c_uint = 0x7;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk8__SHIFT: c_uint = 0x8;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk9__SHIFT: c_uint = 0x9;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk10__SHIFT: c_uint = 0xa;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk11__SHIFT: c_uint = 0xb;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk12__SHIFT: c_uint = 0xc;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk13__SHIFT: c_uint = 0xd;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk14__SHIFT: c_uint = 0xe;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk15__SHIFT: c_uint = 0xf;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk16__SHIFT: c_uint = 0x10;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk17__SHIFT: c_uint = 0x11;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk18__SHIFT: c_uint = 0x12;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk19__SHIFT: c_uint = 0x13;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk20__SHIFT: c_uint = 0x14;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk21__SHIFT: c_uint = 0x15;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk22__SHIFT: c_uint = 0x16;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk23__SHIFT: c_uint = 0x17;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk24__SHIFT: c_uint = 0x18;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk25__SHIFT: c_uint = 0x19;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk26__SHIFT: c_uint = 0x1a;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk27__SHIFT: c_uint = 0x1b;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk28__SHIFT: c_uint = 0x1c;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk29__SHIFT: c_uint = 0x1d;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk30__SHIFT: c_uint = 0x1e;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk31__SHIFT: c_uint = 0x1f;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk0_MASK: c_uint = 0x00000001L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk1_MASK: c_uint = 0x00000002L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk2_MASK: c_uint = 0x00000004L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk3_MASK: c_uint = 0x00000008L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk4_MASK: c_uint = 0x00000010L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk5_MASK: c_uint = 0x00000020L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk6_MASK: c_uint = 0x00000040L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk7_MASK: c_uint = 0x00000080L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk8_MASK: c_uint = 0x00000100L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk9_MASK: c_uint = 0x00000200L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk10_MASK: c_uint = 0x00000400L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk11_MASK: c_uint = 0x00000800L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk12_MASK: c_uint = 0x00001000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk13_MASK: c_uint = 0x00002000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk14_MASK: c_uint = 0x00004000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk15_MASK: c_uint = 0x00008000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk16_MASK: c_uint = 0x00010000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk17_MASK: c_uint = 0x00020000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk18_MASK: c_uint = 0x00040000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk19_MASK: c_uint = 0x00080000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk20_MASK: c_uint = 0x00100000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk21_MASK: c_uint = 0x00200000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk22_MASK: c_uint = 0x00400000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk23_MASK: c_uint = 0x00800000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk24_MASK: c_uint = 0x01000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk25_MASK: c_uint = 0x02000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk26_MASK: c_uint = 0x04000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk27_MASK: c_uint = 0x08000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk28_MASK: c_uint = 0x10000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk29_MASK: c_uint = 0x20000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk30_MASK: c_uint = 0x40000000L;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk31_MASK: c_uint = 0x80000000L;
// DF_NCS_PG0_HardwareAssertMaskHigh
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk0__SHIFT: c_uint = 0x0;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk1__SHIFT: c_uint = 0x1;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk2__SHIFT: c_uint = 0x2;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk3__SHIFT: c_uint = 0x3;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk4__SHIFT: c_uint = 0x4;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk5__SHIFT: c_uint = 0x5;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk6__SHIFT: c_uint = 0x6;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk7__SHIFT: c_uint = 0x7;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk8__SHIFT: c_uint = 0x8;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk9__SHIFT: c_uint = 0x9;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk10__SHIFT: c_uint = 0xa;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk11__SHIFT: c_uint = 0xb;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk12__SHIFT: c_uint = 0xc;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk13__SHIFT: c_uint = 0xd;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk14__SHIFT: c_uint = 0xe;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk15__SHIFT: c_uint = 0xf;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk16__SHIFT: c_uint = 0x10;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk17__SHIFT: c_uint = 0x11;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk18__SHIFT: c_uint = 0x12;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk19__SHIFT: c_uint = 0x13;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk20__SHIFT: c_uint = 0x14;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk21__SHIFT: c_uint = 0x15;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk22__SHIFT: c_uint = 0x16;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk23__SHIFT: c_uint = 0x17;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk24__SHIFT: c_uint = 0x18;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk25__SHIFT: c_uint = 0x19;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk26__SHIFT: c_uint = 0x1a;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk27__SHIFT: c_uint = 0x1b;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk28__SHIFT: c_uint = 0x1c;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk29__SHIFT: c_uint = 0x1d;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk30__SHIFT: c_uint = 0x1e;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk31__SHIFT: c_uint = 0x1f;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk0_MASK: c_uint = 0x00000001L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk1_MASK: c_uint = 0x00000002L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk2_MASK: c_uint = 0x00000004L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk3_MASK: c_uint = 0x00000008L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk4_MASK: c_uint = 0x00000010L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk5_MASK: c_uint = 0x00000020L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk6_MASK: c_uint = 0x00000040L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk7_MASK: c_uint = 0x00000080L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk8_MASK: c_uint = 0x00000100L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk9_MASK: c_uint = 0x00000200L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk10_MASK: c_uint = 0x00000400L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk11_MASK: c_uint = 0x00000800L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk12_MASK: c_uint = 0x00001000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk13_MASK: c_uint = 0x00002000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk14_MASK: c_uint = 0x00004000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk15_MASK: c_uint = 0x00008000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk16_MASK: c_uint = 0x00010000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk17_MASK: c_uint = 0x00020000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk18_MASK: c_uint = 0x00040000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk19_MASK: c_uint = 0x00080000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk20_MASK: c_uint = 0x00100000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk21_MASK: c_uint = 0x00200000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk22_MASK: c_uint = 0x00400000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk23_MASK: c_uint = 0x00800000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk24_MASK: c_uint = 0x01000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk25_MASK: c_uint = 0x02000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk26_MASK: c_uint = 0x04000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk27_MASK: c_uint = 0x08000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk28_MASK: c_uint = 0x10000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk29_MASK: c_uint = 0x20000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk30_MASK: c_uint = 0x40000000L;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk31_MASK: c_uint = 0x80000000L;
