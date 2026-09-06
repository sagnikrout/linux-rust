//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/amp.h
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
// ALSA driver for VIA VT1724 (Envy24HT)
//
// Lowlevel functions for Advanced Micro Peripherals Ltd AUDIO2000
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//

pub const VT1724_SUBDEVICE_AUDIO2000: c_uint = 0x12142417	/* Advanced Micro Peripherals Ltd AUDIO2000 */;

pub const VT1724_SUBDEVICE_AUDIO2000: c_uint = 0x00030003	/* a dummy ID for AMP Audio2000 */;

pub const VT1724_SUBDEVICE_AV710: c_uint = 0x12142417	/* AV710 - the same ID with Audio2000! */;
// WM8728 on I2C for AV710
pub const WM_DEV: c_uint = 0x36;
pub const WM_ATTEN_L: c_uint = 0x00;
pub const WM_ATTEN_R: c_uint = 0x01;
pub const WM_DAC_CTRL: c_uint = 0x02;
pub const WM_INT_CTRL: c_uint = 0x03;
