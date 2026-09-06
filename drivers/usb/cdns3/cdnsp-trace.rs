//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/cdns3/cdnsp-trace.h
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
// Cadence CDNSP DRD Driver.
// Trace support header file
//
// Copyright (C) 2020 Cadence.
//
// Author: Pawel Laszczak <pawell@cadence.com>
//

//
// The TRACE_SYSTEM_VAR defaults to TRACE_SYSTEM, but must be a
// legitimate C variable. It is not exported to user space.
//

//
// There is limitation for single buffer size in TRACEPOINT subsystem.
// By default TRACE_BUF_SIZE is 1024, so no all data will be logged.
// To show more data this must be increased. In most cases the default
// value is sufficient.
//
pub const CDNSP_MSG_MAX: c_int = 500;

// this part must be outside header guard

