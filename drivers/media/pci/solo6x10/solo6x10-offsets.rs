//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/solo6x10/solo6x10-offsets.h
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
// Copyright (C) 2010-2013 Bluecherry, LLC <https://www.bluecherrydvr.com>
//
// Original author:
// Ben Collins <bcollins@ubuntu.com>
//
// Additional work by:
// John Brooks <john.brooks@bluecherry.net>
//
pub const SOLO_DISP_EXT_ADDR: c_uint = 0x00000000;
pub const SOLO_DISP_EXT_SIZE: c_uint = 0x00480000;

pub const SOLO_EOSD_EXT_SIZE_MAX: c_uint = 0x20000;

pub const SOLO_MOTION_EXT_SIZE: c_uint = 0x00080000;

pub const SOLO_G723_EXT_SIZE: c_uint = 0x00010000;

// 18 is the maximum number of pages required for PAL@D1, the largest frame
// possible

// Always allow the encoder enough for 16 channels, even if we have less. The
// exception is if we have card with only 32Megs of memory.

// SOLO_CAP_PAGE_SIZE)

pub const SOLO_EREF_EXT_SIZE: c_uint = 0x00140000;

