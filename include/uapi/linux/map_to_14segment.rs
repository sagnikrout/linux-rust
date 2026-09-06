//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/map_to_14segment.h
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
// Copyright (C) 2021 Glider bv
//
// Based on include/uapi/linux/map_to_7segment.h:
// Copyright (c) 2005 Henk Vergonet <Henk.Vergonet@gmail.com>
//
// This file provides translation primitives and tables for the conversion
// of (ASCII) characters to a 14-segments notation.
//
// The 14 segment's wikipedia notation below is used as standard.
// See: https://en.wikipedia.org/wiki/Fourteen-segment_display
//
// Notation:	+---a---+
// |\  |  /|
// f h i j b
// |  \|/  |
// +-g1+-g2+
// |  /|\  |
// e k l m c
// |/  |  \|
// +---d---+
//
// Usage:
//
// Register a map variable, and fill it with a character set:
// static SEG14_DEFAULT_MAP(map_seg14);
//
// Then use for conversion:
// seg14 = map_to_seg14(&map_seg14, some_char);
// ...
//
// In device drivers it is recommended, if required, to make the char map
// accessible via the sysfs interface using the following scheme:
//
// static ssize_t map_seg14_show(struct device *dev,
// struct device_attribute *attr, char *buf)
// {
// memcpy(buf, &map_seg14, sizeof(map_seg14));
// return sizeof(map_seg14);
// }
// static ssize_t map_seg14_store(struct device *dev,
// struct device_attribute *attr,
// const char *buf, size_t cnt)
// {
// if (cnt != sizeof(map_seg14))
// return -EINVAL;
// memcpy(&map_seg14, buf, cnt);
// return cnt;
// }
// static DEVICE_ATTR_RW(map_seg14);
//

pub const BIT_SEG14_A: c_int = 0;
pub const BIT_SEG14_B: c_int = 1;
pub const BIT_SEG14_C: c_int = 2;
pub const BIT_SEG14_D: c_int = 3;
pub const BIT_SEG14_E: c_int = 4;
pub const BIT_SEG14_F: c_int = 5;
pub const BIT_SEG14_G1: c_int = 6;
pub const BIT_SEG14_G2: c_int = 7;
pub const BIT_SEG14_H: c_int = 8;
pub const BIT_SEG14_I: c_int = 9;
pub const BIT_SEG14_J: c_int = 10;
pub const BIT_SEG14_K: c_int = 11;
pub const BIT_SEG14_L: c_int = 12;
pub const BIT_SEG14_M: c_int = 13;
pub const BIT_SEG14_RESERVED1: c_int = 14;
pub const BIT_SEG14_RESERVED2: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg14_conversion_map {
    pub table: [__be16; 128],
}

extern "C" {
    pub fn __be16_to_cpu(_arg: map->table[c]) -> return;
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

