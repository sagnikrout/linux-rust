//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/counter/microchip-tcb-capture.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Channel numbers used by the microchip-tcb-capture driver
// Copyright (C) 2025 Bence Csókás
//
// The driver defines the following components:
//
// Count 0
// \__  Synapse 0 -- Signal 0 (Channel A, i.e. TIOA)
// \__  Synapse 1 -- Signal 1 (Channel B, i.e. TIOB)
// \__  Extension capture0    (RA register)
// \__  Extension capture1    (RB register)
//
// It also supports the following events:
//
// Channel 0:
// - CV register changed
// - CV overflowed
// - RA captured
// Channel 1:
// - RB captured
// Channel 2:
// - RC compare triggered
//
// Capture extensions
pub const COUNTER_MCHP_EXCAP_RA: c_int = 0;
pub const COUNTER_MCHP_EXCAP_RB: c_int = 1;
// Event channels
pub const COUNTER_MCHP_EVCHN_CV: c_int = 0;
pub const COUNTER_MCHP_EVCHN_RA: c_int = 0;
pub const COUNTER_MCHP_EVCHN_RB: c_int = 1;
pub const COUNTER_MCHP_EVCHN_RC: c_int = 2;
