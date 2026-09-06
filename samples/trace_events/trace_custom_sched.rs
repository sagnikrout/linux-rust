//! Automatically rewritten from C Header to Rust Module
//! Source: samples/trace_events/trace_custom_sched.h
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
// Like the headers that use TRACE_EVENT(), the TRACE_CUSTOM_EVENT()
// needs a header that allows for multiple inclusions.
//
// Test for a unique name (here we have _TRACE_CUSTOM_SCHED_H),
// also allowing to continue if TRACE_CUSTOM_MULTI_READ is defined.
//

// Include linux/trace_events.h for initial defines of TRACE_CUSTOM_EVENT()

//
// TRACE_CUSTOM_EVENT() is just like TRACE_EVENT(). The first parameter
// is the event name of an existing event where the TRACE_EVENT has been included
// in the C file before including this file.
//
// The TP_PROTO() and TP_ARGS must match the trace event
// that the custom event is using.
//
// The next fields are where the customization happens.
// The TP_STRUCT__entry() defines what will be recorded
// in the ring buffer when the custom event triggers.
//
// The rest is just like the TRACE_EVENT() macro except that
// it uses the custom entry.
//

//
// Just like the headers that create TRACE_EVENTs, the below must
// be outside the protection of the above #if block.
//
// It is required that the Makefile includes:
// CFLAGS_<c_file>.o := -I$(src)
//

//
// It is requred that the TRACE_INCLUDE_FILE be the same
// as this file without the ".h".
//

