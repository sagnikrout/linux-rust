//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/speakup_dtlk.h
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


// SPDX-License-Identifier: GPL-2.0
// speakup_dtlk.h - header file for speakups DoubleTalk driver.
pub const SYNTH_IO_EXTENT: c_uint = 0x02;
pub const SYNTH_CLEAR: c_uint = 0x18		/* stops speech */;
// TTS Port Status Flags
pub const TTS_READABLE: c_uint = 0x80	/* mask for bit which is nonzero if a;
// byte can be read from the TTS port
//
pub const TTS_SPEAKING: c_uint = 0x40	/* mask for SYNC bit, which is nonzero;
// while DoubleTalk is producing
// output with TTS, PCM or CVSD
// synthesizers or tone generators
// (that is, all but LPC)
//
pub const TTS_SPEAKING2: c_uint = 0x20	/* mask for SYNC2 bit,;
// which falls to zero up to 0.4 sec
// before speech stops
//
pub const TTS_WRITABLE: c_uint = 0x10	/* mask for RDY bit, which when set to;
// 1, indicates the TTS port is ready
// to accept a byte of data.  The RDY
// bit goes zero 2-3 usec after
// writing, and goes 1 again 180-190
// usec later.
//
pub const TTS_ALMOST_FULL: c_uint = 0x08	/* mask for AF bit: When set to 1,;
// indicates that less than 300 bytes
// are available in the TTS input
// buffer. AF is always 0 in the PCM,
// TGN and CVSD modes.
//
pub const TTS_ALMOST_EMPTY: c_uint = 0x04	/* mask for AE bit: When set to 1,;
// indicates that less than 300 bytes
// are remaining in DoubleTalk's input
// (TTS or PCM) buffer. AE is always 1
// in the TGN and CVSD modes.
//
// data returned by Interrogate command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synth_settings {
    pub /: *mut *mut u16 serial_number; / 0-7Fh:0-7Fh,
    pub /: *mut *mut u_char rom_version[24]; / null terminated string,
    pub /: *mut *mut u_char mode; / 0=Character; 1=Phoneme; 2=Text,
    pub /: *mut *mut u_char punc_level; / nB; 0-7,
    pub /: *mut *mut u_char formant_freq; / nF; 0-9,
    pub /: *mut *mut u_char pitch; / nP; 0-99,
    pub /: *mut *mut u_char speed; / nS; 0-9,
    pub /: *mut *mut u_char volume; / nV; 0-9,
    pub /: *mut *mut u_char tone; / nX; 0-2,
    pub /: *mut *mut u_char expression; / nE; 0-9,
    pub /: *mut *mut u_char ext_dict_loaded; / 1=exception dictionary loaded,
    pub /: *mut *mut u_char ext_dict_status; / 1=exception dictionary enabled,
    pub for: *mut *mut u_char free_ram; / # pages (truncated) remaining,
// text buffer
//
    pub /: *mut *mut u_char articulation; / nA; 0-9,
    pub /: *mut *mut u_char reverb; / nR; 0-9,
    pub of: *mut *mut u_char eob; / 7Fh value indicating end,
// parameter block
//
    pub /: *mut *mut u_char has_indexing; / nonzero if indexing is implemented,
}
