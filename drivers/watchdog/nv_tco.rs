//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/watchdog/nv_tco.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// nv_tco:	TCO timer driver for nVidia chipsets.
//
// (c) Copyright 2005 Google Inc., All Rights Reserved.
//
// Supported Chipsets:
// - MCP51/MCP55
//
// (c) Copyright 2000 kernel concepts <nils@kernelconcepts.de>, All Rights
// Reserved.
// https://www.kernelconcepts.de
//
// Neither kernel concepts nor Nils Faerber admit liability nor provide
// warranty for any of this software. This material is provided
// "AS-IS" and at no charge.
//
// (c) Copyright 2000	kernel concepts <nils@kernelconcepts.de>
// developed for
// Jentro AG, Haar/Munich (Germany)
//
// TCO timer driver for NV chipsets
// based on softdog.c by Alan Cox <alan@redhat.com>
//
// Some address definitions for the TCO
//

//
// TCO Boot Status bit: set on TCO reset, reset by software or standby
// power-good (survives reboots), unfortunately this bit is never
// set.
//

//
// first and 2nd timeout status bits, these also survive a warm boot,
// and they work, so we use them.
//

pub const MCP51_SMBUS_SETUP_B: c_uint = 0xe8;

//
// The SMI_EN register is at the base io address + 0x04,
// while TCOBASE is + 0x40.
//

