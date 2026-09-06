//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ppp-comp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// ppp-comp.h - Definitions for doing PPP packet compression.
//
// Copyright 1994-1998 Paul Mackerras.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// version 2 as published by the Free Software Foundation.
//
// CCP codes.
//
pub const CCP_CONFREQ: c_int = 1;
pub const CCP_CONFACK: c_int = 2;
pub const CCP_TERMREQ: c_int = 5;
pub const CCP_TERMACK: c_int = 6;
pub const CCP_RESETREQ: c_int = 14;
pub const CCP_RESETACK: c_int = 15;
//
// Max # bytes for a CCP option
//
pub const CCP_MAX_OPTION_LENGTH: c_int = 32;
//
// Parts of a CCP packet.
//

pub const CCP_HDRLEN: c_int = 4;

pub const CCP_OPT_MINLEN: c_int = 2;
//
// Definitions for BSD-Compress.
//

// Macros for handling the 3rd byte of the BSD-Compress config option.

//
// Definitions for Deflate.
//

pub const DEFLATE_MIN_SIZE: c_int = 9;
pub const DEFLATE_MAX_SIZE: c_int = 15;
pub const DEFLATE_METHOD_VAL: c_int = 8;

pub const DEFLATE_CHK_SEQUENCE: c_int = 0;
//
// Definitions for MPPE.
//

//
// Definitions for other, as yet unsupported, compression methods.
//

