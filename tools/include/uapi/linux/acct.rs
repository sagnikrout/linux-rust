//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/acct.h
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
// BSD Process Accounting for Linux - Definitions
//
// Author: Marco van Wieringen (mvw@planets.elm.net)
//
// This header file contains the definitions needed to implement
// BSD-style process accounting. The kernel accounting code and all
// user-level programs that try to do something useful with the
// process accounting log must include this file.
//
// Copyright (C) 1995 - 1997 Marco van Wieringen - ELM Consultancy B.V.
//

//
// comp_t is a 16-bit "floating" point number with a 3-bit base 8
// exponent and a 13-bit fraction.
// comp2_t is 24-bit with 5-bit base 2 exponent and 20 bit fraction
// (leading 1 not stored).
// See linux/kernel/acct.c for the specific encoding systems used.
//
pub type comp_t = __u16;
pub type comp2_t = __u32;
//
// accounting file record
//
// This structure contains all of the information written out to the
// process accounting file whenever a process exits.
//
pub const ACCT_COMM: c_int = 16;
// for binary compatibility back until 2.0
// __u32 range means times from 1970 to 2106
// m68k had no padding here.

// __u32 range means times from 1970 to 2106

//
// accounting flags
//
// bit set when the process/task ...
pub const AFORK: c_uint = 0x01	/* ... executed fork, but did not exec */;
pub const ASU: c_uint = 0x02	/* ... used super-user privileges */;
pub const ACOMPAT: c_uint = 0x04	/* ... used compatibility mode (VAX only not used) */;
pub const ACORE: c_uint = 0x08	/* ... dumped core */;
pub const AXSIG: c_uint = 0x10	/* ... was killed by a signal */;
pub const AGROUP: c_uint = 0x20	/* ... was the last task of the process (task group) */;

pub const ACCT_BYTEORDER: c_uint = 0x80	/* accounting file is big endian */;

pub const ACCT_BYTEORDER: c_uint = 0x00	/* accounting file is little endian */;

pub const ACCT_VERSION: c_int = 2;

