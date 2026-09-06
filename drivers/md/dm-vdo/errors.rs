//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/errors.h
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
// Copyright 2023 Red Hat
//

// Custom error codes and error-related utilities
pub const VDO_SUCCESS: c_int = 0;
// Valid status codes for internal UDS functions.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uds_status_codes {
// Successful return
    UDS_SUCCESS = VDO_SUCCESS,
// Used as a base value for reporting internal errors
    UDS_ERROR_CODE_BASE = 1024,
// Index overflow
    UDS_OVERFLOW = UDS_ERROR_CODE_BASE,
// Invalid argument passed to internal routine
    UDS_INVALID_ARGUMENT,
// UDS data structures are in an invalid state
    UDS_BAD_STATE,
// Attempt to enter the same name into an internal structure twice
    UDS_DUPLICATE_NAME,
// An assertion failed
    UDS_ASSERTION_FAILED,
// A request has been queued for later processing (not an error)
    UDS_QUEUED,
// This error range has already been registered
    UDS_ALREADY_REGISTERED,
// Attempt to read or write data outside the valid range
    UDS_OUT_OF_RANGE,
// The index session is disabled
    UDS_DISABLED,
// The index configuration or volume format is no longer supported
    UDS_UNSUPPORTED_VERSION,
// Some index structure is corrupt
    UDS_CORRUPT_DATA,
// No index state found
    UDS_NO_INDEX,
// Attempt to access incomplete index save data
    UDS_INDEX_NOT_SAVED_CLEANLY,
// One more than the last UDS_INTERNAL error code
    UDS_ERROR_CODE_LAST,
// One more than the last error this block will ever use
    UDS_ERROR_CODE_BLOCK_END = UDS_ERROR_CODE_BASE + 440,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct error_info {
    pub name: *const c_char,
    pub message: *const c_char,
}

extern "C" {
    pub fn uds_string_error(errnum: c_int, buf: *mut c_char, buflen: usize) -> *const char  __must_check;
}
extern "C" {
    pub fn uds_status_to_errno(error: c_int) -> c_int;
}
