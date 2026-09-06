//! Automatically rewritten from C Header to Rust Module
//! Source: sound/drivers/pcsp/pcsp.h
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
//
// PC-Speaker driver for Linux
//
// Copyright (C) 1993-1997  Michael Beck
// Copyright (C) 1997-2001  David Woodhouse
// Copyright (C) 2001-2008  Stas Sergeev
//

pub const PCSP_SOUND_VERSION: c_uint = 0x400	/* read 4.00 */;
pub const PCSP_DEBUG: c_int = 0;
// default timer freq for PC-Speaker: 18643 Hz
pub const DIV_18KHZ: c_int = 64;

pub const PCSP_MAX_TREBLE: c_int = 1;
// unfortunately, with hrtimers 37KHz does not work very well :(
pub const PCSP_DEFAULT_TREBLE: c_int = 0;

// wild guess
pub const PCSP_MIN_LPJ: c_int = 1000000;

pub const PCSP_MAX_PERIODS: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcsp {
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub input_dev: *mut input_dev,
    pub timer: hrtimer,
    pub dma: unsigned short port, irq,,
    pub substream_lock: spinlock_t,
    pub playback_substream: *mut snd_pcm_substream,
    pub fmt_size: c_uint,
    pub is_signed: c_uint,
    pub playback_ptr: usize,
    pub period_ptr: usize,
    pub timer_active: core::sync::atomic::AtomicI32,
    pub thalf: c_int,
    pub ns_rem: u64,
    pub val61: c_uchar,
    pub enable: c_int,
    pub max_treble: c_int,
    pub treble: c_int,
    pub pcspkr: c_int,
}

extern "C" {
    pub fn pcsp_do_timer(handle: *mut hrtimer) -> hrtimer_restart;
}
extern "C" {
    pub fn pcsp_sync_stop(chip: *mut snd_pcsp);
}
extern "C" {
    pub fn snd_pcsp_new_pcm(chip: *mut snd_pcsp) -> c_int;
}
extern "C" {
    pub fn snd_pcsp_new_mixer(chip: *mut snd_pcsp, nopcm: c_int) -> c_int;
}
