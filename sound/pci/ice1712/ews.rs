//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/ews.h
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
// Lowlevel functions for Terratec EWS88MT/D, EWX24/96, DMX 6Fire
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
// 2002 Takashi Iwai <tiwai@suse.de>
//

pub const ICE1712_SUBDEVICE_EWX2496: c_uint = 0x3b153011;
pub const ICE1712_SUBDEVICE_EWS88MT: c_uint = 0x3b151511;
pub const ICE1712_SUBDEVICE_EWS88MT_NEW: c_uint = 0x3b152511;
pub const ICE1712_SUBDEVICE_EWS88D: c_uint = 0x3b152b11;
pub const ICE1712_SUBDEVICE_DMX6FIRE: c_uint = 0x3b153811;
pub const ICE1712_SUBDEVICE_PHASE88: c_uint = 0x3b155111;
pub const ICE1712_SUBDEVICE_TS88: c_uint = 0x3b157c11;
// entry point
// TerraTec EWX 24/96 configuration definitions
pub const ICE1712_EWX2496_AK4524_CS: c_uint = 0x01	/* AK4524 chip select; low = active */;
pub const ICE1712_EWX2496_AIN_SEL: c_uint = 0x02	/* input sensitivity switch; high = louder */;
pub const ICE1712_EWX2496_AOUT_SEL: c_uint = 0x04	/* output sensitivity switch; high = louder */;
pub const ICE1712_EWX2496_RW: c_uint = 0x08	/* read/write switch for i2c; high = write  */;
pub const ICE1712_EWX2496_SERIAL_DATA: c_uint = 0x10	/* i2c & ak4524 data */;
pub const ICE1712_EWX2496_SERIAL_CLOCK: c_uint = 0x20	/* i2c & ak4524 clock */;
pub const ICE1712_EWX2496_TX2: c_uint = 0x40	/* MIDI2 (not used) */;
pub const ICE1712_EWX2496_RX2: c_uint = 0x80	/* MIDI2 (not used) */;
// TerraTec EWS 88MT/D configuration definitions
// RW, SDA snd SCLK are identical with EWX24/96
pub const ICE1712_EWS88_CS8414_RATE: c_uint = 0x07	/* CS8414 sample rate: gpio 0-2 */;
pub const ICE1712_EWS88_RW: c_uint = 0x08	/* read/write switch for i2c; high = write  */;
pub const ICE1712_EWS88_SERIAL_DATA: c_uint = 0x10	/* i2c & ak4524 data */;
pub const ICE1712_EWS88_SERIAL_CLOCK: c_uint = 0x20	/* i2c & ak4524 clock */;
pub const ICE1712_EWS88_TX2: c_uint = 0x40	/* MIDI2 (only on 88D) */;
pub const ICE1712_EWS88_RX2: c_uint = 0x80	/* MIDI2 (only on 88D) */;
// i2c address

pub const ICE1712_EWS88MT_OUTPUT_SENSE: c_uint = 0x40	/* mask */;

// TerraTec DMX 6Fire configuration definitions
pub const ICE1712_6FIRE_AK4524_CS_MASK: c_uint = 0x07	/* AK4524 chip select #1-#3 */;
pub const ICE1712_6FIRE_RW: c_uint = 0x08	/* read/write switch for i2c; high = write  */;
pub const ICE1712_6FIRE_SERIAL_DATA: c_uint = 0x10	/* i2c & ak4524 data */;
pub const ICE1712_6FIRE_SERIAL_CLOCK: c_uint = 0x20	/* i2c & ak4524 clock */;
pub const ICE1712_6FIRE_TX2: c_uint = 0x40	/* MIDI2 */;
pub const ICE1712_6FIRE_RX2: c_uint = 0x80	/* MIDI2 */;

