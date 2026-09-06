//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/xgmi/xgmi_6_1_0_sh_mask.h
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

// Macro flag: #define _xgmi_6_1_0_SH_MASK_HEADER
// PCS_XGMI3X16_PCS_ERROR_STATUS
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataLossErr__SHIFT: c_uint = 0x0;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TrainingErr__SHIFT: c_uint = 0x1;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlAckErr__SHIFT: c_uint = 0x2;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoUnderflowErr__SHIFT: c_uint = 0x3;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoOverflowErr__SHIFT: c_uint = 0x4;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__CRCErr__SHIFT: c_uint = 0x5;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__BERExceededErr__SHIFT: c_uint = 0x6;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxVcidDataErr__SHIFT: c_uint = 0x7;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayBufParityErr__SHIFT: c_uint = 0x8;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataParityErr__SHIFT: c_uint = 0x9;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoOverflowErr__SHIFT: c_uint = 0xa;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoUnderflowErr__SHIFT: c_uint = 0xb;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ElasticFifoOverflowErr__SHIFT: c_uint = 0xc;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DeskewErr__SHIFT: c_uint = 0xd;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlCRCErr__SHIFT: c_uint = 0xe;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataStartupLimitErr__SHIFT: c_uint = 0xf;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FCInitTimeoutErr__SHIFT: c_uint = 0x10;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryTimeoutErr__SHIFT: c_uint = 0x11;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialTimeoutErr__SHIFT: c_uint = 0x12;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialAttemptErr__SHIFT: c_uint = 0x13;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryAttemptErr__SHIFT: c_uint = 0x14;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryRelockAttemptErr__SHIFT: c_uint = 0x15;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayAttemptErr__SHIFT: c_uint = 0x16;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__SyncHdrErr__SHIFT: c_uint = 0x17;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxReplayTimeoutErr__SHIFT: c_uint = 0x18;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxReplayTimeoutErr__SHIFT: c_uint = 0x19;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubTxTimeoutErr__SHIFT: c_uint = 0x1a;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubRxTimeoutErr__SHIFT: c_uint = 0x1b;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxCMDPktErr__SHIFT: c_uint = 0x1c;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataLossErr_MASK: c_uint = 0x00000001L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TrainingErr_MASK: c_uint = 0x00000002L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlAckErr_MASK: c_uint = 0x00000004L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoUnderflowErr_MASK: c_uint = 0x00000008L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxFifoOverflowErr_MASK: c_uint = 0x00000010L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__CRCErr_MASK: c_uint = 0x00000020L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__BERExceededErr_MASK: c_uint = 0x00000040L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxVcidDataErr_MASK: c_uint = 0x00000080L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayBufParityErr_MASK: c_uint = 0x00000100L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataParityErr_MASK: c_uint = 0x00000200L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoOverflowErr_MASK: c_uint = 0x00000400L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayFifoUnderflowErr_MASK: c_uint = 0x00000800L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ElasticFifoOverflowErr_MASK: c_uint = 0x00001000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DeskewErr_MASK: c_uint = 0x00002000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FlowCtrlCRCErr_MASK: c_uint = 0x00004000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__DataStartupLimitErr_MASK: c_uint = 0x00008000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__FCInitTimeoutErr_MASK: c_uint = 0x00010000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryTimeoutErr_MASK: c_uint = 0x00020000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialTimeoutErr_MASK: c_uint = 0x00040000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReadySerialAttemptErr_MASK: c_uint = 0x00080000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryAttemptErr_MASK: c_uint = 0x00100000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RecoveryRelockAttemptErr_MASK: c_uint = 0x00200000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__ReplayAttemptErr_MASK: c_uint = 0x00400000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__SyncHdrErr_MASK: c_uint = 0x00800000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__TxReplayTimeoutErr_MASK: c_uint = 0x01000000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxReplayTimeoutErr_MASK: c_uint = 0x02000000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubTxTimeoutErr_MASK: c_uint = 0x04000000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__LinkSubRxTimeoutErr_MASK: c_uint = 0x08000000L;
pub const PCS_XGMI3X16_PCS_ERROR_STATUS__RxCMDPktErr_MASK: c_uint = 0x10000000L;
