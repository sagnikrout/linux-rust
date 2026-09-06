//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ppdev.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// linux/include/linux/ppdev.h
//
// User-space parallel port device driver (header file).
//
// Copyright (C) 1998-9 Tim Waugh <tim@cyberelk.demon.co.uk>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// Added PPGETTIME/PPSETTIME, Fred Barnes, 1999
// Added PPGETMODES/PPGETMODE/PPGETPHASE, Fred Barnes <frmb2@ukc.ac.uk>, 03/01/2001
//

// Set mode for read/write (e.g. IEEE1284_MODE_EPP)

// Read status

// Read/write control

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppdev_frob_struct {
    pub mask: c_uchar,
    pub val: c_uchar,
}

// Read/write data

// Read/write econtrol (not used)

// Read/write FIFO (not used)

// Claim the port to start using it

// Release the port when you aren't using it

// Yield the port (release it if another driver is waiting,
// then reclaim)

// Register device exclusively (must be before PPCLAIM).

// Data line direction: non-zero for input mode.

// Negotiate a particular IEEE 1284 mode.

// Set control lines when an interrupt occurs.

// Clear (and return) interrupt count.

// Set the IEEE 1284 phase that we're in (e.g. IEEE1284_PH_FWD_IDLE)

// Set and get port timeout (struct timeval's)

// Get available modes (what the hardware can do)

// Get the current mode and phaze

// get/set flags

// flags visible to the world

// only masks user-visible flags

