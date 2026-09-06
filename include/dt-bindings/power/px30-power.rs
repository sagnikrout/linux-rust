//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/px30-power.h
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


// SPDX-License-Identifier: GPL-2.0
// VD_CORE
pub const PX30_PD_A35_0: c_int = 0;
pub const PX30_PD_A35_1: c_int = 1;
pub const PX30_PD_A35_2: c_int = 2;
pub const PX30_PD_A35_3: c_int = 3;
pub const PX30_PD_SCU: c_int = 4;
// VD_LOGIC
pub const PX30_PD_USB: c_int = 5;
pub const PX30_PD_DDR: c_int = 6;
pub const PX30_PD_SDCARD: c_int = 7;
pub const PX30_PD_CRYPTO: c_int = 8;
pub const PX30_PD_GMAC: c_int = 9;
pub const PX30_PD_MMC_NAND: c_int = 10;
pub const PX30_PD_VPU: c_int = 11;
pub const PX30_PD_VO: c_int = 12;
pub const PX30_PD_VI: c_int = 13;
pub const PX30_PD_GPU: c_int = 14;
// VD_PMU
pub const PX30_PD_PMU: c_int = 15;
