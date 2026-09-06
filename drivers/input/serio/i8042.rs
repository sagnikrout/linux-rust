//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/serio/i8042.h
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
// Copyright (c) 1999-2002 Vojtech Pavlik
//
// Arch-dependent inline functions and defines.
//

//
// This is in 50us units, the time we wait for the i8042 to react. This
// has to be long enough for the i8042 itself to timeout on sending a byte
// to a non-existent mouse.
//
pub const I8042_CTL_TIMEOUT: c_int = 10000;
//
// Return codes.
//
pub const I8042_RET_CTL_TEST: c_uint = 0x55;
//
// Expected maximum internal i8042 buffer size. This is used for flushing
// the i8042 buffers.
//
pub const I8042_BUFFER_SIZE: c_int = 16;
//
// Number of AUX ports on controllers supporting active multiplexing
// specification
//
pub const I8042_NUM_MUX_PORTS: c_int = 4;
//
// Debug.
//

