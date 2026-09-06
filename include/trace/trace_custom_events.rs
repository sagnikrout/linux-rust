//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/trace_custom_events.h
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
// This is similar to the trace_events.h file, but is to only
// create custom trace events to be attached to existing tracepoints.
// Where as the TRACE_EVENT() macro (from trace_events.h) will create
// both the trace event and the tracepoint it will attach the event to,
// TRACE_CUSTOM_EVENT() is to create only a custom version of an existing
// trace event (created by TRACE_EVENT() or DEFINE_EVENT()), and will
// be placed in the "custom" system.
//

// All custom events are placed in the custom group

// The init stage creates the system string and enum mappings

// Stage 1 creates the structure of the recorded event layout

// Stage 2 creates the custom class

// Stage 3 create the way to print the custom event

// Stage 4 creates the offset layout for the fields

// Stage 5 creates the helper function for dynamic fields

// Stage 6 creates the probe function that records the event

//
// The ftrace_test_custom_probe is compiled out, it is only here as a build time check
// to make sure that if the tracepoint handling changes, the ftrace probe will
// fail to compile unless it too is updated.
//

// Stage 7 creates the actual class and event structure for the custom event

