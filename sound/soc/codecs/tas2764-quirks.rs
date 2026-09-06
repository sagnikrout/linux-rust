//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas2764-quirks.h
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

// Bitmask of enabled Apple quirks
pub const ENABLED_APPLE_QUIRKS: c_uint = 0x3f;
//
// Disable noise gate and flip down reserved bit in NS_CFG0
//

//
// CONV_VBAT_PVDD_MODE=1
//

//
// Reset of DAC modulator when DSP is OFF
//

//
// Unknown 0x133/0x137 writes (maybe TDM related)
//

//
// Unknown 0x614 - 0x61f writes
//

//
// Unknown writes in the 0xfd page (with secondary paging inside)
//

//
// Disable 'Thermal Threshold 1'
//

//
// Imitate Apple's shutdown dance
//

//
// SDZ_MODE=01 (immediate)
//
// We want the shutdown to happen under the influence of
// the magic writes in the 0xfdXX region, so make sure
// the shutdown is immediate and there's no grace period
// followed by the codec part.
//
// Handle power state transition to shutdown
//
// Via devicetree (TODO):
// - switch from spread spectrum to class-D switching
// - disable edge control
// - set BOP settings (the BOP config bits *and* BOP_SRC)
//
// Other setup TODOs:
// - DVC ramp rate
//
