//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/hoontech.h
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
// Lowlevel functions for Hoontech STDSP24
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//

pub const ICE1712_SUBDEVICE_STDSP24: c_uint = 0x12141217	/* Hoontech SoundTrack Audio DSP 24 */;
pub const ICE1712_SUBDEVICE_STDSP24_VALUE: c_uint = 0x00010010	/* A dummy id for Hoontech SoundTrack Audio DSP 24 Value */;
pub const ICE1712_SUBDEVICE_STDSP24_MEDIA7_1: c_uint = 0x16141217	/* Hoontech ST Audio DSP24 Media 7.1 */;
pub const ICE1712_SUBDEVICE_EVENT_EZ8: c_uint = 0x00010001	/* A dummy id for EZ8 */;
pub const ICE1712_SUBDEVICE_STAUDIO_ADCIII: c_uint = 0x00010002	/* A dummy id for STAudio ADCIII */;
// Hoontech SoundTrack Audio DSP 24 GPIO definitions

// Hoontech SoundTrack Audio DSP 24 box configuration definitions

// Hoontech SoundTrack Audio DSP 24 Value definitions for modified hardware
pub const ICE1712_STDSP24_AK4524_CS: c_uint = 0x03	/* AK4524 chip select; low = active */;
pub const ICE1712_STDSP24_SERIAL_DATA: c_uint = 0x0c	/* ak4524 data */;
pub const ICE1712_STDSP24_SERIAL_CLOCK: c_uint = 0x30	/* ak4524 clock */;
