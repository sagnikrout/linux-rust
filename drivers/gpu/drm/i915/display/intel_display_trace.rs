//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_trace.h
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
// Copyright © 2021 Intel Corporation
//

//
// Using identifiers from enum pipe in TP_printk() will confuse tools that
// parse /sys/kernel/debug/tracing/{xe,i915}/<event>/format. So we use CPP
// macros instead.
//
pub const _TRACE_PIPE_A: c_int = 0;
pub const _TRACE_PIPE_B: c_int = 1;
pub const _TRACE_PIPE_C: c_int = 2;
pub const _TRACE_PIPE_D: c_int = 3;
//
// FIXME: Several TP_printk() calls below display frame and scanline numbers for
// all possible pipes (regardless of whether they are available) and that is
// done with a constant format string. A better approach would be to generate
// that info dynamically based on available pipes, but, while we do not have
// that implemented yet, let's assert that the constant format string indeed
// covers all possible pipes.
//

//
// Paranoid sanity check that at least the enumeration starts at the
// same value as _TRACE_PIPE_A.
//

// This part must be outside protection

