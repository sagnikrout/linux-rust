//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/checksum.h
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
// SCTP kernel reference Implementation
// Copyright (c) 1999-2001 Motorola, Inc.
// Copyright (c) 2001-2003 International Business Machines, Corp.
//
// This file is part of the SCTP kernel reference Implementation
//
// SCTP Checksum functions
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Dinakaran Joseph
// Jon Grimm <jgrimm@us.ibm.com>
// Sridhar Samudrala <sri@us.ibm.com>
// Vlad Yasevich <vladislav.yasevich@hp.com>
//

// Macro flag: #define __sctp_checksum_h__

extern "C" {
    pub fn cpu_to_le32(_arg: new) -> return;
}
