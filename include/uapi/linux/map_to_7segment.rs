//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/map_to_7segment.h
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
// Copyright (c) 2005 Henk Vergonet <Henk.Vergonet@gmail.com>
//
// This file provides translation primitives and tables for the conversion
// of (ASCII) characters to a 7-segments notation.
//
// The 7 segment's wikipedia notation below is used as standard.
// See: https://en.wikipedia.org/wiki/Seven_segment_display
//
// Notation:	+-a-+
// f   b
// +-g-+
// e   c
// +-d-+
//
// Usage:
//
// Register a map variable, and fill it with a character set:
// static SEG7_DEFAULT_MAP(map_seg7);
//
// Then use for conversion:
// seg7 = map_to_seg7(&map_seg7, some_char);
// ...
//
// In device drivers it is recommended, if required, to make the char map
// accessible via the sysfs interface using the following scheme:
//
// static ssize_t map_seg7_show(struct device *dev,
// struct device_attribute *attr, char *buf)
// {
// memcpy(buf, &map_seg7, sizeof(map_seg7));
// return sizeof(map_seg7);
// }
// static ssize_t map_seg7_store(struct device *dev,
// struct device_attribute *attr, const char *buf,
// size_t cnt)
// {
// if(cnt != sizeof(map_seg7))
// return -EINVAL;
// memcpy(&map_seg7, buf, cnt);
// return cnt;
// }
// static DEVICE_ATTR_RW(map_seg7);
//
// History:
// 2005-05-31	RFC linux-kernel@vger.kernel.org
//

pub const BIT_SEG7_A: c_int = 0;
pub const BIT_SEG7_B: c_int = 1;
pub const BIT_SEG7_C: c_int = 2;
pub const BIT_SEG7_D: c_int = 3;
pub const BIT_SEG7_E: c_int = 4;
pub const BIT_SEG7_F: c_int = 5;
pub const BIT_SEG7_G: c_int = 6;
pub const BIT_SEG7_RESERVED: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg7_conversion_map {
    pub table: [c_uchar; 128],
}

//
// It is recommended to use a facility that allows user space to redefine
// custom character sets for LCD devices. Please use a sysfs interface
// as described above.
//

//
// ASCII conversion table
//

// Maps
// This set tries to map as close as possible to the visible characteristics
// of the ASCII symbol, lowercase and uppercase letters may differ in
// presentation on the display.
//

// This set tries to map as close as possible to the symbolic characteristics
// of the ASCII character for maximum discrimination.
// For now this means all alpha chars are in lower case representations.
// (This for example facilitates the use of hex numbers with uppercase input.)
//

