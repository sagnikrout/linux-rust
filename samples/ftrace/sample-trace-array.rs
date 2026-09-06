//! Automatically rewritten from C Header to Rust Module
//! Source: samples/ftrace/sample-trace-array.h
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
// If TRACE_SYSTEM is defined, that will be the directory created
// in the ftrace directory under /sys/kernel/tracing/events/<system>
//
// The define_trace.h below will also look for a file name of
// TRACE_SYSTEM.h where TRACE_SYSTEM is what is defined here.
// In this case, it would look for sample-trace.h
//
// If the header name will be different than the system name
// (as in this case), then you can override the header name that
// define_trace.h will look up by defining TRACE_INCLUDE_FILE
//
// This file is called sample-trace-array.h but we want the system
// to be called "sample-subsystem". Therefore we must define the name of this
// file:
//
// #define TRACE_INCLUDE_FILE sample-trace-array
//
// As we do in the bottom of this file.
//
// Notice that TRACE_SYSTEM should be defined outside of #if
// protection, just like TRACE_INCLUDE_FILE.
//

//
// TRACE_SYSTEM is expected to be a C valid variable (alpha-numeric
// and underscore), although it may start with numbers. If for some
// reason it is not, you need to add the following lines:
//

//
// But the above is only needed if TRACE_SYSTEM is not alpha-numeric
// and underscored. By default, TRACE_SYSTEM_VAR will be equal to
// TRACE_SYSTEM. As TRACE_SYSTEM_VAR must be alpha-numeric, if
// TRACE_SYSTEM is not, then TRACE_SYSTEM_VAR must be defined with
// only alpha-numeric and underscores.
//
// The TRACE_SYSTEM_VAR is only used internally and not visible to
// user space.
//
// Notice that this file is not protected like a normal header.
// We also must allow for rereading of this file. The
//
// || defined(TRACE_HEADER_MULTI_READ)
//
// serves this purpose.
//

