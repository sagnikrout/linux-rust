//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/hiface/pcm.h
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
// Linux driver for M2Tech hiFace compatible devices
//
// Copyright 2012-2013 (C) M2TECH S.r.l and Amarula Solutions B.V.
//
// Authors:  Michael Trimarchi <michael@amarulasolutions.com>
// Antonio Ospite <ao2@amarulasolutions.com>
//
// The driver is based on the work done in TerraTec DMX 6Fire USB
//
extern "C" {
    pub fn hiface_pcm_init(chip: *mut hiface_chip, extra_freq: u8) -> c_int;
}
extern "C" {
    pub fn hiface_pcm_abort(chip: *mut hiface_chip);
}
