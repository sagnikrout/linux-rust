//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_lvds_regs.h
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

// LVDS port control

//
// Enables the LVDS port.  This bit must be set before DPLLs are enabled, as
// the DPLL semantics change when the LVDS is assigned to that pipe.
//

// Selects pipe B for LVDS data.  Must be set on pre-965.

// LVDS dithering flag on 965/g4x platform

// LVDS sync polarity flags. Set to invert (i.e. negative)

// Enable border for unscaled (or aspect-scaled) display

//
// Enables the A0-A2 data pairs and CLKA, containing 18 bits of color data per
// pixel.
//

//
// Controls the A3 data pair, which contains the additional LSBs for 24 bit
// mode.  Only enabled if LVDS_A0A2_CLKA_POWER_UP also indicates it should be
// on.
//

//
// Controls the CLKB pair.  This should only be set when LVDS_B0B3_POWER_UP
// is set.
//

//
// Controls the B0-B3 data pairs.  This must be set to match the DPLL p2
// setting for whether we are in dual-channel mode.  The B3 pair will
// additionally only be powered up when LVDS_A3_POWER_UP is set.
//

