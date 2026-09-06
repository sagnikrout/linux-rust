//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lnbh24.h
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
// lnbh24.h - driver for lnb supply and control ic lnbh24
//
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
//
// system register bits
pub const LNBH24_OLF: c_uint = 0x01;
pub const LNBH24_OTF: c_uint = 0x02;
pub const LNBH24_EN: c_uint = 0x04;
pub const LNBH24_VSEL: c_uint = 0x08;
pub const LNBH24_LLC: c_uint = 0x10;
pub const LNBH24_TEN: c_uint = 0x20;
pub const LNBH24_TTX: c_uint = 0x40;
pub const LNBH24_PCL: c_uint = 0x80;

// override_set and override_clear control which

