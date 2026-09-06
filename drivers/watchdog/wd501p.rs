//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/watchdog/wd501p.h
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


// SPDX-License-Identifier: GPL-1.0+
//
// Industrial Computer Source WDT500/501 driver
//
// (c) Copyright 1995	CymruNET Ltd
// Innovation Centre
// Singleton Park
// Swansea
// Wales
// UK
// SA2 8PP
//
// http://www.cymru.net
//
// Release 0.04.
//

// The following are only on the PCI card, they're outside of I/O space on
// the ISA card:

// inverted opto isolated reset output:

// opto isolated reset output:

// programmable outputs:

// FAN 501 500

