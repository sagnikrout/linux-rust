//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/pci_config.h
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
//
// Copyright © 2022 Intel Corporation
//
// PCI BARs
pub const GEN2_GMADR_BAR: c_int = 0;

pub const GEN3_IO_BAR: c_int = 1;
pub const GEN3_GMADR_BAR: c_int = 2;

pub const GEN4_GMADR_BAR: c_int = 2;
pub const GEN4_IO_BAR: c_int = 4;
pub const GEN12_LMEM_BAR: c_int = 2;
// BSM in include/drm/intel/i915_drm.h
pub const MCHBAR_I915: c_uint = 0x44;
pub const MCHBAR_I965: c_uint = 0x48;

pub const DEVEN: c_uint = 0x54;

pub const HPLLCC: c_uint = 0xc0 /* 85x only */;

pub const I915_GDRST: c_uint = 0xc0;

// BSpec only has register offset, PCI device and bit found empirically
pub const I830_CLOCK_GATE: c_uint = 0xc8 /* device 0 */;

pub const GCDGMBUS: c_uint = 0xcc;
pub const GCFGC2: c_uint = 0xda;
pub const GCFGC: c_uint = 0xf0 /* 915+ only */;

pub const ASLE: c_uint = 0xe4;
pub const ASLS: c_uint = 0xfc;
pub const SWSCI: c_uint = 0xe8;

// legacy/combination backlight modes, also called LBB
pub const LBPC: c_uint = 0xf4;
