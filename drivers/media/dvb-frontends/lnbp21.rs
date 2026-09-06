//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lnbp21.h
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
// lnbp21.h - driver for lnb supply and control ic lnbp21
//
// Copyright (C) 2006 Oliver Endriss
//
// the project's page is at https://linuxtv.org
//
// system register bits
// [RO] 0=OK; 1=over current limit flag
pub const LNBP21_OLF: c_uint = 0x01;
// [RO] 0=OK; 1=over temperature flag (150 C)
pub const LNBP21_OTF: c_uint = 0x02;
// [RW] 0=disable LNB power, enable loopthrough
pub const LNBP21_EN: c_uint = 0x04;
// [RW] 0=low voltage (13/14V, vert pol)
pub const LNBP21_VSEL: c_uint = 0x08;
// [RW] increase LNB voltage by 1V:
pub const LNBP21_LLC: c_uint = 0x10;
// [RW] 0=tone controlled by DSQIN pin
pub const LNBP21_TEN: c_uint = 0x20;
// [RW] current limit select:
pub const LNBP21_ISEL: c_uint = 0x40;
// [RW] short-circuit protect:
pub const LNBP21_PCL: c_uint = 0x80;

// override_set and override_clear control which

