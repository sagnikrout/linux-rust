//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/trace_dbg.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2018 Intel Corporation.
//

//
// Note:
// This produces a REALLY ugly trace in the console output when the string is
// too long.
//

pub const MAX_MSG_LEN: c_int = 512;

//
// It may be nice to macroize the __hfi1_trace but the va_* stuff requires an
// actual function to work and can not be in a macro.
//

//
// To create a new trace level simply define it below and as a __hfi1_trace_fn
// in trace.c. This will create all the hooks for calling
// hfi1_cdbg(LVL, fmt, ...); as well as take care of all
// the debugfs stuff.
//

//
// Define HFI1_EARLY_DBG at compile time or here to enable early trace
// messages. Do not check in an enablement for this.
//

