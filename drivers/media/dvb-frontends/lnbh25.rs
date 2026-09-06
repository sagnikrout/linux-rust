//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/lnbh25.h
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
// lnbh25.c
//
// Driver for LNB supply and control IC LNBH25
//
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

// 22 kHz tone enabled. Tone output controlled by DSQIN pin
pub const LNBH25_TEN: c_uint = 0x01;
// Low power mode activated (used only with 22 kHz tone output disabled)
pub const LNBH25_LPM: c_uint = 0x02;
// DSQIN input pin is set to receive external 22 kHz TTL signal source
pub const LNBH25_EXTM: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lnbh25_config {
    pub i2c_address: u8,
    pub data2_config: u8,
}

