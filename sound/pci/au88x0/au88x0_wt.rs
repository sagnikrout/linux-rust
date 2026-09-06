//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/au88x0/au88x0_wt.h
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
// WT register offsets.
//
// Wed Oct 22 13:50:20 2003
// Copyright  2003  mjander
// mjander@users.sourceforge.org
//
// WT channels are grouped in banks. Each bank has 0x20 channels.
// Bank register address boundary is 0x8000
pub const NR_WT_PB: c_uint = 0x20;
// WT bank base register (as dword address).

// WT Bank registers

// WT Voice registers

// Some kind of parameters.
// PARM0, PARM1 : Filter (0xFF000000), SampleRate (0x0000FFFF)
// PARM2, PARM3 : Still unknown

// Numeric indexes used by SetReg() and GetReg()

// End of file
