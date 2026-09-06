//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/i8k.h
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
// i8k.h -- Linux driver for accessing the SMM BIOS on Dell laptops
//
// Copyright (C) 2001  Massimo Dal Zotto <dz@debian.org>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 2, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//

pub const I8K_FAN_LEFT: c_int = 1;
pub const I8K_FAN_RIGHT: c_int = 0;
pub const I8K_FAN_OFF: c_int = 0;
pub const I8K_FAN_LOW: c_int = 1;
pub const I8K_FAN_HIGH: c_int = 2;
pub const I8K_FAN_TURBO: c_int = 3;
// Many machines treat this mode as some sort of automatic mode
pub const I8K_FAN_AUTO: c_int = 3;

pub const I8K_VOL_UP: c_int = 1;
pub const I8K_VOL_DOWN: c_int = 2;
pub const I8K_VOL_MUTE: c_int = 4;
pub const I8K_AC: c_int = 1;
pub const I8K_BATTERY: c_int = 0;
