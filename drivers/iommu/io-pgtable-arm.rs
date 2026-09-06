//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/io-pgtable-arm.h
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
pub const ARM_LPAE_TCR_TG0_4K: c_int = 0;
pub const ARM_LPAE_TCR_TG0_64K: c_int = 1;
pub const ARM_LPAE_TCR_TG0_16K: c_int = 2;
pub const ARM_LPAE_TCR_TG1_16K: c_int = 1;
pub const ARM_LPAE_TCR_TG1_4K: c_int = 2;
pub const ARM_LPAE_TCR_TG1_64K: c_int = 3;
pub const ARM_LPAE_TCR_SH_NS: c_int = 0;
pub const ARM_LPAE_TCR_SH_OS: c_int = 2;
pub const ARM_LPAE_TCR_SH_IS: c_int = 3;
pub const ARM_LPAE_TCR_RGN_NC: c_int = 0;
pub const ARM_LPAE_TCR_RGN_WBWA: c_int = 1;
pub const ARM_LPAE_TCR_RGN_WT: c_int = 2;
pub const ARM_LPAE_TCR_RGN_WB: c_int = 3;
pub const ARM_LPAE_TCR_PS_32_BIT: c_uint = 0x0ULL;
pub const ARM_LPAE_TCR_PS_36_BIT: c_uint = 0x1ULL;
pub const ARM_LPAE_TCR_PS_40_BIT: c_uint = 0x2ULL;
pub const ARM_LPAE_TCR_PS_42_BIT: c_uint = 0x3ULL;
pub const ARM_LPAE_TCR_PS_44_BIT: c_uint = 0x4ULL;
pub const ARM_LPAE_TCR_PS_48_BIT: c_uint = 0x5ULL;
pub const ARM_LPAE_TCR_PS_52_BIT: c_uint = 0x6ULL;
