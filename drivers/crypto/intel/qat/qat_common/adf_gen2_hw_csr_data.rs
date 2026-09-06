//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_gen2_hw_csr_data.h
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

pub const ADF_BANK_INT_SRC_SEL_MASK_0: c_uint = 0x4444444CUL;
pub const ADF_BANK_INT_SRC_SEL_MASK_X: c_uint = 0x44444444UL;
pub const ADF_RING_CSR_RING_CONFIG: c_uint = 0x000;
pub const ADF_RING_CSR_RING_LBASE: c_uint = 0x040;
pub const ADF_RING_CSR_RING_UBASE: c_uint = 0x080;
pub const ADF_RING_CSR_RING_HEAD: c_uint = 0x0C0;
pub const ADF_RING_CSR_RING_TAIL: c_uint = 0x100;
pub const ADF_RING_CSR_E_STAT: c_uint = 0x14C;
pub const ADF_RING_CSR_INT_FLAG: c_uint = 0x170;
pub const ADF_RING_CSR_INT_SRCSEL: c_uint = 0x174;
pub const ADF_RING_CSR_INT_SRCSEL_2: c_uint = 0x178;
pub const ADF_RING_CSR_INT_COL_EN: c_uint = 0x17C;
pub const ADF_RING_CSR_INT_COL_CTL: c_uint = 0x180;
pub const ADF_RING_CSR_INT_FLAG_AND_COL: c_uint = 0x184;
pub const ADF_RING_CSR_INT_COL_CTL_ENABLE: c_uint = 0x80000000;
pub const ADF_RING_BUNDLE_SIZE: c_uint = 0x1000;
pub const ADF_ARB_REG_SLOT: c_uint = 0x1000;
pub const ADF_ARB_RINGSRVARBEN_OFFSET: c_uint = 0x19C;

extern "C" {
    pub fn adf_gen2_init_hw_csr_ops(csr_ops: *mut adf_hw_csr_ops);
}
