//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/papr_scm.h
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
// DIMM health bitmap indicators
// SCM device is unable to persist memory contents

// SCM device failed to persist memory contents

// SCM device contents are persisted from previous IPL

// SCM device contents are not persisted from previous IPL

// SCM device memory life remaining is critically low

// SCM device will be garded off next IPL due to failure

// SCM contents cannot persist due to current platform health status

// SCM device is unable to persist memory contents in certain conditions

// SCM device is encrypted

// SCM device has been scrubbed and locked

// Bits status indicators for health bitmap indicating unarmed dimm

// Bits status indicators for health bitmap indicating unflushed dimm

// Bits status indicators for health bitmap indicating unrestored dimm

// Bit status indicators for smart event notification

pub const PAPR_SCM_PERF_STATS_VERSION: c_uint = 0x1;
