//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_mocs.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2015 Intel Corporation
//
// DOC: Memory Objects Control State (MOCS)
//
// Motivation:
// In previous Gens the MOCS settings was a value that was set by user land as
// part of the batch. In Gen9 this has changed to be a single table (per ring)
// that all batches now reference by index instead of programming the MOCS
// directly.
//
// The one wrinkle in this is that only PART of the MOCS tables are included
// in context (The GFX_MOCS_0 - GFX_MOCS_64 and the LNCFCMOCS0 - LNCFCMOCS32
// registers). The rest are not (the settings for the other rings).
//
// This table needs to be set at system start-up because the way the table
// interacts with the contexts and the GmmLib interface.
//
// Implementation:
//
// The tables (one per supported platform) are defined in intel_mocs.c
// and are programmed in the first batch after the context is loaded
// (with the hardware workarounds). This will then let the usual
// context handling keep the MOCS in step.
//
extern "C" {
    pub fn intel_mocs_init(gt: *mut intel_gt);
}
extern "C" {
    pub fn intel_mocs_init_engine(engine: *mut intel_engine_cs);
}
extern "C" {
    pub fn intel_set_mocs_index(gt: *mut intel_gt);
}
