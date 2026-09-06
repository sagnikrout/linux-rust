//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ultrasound.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// ultrasound.h - Macros for programming the Gravis Ultrasound
// These macros are extremely device dependent
// and not portable.
//
// Copyright (C) by Hannu Savolainen 1993-1997
//
// OSS/Free for Linux is distributed under the GNU GENERAL PUBLIC LICENSE (GPL)
// Version 2 (June 1991). See the "COPYING" file distributed with this software
// for more info.
//
// Private events for Gravis Ultrasound (GUS)
//
// Format:
// byte 0 		- SEQ_PRIVATE (0xfe)
// byte 1 		- Synthesizer device number (0-N)
// byte 2 		- Command (see below)
// byte 3 		- Voice number (0-31)
// bytes 4 and 5	- parameter P1 (unsigned short)
// bytes 6 and 7	- parameter P2 (unsigned short)
//
// Commands:
// Each command affects one voice defined in byte 3.
// Unused parameters (P1 and/or P2 *MUST* be initialized to zero).
// _GUS_NUMVOICES	- Sets max. number of concurrent voices (P1=14-31, default 16)
// _GUS_VOICESAMPLE- ************ OBSOLETE
// _GUS_VOICEON	- Starts voice (P1=voice mode)
// _GUS_VOICEOFF	- Stops voice (no parameters)
// _GUS_VOICEFADE	- Stops the voice smoothly.
// _GUS_VOICEMODE	- Alters the voice mode, don't start or stop voice (P1=voice mode)
// _GUS_VOICEBALA	- Sets voice balance (P1, 0=left, 7=middle and 15=right, default 7)
// _GUS_VOICEFREQ	- Sets voice (sample) playback frequency (P1=Hz)
// _GUS_VOICEVOL	- Sets voice volume (P1=volume, 0xfff=max, 0xeff=half, 0x000=off)
// _GUS_VOICEVOL2	- Sets voice volume (P1=volume, 0xfff=max, 0xeff=half, 0x000=off)
// (Like GUS_VOICEVOL but doesn't change the hw
// volume. It just updates volume in the voice table).
//
// _GUS_RAMPRANGE	- Sets limits for volume ramping (P1=low volume, P2=high volume)
// _GUS_RAMPRATE	- Sets the speed for volume ramping (P1=scale, P2=rate)
// _GUS_RAMPMODE	- Sets the volume ramping mode (P1=ramping mode)
// _GUS_RAMPON	- Starts volume ramping (no parameters)
// _GUS_RAMPOFF	- Stops volume ramping (no parameters)
// _GUS_VOLUME_SCALE - Changes the volume calculation constants
// for all voices.
//
pub const _GUS_NUMVOICES: c_uint = 0x00;
pub const _GUS_VOICESAMPLE: c_uint = 0x01	/* OBSOLETE */;
pub const _GUS_VOICEON: c_uint = 0x02;
pub const _GUS_VOICEOFF: c_uint = 0x03;
pub const _GUS_VOICEMODE: c_uint = 0x04;
pub const _GUS_VOICEBALA: c_uint = 0x05;
pub const _GUS_VOICEFREQ: c_uint = 0x06;
pub const _GUS_VOICEVOL: c_uint = 0x07;
pub const _GUS_RAMPRANGE: c_uint = 0x08;
pub const _GUS_RAMPRATE: c_uint = 0x09;
pub const _GUS_RAMPMODE: c_uint = 0x0a;
pub const _GUS_RAMPON: c_uint = 0x0b;
pub const _GUS_RAMPOFF: c_uint = 0x0c;
pub const _GUS_VOICEFADE: c_uint = 0x0d;
pub const _GUS_VOLUME_SCALE: c_uint = 0x0e;
pub const _GUS_VOICEVOL2: c_uint = 0x0f;
pub const _GUS_VOICE_POS: c_uint = 0x10;
//
// GUS API macros
//

// (unsigned short*)&_seqbuf[_seqbufptr+4] = p1;\
// (unsigned short*)&_seqbuf[_seqbufptr+6] = p2;\

