//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/nomadik.h
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
//
// nomadik.h
//
// Copyright (C) ST-Ericsson SA 2013
// Author: Gabriel Fernandez <gabriel.fernandez@st.com> for ST-Ericsson.
//
pub const INPUT_NOPULL: c_int = 0;
pub const INPUT_PULLUP: c_int = 1;
pub const INPUT_PULLDOWN: c_int = 2;
pub const OUTPUT_LOW: c_int = 0;
pub const OUTPUT_HIGH: c_int = 1;
pub const DIR_OUTPUT: c_int = 2;
pub const SLPM_DISABLED: c_int = 0;
pub const SLPM_ENABLED: c_int = 1;
pub const SLPM_INPUT_NOPULL: c_int = 0;
pub const SLPM_INPUT_PULLUP: c_int = 1;
pub const SLPM_INPUT_PULLDOWN: c_int = 2;
pub const SLPM_DIR_INPUT: c_int = 3;
pub const SLPM_OUTPUT_LOW: c_int = 0;
pub const SLPM_OUTPUT_HIGH: c_int = 1;
pub const SLPM_DIR_OUTPUT: c_int = 2;
pub const SLPM_WAKEUP_DISABLE: c_int = 0;
pub const SLPM_WAKEUP_ENABLE: c_int = 1;
pub const GPIOMODE_DISABLED: c_int = 0;
pub const GPIOMODE_ENABLED: c_int = 1;
pub const SLPM_PDIS_DISABLED: c_int = 0;
pub const SLPM_PDIS_ENABLED: c_int = 1;
