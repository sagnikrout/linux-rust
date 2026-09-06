//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/dm814x.h
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
//
// This header provides constants specific to DM814X pinctrl bindings.
//

//
// Note that dm814x silicon revision 2.1 and older require input enabled
// (bit 18 set) for all 3.3V I/Os to avoid cumulative hardware damage. For
// more info, see errata advisory 2.1.87. We leave bit 18 out of
// function-mask in dm814x.h and rely on the bootloader for it.
//

// update macro depending on INPUT_EN and PULL_ENA

pub const PIN_OUTPUT_PULLDOWN: c_int = 0;

// undef non-existing modes

