//! Automatically rewritten from C Header to Rust Module
//! Source: sound/ppc/awacs.h
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
// Driver for PowerMac AWACS onboard soundchips
// Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
// based on dmasound.c.
//
// AWACs Audio Register Layout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct awacs_regs {
    pub /: *mut *mut unsigned control; / Audio control register,
    pub pad0: [unsigned; 3],
    pub /: *mut *mut unsigned codec_ctrl; / Codec control register,
    pub pad1: [unsigned; 3],
    pub /: *mut *mut unsigned codec_stat; / Codec status register,
    pub pad2: [unsigned; 3],
    pub /: *mut *mut unsigned clip_count; / Clipping count register,
    pub pad3: [unsigned; 3],
    pub /: *mut *mut unsigned byteswap; / Data is little-endian if 1,
}

//
// Audio Bit Masks
//
// Audio Control Reg Bit Masks
// ----- ------- --- --- -----

// Audio Codec Control Reg Bit Masks
// ----- ----- ------- --- --- -----

// Audio Codec Control Address Values / Masks
// ----- ----- ------- ------- ------ - -----

// additional registers of screamer

// Address 0 Bit Masks & Macros
// ------- - --- ----- - ------

pub const SHIFT_GAINLINE: c_int = 8;
pub const SHIFT_MUX_CD: c_int = 9;
pub const SHIFT_MUX_MIC: c_int = 10;
pub const SHIFT_MUX_LINE: c_int = 11;

// Address 1 Bit Masks
// ------- - --- -----

pub const SHIFT_LOOPTHRU: c_int = 6;

pub const SHIFT_SPKMUTE: c_int = 7;

pub const SHIFT_HDMUTE: c_int = 9;

pub const SHIFT_PAROUT: c_int = 10;
pub const SHIFT_PAROUT0: c_int = 10;
pub const SHIFT_PAROUT1: c_int = 11;

// Address 2 & 4 Bit Masks & Macros
// ------- - - - --- ----- - ------

// address 6

pub const SHIFT_MIC_BOOST: c_int = 2;
// Audio Codec Status Reg Bit Masks
// ----- ----- ------ --- --- -----

// Clipping Count Reg Bit Masks
// -------- ----- --- --- -----

// DBDMA ChannelStatus Bit Masks
// ----- ------------- --- -----

// Various Rates
// ------- -----

