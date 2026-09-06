//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/scsi_logging.h
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
// This defines the scsi logging feature.  It is a means by which the user can
// select how much information they get about various goings on, and it can be
// really useful for fault tracing.  The logging word is divided into 10 3-bit
// bitfields, each of which describes a loglevel.  The division of things is
// somewhat arbitrary, and the division of the word could be changed if it
// were really needed for any reason.  The numbers below are the only place
// where these are specified.  For a first go-around, 3 bits is more than
// enough, since this gives 8 levels of logging (really 7, since 0 is always
// off).  Cutting to 2 bits might be wise at some point.
//
pub const SCSI_LOG_ERROR_SHIFT: c_int = 0;
pub const SCSI_LOG_TIMEOUT_SHIFT: c_int = 3;
pub const SCSI_LOG_SCAN_SHIFT: c_int = 6;
pub const SCSI_LOG_MLQUEUE_SHIFT: c_int = 9;
pub const SCSI_LOG_MLCOMPLETE_SHIFT: c_int = 12;
pub const SCSI_LOG_LLQUEUE_SHIFT: c_int = 15;
pub const SCSI_LOG_LLCOMPLETE_SHIFT: c_int = 18;
pub const SCSI_LOG_HLQUEUE_SHIFT: c_int = 21;
pub const SCSI_LOG_HLCOMPLETE_SHIFT: c_int = 24;
pub const SCSI_LOG_IOCTL_SHIFT: c_int = 27;
pub const SCSI_LOG_ERROR_BITS: c_int = 3;
pub const SCSI_LOG_TIMEOUT_BITS: c_int = 3;
pub const SCSI_LOG_SCAN_BITS: c_int = 3;
pub const SCSI_LOG_MLQUEUE_BITS: c_int = 3;
pub const SCSI_LOG_MLCOMPLETE_BITS: c_int = 3;
pub const SCSI_LOG_LLQUEUE_BITS: c_int = 3;
pub const SCSI_LOG_LLCOMPLETE_BITS: c_int = 3;
pub const SCSI_LOG_HLQUEUE_BITS: c_int = 3;
pub const SCSI_LOG_HLCOMPLETE_BITS: c_int = 3;
pub const SCSI_LOG_IOCTL_BITS: c_int = 3;

//
// These are the macros that are actually used throughout the code to
// log events.  If logging isn't enabled, they are no-ops and will be
// completely absent from the user's code.
//

