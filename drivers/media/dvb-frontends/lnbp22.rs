//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lnbp22.h
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
// lnbp22.h - driver for lnb supply and control ic lnbp22
//
// Copyright (C) 2006 Dominik Kuhlen
// Based on lnbp21.h
//
// the project's page is at https://linuxtv.org
//
// Enable
pub const LNBP22_EN: c_uint = 0x10;
// Voltage selection
pub const LNBP22_VSEL: c_uint = 0x02;
// Plus 1 Volt Bit
pub const LNBP22_LLC: c_uint = 0x01;

//
// override_set and override_clear control which system register bits (above)
// to always set & clear
//

