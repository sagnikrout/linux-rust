//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/i9xx_plane_regs.h
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
// Copyright © 2024 Intel Corporation
//

pub const _DSPAADDR_VLV: c_uint = 0x7017C /* vlv/chv */;

pub const _DSPACNTR: c_uint = 0x70180;

pub const _DSPAADDR: c_uint = 0x70184 /* pre-i965 */;

pub const _DSPALINOFF: c_uint = 0x70184 /* i965+ */;

pub const _DSPASTRIDE: c_uint = 0x70188;

pub const _DSPAPOS: c_uint = 0x7018C /* pre-g4x */;

pub const _DSPASIZE: c_uint = 0x70190 /* pre-g4x */;

pub const _DSPASURF: c_uint = 0x7019C /* i965+ */;

pub const _DSPATILEOFF: c_uint = 0x701A4 /* i965+ */;

pub const _DSPAOFFSET: c_uint = 0x701A4 /* hsw+ */;

pub const _DSPASURFLIVE: c_uint = 0x701AC /* g4x+ */;

pub const _DSPAGAMC: c_uint = 0x701E0 /* pre-g4x */;

// CHV pipe B primary plane
pub const _PRIMPOS_A: c_uint = 0x60a08;

pub const _PRIMSIZE_A: c_uint = 0x60a0c;

pub const _PRIMCNSTALPHA_A: c_uint = 0x60a10;

