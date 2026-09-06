//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/vt1720_mobo.h
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
// ALSA driver for VT1720/VT1724 (Envy24PT/Envy24HT)
//
// Lowlevel functions for VT1720-based motherboards
//
// Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
//

pub const VT1720_SUBDEVICE_K8X800: c_uint = 0xf217052c;
pub const VT1720_SUBDEVICE_ZNF3_150: c_uint = 0x0f2741f6;
pub const VT1720_SUBDEVICE_ZNF3_250: c_uint = 0x0f2745f6;
pub const VT1720_SUBDEVICE_9CJS: c_uint = 0x0f272327;
pub const VT1720_SUBDEVICE_SN25P: c_uint = 0x97123650;
