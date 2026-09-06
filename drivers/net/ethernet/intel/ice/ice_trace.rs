//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_trace.h
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
// Copyright (C) 2021 Intel Corporation.
// Modeled on trace-events-sample.h
// The trace subsystem name for ice will be "ice".
//
// This file is named ice_trace.h.
//
// Since this include file's name is different from the trace
// subsystem name, we'll have to define TRACE_INCLUDE_FILE at the end
// of this file.
//

// See trace-events-sample.h for a detailed description of why this
// guard clause is different from most normal include files.
//

// ice_trace() macro enables shared code to refer to trace points
// like:
//
// trace_ice_example(args...)
//
// ... as:
//
// ice_trace(example, args...)
//
// ... to resolve to the PF version of the tracepoint without
// ifdefs, and to allow tracepoints to be disabled entirely at build
// time.
//
// Trace point should always be referred to in the driver via this
// macro.
//
// Similarly, ice_trace_enabled(trace_name) wraps references to
// trace_ice_<trace_name>_enabled() functions.
// @trace_name: name of tracepoint
//

// This is for events common to PF. Corresponding versions will be named
// trace_ice_*. The ice_trace() macro above will select the right trace point
// name for the driver.
//
// Begin tracepoints
// Global tracepoints
// Events related to DIM, q_vectors and ring containers
// Events related to a vsi & ring

pub const ICE_ESW_BR_PORT_NAME_L: c_int = 16;
// End tracepoints

// This must be outside ifdef _ICE_TRACE_H
// This trace include file is not located in the .../include/trace
// with the kernel tracepoint definitions, because we're a loadable
// module.
//

