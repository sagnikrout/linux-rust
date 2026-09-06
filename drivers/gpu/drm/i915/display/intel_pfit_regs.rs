//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_pfit_regs.h
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

// Panel fitting

// CPU panel fitter
// IVB+ has 3 fitters, 0 is 7x5 capable, the other two only 3x3
pub const _PFA_CTL_1: c_uint = 0x68080;
pub const _PFB_CTL_1: c_uint = 0x68880;

pub const _PFA_WIN_SZ: c_uint = 0x68074;
pub const _PFB_WIN_SZ: c_uint = 0x68874;

pub const _PFA_WIN_POS: c_uint = 0x68070;
pub const _PFB_WIN_POS: c_uint = 0x68870;

pub const _PFA_VSCALE: c_uint = 0x68084;
pub const _PFB_VSCALE: c_uint = 0x68884;

pub const _PFA_HSCALE: c_uint = 0x68090;
pub const _PFB_HSCALE: c_uint = 0x68890;

