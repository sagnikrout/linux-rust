//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/parport.h
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


//
// Any part of this program may be used in documents licensed under
// the GNU Free Documentation License, Version 1.1 or any later version
// published by the Free Software Foundation.
//
// Start off with user-visible constants
// Maximum of 16 ports per machine
pub const PARPORT_MAX: c_int = 16;
// Magic numbers

pub const PARPORT_CONTROL_STROBE: c_uint = 0x1;
pub const PARPORT_CONTROL_AUTOFD: c_uint = 0x2;
pub const PARPORT_CONTROL_INIT: c_uint = 0x4;
pub const PARPORT_CONTROL_SELECT: c_uint = 0x8;
pub const PARPORT_STATUS_ERROR: c_uint = 0x8;
pub const PARPORT_STATUS_SELECT: c_uint = 0x10;
pub const PARPORT_STATUS_PAPEROUT: c_uint = 0x20;
pub const PARPORT_STATUS_ACK: c_uint = 0x40;
pub const PARPORT_STATUS_BUSY: c_uint = 0x80;
// Type classes for Plug-and-Play probe.
// The "modes" entry in parport is a bit field representing the

// IEEE1284 modes:
pub const IEEE1284_MODE_NIBBLE: c_int = 0;

// extensibility link to
// be requested, using
// bits 0-6.
// For the benefit of parport_read/write, you can use these with
// parport_negotiate to use address operations.  They have no effect
// other than to make parport_read/write use address transfers.

// Flags for block transfer operations.

// The rest is for the kernel only
