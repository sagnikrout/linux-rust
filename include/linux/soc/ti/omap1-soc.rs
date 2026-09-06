//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/ti/omap1-soc.h
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
// OMAP cpu type detection
//
// Copyright (C) 2004, 2008 Nokia Corporation
//
// Copyright (C) 2009-11 Texas Instruments.
//
// Written by Tony Lindgren <tony.lindgren@nokia.com>
//
// Added OMAP4/5 specific defines - Santosh Shilimkar<santosh.shilimkar@ti.com>
//
// Test if multicore OMAP support is needed
//

//
// omap_rev bits:
// CPU id bits	(0730, 1510, 1710, 2422...)	[31:16]
// CPU revision	(See _REV_ defined in cpu.h)	[15:08]
// CPU class bits (15xx, 16xx, 24xx, 34xx...)	[07:00]
//
extern "C" {
    pub fn omap_rev() -> c_uint;
}
//
// Get the CPU revision for OMAP devices
//

//
// Macros to group OMAP into cpu classes.
// These can be used in most places.
// cpu_is_omap15xx():	True for OMAP1510, OMAP5910 and OMAP310
// cpu_is_omap16xx():	True for OMAP1610, OMAP5912 and OMAP1710
//

pub const cpu_is_omap15xx(): c_int = 0;
pub const cpu_is_omap16xx(): c_int = 0;

//
// Macros to detect individual cpu types.
// These are only rarely needed.
// cpu_is_omap310():	True for OMAP310
// cpu_is_omap1510():	True for OMAP1510
// cpu_is_omap1610():	True for OMAP1610
// cpu_is_omap1611():	True for OMAP1611
// cpu_is_omap5912():	True for OMAP5912
// cpu_is_omap1621():	True for OMAP1621
// cpu_is_omap1710():	True for OMAP1710
//

pub const cpu_is_omap310(): c_int = 0;
pub const cpu_is_omap1510(): c_int = 0;
pub const cpu_is_omap1610(): c_int = 0;
pub const cpu_is_omap5912(): c_int = 0;
pub const cpu_is_omap1611(): c_int = 0;
pub const cpu_is_omap1621(): c_int = 0;
pub const cpu_is_omap1710(): c_int = 0;
pub const cpu_class_is_omap1(): c_int = 1;
//
// Whether we have MULTI_OMAP1 or not, we still need to distinguish
// between 310 vs. 1510 and 1611B/5912 vs. 1710.
//

