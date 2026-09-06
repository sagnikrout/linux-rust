//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_pps_regs.h
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
// Copyright © 2023 Intel Corporation
//

// Panel power sequencing
pub const PPS_BASE: c_uint = 0x61200;

pub const PCH_PPS_BASE: c_uint = 0xC7200;

pub const _PP_STATUS: c_uint = 0x61200;

//
// Indicates that all dependencies of the panel are on:
//
// - PLL enabled
// - pipe enabled
// - LVDS/DVOB/DVOC on
//

pub const _PP_CONTROL: c_uint = 0x61204;

pub const _PP_ON_DELAYS: c_uint = 0x61208;

pub const _PP_OFF_DELAYS: c_uint = 0x6120C;

pub const _PP_DIVISOR: c_uint = 0x61210;

