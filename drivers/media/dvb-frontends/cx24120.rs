//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cx24120.h
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
// Conexant CX24120/CX24118 - DVB-S/S2 demod/tuner driver
//
// Copyright (C) 2008 Patrick Boettcher <pb@linuxtv.org>
// Copyright (C) 2009 Sergey Tyurin <forum.free-x.de>
// Updated 2012 by Jannis Achstetter <jannis_achstetter@web.de>
// Copyright (C) 2015 Jemma Denson <jdenson@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx24120_initial_mpeg_config {
    pub x1: u8,
    pub x2: u8,
    pub x3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx24120_config {
    pub i2c_addr: u8,
    pub xtal_khz: u32,
    pub initial_mpeg_config: cx24120_initial_mpeg_config,
    pub name): *const *const *const firmware fw, char,
// max bytes I2C provider can write at once
    pub i2c_wr_max: u16,
}

