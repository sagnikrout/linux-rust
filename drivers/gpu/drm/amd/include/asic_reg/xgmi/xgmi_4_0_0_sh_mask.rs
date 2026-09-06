//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/asic_reg/xgmi/xgmi_4_0_0_sh_mask.h
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

// Macro flag: #define _xgmi_4_0_0_SH_MASK_HEADER
// PCS_GOPX16_PCS_ERROR_STATUS
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DataLossErr__SHIFT: c_uint = 0x0;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__TrainingErr__SHIFT: c_uint = 0x1;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__CRCErr__SHIFT: c_uint = 0x5;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__BERExceededErr__SHIFT: c_uint = 0x6;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__TxMetaDataErr__SHIFT: c_uint = 0x7;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReplayBufParityErr__SHIFT: c_uint = 0x8;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DataParityErr__SHIFT: c_uint = 0x9;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReplayFifoOverflowErr__SHIFT: c_uint = 0xa;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReplayFifoUnderflowErr__SHIFT: c_uint = 0xb;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ElasticFifoOverflowErr__SHIFT: c_uint = 0xc;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DeskewErr__SHIFT: c_uint = 0xd;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DataStartupLimitErr__SHIFT: c_uint = 0xf;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__FCInitTimeoutErr__SHIFT: c_uint = 0x10;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__RecoveryTimeoutErr__SHIFT: c_uint = 0x11;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReadySerialTimeoutErr__SHIFT: c_uint = 0x12;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReadySerialAttemptErr__SHIFT: c_uint = 0x13;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__RecoveryAttemptErr__SHIFT: c_uint = 0x14;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__RecoveryRelockAttemptErr__SHIFT: c_uint = 0x15;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ClearBERAccum__SHIFT: c_uint = 0x17;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__BERAccumulator__SHIFT: c_uint = 0x18;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DataLossErr_MASK: c_uint = 0x00000001L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__TrainingErr_MASK: c_uint = 0x00000002L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__CRCErr_MASK: c_uint = 0x00000020L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__BERExceededErr_MASK: c_uint = 0x00000040L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__TxMetaDataErr_MASK: c_uint = 0x00000080L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReplayBufParityErr_MASK: c_uint = 0x00000100L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DataParityErr_MASK: c_uint = 0x00000200L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReplayFifoOverflowErr_MASK: c_uint = 0x00000400L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReplayFifoUnderflowErr_MASK: c_uint = 0x00000800L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ElasticFifoOverflowErr_MASK: c_uint = 0x00001000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DeskewErr_MASK: c_uint = 0x00002000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__DataStartupLimitErr_MASK: c_uint = 0x00008000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__FCInitTimeoutErr_MASK: c_uint = 0x00010000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__RecoveryTimeoutErr_MASK: c_uint = 0x00020000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReadySerialTimeoutErr_MASK: c_uint = 0x00040000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ReadySerialAttemptErr_MASK: c_uint = 0x00080000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__RecoveryAttemptErr_MASK: c_uint = 0x00100000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__RecoveryRelockAttemptErr_MASK: c_uint = 0x00200000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__ClearBERAccum_MASK: c_uint = 0x00800000L;
pub const XGMI0_PCS_GOPX16_PCS_ERROR_STATUS__BERAccumulator_MASK: c_uint = 0xFF000000L;
