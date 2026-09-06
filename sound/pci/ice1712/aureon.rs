//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/aureon.h
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
// Lowlevel functions for Terratec Aureon cards
//
// Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
//

pub const VT1724_SUBDEVICE_AUREON51_SKY: c_uint = 0x3b154711	/* Aureon 5.1 Sky */;
pub const VT1724_SUBDEVICE_AUREON71_SPACE: c_uint = 0x3b154511	/* Aureon 7.1 Space */;
pub const VT1724_SUBDEVICE_AUREON71_UNIVERSE: c_uint = 0x3b155311	/* Aureon 7.1 Universe */;
pub const VT1724_SUBDEVICE_PRODIGY71: c_uint = 0x33495345	/* PRODIGY 7.1 */;
pub const VT1724_SUBDEVICE_PRODIGY71LT: c_uint = 0x32315441	/* PRODIGY 7.1 LT */;
pub const VT1724_SUBDEVICE_PRODIGY71XT: c_uint = 0x36315441	/* PRODIGY 7.1 XT*/;
// GPIO bits

pub const AUREON_AC97_DATA_MASK: c_uint = 0xFF;

