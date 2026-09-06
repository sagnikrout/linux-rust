//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/ipi.h
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
// ipi_raise - called when a smp cross call is made
//
// @mask: mask of recipient CPUs for the IPI
// @reason: string identifying the IPI purpose
//
// It is necessary for @reason to be a static string declared with
// __tracepoint_string.
//
// ipi_entry - called immediately before the IPI handler
//
// @reason: string identifying the IPI purpose
//
// It is necessary for @reason to be a static string declared with
// __tracepoint_string, ideally the same as used with trace_ipi_raise
// for that IPI.
//
// ipi_exit - called immediately after the IPI handler returns
//
// @reason: string identifying the IPI purpose
//
// It is necessary for @reason to be a static string declared with
// __tracepoint_string, ideally the same as used with trace_ipi_raise for
// that IPI.
//

// This part must be outside protection
