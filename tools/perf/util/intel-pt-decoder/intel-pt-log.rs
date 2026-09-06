//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/intel-pt-decoder/intel-pt-log.h
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
//
// intel_pt_log.h: Intel Processor Trace support
// Copyright (c) 2013-2014, Intel Corporation.
//

// Macro flag: #define INCLUDE__INTEL_PT_LOG_H__

extern "C" {
    pub fn intel_pt_log_enable(dump_log_on_error: bool, log_on_error_size: c_uint);
}
extern "C" {
    pub fn intel_pt_log_disable();
}
extern "C" {
    pub fn intel_pt_log_set_name(name: *const c_char);
}
extern "C" {
    pub fn intel_pt_log_dump_buf();
}
extern "C" {
    pub fn __intel_pt_log_insn(intel_pt_insn: *mut intel_pt_insn, ip: u64);
}
extern "C" {
    pub fn __intel_pt_log(fmt: *const c_char, __printf(1: ...), _arg: 2);
}

