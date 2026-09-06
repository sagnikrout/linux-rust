//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/skl_watermark_regs.h
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

pub const _PIPEA_MBUS_DBOX_CTL: c_uint = 0x7003C;
pub const _PIPEB_MBUS_DBOX_CTL: c_uint = 0x7103C;

//
// The below are numbered starting from "S1" on gen11/gen12, but starting
// with display 13, the bspec switches to a 0-based numbering scheme
// (although the addresses stay the same so new S0 = old S1, new S1 = old S2).
// We'll just use the 0-based numbering here for all platforms since it's the
// way things will be named by the hardware team going forward, plus it's more
// consistent with how most of the rest of our registers are named.
//
pub const _DBUF_CTL_S0: c_uint = 0x45008;
pub const _DBUF_CTL_S1: c_uint = 0x44FE8;
pub const _DBUF_CTL_S2: c_uint = 0x44300;
pub const _DBUF_CTL_S3: c_uint = 0x44304;

