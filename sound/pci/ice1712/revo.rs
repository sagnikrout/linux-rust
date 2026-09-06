//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/revo.h
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
// ALSA driver for ICEnsemble ICE1712 (Envy24)
//
// Lowlevel functions for M-Audio Revolution 7.1
//
// Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
//

pub const VT1724_SUBDEVICE_REVOLUTION71: c_uint = 0x12143036;
pub const VT1724_SUBDEVICE_REVOLUTION51: c_uint = 0x12143136;
pub const VT1724_SUBDEVICE_AUDIOPHILE192: c_uint = 0x12143236;
// entry point
//
// MidiMan M-Audio Revolution GPIO definitions
//
pub const VT1724_REVO_CCLK: c_uint = 0x02;
pub const VT1724_REVO_CDIN: c_uint = 0x04	/* not used */;
pub const VT1724_REVO_CDOUT: c_uint = 0x08;
pub const VT1724_REVO_CS0: c_uint = 0x10	/* AK5365 chipselect for (revo51) */;
pub const VT1724_REVO_CS1: c_uint = 0x20	/* front AKM4381 chipselect */;
pub const VT1724_REVO_CS2: c_uint = 0x40	/* surround AKM4355 CS (revo71) */;
pub const VT1724_REVO_I2C_DATA: c_uint = 0x40    /* I2C: PT 2258 SDA (on revo51) */;
pub const VT1724_REVO_I2C_CLOCK: c_uint = 0x80    /* I2C: PT 2258 SCL (on revo51) */;
pub const VT1724_REVO_CS3: c_uint = 0x80	/* AK4114 for AP192 */;

