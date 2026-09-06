//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mmtimer.h
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
// Intel Multimedia Timer device interface
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (c) 2001-2004 Silicon Graphics, Inc.  All rights reserved.
//
// This file should define an interface compatible with the IA-PC Multimedia
// Timers Draft Specification (rev. 0.97) from Intel.  Note that some
// hardware may not be able to safely export its registers to userspace,
// so the ioctl interface should support all necessary functionality.
//
// 11/01/01 - jbarnes - initial revision
// 9/10/04 - Christoph Lameter - remove interrupt support
// 9/17/04 - jbarnes - remove test program, move some #defines to the driver
//
// Breakdown of the ioctl's available.  An 'optional' next to the command
// indicates that supporting this command is optional, while 'required'
// commands must be implemented if conformance is desired.
//
// MMTIMER_GETOFFSET - optional
// Should return the offset (relative to the start of the page where the
// registers are mapped) for the counter in question.
//
// MMTIMER_GETRES - required
// The resolution of the clock in femto (10^-15) seconds
//
// MMTIMER_GETFREQ - required
// Frequency of the clock in Hz
//
// MMTIMER_GETBITS - required
// Number of bits in the clock's counter
//
// MMTIMER_MMAPAVAIL - required
// Returns nonzero if the registers can be mmap'd into userspace, 0 otherwise
//
// MMTIMER_GETCOUNTER - required
// Gets the current value in the counter
//

