//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rio_ids.h
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
// RapidIO devices
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//
pub const RIO_VID_IDT: c_uint = 0x0038;
pub const RIO_DID_IDT70K200: c_uint = 0x0310;
pub const RIO_DID_IDTCPS8: c_uint = 0x035c;
pub const RIO_DID_IDTCPS12: c_uint = 0x035d;
pub const RIO_DID_IDTCPS16: c_uint = 0x035b;
pub const RIO_DID_IDTCPS6Q: c_uint = 0x035f;
pub const RIO_DID_IDTCPS10Q: c_uint = 0x035e;
pub const RIO_DID_IDTCPS1848: c_uint = 0x0374;
pub const RIO_DID_IDTCPS1432: c_uint = 0x0375;
pub const RIO_DID_IDTCPS1616: c_uint = 0x0379;
pub const RIO_DID_IDTVPS1616: c_uint = 0x0377;
pub const RIO_DID_IDTSPS1616: c_uint = 0x0378;
pub const RIO_DID_IDTRXS1632: c_uint = 0x80e5;
pub const RIO_DID_IDTRXS2448: c_uint = 0x80e6;
