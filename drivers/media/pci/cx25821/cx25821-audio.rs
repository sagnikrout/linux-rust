//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx25821/cx25821-audio.h
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
//
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
//
pub const USE_RISC_NOOP: c_int = 1;
pub const LINES_PER_BUFFER: c_int = 15;
pub const AUDIO_LINE_SIZE: c_int = 128;
// Number of buffer programs to use at once.
pub const NUMBER_OF_PROGRAMS: c_int = 8;
//
// Max size of the RISC program for a buffer. - worst case is 2 writes per line
// Space is also added for the 4 no-op instructions added on the end.
//

// MAE 12 July 2005 Try to use NOOP RISC instruction instead

// Sizes of various instructions in bytes.  Used when adding instructions.
pub const RISC_WRITE_INSTRUCTION_SIZE: c_int = 12;
pub const RISC_JUMP_INSTRUCTION_SIZE: c_int = 12;
pub const RISC_SKIP_INSTRUCTION_SIZE: c_int = 4;
pub const RISC_SYNC_INSTRUCTION_SIZE: c_int = 4;
pub const RISC_WRITECR_INSTRUCTION_SIZE: c_int = 16;
pub const RISC_NOOP_INSTRUCTION_SIZE: c_int = 4;

