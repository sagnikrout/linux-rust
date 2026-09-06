//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/status-codes.h
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

// VDO-specific status codes.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdo_status_codes {
// base of all VDO errors
    VDO_STATUS_CODE_BASE = VDO_ERRORS_BLOCK_START,
// we haven't written this yet
    VDO_NOT_IMPLEMENTED = VDO_STATUS_CODE_BASE,
// input out of range
    VDO_OUT_OF_RANGE,
// an invalid reference count would result
    VDO_REF_COUNT_INVALID,
// a free block could not be allocated
    VDO_NO_SPACE,
// improper or missing configuration option
    VDO_BAD_CONFIGURATION,
// prior operation still in progress
    VDO_COMPONENT_BUSY,
// page contents incorrect or corrupt data
    VDO_BAD_PAGE,
// unsupported version of some component
    VDO_UNSUPPORTED_VERSION,
// component id mismatch in decoder
    VDO_INCORRECT_COMPONENT,
// parameters have conflicting values
    VDO_PARAMETER_MISMATCH,
// no partition exists with a given id
    VDO_UNKNOWN_PARTITION,
// a partition already exists with a given id
    VDO_PARTITION_EXISTS,
// physical block growth of too few blocks
    VDO_INCREMENT_TOO_SMALL,
// incorrect checksum
    VDO_CHECKSUM_MISMATCH,
// a lock is held incorrectly
    VDO_LOCK_ERROR,
// the VDO is in read-only mode
    VDO_READ_ONLY,
// the VDO is shutting down
    VDO_SHUTTING_DOWN,
// the recovery journal has corrupt entries or corrupt metadata
    VDO_CORRUPT_JOURNAL,
// exceeds maximum number of slabs supported
    VDO_TOO_MANY_SLABS,
// a compressed block fragment is invalid
    VDO_INVALID_FRAGMENT,
// action is unsupported while rebuilding
    VDO_RETRY_AFTER_REBUILD,
// a block map entry is invalid
    VDO_BAD_MAPPING,
// bio_add_page failed
    VDO_BIO_CREATION_FAILED,
// bad magic number
    VDO_BAD_MAGIC,
// bad nonce
    VDO_BAD_NONCE,
// sequence number overflow
    VDO_JOURNAL_OVERFLOW,
// the VDO is not in a state to perform an admin operation
    VDO_INVALID_ADMIN_STATE,
// one more than last error code
    VDO_STATUS_CODE_LAST,
    VDO_STATUS_CODE_BLOCK_END = VDO_ERRORS_BLOCK_END
}

extern "C" {
    pub fn vdo_register_status_codes() -> c_int;
}
extern "C" {
    pub fn vdo_status_to_errno(error: c_int) -> c_int;
}
