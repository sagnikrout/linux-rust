//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/speakup_acnt.h
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
// speakup_acntpc.h - header file for speakups Accent-PC driver.
pub const SYNTH_IO_EXTENT: c_uint = 0x02;
pub const SYNTH_CLEAR: c_uint = 0x18		/* stops speech */;
// Port Status Flags
pub const SYNTH_READABLE: c_uint = 0x01	/* mask for bit which is nonzero if a;
// byte can be read from the data port
//
pub const SYNTH_WRITABLE: c_uint = 0x02	/* mask for RDY bit, which when set to;
// 1, indicates the data port is ready
// to accept a byte of data.
//

