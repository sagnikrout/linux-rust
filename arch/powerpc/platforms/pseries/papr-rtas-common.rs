//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/pseries/papr-rtas-common.h
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
// Return codes for sequence based RTAS calls.
// Not listed under PAPR+ v2.13 7.2.8: "Return Codes".
// But defined in the specific section of each RTAS call.
//

//
// Internal "blob" APIs for accumulating RTAS call results into
// an immutable buffer to be attached to a file descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_rtas_blob {
    pub data: *const c_char,
    pub len: usize,
}

//
// struct papr_sequence - State for managing a sequence of RTAS calls.
// @error:  Shall be zero as long as the sequence has not encountered an error,
// -ve errno otherwise. Use papr_rtas_sequence_set_err() to update.
// @params: Parameter block to pass to rtas_*() calls.
// @begin: Work area allocation and initialize the needed parameter
// values passed to RTAS call
// @end: Free the allocated work area
// @work: Obtain data with RTAS call and invoke it until the sequence is
// completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct papr_rtas_sequence {
    pub error: c_int,
    pub params: *mut c_void,
    pub seq): *mut *mut void (begin)(struct papr_rtas_sequence,
    pub seq): *mut *mut void (end)(struct papr_rtas_sequence,
    pub len): *const *const *const *const char (work)(struct papr_rtas_sequence seq, size_t,
}

extern "C" {
    pub fn papr_rtas_blob_has_data(blob: *const papr_rtas_blob) -> bool;
}
extern "C" {
    pub fn papr_rtas_blob_free(blob: *const papr_rtas_blob);
}
