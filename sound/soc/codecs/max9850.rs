//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max9850.h
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
// max9850.h  --  codec driver for max9850
//
// Copyright (C) 2011 taskit GmbH
// Author: Christian Glindkamp <christian.glindkamp@taskit.de>
//
pub const MAX9850_STATUSA: c_uint = 0x00;
pub const MAX9850_STATUSB: c_uint = 0x01;
pub const MAX9850_VOLUME: c_uint = 0x02;
pub const MAX9850_GENERAL_PURPOSE: c_uint = 0x03;
pub const MAX9850_INTERRUPT: c_uint = 0x04;
pub const MAX9850_ENABLE: c_uint = 0x05;
pub const MAX9850_CLOCK: c_uint = 0x06;
pub const MAX9850_CHARGE_PUMP: c_uint = 0x07;
pub const MAX9850_LRCLK_MSB: c_uint = 0x08;
pub const MAX9850_LRCLK_LSB: c_uint = 0x09;
pub const MAX9850_DIGITAL_AUDIO: c_uint = 0x0a;
pub const MAX9850_CACHEREGNUM: c_int = 11;
// MAX9850_DIGITAL_AUDIO

