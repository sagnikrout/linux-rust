//! Automatically rewritten from C Header to Rust Module
//! Source: sound/ppc/burgundy.h
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
// Driver for PowerMac Burgundy onboard soundchips
// Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
// based on dmasound.c.
//

// These are all default values for the burgundy

// Remember: lowest volume here is 0x9B (155)

// #define DEF_BURGUNDY_MASTER_VOLUME (0xFFFFFFFF) */ /* too loud

// MORE_OUTPUTENABLES bits
pub const BURGUNDY_OUTPUT_LEFT: c_uint = 0x02;
pub const BURGUNDY_OUTPUT_RIGHT: c_uint = 0x04;
pub const BURGUNDY_LINEOUT_LEFT: c_uint = 0x08;
pub const BURGUNDY_LINEOUT_RIGHT: c_uint = 0x10;
pub const BURGUNDY_HP_LEFT: c_uint = 0x20;
pub const BURGUNDY_HP_RIGHT: c_uint = 0x40;
pub const BURGUNDY_OUTPUT_INTERN: c_uint = 0x80;
// Headphone detection bits
pub const BURGUNDY_HPDETECT_PMAC_BACK: c_uint = 0x04;
pub const BURGUNDY_HPDETECT_IMAC_SIDE: c_uint = 0x04;
pub const BURGUNDY_HPDETECT_IMAC_UPPER: c_uint = 0x08;
pub const BURGUNDY_HPDETECT_IMAC_LOWER: c_uint = 0x01;
// Volume offset
pub const BURGUNDY_VOLUME_OFFSET: c_int = 155;
