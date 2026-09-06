//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/isl6405.h
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
// isl6405.h - driver for dual lnb supply and control ic ISL6405
//
// Copyright (C) 2008 Hartmut Hackmann
// Copyright (C) 2006 Oliver Endriss
//
// the project's page is at https://linuxtv.org
//

// system register bits
// this bit selects register (control) 1 or 2
pub const ISL6405_SR: c_uint = 0x80;
// SR = 0
pub const ISL6405_OLF1: c_uint = 0x01;
pub const ISL6405_EN1: c_uint = 0x02;
pub const ISL6405_VSEL1: c_uint = 0x04;
pub const ISL6405_LLC1: c_uint = 0x08;
pub const ISL6405_ENT1: c_uint = 0x10;
pub const ISL6405_ISEL1: c_uint = 0x20;
pub const ISL6405_DCL: c_uint = 0x40;
// SR = 1
pub const ISL6405_OLF2: c_uint = 0x01;
pub const ISL6405_OTF: c_uint = 0x02;
pub const ISL6405_EN2: c_uint = 0x04;
pub const ISL6405_VSEL2: c_uint = 0x08;
pub const ISL6405_LLC2: c_uint = 0x10;
pub const ISL6405_ENT2: c_uint = 0x20;
pub const ISL6405_ISEL2: c_uint = 0x40;

// override_set and override_clear control which system register bits (above)
// to always set & clear
//

