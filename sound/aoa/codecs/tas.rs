//! Automatically rewritten from C Header to Rust Module
//! Source: sound/aoa/codecs/tas.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Apple Onboard Audio driver for tas codec (header)
//
// Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
//
pub const TAS_REG_MCS: c_uint = 0x01	/* main control */;

pub const TAS_REG_DRC: c_uint = 0x02;
pub const TAS_REG_VOL: c_uint = 0x04;
pub const TAS_REG_TREBLE: c_uint = 0x05;
pub const TAS_REG_BASS: c_uint = 0x06;
pub const TAS_REG_LMIX: c_uint = 0x07;
pub const TAS_REG_RMIX: c_uint = 0x08;
pub const TAS_REG_ACR: c_uint = 0x40	/* analog control */;

pub const TAS_REG_MCS2: c_uint = 0x43	/* main control 2 */;

pub const TAS_REG_LEFT_BIQUAD6: c_uint = 0x10;
pub const TAS_REG_RIGHT_BIQUAD6: c_uint = 0x19;
pub const TAS_REG_LEFT_LOUDNESS: c_uint = 0x21;
pub const TAS_REG_RIGHT_LOUDNESS: c_uint = 0x22;
pub const TAS_REG_LEFT_LOUDNESS_GAIN: c_uint = 0x23;
pub const TAS_REG_RIGHT_LOUDNESS_GAIN: c_uint = 0x24;
pub const TAS3001_DRC_MAX: c_uint = 0x5f;
pub const TAS3004_DRC_MAX: c_uint = 0xef;
