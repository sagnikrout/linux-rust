//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/umc/umc_6_7_0_sh_mask.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

// Macro flag: #define _umc_6_7_0_SH_MASK_HEADER
// addressBlock: umc_w_phy_umc0_mca_ip_umc0_mca_map
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
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__Reserved_MASK: c_uint = 0xFF00000000000000L;
// addressBlock: umc_w_phy_umc0_umcch0_umcchdec
// UMCCH0_0_BaseAddrCS0
pub const UMCCH0_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_0_AddrMaskCS01
pub const UMCCH0_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_0_AddrSelCS01
pub const UMCCH0_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH0_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH0_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH0_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH0_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH0_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH0_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH0_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH0_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH0_0_AddrHashBank0
pub const UMCCH0_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_0_AddrHashBank1
pub const UMCCH0_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_0_AddrHashBank2
pub const UMCCH0_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_0_AddrHashBank3
pub const UMCCH0_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_0_AddrHashBank4
pub const UMCCH0_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_0_AddrHashBank5
pub const UMCCH0_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_0_UMC_CONFIG
pub const UMCCH0_0_UMC_CONFIG__DDR_TYPE__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_UMC_CONFIG__BurstLength__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_UMC_CONFIG__BurstCtrl__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_UMC_CONFIG__DramReady__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_UMC_CONFIG__DDR_TYPE_MASK: c_uint = 0x00000007L;
pub const UMCCH0_0_UMC_CONFIG__BurstLength_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_UMC_CONFIG__BurstCtrl_MASK: c_uint = 0x00000C00L;
pub const UMCCH0_0_UMC_CONFIG__DramReady_MASK: c_uint = 0x80000000L;
// UMCCH0_0_EccCtrl
pub const UMCCH0_0_EccCtrl__WrEccEn__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_EccCtrl__EccReplayEn__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_EccCtrl__UCFatalEn__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_EccCtrl__RdEccEn__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_EccCtrl__PoisonFatalDis__SHIFT: c_uint = 0xc;
pub const UMCCH0_0_EccCtrl__PoisonInhibit__SHIFT: c_uint = 0xd;
pub const UMCCH0_0_EccCtrl__WrEccEn_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_EccCtrl__EccReplayEn_MASK: c_uint = 0x00000002L;
pub const UMCCH0_0_EccCtrl__UCFatalEn_MASK: c_uint = 0x00000100L;
pub const UMCCH0_0_EccCtrl__RdEccEn_MASK: c_uint = 0x00000400L;
pub const UMCCH0_0_EccCtrl__PoisonFatalDis_MASK: c_uint = 0x00001000L;
pub const UMCCH0_0_EccCtrl__PoisonInhibit_MASK: c_uint = 0x00002000L;
// UMCCH0_0_UmcLocalCap
pub const UMCCH0_0_UmcLocalCap__EccDis__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_UmcLocalCap__Spare__SHIFT: c_uint = 0x1;
pub const UMCCH0_0_UmcLocalCap__WrDis__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_UmcLocalCap__EccDis_MASK: c_uint = 0x00000001L;
pub const UMCCH0_0_UmcLocalCap__Spare_MASK: c_uint = 0x0000003EL;
pub const UMCCH0_0_UmcLocalCap__WrDis_MASK: c_uint = 0x80000000L;
// UMCCH0_0_EccErrCntSel
pub const UMCCH0_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH0_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH0_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH0_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH0_0_EccErrCnt
pub const UMCCH0_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH0_0_PerfMonCtlClk
pub const UMCCH0_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH0_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH0_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH0_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH0_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH0_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH0_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH0_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH0_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtrClk_Lo
pub const UMCCH0_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtrClk_Hi
pub const UMCCH0_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH0_0_PerfMonCtl1
pub const UMCCH0_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr1_Lo
pub const UMCCH0_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr1_Hi
pub const UMCCH0_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl2
pub const UMCCH0_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr2_Lo
pub const UMCCH0_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr2_Hi
pub const UMCCH0_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl3
pub const UMCCH0_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr3_Lo
pub const UMCCH0_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr3_Hi
pub const UMCCH0_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl4
pub const UMCCH0_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr4_Lo
pub const UMCCH0_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr4_Hi
pub const UMCCH0_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl5
pub const UMCCH0_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr5_Lo
pub const UMCCH0_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr5_Hi
pub const UMCCH0_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl6
pub const UMCCH0_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr6_Lo
pub const UMCCH0_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr6_Hi
pub const UMCCH0_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl7
pub const UMCCH0_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr7_Lo
pub const UMCCH0_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr7_Hi
pub const UMCCH0_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_0_PerfMonCtl8
pub const UMCCH0_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_0_PerfMonCtr8_Lo
pub const UMCCH0_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_0_PerfMonCtr8_Hi
pub const UMCCH0_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch1_umcchdec
// UMCCH1_0_BaseAddrCS0
pub const UMCCH1_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_0_AddrMaskCS01
pub const UMCCH1_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_0_AddrSelCS01
pub const UMCCH1_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH1_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH1_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH1_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH1_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH1_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH1_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH1_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH1_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH1_0_AddrHashBank0
pub const UMCCH1_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_0_AddrHashBank1
pub const UMCCH1_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_0_AddrHashBank2
pub const UMCCH1_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_0_AddrHashBank3
pub const UMCCH1_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_0_AddrHashBank4
pub const UMCCH1_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_0_AddrHashBank5
pub const UMCCH1_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_0_UMC_CONFIG
pub const UMCCH1_0_UMC_CONFIG__DDR_TYPE__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_UMC_CONFIG__BurstLength__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_UMC_CONFIG__BurstCtrl__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_UMC_CONFIG__DramReady__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_UMC_CONFIG__DDR_TYPE_MASK: c_uint = 0x00000007L;
pub const UMCCH1_0_UMC_CONFIG__BurstLength_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_UMC_CONFIG__BurstCtrl_MASK: c_uint = 0x00000C00L;
pub const UMCCH1_0_UMC_CONFIG__DramReady_MASK: c_uint = 0x80000000L;
// UMCCH1_0_EccCtrl
pub const UMCCH1_0_EccCtrl__WrEccEn__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_EccCtrl__EccReplayEn__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_EccCtrl__UCFatalEn__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_EccCtrl__RdEccEn__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_EccCtrl__PoisonFatalDis__SHIFT: c_uint = 0xc;
pub const UMCCH1_0_EccCtrl__PoisonInhibit__SHIFT: c_uint = 0xd;
pub const UMCCH1_0_EccCtrl__WrEccEn_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_EccCtrl__EccReplayEn_MASK: c_uint = 0x00000002L;
pub const UMCCH1_0_EccCtrl__UCFatalEn_MASK: c_uint = 0x00000100L;
pub const UMCCH1_0_EccCtrl__RdEccEn_MASK: c_uint = 0x00000400L;
pub const UMCCH1_0_EccCtrl__PoisonFatalDis_MASK: c_uint = 0x00001000L;
pub const UMCCH1_0_EccCtrl__PoisonInhibit_MASK: c_uint = 0x00002000L;
// UMCCH1_0_UmcLocalCap
pub const UMCCH1_0_UmcLocalCap__EccDis__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_UmcLocalCap__Spare__SHIFT: c_uint = 0x1;
pub const UMCCH1_0_UmcLocalCap__WrDis__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_UmcLocalCap__EccDis_MASK: c_uint = 0x00000001L;
pub const UMCCH1_0_UmcLocalCap__Spare_MASK: c_uint = 0x0000003EL;
pub const UMCCH1_0_UmcLocalCap__WrDis_MASK: c_uint = 0x80000000L;
// UMCCH1_0_EccErrCntSel
pub const UMCCH1_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH1_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH1_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH1_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH1_0_EccErrCnt
pub const UMCCH1_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH1_0_PerfMonCtlClk
pub const UMCCH1_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH1_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH1_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH1_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH1_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH1_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH1_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH1_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH1_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtrClk_Lo
pub const UMCCH1_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtrClk_Hi
pub const UMCCH1_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH1_0_PerfMonCtl1
pub const UMCCH1_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr1_Lo
pub const UMCCH1_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr1_Hi
pub const UMCCH1_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl2
pub const UMCCH1_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr2_Lo
pub const UMCCH1_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr2_Hi
pub const UMCCH1_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl3
pub const UMCCH1_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr3_Lo
pub const UMCCH1_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr3_Hi
pub const UMCCH1_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl4
pub const UMCCH1_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr4_Lo
pub const UMCCH1_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr4_Hi
pub const UMCCH1_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl5
pub const UMCCH1_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr5_Lo
pub const UMCCH1_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr5_Hi
pub const UMCCH1_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl6
pub const UMCCH1_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr6_Lo
pub const UMCCH1_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr6_Hi
pub const UMCCH1_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl7
pub const UMCCH1_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr7_Lo
pub const UMCCH1_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr7_Hi
pub const UMCCH1_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_0_PerfMonCtl8
pub const UMCCH1_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_0_PerfMonCtr8_Lo
pub const UMCCH1_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_0_PerfMonCtr8_Hi
pub const UMCCH1_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch2_umcchdec
// UMCCH2_0_BaseAddrCS0
pub const UMCCH2_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_0_AddrMaskCS01
pub const UMCCH2_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_0_AddrSelCS01
pub const UMCCH2_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH2_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH2_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH2_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH2_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH2_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH2_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH2_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH2_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH2_0_AddrHashBank0
pub const UMCCH2_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_0_AddrHashBank1
pub const UMCCH2_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_0_AddrHashBank2
pub const UMCCH2_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_0_AddrHashBank3
pub const UMCCH2_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_0_AddrHashBank4
pub const UMCCH2_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_0_AddrHashBank5
pub const UMCCH2_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_0_UMC_CONFIG
pub const UMCCH2_0_UMC_CONFIG__DDR_TYPE__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_UMC_CONFIG__BurstLength__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_UMC_CONFIG__BurstCtrl__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_UMC_CONFIG__DramReady__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_UMC_CONFIG__DDR_TYPE_MASK: c_uint = 0x00000007L;
pub const UMCCH2_0_UMC_CONFIG__BurstLength_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_UMC_CONFIG__BurstCtrl_MASK: c_uint = 0x00000C00L;
pub const UMCCH2_0_UMC_CONFIG__DramReady_MASK: c_uint = 0x80000000L;
// UMCCH2_0_EccCtrl
pub const UMCCH2_0_EccCtrl__WrEccEn__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_EccCtrl__EccReplayEn__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_EccCtrl__UCFatalEn__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_EccCtrl__RdEccEn__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_EccCtrl__PoisonFatalDis__SHIFT: c_uint = 0xc;
pub const UMCCH2_0_EccCtrl__PoisonInhibit__SHIFT: c_uint = 0xd;
pub const UMCCH2_0_EccCtrl__WrEccEn_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_EccCtrl__EccReplayEn_MASK: c_uint = 0x00000002L;
pub const UMCCH2_0_EccCtrl__UCFatalEn_MASK: c_uint = 0x00000100L;
pub const UMCCH2_0_EccCtrl__RdEccEn_MASK: c_uint = 0x00000400L;
pub const UMCCH2_0_EccCtrl__PoisonFatalDis_MASK: c_uint = 0x00001000L;
pub const UMCCH2_0_EccCtrl__PoisonInhibit_MASK: c_uint = 0x00002000L;
// UMCCH2_0_UmcLocalCap
pub const UMCCH2_0_UmcLocalCap__EccDis__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_UmcLocalCap__Spare__SHIFT: c_uint = 0x1;
pub const UMCCH2_0_UmcLocalCap__WrDis__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_UmcLocalCap__EccDis_MASK: c_uint = 0x00000001L;
pub const UMCCH2_0_UmcLocalCap__Spare_MASK: c_uint = 0x0000003EL;
pub const UMCCH2_0_UmcLocalCap__WrDis_MASK: c_uint = 0x80000000L;
// UMCCH2_0_EccErrCntSel
pub const UMCCH2_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH2_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH2_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH2_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH2_0_EccErrCnt
pub const UMCCH2_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH2_0_PerfMonCtlClk
pub const UMCCH2_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH2_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH2_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH2_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH2_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH2_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH2_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH2_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH2_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtrClk_Lo
pub const UMCCH2_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtrClk_Hi
pub const UMCCH2_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH2_0_PerfMonCtl1
pub const UMCCH2_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr1_Lo
pub const UMCCH2_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr1_Hi
pub const UMCCH2_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl2
pub const UMCCH2_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr2_Lo
pub const UMCCH2_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr2_Hi
pub const UMCCH2_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl3
pub const UMCCH2_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr3_Lo
pub const UMCCH2_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr3_Hi
pub const UMCCH2_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl4
pub const UMCCH2_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr4_Lo
pub const UMCCH2_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr4_Hi
pub const UMCCH2_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl5
pub const UMCCH2_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr5_Lo
pub const UMCCH2_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr5_Hi
pub const UMCCH2_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl6
pub const UMCCH2_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr6_Lo
pub const UMCCH2_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr6_Hi
pub const UMCCH2_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl7
pub const UMCCH2_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr7_Lo
pub const UMCCH2_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr7_Hi
pub const UMCCH2_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_0_PerfMonCtl8
pub const UMCCH2_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_0_PerfMonCtr8_Lo
pub const UMCCH2_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_0_PerfMonCtr8_Hi
pub const UMCCH2_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch3_umcchdec
// UMCCH3_0_BaseAddrCS0
pub const UMCCH3_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_0_AddrMaskCS01
pub const UMCCH3_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_0_AddrSelCS01
pub const UMCCH3_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH3_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH3_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH3_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH3_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH3_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH3_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH3_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH3_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH3_0_AddrHashBank0
pub const UMCCH3_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_0_AddrHashBank1
pub const UMCCH3_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_0_AddrHashBank2
pub const UMCCH3_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_0_AddrHashBank3
pub const UMCCH3_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_0_AddrHashBank4
pub const UMCCH3_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_0_AddrHashBank5
pub const UMCCH3_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_0_UMC_CONFIG
pub const UMCCH3_0_UMC_CONFIG__DDR_TYPE__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_UMC_CONFIG__BurstLength__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_UMC_CONFIG__BurstCtrl__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_UMC_CONFIG__DramReady__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_UMC_CONFIG__DDR_TYPE_MASK: c_uint = 0x00000007L;
pub const UMCCH3_0_UMC_CONFIG__BurstLength_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_UMC_CONFIG__BurstCtrl_MASK: c_uint = 0x00000C00L;
pub const UMCCH3_0_UMC_CONFIG__DramReady_MASK: c_uint = 0x80000000L;
// UMCCH3_0_EccCtrl
pub const UMCCH3_0_EccCtrl__WrEccEn__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_EccCtrl__EccReplayEn__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_EccCtrl__UCFatalEn__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_EccCtrl__RdEccEn__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_EccCtrl__PoisonFatalDis__SHIFT: c_uint = 0xc;
pub const UMCCH3_0_EccCtrl__PoisonInhibit__SHIFT: c_uint = 0xd;
pub const UMCCH3_0_EccCtrl__WrEccEn_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_EccCtrl__EccReplayEn_MASK: c_uint = 0x00000002L;
pub const UMCCH3_0_EccCtrl__UCFatalEn_MASK: c_uint = 0x00000100L;
pub const UMCCH3_0_EccCtrl__RdEccEn_MASK: c_uint = 0x00000400L;
pub const UMCCH3_0_EccCtrl__PoisonFatalDis_MASK: c_uint = 0x00001000L;
pub const UMCCH3_0_EccCtrl__PoisonInhibit_MASK: c_uint = 0x00002000L;
// UMCCH3_0_UmcLocalCap
pub const UMCCH3_0_UmcLocalCap__EccDis__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_UmcLocalCap__Spare__SHIFT: c_uint = 0x1;
pub const UMCCH3_0_UmcLocalCap__WrDis__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_UmcLocalCap__EccDis_MASK: c_uint = 0x00000001L;
pub const UMCCH3_0_UmcLocalCap__Spare_MASK: c_uint = 0x0000003EL;
pub const UMCCH3_0_UmcLocalCap__WrDis_MASK: c_uint = 0x80000000L;
// UMCCH3_0_EccErrCntSel
pub const UMCCH3_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH3_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH3_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH3_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH3_0_EccErrCnt
pub const UMCCH3_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH3_0_PerfMonCtlClk
pub const UMCCH3_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH3_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH3_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH3_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH3_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH3_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH3_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH3_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH3_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtrClk_Lo
pub const UMCCH3_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtrClk_Hi
pub const UMCCH3_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH3_0_PerfMonCtl1
pub const UMCCH3_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr1_Lo
pub const UMCCH3_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr1_Hi
pub const UMCCH3_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl2
pub const UMCCH3_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr2_Lo
pub const UMCCH3_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr2_Hi
pub const UMCCH3_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl3
pub const UMCCH3_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr3_Lo
pub const UMCCH3_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr3_Hi
pub const UMCCH3_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl4
pub const UMCCH3_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr4_Lo
pub const UMCCH3_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr4_Hi
pub const UMCCH3_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl5
pub const UMCCH3_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr5_Lo
pub const UMCCH3_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr5_Hi
pub const UMCCH3_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl6
pub const UMCCH3_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr6_Lo
pub const UMCCH3_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr6_Hi
pub const UMCCH3_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl7
pub const UMCCH3_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr7_Lo
pub const UMCCH3_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr7_Hi
pub const UMCCH3_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_0_PerfMonCtl8
pub const UMCCH3_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_0_PerfMonCtr8_Lo
pub const UMCCH3_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_0_PerfMonCtr8_Hi
pub const UMCCH3_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch4_umcchdec
// UMCCH4_0_BaseAddrCS0
pub const UMCCH4_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_0_AddrMaskCS01
pub const UMCCH4_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_0_AddrSelCS01
pub const UMCCH4_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH4_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH4_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH4_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH4_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH4_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH4_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH4_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH4_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH4_0_AddrHashBank0
pub const UMCCH4_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_0_AddrHashBank1
pub const UMCCH4_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_0_AddrHashBank2
pub const UMCCH4_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_0_AddrHashBank3
pub const UMCCH4_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_0_AddrHashBank4
pub const UMCCH4_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_0_AddrHashBank5
pub const UMCCH4_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_0_EccErrCntSel
pub const UMCCH4_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH4_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH4_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH4_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH4_0_EccErrCnt
pub const UMCCH4_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH4_0_PerfMonCtlClk
pub const UMCCH4_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH4_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH4_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH4_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH4_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH4_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH4_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH4_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH4_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtrClk_Lo
pub const UMCCH4_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtrClk_Hi
pub const UMCCH4_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH4_0_PerfMonCtl1
pub const UMCCH4_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr1_Lo
pub const UMCCH4_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr1_Hi
pub const UMCCH4_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl2
pub const UMCCH4_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr2_Lo
pub const UMCCH4_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr2_Hi
pub const UMCCH4_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl3
pub const UMCCH4_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr3_Lo
pub const UMCCH4_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr3_Hi
pub const UMCCH4_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl4
pub const UMCCH4_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr4_Lo
pub const UMCCH4_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr4_Hi
pub const UMCCH4_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl5
pub const UMCCH4_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr5_Lo
pub const UMCCH4_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr5_Hi
pub const UMCCH4_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl6
pub const UMCCH4_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr6_Lo
pub const UMCCH4_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr6_Hi
pub const UMCCH4_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl7
pub const UMCCH4_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr7_Lo
pub const UMCCH4_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr7_Hi
pub const UMCCH4_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_0_PerfMonCtl8
pub const UMCCH4_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_0_PerfMonCtr8_Lo
pub const UMCCH4_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_0_PerfMonCtr8_Hi
pub const UMCCH4_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch5_umcchdec
// UMCCH5_0_BaseAddrCS0
pub const UMCCH5_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_0_AddrMaskCS01
pub const UMCCH5_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_0_AddrSelCS01
pub const UMCCH5_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH5_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH5_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH5_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH5_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH5_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH5_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH5_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH5_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH5_0_AddrHashBank0
pub const UMCCH5_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_0_AddrHashBank1
pub const UMCCH5_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_0_AddrHashBank2
pub const UMCCH5_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_0_AddrHashBank3
pub const UMCCH5_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_0_AddrHashBank4
pub const UMCCH5_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_0_AddrHashBank5
pub const UMCCH5_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_0_EccErrCntSel
pub const UMCCH5_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH5_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH5_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH5_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH5_0_EccErrCnt
pub const UMCCH5_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH5_0_PerfMonCtlClk
pub const UMCCH5_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH5_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH5_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH5_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH5_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH5_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH5_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH5_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH5_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtrClk_Lo
pub const UMCCH5_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtrClk_Hi
pub const UMCCH5_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH5_0_PerfMonCtl1
pub const UMCCH5_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr1_Lo
pub const UMCCH5_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr1_Hi
pub const UMCCH5_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl2
pub const UMCCH5_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr2_Lo
pub const UMCCH5_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr2_Hi
pub const UMCCH5_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl3
pub const UMCCH5_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr3_Lo
pub const UMCCH5_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr3_Hi
pub const UMCCH5_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl4
pub const UMCCH5_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr4_Lo
pub const UMCCH5_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr4_Hi
pub const UMCCH5_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl5
pub const UMCCH5_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr5_Lo
pub const UMCCH5_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr5_Hi
pub const UMCCH5_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl6
pub const UMCCH5_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr6_Lo
pub const UMCCH5_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr6_Hi
pub const UMCCH5_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl7
pub const UMCCH5_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr7_Lo
pub const UMCCH5_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr7_Hi
pub const UMCCH5_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_0_PerfMonCtl8
pub const UMCCH5_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_0_PerfMonCtr8_Lo
pub const UMCCH5_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_0_PerfMonCtr8_Hi
pub const UMCCH5_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch6_umcchdec
// UMCCH6_0_BaseAddrCS0
pub const UMCCH6_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_0_AddrMaskCS01
pub const UMCCH6_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_0_AddrSelCS01
pub const UMCCH6_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH6_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH6_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH6_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH6_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH6_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH6_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH6_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH6_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH6_0_AddrHashBank0
pub const UMCCH6_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_0_AddrHashBank1
pub const UMCCH6_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_0_AddrHashBank2
pub const UMCCH6_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_0_AddrHashBank3
pub const UMCCH6_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_0_AddrHashBank4
pub const UMCCH6_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_0_AddrHashBank5
pub const UMCCH6_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_0_EccErrCntSel
pub const UMCCH6_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH6_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH6_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH6_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH6_0_EccErrCnt
pub const UMCCH6_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH6_0_PerfMonCtlClk
pub const UMCCH6_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH6_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH6_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH6_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH6_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH6_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH6_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH6_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH6_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtrClk_Lo
pub const UMCCH6_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtrClk_Hi
pub const UMCCH6_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH6_0_PerfMonCtl1
pub const UMCCH6_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr1_Lo
pub const UMCCH6_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr1_Hi
pub const UMCCH6_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl2
pub const UMCCH6_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr2_Lo
pub const UMCCH6_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr2_Hi
pub const UMCCH6_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl3
pub const UMCCH6_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr3_Lo
pub const UMCCH6_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr3_Hi
pub const UMCCH6_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl4
pub const UMCCH6_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr4_Lo
pub const UMCCH6_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr4_Hi
pub const UMCCH6_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl5
pub const UMCCH6_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr5_Lo
pub const UMCCH6_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr5_Hi
pub const UMCCH6_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl6
pub const UMCCH6_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr6_Lo
pub const UMCCH6_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr6_Hi
pub const UMCCH6_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl7
pub const UMCCH6_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr7_Lo
pub const UMCCH6_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr7_Hi
pub const UMCCH6_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_0_PerfMonCtl8
pub const UMCCH6_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_0_PerfMonCtr8_Lo
pub const UMCCH6_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_0_PerfMonCtr8_Hi
pub const UMCCH6_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc0_umcch7_umcchdec
// UMCCH7_0_BaseAddrCS0
pub const UMCCH7_0_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_0_AddrMaskCS01
pub const UMCCH7_0_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_0_AddrSelCS01
pub const UMCCH7_0_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH7_0_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH7_0_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH7_0_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_0_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH7_0_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH7_0_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH7_0_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH7_0_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH7_0_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH7_0_AddrHashBank0
pub const UMCCH7_0_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_0_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_0_AddrHashBank1
pub const UMCCH7_0_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_0_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_0_AddrHashBank2
pub const UMCCH7_0_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_0_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_0_AddrHashBank3
pub const UMCCH7_0_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_0_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_0_AddrHashBank4
pub const UMCCH7_0_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_0_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_0_AddrHashBank5
pub const UMCCH7_0_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_0_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_0_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_0_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_0_EccErrCntSel
pub const UMCCH7_0_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH7_0_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH7_0_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_0_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH7_0_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH7_0_EccErrCnt
pub const UMCCH7_0_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH7_0_PerfMonCtlClk
pub const UMCCH7_0_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH7_0_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH7_0_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH7_0_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH7_0_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH7_0_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH7_0_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH7_0_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH7_0_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtrClk_Lo
pub const UMCCH7_0_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtrClk_Hi
pub const UMCCH7_0_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH7_0_PerfMonCtl1
pub const UMCCH7_0_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr1_Lo
pub const UMCCH7_0_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr1_Hi
pub const UMCCH7_0_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl2
pub const UMCCH7_0_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr2_Lo
pub const UMCCH7_0_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr2_Hi
pub const UMCCH7_0_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl3
pub const UMCCH7_0_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr3_Lo
pub const UMCCH7_0_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr3_Hi
pub const UMCCH7_0_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl4
pub const UMCCH7_0_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr4_Lo
pub const UMCCH7_0_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr4_Hi
pub const UMCCH7_0_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl5
pub const UMCCH7_0_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr5_Lo
pub const UMCCH7_0_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr5_Hi
pub const UMCCH7_0_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl6
pub const UMCCH7_0_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr6_Lo
pub const UMCCH7_0_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr6_Hi
pub const UMCCH7_0_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl7
pub const UMCCH7_0_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr7_Lo
pub const UMCCH7_0_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr7_Hi
pub const UMCCH7_0_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_0_PerfMonCtl8
pub const UMCCH7_0_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_0_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_0_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_0_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_0_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_0_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_0_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_0_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_0_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_0_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_0_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_0_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_0_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_0_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_0_PerfMonCtr8_Lo
pub const UMCCH7_0_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_0_PerfMonCtr8_Hi
pub const UMCCH7_0_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_0_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_0_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_0_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_0_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_0_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch0_umcchdec
// UMCCH0_1_BaseAddrCS0
pub const UMCCH0_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_1_AddrMaskCS01
pub const UMCCH0_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_1_AddrSelCS01
pub const UMCCH0_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH0_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH0_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH0_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH0_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH0_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH0_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH0_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH0_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH0_1_AddrHashBank0
pub const UMCCH0_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_1_AddrHashBank1
pub const UMCCH0_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_1_AddrHashBank2
pub const UMCCH0_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_1_AddrHashBank3
pub const UMCCH0_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_1_AddrHashBank4
pub const UMCCH0_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_1_AddrHashBank5
pub const UMCCH0_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_1_EccErrCntSel
pub const UMCCH0_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH0_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH0_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH0_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH0_1_EccErrCnt
pub const UMCCH0_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH0_1_PerfMonCtlClk
pub const UMCCH0_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH0_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH0_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH0_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH0_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH0_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH0_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH0_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH0_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtrClk_Lo
pub const UMCCH0_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtrClk_Hi
pub const UMCCH0_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH0_1_PerfMonCtl1
pub const UMCCH0_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr1_Lo
pub const UMCCH0_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr1_Hi
pub const UMCCH0_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl2
pub const UMCCH0_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr2_Lo
pub const UMCCH0_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr2_Hi
pub const UMCCH0_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl3
pub const UMCCH0_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr3_Lo
pub const UMCCH0_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr3_Hi
pub const UMCCH0_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl4
pub const UMCCH0_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr4_Lo
pub const UMCCH0_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr4_Hi
pub const UMCCH0_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl5
pub const UMCCH0_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr5_Lo
pub const UMCCH0_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr5_Hi
pub const UMCCH0_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl6
pub const UMCCH0_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr6_Lo
pub const UMCCH0_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr6_Hi
pub const UMCCH0_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl7
pub const UMCCH0_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr7_Lo
pub const UMCCH0_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr7_Hi
pub const UMCCH0_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_1_PerfMonCtl8
pub const UMCCH0_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_1_PerfMonCtr8_Lo
pub const UMCCH0_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_1_PerfMonCtr8_Hi
pub const UMCCH0_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch1_umcchdec
// UMCCH1_1_BaseAddrCS0
pub const UMCCH1_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_1_AddrMaskCS01
pub const UMCCH1_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_1_AddrSelCS01
pub const UMCCH1_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH1_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH1_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH1_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH1_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH1_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH1_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH1_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH1_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH1_1_AddrHashBank0
pub const UMCCH1_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_1_AddrHashBank1
pub const UMCCH1_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_1_AddrHashBank2
pub const UMCCH1_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_1_AddrHashBank3
pub const UMCCH1_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_1_AddrHashBank4
pub const UMCCH1_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_1_AddrHashBank5
pub const UMCCH1_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_1_EccErrCntSel
pub const UMCCH1_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH1_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH1_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH1_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH1_1_EccErrCnt
pub const UMCCH1_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH1_1_PerfMonCtlClk
pub const UMCCH1_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH1_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH1_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH1_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH1_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH1_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH1_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH1_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH1_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtrClk_Lo
pub const UMCCH1_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtrClk_Hi
pub const UMCCH1_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH1_1_PerfMonCtl1
pub const UMCCH1_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr1_Lo
pub const UMCCH1_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr1_Hi
pub const UMCCH1_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl2
pub const UMCCH1_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr2_Lo
pub const UMCCH1_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr2_Hi
pub const UMCCH1_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl3
pub const UMCCH1_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr3_Lo
pub const UMCCH1_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr3_Hi
pub const UMCCH1_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl4
pub const UMCCH1_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr4_Lo
pub const UMCCH1_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr4_Hi
pub const UMCCH1_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl5
pub const UMCCH1_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr5_Lo
pub const UMCCH1_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr5_Hi
pub const UMCCH1_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl6
pub const UMCCH1_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr6_Lo
pub const UMCCH1_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr6_Hi
pub const UMCCH1_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl7
pub const UMCCH1_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr7_Lo
pub const UMCCH1_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr7_Hi
pub const UMCCH1_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_1_PerfMonCtl8
pub const UMCCH1_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_1_PerfMonCtr8_Lo
pub const UMCCH1_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_1_PerfMonCtr8_Hi
pub const UMCCH1_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch2_umcchdec
// UMCCH2_1_BaseAddrCS0
pub const UMCCH2_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_1_AddrMaskCS01
pub const UMCCH2_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_1_AddrSelCS01
pub const UMCCH2_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH2_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH2_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH2_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH2_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH2_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH2_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH2_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH2_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH2_1_AddrHashBank0
pub const UMCCH2_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_1_AddrHashBank1
pub const UMCCH2_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_1_AddrHashBank2
pub const UMCCH2_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_1_AddrHashBank3
pub const UMCCH2_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_1_AddrHashBank4
pub const UMCCH2_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_1_AddrHashBank5
pub const UMCCH2_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_1_EccErrCntSel
pub const UMCCH2_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH2_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH2_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH2_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH2_1_EccErrCnt
pub const UMCCH2_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH2_1_PerfMonCtlClk
pub const UMCCH2_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH2_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH2_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH2_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH2_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH2_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH2_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH2_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH2_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtrClk_Lo
pub const UMCCH2_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtrClk_Hi
pub const UMCCH2_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH2_1_PerfMonCtl1
pub const UMCCH2_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr1_Lo
pub const UMCCH2_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr1_Hi
pub const UMCCH2_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl2
pub const UMCCH2_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr2_Lo
pub const UMCCH2_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr2_Hi
pub const UMCCH2_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl3
pub const UMCCH2_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr3_Lo
pub const UMCCH2_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr3_Hi
pub const UMCCH2_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl4
pub const UMCCH2_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr4_Lo
pub const UMCCH2_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr4_Hi
pub const UMCCH2_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl5
pub const UMCCH2_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr5_Lo
pub const UMCCH2_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr5_Hi
pub const UMCCH2_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl6
pub const UMCCH2_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr6_Lo
pub const UMCCH2_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr6_Hi
pub const UMCCH2_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl7
pub const UMCCH2_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr7_Lo
pub const UMCCH2_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr7_Hi
pub const UMCCH2_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_1_PerfMonCtl8
pub const UMCCH2_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_1_PerfMonCtr8_Lo
pub const UMCCH2_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_1_PerfMonCtr8_Hi
pub const UMCCH2_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch3_umcchdec
// UMCCH3_1_BaseAddrCS0
pub const UMCCH3_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_1_AddrMaskCS01
pub const UMCCH3_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_1_AddrSelCS01
pub const UMCCH3_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH3_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH3_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH3_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH3_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH3_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH3_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH3_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH3_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH3_1_AddrHashBank0
pub const UMCCH3_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_1_AddrHashBank1
pub const UMCCH3_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_1_AddrHashBank2
pub const UMCCH3_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_1_AddrHashBank3
pub const UMCCH3_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_1_AddrHashBank4
pub const UMCCH3_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_1_AddrHashBank5
pub const UMCCH3_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_1_EccErrCntSel
pub const UMCCH3_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH3_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH3_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH3_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH3_1_EccErrCnt
pub const UMCCH3_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH3_1_PerfMonCtlClk
pub const UMCCH3_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH3_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH3_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH3_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH3_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH3_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH3_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH3_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH3_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtrClk_Lo
pub const UMCCH3_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtrClk_Hi
pub const UMCCH3_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH3_1_PerfMonCtl1
pub const UMCCH3_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr1_Lo
pub const UMCCH3_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr1_Hi
pub const UMCCH3_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl2
pub const UMCCH3_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr2_Lo
pub const UMCCH3_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr2_Hi
pub const UMCCH3_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl3
pub const UMCCH3_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr3_Lo
pub const UMCCH3_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr3_Hi
pub const UMCCH3_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl4
pub const UMCCH3_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr4_Lo
pub const UMCCH3_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr4_Hi
pub const UMCCH3_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl5
pub const UMCCH3_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr5_Lo
pub const UMCCH3_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr5_Hi
pub const UMCCH3_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl6
pub const UMCCH3_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr6_Lo
pub const UMCCH3_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr6_Hi
pub const UMCCH3_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl7
pub const UMCCH3_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr7_Lo
pub const UMCCH3_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr7_Hi
pub const UMCCH3_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_1_PerfMonCtl8
pub const UMCCH3_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_1_PerfMonCtr8_Lo
pub const UMCCH3_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_1_PerfMonCtr8_Hi
pub const UMCCH3_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch4_umcchdec
// UMCCH4_1_BaseAddrCS0
pub const UMCCH4_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_1_AddrMaskCS01
pub const UMCCH4_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_1_AddrSelCS01
pub const UMCCH4_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH4_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH4_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH4_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH4_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH4_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH4_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH4_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH4_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH4_1_AddrHashBank0
pub const UMCCH4_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_1_AddrHashBank1
pub const UMCCH4_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_1_AddrHashBank2
pub const UMCCH4_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_1_AddrHashBank3
pub const UMCCH4_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_1_AddrHashBank4
pub const UMCCH4_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_1_AddrHashBank5
pub const UMCCH4_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_1_EccErrCntSel
pub const UMCCH4_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH4_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH4_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH4_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH4_1_EccErrCnt
pub const UMCCH4_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH4_1_PerfMonCtlClk
pub const UMCCH4_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH4_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH4_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH4_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH4_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH4_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH4_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH4_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH4_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtrClk_Lo
pub const UMCCH4_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtrClk_Hi
pub const UMCCH4_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH4_1_PerfMonCtl1
pub const UMCCH4_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr1_Lo
pub const UMCCH4_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr1_Hi
pub const UMCCH4_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl2
pub const UMCCH4_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr2_Lo
pub const UMCCH4_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr2_Hi
pub const UMCCH4_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl3
pub const UMCCH4_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr3_Lo
pub const UMCCH4_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr3_Hi
pub const UMCCH4_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl4
pub const UMCCH4_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr4_Lo
pub const UMCCH4_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr4_Hi
pub const UMCCH4_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl5
pub const UMCCH4_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr5_Lo
pub const UMCCH4_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr5_Hi
pub const UMCCH4_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl6
pub const UMCCH4_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr6_Lo
pub const UMCCH4_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr6_Hi
pub const UMCCH4_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl7
pub const UMCCH4_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr7_Lo
pub const UMCCH4_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr7_Hi
pub const UMCCH4_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_1_PerfMonCtl8
pub const UMCCH4_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_1_PerfMonCtr8_Lo
pub const UMCCH4_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_1_PerfMonCtr8_Hi
pub const UMCCH4_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch5_umcchdec
// UMCCH5_1_BaseAddrCS0
pub const UMCCH5_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_1_AddrMaskCS01
pub const UMCCH5_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_1_AddrSelCS01
pub const UMCCH5_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH5_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH5_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH5_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH5_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH5_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH5_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH5_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH5_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH5_1_AddrHashBank0
pub const UMCCH5_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_1_AddrHashBank1
pub const UMCCH5_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_1_AddrHashBank2
pub const UMCCH5_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_1_AddrHashBank3
pub const UMCCH5_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_1_AddrHashBank4
pub const UMCCH5_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_1_AddrHashBank5
pub const UMCCH5_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_1_EccErrCntSel
pub const UMCCH5_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH5_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH5_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH5_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH5_1_EccErrCnt
pub const UMCCH5_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH5_1_PerfMonCtlClk
pub const UMCCH5_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH5_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH5_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH5_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH5_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH5_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH5_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH5_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH5_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtrClk_Lo
pub const UMCCH5_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtrClk_Hi
pub const UMCCH5_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH5_1_PerfMonCtl1
pub const UMCCH5_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr1_Lo
pub const UMCCH5_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr1_Hi
pub const UMCCH5_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl2
pub const UMCCH5_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr2_Lo
pub const UMCCH5_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr2_Hi
pub const UMCCH5_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl3
pub const UMCCH5_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr3_Lo
pub const UMCCH5_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr3_Hi
pub const UMCCH5_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl4
pub const UMCCH5_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr4_Lo
pub const UMCCH5_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr4_Hi
pub const UMCCH5_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl5
pub const UMCCH5_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr5_Lo
pub const UMCCH5_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr5_Hi
pub const UMCCH5_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl6
pub const UMCCH5_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr6_Lo
pub const UMCCH5_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr6_Hi
pub const UMCCH5_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl7
pub const UMCCH5_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr7_Lo
pub const UMCCH5_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr7_Hi
pub const UMCCH5_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_1_PerfMonCtl8
pub const UMCCH5_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_1_PerfMonCtr8_Lo
pub const UMCCH5_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_1_PerfMonCtr8_Hi
pub const UMCCH5_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch6_umcchdec
// UMCCH6_1_BaseAddrCS0
pub const UMCCH6_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_1_AddrMaskCS01
pub const UMCCH6_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_1_AddrSelCS01
pub const UMCCH6_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH6_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH6_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH6_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH6_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH6_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH6_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH6_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH6_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH6_1_AddrHashBank0
pub const UMCCH6_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_1_AddrHashBank1
pub const UMCCH6_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_1_AddrHashBank2
pub const UMCCH6_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_1_AddrHashBank3
pub const UMCCH6_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_1_AddrHashBank4
pub const UMCCH6_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_1_AddrHashBank5
pub const UMCCH6_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_1_EccErrCntSel
pub const UMCCH6_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH6_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH6_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH6_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH6_1_EccErrCnt
pub const UMCCH6_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH6_1_PerfMonCtlClk
pub const UMCCH6_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH6_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH6_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH6_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH6_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH6_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH6_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH6_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH6_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtrClk_Lo
pub const UMCCH6_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtrClk_Hi
pub const UMCCH6_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH6_1_PerfMonCtl1
pub const UMCCH6_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr1_Lo
pub const UMCCH6_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr1_Hi
pub const UMCCH6_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl2
pub const UMCCH6_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr2_Lo
pub const UMCCH6_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr2_Hi
pub const UMCCH6_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl3
pub const UMCCH6_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr3_Lo
pub const UMCCH6_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr3_Hi
pub const UMCCH6_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl4
pub const UMCCH6_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr4_Lo
pub const UMCCH6_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr4_Hi
pub const UMCCH6_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl5
pub const UMCCH6_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr5_Lo
pub const UMCCH6_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr5_Hi
pub const UMCCH6_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl6
pub const UMCCH6_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr6_Lo
pub const UMCCH6_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr6_Hi
pub const UMCCH6_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl7
pub const UMCCH6_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr7_Lo
pub const UMCCH6_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr7_Hi
pub const UMCCH6_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_1_PerfMonCtl8
pub const UMCCH6_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_1_PerfMonCtr8_Lo
pub const UMCCH6_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_1_PerfMonCtr8_Hi
pub const UMCCH6_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc1_umcch7_umcchdec
// UMCCH7_1_BaseAddrCS0
pub const UMCCH7_1_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_1_AddrMaskCS01
pub const UMCCH7_1_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_1_AddrSelCS01
pub const UMCCH7_1_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH7_1_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH7_1_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH7_1_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_1_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH7_1_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH7_1_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH7_1_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH7_1_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH7_1_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH7_1_AddrHashBank0
pub const UMCCH7_1_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_1_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_1_AddrHashBank1
pub const UMCCH7_1_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_1_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_1_AddrHashBank2
pub const UMCCH7_1_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_1_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_1_AddrHashBank3
pub const UMCCH7_1_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_1_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_1_AddrHashBank4
pub const UMCCH7_1_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_1_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_1_AddrHashBank5
pub const UMCCH7_1_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_1_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_1_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_1_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_1_EccErrCntSel
pub const UMCCH7_1_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH7_1_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH7_1_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_1_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH7_1_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH7_1_EccErrCnt
pub const UMCCH7_1_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH7_1_PerfMonCtlClk
pub const UMCCH7_1_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH7_1_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH7_1_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH7_1_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH7_1_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH7_1_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH7_1_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH7_1_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH7_1_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtrClk_Lo
pub const UMCCH7_1_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtrClk_Hi
pub const UMCCH7_1_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH7_1_PerfMonCtl1
pub const UMCCH7_1_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr1_Lo
pub const UMCCH7_1_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr1_Hi
pub const UMCCH7_1_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl2
pub const UMCCH7_1_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr2_Lo
pub const UMCCH7_1_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr2_Hi
pub const UMCCH7_1_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl3
pub const UMCCH7_1_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr3_Lo
pub const UMCCH7_1_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr3_Hi
pub const UMCCH7_1_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl4
pub const UMCCH7_1_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr4_Lo
pub const UMCCH7_1_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr4_Hi
pub const UMCCH7_1_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl5
pub const UMCCH7_1_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr5_Lo
pub const UMCCH7_1_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr5_Hi
pub const UMCCH7_1_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl6
pub const UMCCH7_1_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr6_Lo
pub const UMCCH7_1_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr6_Hi
pub const UMCCH7_1_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl7
pub const UMCCH7_1_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr7_Lo
pub const UMCCH7_1_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr7_Hi
pub const UMCCH7_1_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_1_PerfMonCtl8
pub const UMCCH7_1_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_1_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_1_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_1_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_1_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_1_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_1_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_1_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_1_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_1_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_1_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_1_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_1_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_1_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_1_PerfMonCtr8_Lo
pub const UMCCH7_1_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_1_PerfMonCtr8_Hi
pub const UMCCH7_1_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_1_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_1_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_1_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_1_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_1_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch0_umcchdec
// UMCCH0_2_BaseAddrCS0
pub const UMCCH0_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_2_AddrMaskCS01
pub const UMCCH0_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_2_AddrSelCS01
pub const UMCCH0_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH0_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH0_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH0_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH0_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH0_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH0_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH0_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH0_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH0_2_AddrHashBank0
pub const UMCCH0_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_2_AddrHashBank1
pub const UMCCH0_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_2_AddrHashBank2
pub const UMCCH0_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_2_AddrHashBank3
pub const UMCCH0_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_2_AddrHashBank4
pub const UMCCH0_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_2_AddrHashBank5
pub const UMCCH0_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_2_EccErrCntSel
pub const UMCCH0_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH0_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH0_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH0_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH0_2_EccErrCnt
pub const UMCCH0_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH0_2_PerfMonCtlClk
pub const UMCCH0_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH0_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH0_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH0_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH0_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH0_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH0_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH0_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH0_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtrClk_Lo
pub const UMCCH0_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtrClk_Hi
pub const UMCCH0_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH0_2_PerfMonCtl1
pub const UMCCH0_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr1_Lo
pub const UMCCH0_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr1_Hi
pub const UMCCH0_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl2
pub const UMCCH0_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr2_Lo
pub const UMCCH0_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr2_Hi
pub const UMCCH0_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl3
pub const UMCCH0_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr3_Lo
pub const UMCCH0_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr3_Hi
pub const UMCCH0_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl4
pub const UMCCH0_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr4_Lo
pub const UMCCH0_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr4_Hi
pub const UMCCH0_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl5
pub const UMCCH0_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr5_Lo
pub const UMCCH0_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr5_Hi
pub const UMCCH0_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl6
pub const UMCCH0_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr6_Lo
pub const UMCCH0_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr6_Hi
pub const UMCCH0_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl7
pub const UMCCH0_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr7_Lo
pub const UMCCH0_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr7_Hi
pub const UMCCH0_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_2_PerfMonCtl8
pub const UMCCH0_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_2_PerfMonCtr8_Lo
pub const UMCCH0_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_2_PerfMonCtr8_Hi
pub const UMCCH0_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch1_umcchdec
// UMCCH1_2_BaseAddrCS0
pub const UMCCH1_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_2_AddrMaskCS01
pub const UMCCH1_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_2_AddrSelCS01
pub const UMCCH1_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH1_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH1_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH1_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH1_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH1_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH1_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH1_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH1_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH1_2_AddrHashBank0
pub const UMCCH1_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_2_AddrHashBank1
pub const UMCCH1_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_2_AddrHashBank2
pub const UMCCH1_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_2_AddrHashBank3
pub const UMCCH1_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_2_AddrHashBank4
pub const UMCCH1_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_2_AddrHashBank5
pub const UMCCH1_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_2_EccErrCntSel
pub const UMCCH1_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH1_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH1_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH1_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH1_2_EccErrCnt
pub const UMCCH1_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH1_2_PerfMonCtlClk
pub const UMCCH1_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH1_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH1_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH1_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH1_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH1_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH1_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH1_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH1_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtrClk_Lo
pub const UMCCH1_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtrClk_Hi
pub const UMCCH1_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH1_2_PerfMonCtl1
pub const UMCCH1_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr1_Lo
pub const UMCCH1_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr1_Hi
pub const UMCCH1_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl2
pub const UMCCH1_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr2_Lo
pub const UMCCH1_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr2_Hi
pub const UMCCH1_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl3
pub const UMCCH1_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr3_Lo
pub const UMCCH1_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr3_Hi
pub const UMCCH1_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl4
pub const UMCCH1_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr4_Lo
pub const UMCCH1_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr4_Hi
pub const UMCCH1_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl5
pub const UMCCH1_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr5_Lo
pub const UMCCH1_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr5_Hi
pub const UMCCH1_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl6
pub const UMCCH1_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr6_Lo
pub const UMCCH1_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr6_Hi
pub const UMCCH1_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl7
pub const UMCCH1_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr7_Lo
pub const UMCCH1_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr7_Hi
pub const UMCCH1_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_2_PerfMonCtl8
pub const UMCCH1_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_2_PerfMonCtr8_Lo
pub const UMCCH1_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_2_PerfMonCtr8_Hi
pub const UMCCH1_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch2_umcchdec
// UMCCH2_2_BaseAddrCS0
pub const UMCCH2_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_2_AddrMaskCS01
pub const UMCCH2_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_2_AddrSelCS01
pub const UMCCH2_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH2_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH2_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH2_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH2_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH2_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH2_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH2_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH2_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH2_2_AddrHashBank0
pub const UMCCH2_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_2_AddrHashBank1
pub const UMCCH2_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_2_AddrHashBank2
pub const UMCCH2_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_2_AddrHashBank3
pub const UMCCH2_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_2_AddrHashBank4
pub const UMCCH2_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_2_AddrHashBank5
pub const UMCCH2_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_2_EccErrCntSel
pub const UMCCH2_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH2_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH2_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH2_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH2_2_EccErrCnt
pub const UMCCH2_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH2_2_PerfMonCtlClk
pub const UMCCH2_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH2_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH2_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH2_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH2_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH2_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH2_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH2_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH2_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtrClk_Lo
pub const UMCCH2_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtrClk_Hi
pub const UMCCH2_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH2_2_PerfMonCtl1
pub const UMCCH2_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr1_Lo
pub const UMCCH2_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr1_Hi
pub const UMCCH2_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl2
pub const UMCCH2_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr2_Lo
pub const UMCCH2_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr2_Hi
pub const UMCCH2_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl3
pub const UMCCH2_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr3_Lo
pub const UMCCH2_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr3_Hi
pub const UMCCH2_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl4
pub const UMCCH2_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr4_Lo
pub const UMCCH2_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr4_Hi
pub const UMCCH2_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl5
pub const UMCCH2_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr5_Lo
pub const UMCCH2_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr5_Hi
pub const UMCCH2_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl6
pub const UMCCH2_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr6_Lo
pub const UMCCH2_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr6_Hi
pub const UMCCH2_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl7
pub const UMCCH2_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr7_Lo
pub const UMCCH2_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr7_Hi
pub const UMCCH2_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_2_PerfMonCtl8
pub const UMCCH2_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_2_PerfMonCtr8_Lo
pub const UMCCH2_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_2_PerfMonCtr8_Hi
pub const UMCCH2_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch3_umcchdec
// UMCCH3_2_BaseAddrCS0
pub const UMCCH3_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_2_AddrMaskCS01
pub const UMCCH3_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_2_AddrSelCS01
pub const UMCCH3_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH3_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH3_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH3_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH3_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH3_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH3_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH3_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH3_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH3_2_AddrHashBank0
pub const UMCCH3_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_2_AddrHashBank1
pub const UMCCH3_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_2_AddrHashBank2
pub const UMCCH3_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_2_AddrHashBank3
pub const UMCCH3_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_2_AddrHashBank4
pub const UMCCH3_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_2_AddrHashBank5
pub const UMCCH3_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_2_EccErrCntSel
pub const UMCCH3_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH3_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH3_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH3_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH3_2_EccErrCnt
pub const UMCCH3_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH3_2_PerfMonCtlClk
pub const UMCCH3_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH3_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH3_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH3_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH3_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH3_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH3_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH3_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH3_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtrClk_Lo
pub const UMCCH3_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtrClk_Hi
pub const UMCCH3_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH3_2_PerfMonCtl1
pub const UMCCH3_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr1_Lo
pub const UMCCH3_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr1_Hi
pub const UMCCH3_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl2
pub const UMCCH3_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr2_Lo
pub const UMCCH3_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr2_Hi
pub const UMCCH3_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl3
pub const UMCCH3_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr3_Lo
pub const UMCCH3_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr3_Hi
pub const UMCCH3_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl4
pub const UMCCH3_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr4_Lo
pub const UMCCH3_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr4_Hi
pub const UMCCH3_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl5
pub const UMCCH3_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr5_Lo
pub const UMCCH3_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr5_Hi
pub const UMCCH3_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl6
pub const UMCCH3_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr6_Lo
pub const UMCCH3_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr6_Hi
pub const UMCCH3_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl7
pub const UMCCH3_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr7_Lo
pub const UMCCH3_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr7_Hi
pub const UMCCH3_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_2_PerfMonCtl8
pub const UMCCH3_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_2_PerfMonCtr8_Lo
pub const UMCCH3_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_2_PerfMonCtr8_Hi
pub const UMCCH3_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch4_umcchdec
// UMCCH4_2_BaseAddrCS0
pub const UMCCH4_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_2_AddrMaskCS01
pub const UMCCH4_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_2_AddrSelCS01
pub const UMCCH4_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH4_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH4_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH4_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH4_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH4_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH4_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH4_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH4_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH4_2_AddrHashBank0
pub const UMCCH4_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_2_AddrHashBank1
pub const UMCCH4_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_2_AddrHashBank2
pub const UMCCH4_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_2_AddrHashBank3
pub const UMCCH4_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_2_AddrHashBank4
pub const UMCCH4_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_2_AddrHashBank5
pub const UMCCH4_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_2_EccErrCntSel
pub const UMCCH4_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH4_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH4_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH4_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH4_2_EccErrCnt
pub const UMCCH4_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH4_2_PerfMonCtlClk
pub const UMCCH4_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH4_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH4_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH4_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH4_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH4_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH4_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH4_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH4_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtrClk_Lo
pub const UMCCH4_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtrClk_Hi
pub const UMCCH4_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH4_2_PerfMonCtl1
pub const UMCCH4_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr1_Lo
pub const UMCCH4_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr1_Hi
pub const UMCCH4_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl2
pub const UMCCH4_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr2_Lo
pub const UMCCH4_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr2_Hi
pub const UMCCH4_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl3
pub const UMCCH4_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr3_Lo
pub const UMCCH4_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr3_Hi
pub const UMCCH4_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl4
pub const UMCCH4_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr4_Lo
pub const UMCCH4_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr4_Hi
pub const UMCCH4_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl5
pub const UMCCH4_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr5_Lo
pub const UMCCH4_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr5_Hi
pub const UMCCH4_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl6
pub const UMCCH4_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr6_Lo
pub const UMCCH4_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr6_Hi
pub const UMCCH4_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl7
pub const UMCCH4_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr7_Lo
pub const UMCCH4_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr7_Hi
pub const UMCCH4_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_2_PerfMonCtl8
pub const UMCCH4_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_2_PerfMonCtr8_Lo
pub const UMCCH4_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_2_PerfMonCtr8_Hi
pub const UMCCH4_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch5_umcchdec
// UMCCH5_2_BaseAddrCS0
pub const UMCCH5_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_2_AddrMaskCS01
pub const UMCCH5_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_2_AddrSelCS01
pub const UMCCH5_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH5_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH5_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH5_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH5_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH5_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH5_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH5_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH5_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH5_2_AddrHashBank0
pub const UMCCH5_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_2_AddrHashBank1
pub const UMCCH5_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_2_AddrHashBank2
pub const UMCCH5_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_2_AddrHashBank3
pub const UMCCH5_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_2_AddrHashBank4
pub const UMCCH5_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_2_AddrHashBank5
pub const UMCCH5_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_2_EccErrCntSel
pub const UMCCH5_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH5_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH5_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH5_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH5_2_EccErrCnt
pub const UMCCH5_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH5_2_PerfMonCtlClk
pub const UMCCH5_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH5_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH5_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH5_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH5_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH5_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH5_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH5_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH5_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtrClk_Lo
pub const UMCCH5_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtrClk_Hi
pub const UMCCH5_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH5_2_PerfMonCtl1
pub const UMCCH5_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr1_Lo
pub const UMCCH5_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr1_Hi
pub const UMCCH5_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl2
pub const UMCCH5_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr2_Lo
pub const UMCCH5_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr2_Hi
pub const UMCCH5_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl3
pub const UMCCH5_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr3_Lo
pub const UMCCH5_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr3_Hi
pub const UMCCH5_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl4
pub const UMCCH5_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr4_Lo
pub const UMCCH5_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr4_Hi
pub const UMCCH5_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl5
pub const UMCCH5_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr5_Lo
pub const UMCCH5_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr5_Hi
pub const UMCCH5_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl6
pub const UMCCH5_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr6_Lo
pub const UMCCH5_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr6_Hi
pub const UMCCH5_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl7
pub const UMCCH5_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr7_Lo
pub const UMCCH5_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr7_Hi
pub const UMCCH5_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_2_PerfMonCtl8
pub const UMCCH5_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_2_PerfMonCtr8_Lo
pub const UMCCH5_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_2_PerfMonCtr8_Hi
pub const UMCCH5_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch6_umcchdec
// UMCCH6_2_BaseAddrCS0
pub const UMCCH6_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_2_AddrMaskCS01
pub const UMCCH6_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_2_AddrSelCS01
pub const UMCCH6_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH6_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH6_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH6_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH6_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH6_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH6_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH6_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH6_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH6_2_AddrHashBank0
pub const UMCCH6_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_2_AddrHashBank1
pub const UMCCH6_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_2_AddrHashBank2
pub const UMCCH6_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_2_AddrHashBank3
pub const UMCCH6_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_2_AddrHashBank4
pub const UMCCH6_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_2_AddrHashBank5
pub const UMCCH6_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_2_EccErrCntSel
pub const UMCCH6_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH6_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH6_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH6_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH6_2_EccErrCnt
pub const UMCCH6_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH6_2_PerfMonCtlClk
pub const UMCCH6_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH6_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH6_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH6_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH6_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH6_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH6_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH6_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH6_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtrClk_Lo
pub const UMCCH6_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtrClk_Hi
pub const UMCCH6_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH6_2_PerfMonCtl1
pub const UMCCH6_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr1_Lo
pub const UMCCH6_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr1_Hi
pub const UMCCH6_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl2
pub const UMCCH6_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr2_Lo
pub const UMCCH6_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr2_Hi
pub const UMCCH6_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl3
pub const UMCCH6_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr3_Lo
pub const UMCCH6_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr3_Hi
pub const UMCCH6_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl4
pub const UMCCH6_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr4_Lo
pub const UMCCH6_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr4_Hi
pub const UMCCH6_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl5
pub const UMCCH6_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr5_Lo
pub const UMCCH6_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr5_Hi
pub const UMCCH6_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl6
pub const UMCCH6_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr6_Lo
pub const UMCCH6_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr6_Hi
pub const UMCCH6_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl7
pub const UMCCH6_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr7_Lo
pub const UMCCH6_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr7_Hi
pub const UMCCH6_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_2_PerfMonCtl8
pub const UMCCH6_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_2_PerfMonCtr8_Lo
pub const UMCCH6_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_2_PerfMonCtr8_Hi
pub const UMCCH6_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc2_umcch7_umcchdec
// UMCCH7_2_BaseAddrCS0
pub const UMCCH7_2_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_2_AddrMaskCS01
pub const UMCCH7_2_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_2_AddrSelCS01
pub const UMCCH7_2_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH7_2_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH7_2_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH7_2_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_2_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH7_2_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH7_2_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH7_2_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH7_2_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH7_2_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH7_2_AddrHashBank0
pub const UMCCH7_2_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_2_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_2_AddrHashBank1
pub const UMCCH7_2_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_2_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_2_AddrHashBank2
pub const UMCCH7_2_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_2_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_2_AddrHashBank3
pub const UMCCH7_2_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_2_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_2_AddrHashBank4
pub const UMCCH7_2_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_2_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_2_AddrHashBank5
pub const UMCCH7_2_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_2_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_2_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_2_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_2_EccErrCntSel
pub const UMCCH7_2_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH7_2_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH7_2_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_2_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH7_2_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH7_2_EccErrCnt
pub const UMCCH7_2_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH7_2_PerfMonCtlClk
pub const UMCCH7_2_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH7_2_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH7_2_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH7_2_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH7_2_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH7_2_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH7_2_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH7_2_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH7_2_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtrClk_Lo
pub const UMCCH7_2_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtrClk_Hi
pub const UMCCH7_2_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH7_2_PerfMonCtl1
pub const UMCCH7_2_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr1_Lo
pub const UMCCH7_2_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr1_Hi
pub const UMCCH7_2_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl2
pub const UMCCH7_2_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr2_Lo
pub const UMCCH7_2_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr2_Hi
pub const UMCCH7_2_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl3
pub const UMCCH7_2_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr3_Lo
pub const UMCCH7_2_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr3_Hi
pub const UMCCH7_2_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl4
pub const UMCCH7_2_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr4_Lo
pub const UMCCH7_2_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr4_Hi
pub const UMCCH7_2_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl5
pub const UMCCH7_2_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr5_Lo
pub const UMCCH7_2_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr5_Hi
pub const UMCCH7_2_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl6
pub const UMCCH7_2_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr6_Lo
pub const UMCCH7_2_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr6_Hi
pub const UMCCH7_2_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl7
pub const UMCCH7_2_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr7_Lo
pub const UMCCH7_2_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr7_Hi
pub const UMCCH7_2_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_2_PerfMonCtl8
pub const UMCCH7_2_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_2_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_2_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_2_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_2_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_2_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_2_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_2_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_2_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_2_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_2_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_2_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_2_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_2_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_2_PerfMonCtr8_Lo
pub const UMCCH7_2_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_2_PerfMonCtr8_Hi
pub const UMCCH7_2_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_2_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_2_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_2_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_2_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_2_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch0_umcchdec
// UMCCH0_3_BaseAddrCS0
pub const UMCCH0_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_3_AddrMaskCS01
pub const UMCCH0_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH0_3_AddrSelCS01
pub const UMCCH0_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH0_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH0_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH0_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH0_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH0_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH0_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH0_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH0_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH0_3_AddrHashBank0
pub const UMCCH0_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_3_AddrHashBank1
pub const UMCCH0_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_3_AddrHashBank2
pub const UMCCH0_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_3_AddrHashBank3
pub const UMCCH0_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_3_AddrHashBank4
pub const UMCCH0_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_3_AddrHashBank5
pub const UMCCH0_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH0_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH0_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH0_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH0_3_EccErrCntSel
pub const UMCCH0_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH0_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH0_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH0_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH0_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH0_3_EccErrCnt
pub const UMCCH0_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH0_3_PerfMonCtlClk
pub const UMCCH0_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH0_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH0_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH0_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH0_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH0_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH0_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH0_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH0_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtrClk_Lo
pub const UMCCH0_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtrClk_Hi
pub const UMCCH0_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH0_3_PerfMonCtl1
pub const UMCCH0_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr1_Lo
pub const UMCCH0_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr1_Hi
pub const UMCCH0_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl2
pub const UMCCH0_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr2_Lo
pub const UMCCH0_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr2_Hi
pub const UMCCH0_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl3
pub const UMCCH0_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr3_Lo
pub const UMCCH0_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr3_Hi
pub const UMCCH0_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl4
pub const UMCCH0_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr4_Lo
pub const UMCCH0_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr4_Hi
pub const UMCCH0_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl5
pub const UMCCH0_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr5_Lo
pub const UMCCH0_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr5_Hi
pub const UMCCH0_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl6
pub const UMCCH0_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr6_Lo
pub const UMCCH0_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr6_Hi
pub const UMCCH0_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl7
pub const UMCCH0_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr7_Lo
pub const UMCCH0_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr7_Hi
pub const UMCCH0_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH0_3_PerfMonCtl8
pub const UMCCH0_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH0_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH0_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH0_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH0_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH0_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH0_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH0_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH0_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH0_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH0_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH0_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH0_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH0_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH0_3_PerfMonCtr8_Lo
pub const UMCCH0_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH0_3_PerfMonCtr8_Hi
pub const UMCCH0_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH0_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH0_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH0_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH0_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH0_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH0_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH0_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch1_umcchdec
// UMCCH1_3_BaseAddrCS0
pub const UMCCH1_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_3_AddrMaskCS01
pub const UMCCH1_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH1_3_AddrSelCS01
pub const UMCCH1_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH1_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH1_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH1_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH1_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH1_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH1_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH1_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH1_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH1_3_AddrHashBank0
pub const UMCCH1_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_3_AddrHashBank1
pub const UMCCH1_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_3_AddrHashBank2
pub const UMCCH1_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_3_AddrHashBank3
pub const UMCCH1_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_3_AddrHashBank4
pub const UMCCH1_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_3_AddrHashBank5
pub const UMCCH1_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH1_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH1_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH1_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH1_3_EccErrCntSel
pub const UMCCH1_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH1_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH1_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH1_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH1_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH1_3_EccErrCnt
pub const UMCCH1_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH1_3_PerfMonCtlClk
pub const UMCCH1_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH1_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH1_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH1_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH1_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH1_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH1_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH1_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH1_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtrClk_Lo
pub const UMCCH1_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtrClk_Hi
pub const UMCCH1_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH1_3_PerfMonCtl1
pub const UMCCH1_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr1_Lo
pub const UMCCH1_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr1_Hi
pub const UMCCH1_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl2
pub const UMCCH1_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr2_Lo
pub const UMCCH1_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr2_Hi
pub const UMCCH1_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl3
pub const UMCCH1_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr3_Lo
pub const UMCCH1_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr3_Hi
pub const UMCCH1_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl4
pub const UMCCH1_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr4_Lo
pub const UMCCH1_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr4_Hi
pub const UMCCH1_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl5
pub const UMCCH1_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr5_Lo
pub const UMCCH1_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr5_Hi
pub const UMCCH1_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl6
pub const UMCCH1_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr6_Lo
pub const UMCCH1_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr6_Hi
pub const UMCCH1_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl7
pub const UMCCH1_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr7_Lo
pub const UMCCH1_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr7_Hi
pub const UMCCH1_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH1_3_PerfMonCtl8
pub const UMCCH1_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH1_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH1_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH1_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH1_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH1_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH1_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH1_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH1_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH1_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH1_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH1_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH1_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH1_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH1_3_PerfMonCtr8_Lo
pub const UMCCH1_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH1_3_PerfMonCtr8_Hi
pub const UMCCH1_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH1_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH1_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH1_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH1_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH1_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH1_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH1_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch2_umcchdec
// UMCCH2_3_BaseAddrCS0
pub const UMCCH2_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_3_AddrMaskCS01
pub const UMCCH2_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH2_3_AddrSelCS01
pub const UMCCH2_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH2_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH2_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH2_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH2_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH2_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH2_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH2_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH2_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH2_3_AddrHashBank0
pub const UMCCH2_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_3_AddrHashBank1
pub const UMCCH2_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_3_AddrHashBank2
pub const UMCCH2_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_3_AddrHashBank3
pub const UMCCH2_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_3_AddrHashBank4
pub const UMCCH2_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_3_AddrHashBank5
pub const UMCCH2_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH2_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH2_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH2_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH2_3_EccErrCntSel
pub const UMCCH2_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH2_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH2_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH2_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH2_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH2_3_EccErrCnt
pub const UMCCH2_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH2_3_PerfMonCtlClk
pub const UMCCH2_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH2_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH2_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH2_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH2_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH2_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH2_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH2_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH2_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtrClk_Lo
pub const UMCCH2_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtrClk_Hi
pub const UMCCH2_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH2_3_PerfMonCtl1
pub const UMCCH2_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr1_Lo
pub const UMCCH2_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr1_Hi
pub const UMCCH2_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl2
pub const UMCCH2_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr2_Lo
pub const UMCCH2_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr2_Hi
pub const UMCCH2_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl3
pub const UMCCH2_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr3_Lo
pub const UMCCH2_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr3_Hi
pub const UMCCH2_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl4
pub const UMCCH2_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr4_Lo
pub const UMCCH2_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr4_Hi
pub const UMCCH2_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl5
pub const UMCCH2_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr5_Lo
pub const UMCCH2_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr5_Hi
pub const UMCCH2_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl6
pub const UMCCH2_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr6_Lo
pub const UMCCH2_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr6_Hi
pub const UMCCH2_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl7
pub const UMCCH2_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr7_Lo
pub const UMCCH2_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr7_Hi
pub const UMCCH2_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH2_3_PerfMonCtl8
pub const UMCCH2_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH2_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH2_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH2_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH2_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH2_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH2_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH2_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH2_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH2_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH2_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH2_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH2_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH2_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH2_3_PerfMonCtr8_Lo
pub const UMCCH2_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH2_3_PerfMonCtr8_Hi
pub const UMCCH2_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH2_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH2_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH2_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH2_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH2_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH2_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH2_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch3_umcchdec
// UMCCH3_3_BaseAddrCS0
pub const UMCCH3_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_3_AddrMaskCS01
pub const UMCCH3_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH3_3_AddrSelCS01
pub const UMCCH3_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH3_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH3_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH3_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH3_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH3_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH3_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH3_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH3_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH3_3_AddrHashBank0
pub const UMCCH3_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_3_AddrHashBank1
pub const UMCCH3_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_3_AddrHashBank2
pub const UMCCH3_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_3_AddrHashBank3
pub const UMCCH3_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_3_AddrHashBank4
pub const UMCCH3_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_3_AddrHashBank5
pub const UMCCH3_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH3_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH3_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH3_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH3_3_EccErrCntSel
pub const UMCCH3_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH3_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH3_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH3_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH3_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH3_3_EccErrCnt
pub const UMCCH3_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH3_3_PerfMonCtlClk
pub const UMCCH3_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH3_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH3_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH3_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH3_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH3_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH3_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH3_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH3_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtrClk_Lo
pub const UMCCH3_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtrClk_Hi
pub const UMCCH3_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH3_3_PerfMonCtl1
pub const UMCCH3_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr1_Lo
pub const UMCCH3_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr1_Hi
pub const UMCCH3_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl2
pub const UMCCH3_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr2_Lo
pub const UMCCH3_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr2_Hi
pub const UMCCH3_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl3
pub const UMCCH3_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr3_Lo
pub const UMCCH3_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr3_Hi
pub const UMCCH3_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl4
pub const UMCCH3_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr4_Lo
pub const UMCCH3_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr4_Hi
pub const UMCCH3_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl5
pub const UMCCH3_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr5_Lo
pub const UMCCH3_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr5_Hi
pub const UMCCH3_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl6
pub const UMCCH3_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr6_Lo
pub const UMCCH3_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr6_Hi
pub const UMCCH3_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl7
pub const UMCCH3_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr7_Lo
pub const UMCCH3_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr7_Hi
pub const UMCCH3_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH3_3_PerfMonCtl8
pub const UMCCH3_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH3_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH3_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH3_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH3_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH3_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH3_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH3_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH3_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH3_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH3_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH3_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH3_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH3_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH3_3_PerfMonCtr8_Lo
pub const UMCCH3_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH3_3_PerfMonCtr8_Hi
pub const UMCCH3_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH3_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH3_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH3_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH3_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH3_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH3_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH3_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch4_umcchdec
// UMCCH4_3_BaseAddrCS0
pub const UMCCH4_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_3_AddrMaskCS01
pub const UMCCH4_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH4_3_AddrSelCS01
pub const UMCCH4_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH4_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH4_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH4_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH4_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH4_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH4_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH4_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH4_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH4_3_AddrHashBank0
pub const UMCCH4_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_3_AddrHashBank1
pub const UMCCH4_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_3_AddrHashBank2
pub const UMCCH4_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_3_AddrHashBank3
pub const UMCCH4_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_3_AddrHashBank4
pub const UMCCH4_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_3_AddrHashBank5
pub const UMCCH4_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH4_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH4_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH4_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH4_3_EccErrCntSel
pub const UMCCH4_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH4_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH4_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH4_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH4_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH4_3_EccErrCnt
pub const UMCCH4_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH4_3_PerfMonCtlClk
pub const UMCCH4_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH4_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH4_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH4_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH4_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH4_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH4_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH4_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH4_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtrClk_Lo
pub const UMCCH4_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtrClk_Hi
pub const UMCCH4_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH4_3_PerfMonCtl1
pub const UMCCH4_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr1_Lo
pub const UMCCH4_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr1_Hi
pub const UMCCH4_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl2
pub const UMCCH4_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr2_Lo
pub const UMCCH4_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr2_Hi
pub const UMCCH4_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl3
pub const UMCCH4_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr3_Lo
pub const UMCCH4_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr3_Hi
pub const UMCCH4_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl4
pub const UMCCH4_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr4_Lo
pub const UMCCH4_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr4_Hi
pub const UMCCH4_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl5
pub const UMCCH4_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr5_Lo
pub const UMCCH4_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr5_Hi
pub const UMCCH4_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl6
pub const UMCCH4_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr6_Lo
pub const UMCCH4_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr6_Hi
pub const UMCCH4_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl7
pub const UMCCH4_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr7_Lo
pub const UMCCH4_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr7_Hi
pub const UMCCH4_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH4_3_PerfMonCtl8
pub const UMCCH4_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH4_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH4_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH4_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH4_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH4_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH4_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH4_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH4_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH4_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH4_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH4_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH4_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH4_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH4_3_PerfMonCtr8_Lo
pub const UMCCH4_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH4_3_PerfMonCtr8_Hi
pub const UMCCH4_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH4_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH4_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH4_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH4_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH4_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH4_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH4_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch5_umcchdec
// UMCCH5_3_BaseAddrCS0
pub const UMCCH5_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_3_AddrMaskCS01
pub const UMCCH5_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH5_3_AddrSelCS01
pub const UMCCH5_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH5_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH5_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH5_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH5_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH5_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH5_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH5_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH5_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH5_3_AddrHashBank0
pub const UMCCH5_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_3_AddrHashBank1
pub const UMCCH5_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_3_AddrHashBank2
pub const UMCCH5_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_3_AddrHashBank3
pub const UMCCH5_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_3_AddrHashBank4
pub const UMCCH5_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_3_AddrHashBank5
pub const UMCCH5_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH5_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH5_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH5_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH5_3_EccErrCntSel
pub const UMCCH5_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH5_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH5_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH5_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH5_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH5_3_EccErrCnt
pub const UMCCH5_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH5_3_PerfMonCtlClk
pub const UMCCH5_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH5_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH5_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH5_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH5_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH5_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH5_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH5_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH5_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtrClk_Lo
pub const UMCCH5_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtrClk_Hi
pub const UMCCH5_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH5_3_PerfMonCtl1
pub const UMCCH5_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr1_Lo
pub const UMCCH5_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr1_Hi
pub const UMCCH5_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl2
pub const UMCCH5_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr2_Lo
pub const UMCCH5_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr2_Hi
pub const UMCCH5_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl3
pub const UMCCH5_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr3_Lo
pub const UMCCH5_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr3_Hi
pub const UMCCH5_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl4
pub const UMCCH5_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr4_Lo
pub const UMCCH5_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr4_Hi
pub const UMCCH5_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl5
pub const UMCCH5_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr5_Lo
pub const UMCCH5_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr5_Hi
pub const UMCCH5_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl6
pub const UMCCH5_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr6_Lo
pub const UMCCH5_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr6_Hi
pub const UMCCH5_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl7
pub const UMCCH5_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr7_Lo
pub const UMCCH5_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr7_Hi
pub const UMCCH5_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH5_3_PerfMonCtl8
pub const UMCCH5_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH5_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH5_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH5_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH5_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH5_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH5_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH5_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH5_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH5_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH5_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH5_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH5_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH5_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH5_3_PerfMonCtr8_Lo
pub const UMCCH5_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH5_3_PerfMonCtr8_Hi
pub const UMCCH5_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH5_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH5_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH5_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH5_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH5_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH5_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH5_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch6_umcchdec
// UMCCH6_3_BaseAddrCS0
pub const UMCCH6_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_3_AddrMaskCS01
pub const UMCCH6_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH6_3_AddrSelCS01
pub const UMCCH6_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH6_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH6_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH6_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH6_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH6_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH6_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH6_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH6_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH6_3_AddrHashBank0
pub const UMCCH6_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_3_AddrHashBank1
pub const UMCCH6_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_3_AddrHashBank2
pub const UMCCH6_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_3_AddrHashBank3
pub const UMCCH6_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_3_AddrHashBank4
pub const UMCCH6_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_3_AddrHashBank5
pub const UMCCH6_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH6_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH6_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH6_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH6_3_EccErrCntSel
pub const UMCCH6_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH6_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH6_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH6_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH6_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH6_3_EccErrCnt
pub const UMCCH6_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH6_3_PerfMonCtlClk
pub const UMCCH6_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH6_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH6_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH6_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH6_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH6_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH6_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH6_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH6_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtrClk_Lo
pub const UMCCH6_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtrClk_Hi
pub const UMCCH6_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH6_3_PerfMonCtl1
pub const UMCCH6_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr1_Lo
pub const UMCCH6_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr1_Hi
pub const UMCCH6_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl2
pub const UMCCH6_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr2_Lo
pub const UMCCH6_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr2_Hi
pub const UMCCH6_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl3
pub const UMCCH6_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr3_Lo
pub const UMCCH6_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr3_Hi
pub const UMCCH6_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl4
pub const UMCCH6_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr4_Lo
pub const UMCCH6_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr4_Hi
pub const UMCCH6_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl5
pub const UMCCH6_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr5_Lo
pub const UMCCH6_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr5_Hi
pub const UMCCH6_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl6
pub const UMCCH6_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr6_Lo
pub const UMCCH6_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr6_Hi
pub const UMCCH6_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl7
pub const UMCCH6_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr7_Lo
pub const UMCCH6_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr7_Hi
pub const UMCCH6_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH6_3_PerfMonCtl8
pub const UMCCH6_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH6_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH6_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH6_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH6_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH6_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH6_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH6_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH6_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH6_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH6_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH6_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH6_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH6_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH6_3_PerfMonCtr8_Lo
pub const UMCCH6_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH6_3_PerfMonCtr8_Hi
pub const UMCCH6_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH6_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH6_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH6_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH6_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH6_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH6_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH6_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// addressBlock: umc_w_phy_umc3_umcch7_umcchdec
// UMCCH7_3_BaseAddrCS0
pub const UMCCH7_3_BaseAddrCS0__CSEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_BaseAddrCS0__BaseAddr__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_BaseAddrCS0__CSEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_BaseAddrCS0__BaseAddr_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_3_AddrMaskCS01
pub const UMCCH7_3_AddrMaskCS01__AddrMask__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrMaskCS01__AddrMask_MASK: c_uint = 0xFFFFFFFEL;
// UMCCH7_3_AddrSelCS01
pub const UMCCH7_3_AddrSelCS01__BankBit0__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrSelCS01__BankBit1__SHIFT: c_uint = 0x4;
pub const UMCCH7_3_AddrSelCS01__BankBit2__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_AddrSelCS01__BankBit3__SHIFT: c_uint = 0xc;
pub const UMCCH7_3_AddrSelCS01__BankBit4__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_AddrSelCS01__RowLo__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_AddrSelCS01__RowHi__SHIFT: c_uint = 0x1c;
pub const UMCCH7_3_AddrSelCS01__BankBit0_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_3_AddrSelCS01__BankBit1_MASK: c_uint = 0x000000F0L;
pub const UMCCH7_3_AddrSelCS01__BankBit2_MASK: c_uint = 0x00000F00L;
pub const UMCCH7_3_AddrSelCS01__BankBit3_MASK: c_uint = 0x0000F000L;
pub const UMCCH7_3_AddrSelCS01__BankBit4_MASK: c_uint = 0x001F0000L;
pub const UMCCH7_3_AddrSelCS01__RowLo_MASK: c_uint = 0x0F000000L;
pub const UMCCH7_3_AddrSelCS01__RowHi_MASK: c_uint = 0xF0000000L;
// UMCCH7_3_AddrHashBank0
pub const UMCCH7_3_AddrHashBank0__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrHashBank0__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrHashBank0__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_AddrHashBank0__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_AddrHashBank0__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_3_AddrHashBank0__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_3_AddrHashBank1
pub const UMCCH7_3_AddrHashBank1__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrHashBank1__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrHashBank1__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_AddrHashBank1__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_AddrHashBank1__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_3_AddrHashBank1__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_3_AddrHashBank2
pub const UMCCH7_3_AddrHashBank2__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrHashBank2__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrHashBank2__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_AddrHashBank2__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_AddrHashBank2__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_3_AddrHashBank2__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_3_AddrHashBank3
pub const UMCCH7_3_AddrHashBank3__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrHashBank3__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrHashBank3__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_AddrHashBank3__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_AddrHashBank3__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_3_AddrHashBank3__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_3_AddrHashBank4
pub const UMCCH7_3_AddrHashBank4__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrHashBank4__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrHashBank4__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_AddrHashBank4__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_AddrHashBank4__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_3_AddrHashBank4__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_3_AddrHashBank5
pub const UMCCH7_3_AddrHashBank5__XorEnable__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_AddrHashBank5__ColXor__SHIFT: c_uint = 0x1;
pub const UMCCH7_3_AddrHashBank5__RowXor__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_AddrHashBank5__XorEnable_MASK: c_uint = 0x00000001L;
pub const UMCCH7_3_AddrHashBank5__ColXor_MASK: c_uint = 0x00003FFEL;
pub const UMCCH7_3_AddrHashBank5__RowXor_MASK: c_uint = 0xFFFFC000L;
// UMCCH7_3_EccErrCntSel
pub const UMCCH7_3_EccErrCntSel__EccErrCntCsSel__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_EccErrCntSel__EccErrInt__SHIFT: c_uint = 0xc;
pub const UMCCH7_3_EccErrCntSel__EccErrCntEn__SHIFT: c_uint = 0xf;
pub const UMCCH7_3_EccErrCntSel__EccErrCntCsSel_MASK: c_uint = 0x0000000FL;
pub const UMCCH7_3_EccErrCntSel__EccErrInt_MASK: c_uint = 0x00003000L;
pub const UMCCH7_3_EccErrCntSel__EccErrCntEn_MASK: c_uint = 0x00008000L;
// UMCCH7_3_EccErrCnt
pub const UMCCH7_3_EccErrCnt__EccErrCnt__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_EccErrCnt__EccErrCnt_MASK: c_uint = 0x0000FFFFL;
// UMCCH7_3_PerfMonCtlClk
pub const UMCCH7_3_PerfMonCtlClk__GlblResetMsk__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtlClk__ClkGate__SHIFT: c_uint = 0x16;
pub const UMCCH7_3_PerfMonCtlClk__GlblReset__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtlClk__GlblMonEn__SHIFT: c_uint = 0x19;
pub const UMCCH7_3_PerfMonCtlClk__NumCounters__SHIFT: c_uint = 0x1a;
pub const UMCCH7_3_PerfMonCtlClk__CtrClkEn__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtlClk__GlblResetMsk_MASK: c_uint = 0x000001FFL;
pub const UMCCH7_3_PerfMonCtlClk__ClkGate_MASK: c_uint = 0x00400000L;
pub const UMCCH7_3_PerfMonCtlClk__GlblReset_MASK: c_uint = 0x01000000L;
pub const UMCCH7_3_PerfMonCtlClk__GlblMonEn_MASK: c_uint = 0x02000000L;
pub const UMCCH7_3_PerfMonCtlClk__NumCounters_MASK: c_uint = 0x3C000000L;
pub const UMCCH7_3_PerfMonCtlClk__CtrClkEn_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtrClk_Lo
pub const UMCCH7_3_PerfMonCtrClk_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtrClk_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtrClk_Hi
pub const UMCCH7_3_PerfMonCtrClk_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtrClk_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtrClk_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtrClk_Hi__Overflow_MASK: c_uint = 0x00010000L;
// UMCCH7_3_PerfMonCtl1
pub const UMCCH7_3_PerfMonCtl1__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl1__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl1__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl1__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl1__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl1__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl1__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl1__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl1__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl1__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl1__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl1__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl1__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl1__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl1__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl1__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr1_Lo
pub const UMCCH7_3_PerfMonCtr1_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr1_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr1_Hi
pub const UMCCH7_3_PerfMonCtr1_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr1_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr1_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr1_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr1_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr1_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl2
pub const UMCCH7_3_PerfMonCtl2__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl2__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl2__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl2__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl2__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl2__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl2__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl2__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl2__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl2__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl2__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl2__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl2__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl2__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl2__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl2__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr2_Lo
pub const UMCCH7_3_PerfMonCtr2_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr2_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr2_Hi
pub const UMCCH7_3_PerfMonCtr2_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr2_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr2_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr2_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr2_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr2_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl3
pub const UMCCH7_3_PerfMonCtl3__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl3__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl3__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl3__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl3__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl3__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl3__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl3__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl3__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl3__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl3__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl3__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl3__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl3__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl3__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl3__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr3_Lo
pub const UMCCH7_3_PerfMonCtr3_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr3_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr3_Hi
pub const UMCCH7_3_PerfMonCtr3_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr3_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr3_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr3_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr3_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr3_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl4
pub const UMCCH7_3_PerfMonCtl4__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl4__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl4__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl4__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl4__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl4__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl4__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl4__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl4__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl4__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl4__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl4__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl4__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl4__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl4__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl4__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr4_Lo
pub const UMCCH7_3_PerfMonCtr4_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr4_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr4_Hi
pub const UMCCH7_3_PerfMonCtr4_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr4_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr4_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr4_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr4_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr4_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl5
pub const UMCCH7_3_PerfMonCtl5__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl5__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl5__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl5__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl5__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl5__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl5__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl5__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl5__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl5__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl5__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl5__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl5__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl5__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl5__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl5__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr5_Lo
pub const UMCCH7_3_PerfMonCtr5_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr5_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr5_Hi
pub const UMCCH7_3_PerfMonCtr5_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr5_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr5_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr5_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr5_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr5_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl6
pub const UMCCH7_3_PerfMonCtl6__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl6__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl6__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl6__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl6__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl6__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl6__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl6__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl6__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl6__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl6__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl6__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl6__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl6__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl6__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl6__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr6_Lo
pub const UMCCH7_3_PerfMonCtr6_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr6_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr6_Hi
pub const UMCCH7_3_PerfMonCtr6_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr6_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr6_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr6_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr6_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr6_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl7
pub const UMCCH7_3_PerfMonCtl7__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl7__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl7__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl7__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl7__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl7__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl7__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl7__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl7__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl7__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl7__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl7__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl7__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl7__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl7__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl7__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr7_Lo
pub const UMCCH7_3_PerfMonCtr7_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr7_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr7_Hi
pub const UMCCH7_3_PerfMonCtr7_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr7_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr7_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr7_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr7_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr7_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
// UMCCH7_3_PerfMonCtl8
pub const UMCCH7_3_PerfMonCtl8__EventSelect__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtl8__RdWrMask__SHIFT: c_uint = 0x8;
pub const UMCCH7_3_PerfMonCtl8__PriorityMask__SHIFT: c_uint = 0xa;
pub const UMCCH7_3_PerfMonCtl8__ReqSizeMask__SHIFT: c_uint = 0xe;
pub const UMCCH7_3_PerfMonCtl8__BankSel__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtl8__VCSel__SHIFT: c_uint = 0x18;
pub const UMCCH7_3_PerfMonCtl8__SubChanMask__SHIFT: c_uint = 0x1d;
pub const UMCCH7_3_PerfMonCtl8__Enable__SHIFT: c_uint = 0x1f;
pub const UMCCH7_3_PerfMonCtl8__EventSelect_MASK: c_uint = 0x000000FFL;
pub const UMCCH7_3_PerfMonCtl8__RdWrMask_MASK: c_uint = 0x00000300L;
pub const UMCCH7_3_PerfMonCtl8__PriorityMask_MASK: c_uint = 0x00003C00L;
pub const UMCCH7_3_PerfMonCtl8__ReqSizeMask_MASK: c_uint = 0x0000C000L;
pub const UMCCH7_3_PerfMonCtl8__BankSel_MASK: c_uint = 0x00FF0000L;
pub const UMCCH7_3_PerfMonCtl8__VCSel_MASK: c_uint = 0x1F000000L;
pub const UMCCH7_3_PerfMonCtl8__SubChanMask_MASK: c_uint = 0x60000000L;
pub const UMCCH7_3_PerfMonCtl8__Enable_MASK: c_uint = 0x80000000L;
// UMCCH7_3_PerfMonCtr8_Lo
pub const UMCCH7_3_PerfMonCtr8_Lo__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr8_Lo__Data_MASK: c_uint = 0xFFFFFFFFL;
// UMCCH7_3_PerfMonCtr8_Hi
pub const UMCCH7_3_PerfMonCtr8_Hi__Data__SHIFT: c_uint = 0x0;
pub const UMCCH7_3_PerfMonCtr8_Hi__Overflow__SHIFT: c_uint = 0x10;
pub const UMCCH7_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: c_uint = 0x12;
pub const UMCCH7_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: c_uint = 0x14;
pub const UMCCH7_3_PerfMonCtr8_Hi__Data_MASK: c_uint = 0x0000FFFFL;
pub const UMCCH7_3_PerfMonCtr8_Hi__Overflow_MASK: c_uint = 0x00010000L;
pub const UMCCH7_3_PerfMonCtr8_Hi__ThreshCntEn_MASK: c_uint = 0x000C0000L;
pub const UMCCH7_3_PerfMonCtr8_Hi__ThreshCnt_MASK: c_uint = 0xFFF00000L;
