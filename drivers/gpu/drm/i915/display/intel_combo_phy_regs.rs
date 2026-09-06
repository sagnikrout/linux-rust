//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_combo_phy_regs.h
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

pub const _ICL_COMBOPHY_A: c_uint = 0x162000;
pub const _ICL_COMBOPHY_B: c_uint = 0x6C000;
pub const _EHL_COMBOPHY_C: c_uint = 0x160000;
pub const _RKL_COMBOPHY_D: c_uint = 0x161000;
pub const _ADL_COMBOPHY_E: c_uint = 0x16B000;

// ICL Port CL_DW registers

// ICL Port COMP_DW registers
pub const _ICL_PORT_COMP: c_uint = 0x100;

// ICL Port PCS registers
pub const _ICL_PORT_PCS_AUX: c_uint = 0x300;
pub const _ICL_PORT_PCS_GRP: c_uint = 0x600;

// ICL Port TX registers
pub const _ICL_PORT_TX_AUX: c_uint = 0x380;
pub const _ICL_PORT_TX_GRP: c_uint = 0x680;

pub const _ICL_DPHY_CHKN_REG: c_uint = 0x194;

