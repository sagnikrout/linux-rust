//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/perf_event_fsl_emb.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Performance event support - Freescale embedded specific definitions.
//
// Copyright 2008-2009 Paul Mackerras, IBM Corporation.
// Copyright 2010 Freescale Semiconductor, Inc.
//

pub const MAX_HWEVENTS: c_int = 6;
// event flags
pub const FSL_EMB_EVENT_VALID: c_int = 1;
pub const FSL_EMB_EVENT_RESTRICTED: c_int = 2;
// upper half of event flags is PMLCb
pub const FSL_EMB_EVENT_THRESHMUL: c_uint = 0x0000070000000000ULL;
pub const FSL_EMB_EVENT_THRESH: c_uint = 0x0000003f00000000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_emb_pmu {
    pub name: *const c_char,
    pub /: *mut *mut int n_counter; / total number of counters,
//
// The number of contiguous counters starting at zero that
// can hold restricted events, or zero if there are no
// restricted events.
//
// This isn't a very flexible method of expressing constraints,
// but it's very simple and is adequate for existing chips.
//
    pub n_restricted: c_int,
// Returns event flags and PMLCb (FSL_EMB_EVENT_*)
    pub event_id): *mut *mut u64 (xlate_event)(u64,
    pub n_generic: c_int,
    pub generic_events: *mut c_int,
}

extern "C" {
    pub fn register_fsl_emb_pmu(: *mut fsl_emb_pmu) -> c_int;
}
