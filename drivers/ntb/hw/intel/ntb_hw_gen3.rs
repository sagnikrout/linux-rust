//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ntb/hw/intel/ntb_hw_gen3.h
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
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2012-2017 Intel Corporation. All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// BSD LICENSE
//
// Copyright(c) 2012-2017 Intel Corporation. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copy
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// Intel Skylake Xeon hardware
pub const GEN3_IMBAR1SZ_OFFSET: c_uint = 0x00d0;
pub const GEN3_IMBAR2SZ_OFFSET: c_uint = 0x00d1;
pub const GEN3_EMBAR1SZ_OFFSET: c_uint = 0x00d2;
pub const GEN3_EMBAR2SZ_OFFSET: c_uint = 0x00d3;
pub const GEN3_DEVCTRL_OFFSET: c_uint = 0x0098;
pub const GEN3_DEVSTS_OFFSET: c_uint = 0x009a;
pub const GEN3_UNCERRSTS_OFFSET: c_uint = 0x014c;
pub const GEN3_CORERRSTS_OFFSET: c_uint = 0x0158;
pub const GEN3_LINK_STATUS_OFFSET: c_uint = 0x01a2;
pub const GEN3_NTBCNTL_OFFSET: c_uint = 0x0000;
pub const GEN3_IMBAR1XBASE_OFFSET: c_uint = 0x0010		/* SBAR2XLAT */;
pub const GEN3_IMBAR1XLMT_OFFSET: c_uint = 0x0018		/* SBAR2LMT */;
pub const GEN3_IMBAR2XBASE_OFFSET: c_uint = 0x0020		/* SBAR4XLAT */;
pub const GEN3_IMBAR2XLMT_OFFSET: c_uint = 0x0028		/* SBAR4LMT */;
pub const GEN3_IM_INT_STATUS_OFFSET: c_uint = 0x0040;
pub const GEN3_IM_INT_DISABLE_OFFSET: c_uint = 0x0048;
pub const GEN3_IM_SPAD_OFFSET: c_uint = 0x0080		/* SPAD */;
pub const GEN3_USMEMMISS_OFFSET: c_uint = 0x0070;
pub const GEN3_INTVEC_OFFSET: c_uint = 0x00d0;
pub const GEN3_IM_DOORBELL_OFFSET: c_uint = 0x0100		/* SDOORBELL0 */;
pub const GEN3_B2B_SPAD_OFFSET: c_uint = 0x0180		/* B2B SPAD */;
pub const GEN3_EMBAR0XBASE_OFFSET: c_uint = 0x4008		/* B2B_XLAT */;
pub const GEN3_EMBAR1XBASE_OFFSET: c_uint = 0x4010		/* PBAR2XLAT */;
pub const GEN3_EMBAR1XLMT_OFFSET: c_uint = 0x4018		/* PBAR2LMT */;
pub const GEN3_EMBAR2XBASE_OFFSET: c_uint = 0x4020		/* PBAR4XLAT */;
pub const GEN3_EMBAR2XLMT_OFFSET: c_uint = 0x4028		/* PBAR4LMT */;
pub const GEN3_EM_INT_STATUS_OFFSET: c_uint = 0x4040;
pub const GEN3_EM_INT_DISABLE_OFFSET: c_uint = 0x4048;
pub const GEN3_EM_SPAD_OFFSET: c_uint = 0x4080		/* remote SPAD */;
pub const GEN3_EM_DOORBELL_OFFSET: c_uint = 0x4100		/* PDOORBELL0 */;
pub const GEN3_SPCICMD_OFFSET: c_uint = 0x4504		/* SPCICMD */;
pub const GEN3_EMBAR0_OFFSET: c_uint = 0x4510		/* SBAR0BASE */;
pub const GEN3_EMBAR1_OFFSET: c_uint = 0x4518		/* SBAR23BASE */;
pub const GEN3_EMBAR2_OFFSET: c_uint = 0x4520		/* SBAR45BASE */;
pub const GEN3_DB_COUNT: c_int = 32;
pub const GEN3_DB_LINK: c_int = 32;

pub const GEN3_DB_MSIX_VECTOR_COUNT: c_int = 33;
pub const GEN3_DB_MSIX_VECTOR_SHIFT: c_int = 1;
pub const GEN3_DB_TOTAL_SHIFT: c_int = 33;
pub const GEN3_SPAD_COUNT: c_int = 16;
extern "C" {
    pub fn ioread64(_arg: mmio) -> return;
}
extern "C" {
    pub fn gen3_init_dev(ndev: *mut intel_ntb_dev) -> c_int;
}
extern "C" {
    pub fn intel_ntb3_db_read(ntb: *mut ntb_dev) -> u64;
}
extern "C" {
    pub fn intel_ntb3_db_clear(ntb: *mut ntb_dev, db_bits: u64) -> c_int;
}
extern "C" {
    pub fn intel_ntb3_peer_db_set(ntb: *mut ntb_dev, db_bits: u64) -> c_int;
}
