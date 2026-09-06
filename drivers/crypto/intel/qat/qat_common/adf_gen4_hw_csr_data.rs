//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen4_hw_csr_data.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2024 Intel Corporation

pub const ADF_BANK_INT_SRC_SEL_MASK: c_uint = 0x44UL;
pub const ADF_RING_CSR_RING_CONFIG: c_uint = 0x1000;
pub const ADF_RING_CSR_RING_LBASE: c_uint = 0x1040;
pub const ADF_RING_CSR_RING_UBASE: c_uint = 0x1080;
pub const ADF_RING_CSR_RING_HEAD: c_uint = 0x0C0;
pub const ADF_RING_CSR_RING_TAIL: c_uint = 0x100;
pub const ADF_RING_CSR_STAT: c_uint = 0x140;
pub const ADF_RING_CSR_UO_STAT: c_uint = 0x148;
pub const ADF_RING_CSR_E_STAT: c_uint = 0x14C;
pub const ADF_RING_CSR_NE_STAT: c_uint = 0x150;
pub const ADF_RING_CSR_NF_STAT: c_uint = 0x154;
pub const ADF_RING_CSR_F_STAT: c_uint = 0x158;
pub const ADF_RING_CSR_C_STAT: c_uint = 0x15C;
pub const ADF_RING_CSR_INT_FLAG_EN: c_uint = 0x16C;
pub const ADF_RING_CSR_INT_FLAG: c_uint = 0x170;
pub const ADF_RING_CSR_INT_SRCSEL: c_uint = 0x174;
pub const ADF_RING_CSR_INT_COL_EN: c_uint = 0x17C;
pub const ADF_RING_CSR_INT_COL_CTL: c_uint = 0x180;
pub const ADF_RING_CSR_INT_FLAG_AND_COL: c_uint = 0x184;
pub const ADF_RING_CSR_EXP_STAT: c_uint = 0x188;
pub const ADF_RING_CSR_EXP_INT_EN: c_uint = 0x18C;
pub const ADF_RING_CSR_INT_COL_CTL_ENABLE: c_uint = 0x80000000;
pub const ADF_RING_CSR_ADDR_OFFSET: c_uint = 0x100000;
pub const ADF_RING_BUNDLE_SIZE: c_uint = 0x2000;
pub const ADF_RING_CSR_RING_SRV_ARB_EN: c_uint = 0x19C;

//
// Use special IO wrapper for ring base as LBASE and UBASE are
// not physically contigious
//

extern "C" {
    pub fn adf_gen4_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops);
}
