//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sonypi.h
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
// Sony Programmable I/O Control Device driver for VAIO
//
// Copyright (C) 2001-2005 Stelian Pop <stelian@popies.net>
//
// Copyright (C) 2005 Narayanan R S <nars@kadamba.org>
// Copyright (C) 2001-2002 Alcôve <www.alcove.com>
//
// Copyright (C) 2001 Michael Ashley <m.ashley@unsw.edu.au>
//
// Copyright (C) 2001 Junichi Morita <jun1m@mars.dti.ne.jp>
//
// Copyright (C) 2000 Takaya Kinjo <t-kinjo@tc4.so-net.ne.jp>
//
// Copyright (C) 2000 Andrew Tridgell <tridge@valinux.com>
//
// Earlier work by Werner Almesberger, Paul `Rusty' Russell and Paul Mackerras.
//

// used only for communication between v4l and sonypi

pub const SONYPI_COMMAND_SETCAMERA: c_int = 2;

pub const SONYPI_COMMAND_SETCAMERABRIGHTNESS: c_int = 4;

pub const SONYPI_COMMAND_SETCAMERACONTRAST: c_int = 6;

pub const SONYPI_COMMAND_SETCAMERAHUE: c_int = 8;

pub const SONYPI_COMMAND_SETCAMERACOLOR: c_int = 10;

pub const SONYPI_COMMAND_SETCAMERASHARPNESS: c_int = 12;

pub const SONYPI_COMMAND_SETCAMERAPICTURE: c_int = 14;

pub const SONYPI_COMMAND_SETCAMERAAGC: c_int = 16;

