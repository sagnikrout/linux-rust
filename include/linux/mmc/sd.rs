//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/sd.h
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
// include/linux/mmc/sd.h
//
// Copyright (C) 2005-2007 Pierre Ossman, All Rights Reserved.
//
// SD commands                           type  argument     response
// class 0
// This is basically the same command as for MMC with some quirks.

// Class 2

// class 10

// class 5

// Application commands

// class 11

// OCR bit definitions

//
// SD_SWITCH argument format:
//
// [31] Check (0) or switch (1)
// [30:24] Reserved (0)
// [23:20] Function group 6
// [19:16] Function group 5
// [15:12] Function group 4
// [11:8] Function group 3
// [7:4] Function group 2
// [3:0] Function group 1
//
// SD_SEND_IF_COND argument format:
//
// [31:12] Reserved (0)
// [11:8] Host Voltage Supply Flags
// [7:0] Check Pattern (0xAA)
//
// SCR field definitions
//

//
// SD bus widths
//
pub const SD_BUS_WIDTH_1: c_int = 0;
pub const SD_BUS_WIDTH_4: c_int = 2;
//
// SD_SWITCH mode
//
pub const SD_SWITCH_CHECK: c_int = 0;
pub const SD_SWITCH_SET: c_int = 1;
//
// SD_SWITCH function groups
//
pub const SD_SWITCH_GRP_ACCESS: c_int = 0;
//
// SD_SWITCH access modes
//
pub const SD_SWITCH_ACCESS_DEF: c_int = 0;
pub const SD_SWITCH_ACCESS_HS: c_int = 1;
//
// Erase/discard
//
pub const SD_ERASE_ARG: c_uint = 0x00000000;
pub const SD_DISCARD_ARG: c_uint = 0x00000001;
