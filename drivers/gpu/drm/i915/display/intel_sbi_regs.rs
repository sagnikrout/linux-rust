//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_sbi_regs.h
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


// SPDX-License-Identifier: MIT
// Copyright © 2025 Intel Corporation

//
// Sideband Interface (SBI) is programmed indirectly, via SBI_ADDR, which
// contains the register offset; and SBI_DATA, which contains the payload.
//

// SBI offsets
pub const SBI_SSCDIVINTPHASE: c_uint = 0x0200;
pub const SBI_SSCDIVINTPHASE6: c_uint = 0x0600;
pub const SBI_SSCDIVINTPHASE_DIVSEL_SHIFT: c_int = 1;

pub const SBI_SSCDIVINTPHASE_INCVAL_SHIFT: c_int = 8;

pub const SBI_SSCDITHPHASE: c_uint = 0x0204;
pub const SBI_SSCCTL: c_uint = 0x020c;
pub const SBI_SSCCTL6: c_uint = 0x060C;

pub const SBI_SSCAUXDIV6: c_uint = 0x0610;
pub const SBI_SSCAUXDIV_FINALDIV2SEL_SHIFT: c_int = 4;

pub const SBI_DBUFF0: c_uint = 0x2a00;
pub const SBI_GEN0: c_uint = 0x1f00;

