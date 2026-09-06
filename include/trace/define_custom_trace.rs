//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/define_custom_trace.h
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
// Trace files that want to automate creation of all tracepoints defined
// in their file should include this file. The following are macros that the
// trace file may define:
//
// TRACE_SYSTEM defines the system the tracepoint is for
//
// TRACE_INCLUDE_FILE if the file name is something other than TRACE_SYSTEM.h
// This macro may be defined to tell define_trace.h what file to include.
// Note, leave off the ".h".
//
// TRACE_INCLUDE_PATH if the path is something other than core kernel include/trace
// then this macro can define the path to use. Note, the path is relative to
// define_trace.h, not the file including it. Full path names for out of tree
// modules must be used.
//

// Prevent recursion

// Let the trace headers be reread
// Macro flag: #define TRACE_CUSTOM_MULTI_READ

// Only undef what we defined in this file

// We may be processing more files
// Macro flag: #define CREATE_CUSTOM_TRACE_POINTS
