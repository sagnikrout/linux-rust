//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/gameport.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (c) 1999-2002 Vojtech Pavlik
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//
pub const GAMEPORT_MODE_DISABLED: c_int = 0;
pub const GAMEPORT_MODE_RAW: c_int = 1;
pub const GAMEPORT_MODE_COOKED: c_int = 2;
pub const GAMEPORT_ID_VENDOR_ANALOG: c_uint = 0x0001;
pub const GAMEPORT_ID_VENDOR_MADCATZ: c_uint = 0x0002;
pub const GAMEPORT_ID_VENDOR_LOGITECH: c_uint = 0x0003;
pub const GAMEPORT_ID_VENDOR_CREATIVE: c_uint = 0x0004;
pub const GAMEPORT_ID_VENDOR_GENIUS: c_uint = 0x0005;
pub const GAMEPORT_ID_VENDOR_INTERACT: c_uint = 0x0006;
pub const GAMEPORT_ID_VENDOR_MICROSOFT: c_uint = 0x0007;
pub const GAMEPORT_ID_VENDOR_THRUSTMASTER: c_uint = 0x0008;
pub const GAMEPORT_ID_VENDOR_GRAVIS: c_uint = 0x0009;
pub const GAMEPORT_ID_VENDOR_GUILLEMOT: c_uint = 0x000a;
