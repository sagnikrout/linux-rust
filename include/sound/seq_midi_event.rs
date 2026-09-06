//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/seq_midi_event.h
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
// MIDI byte <-> sequencer event coder
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>,
// Jaroslav Kysela <perex@perex.cz>
//

pub const MAX_MIDI_EVENT_BUF: c_int = 256;
// midi status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_midi_event {
    pub /: *mut *mut int qlen; / queue length,
    pub /: *mut *mut int read; / chars read,
    pub /: *mut *mut int type; / current event type,
    pub /: *mut *mut unsigned char lastcmd; / last command (for MIDI state handling),
    pub /: *mut *mut unsigned char nostat; / no state flag,
    pub /: *mut *mut int bufsize; / allocated buffer size,
    pub /: *mut *mut *mut unsigned char buf; / input buffer,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn snd_midi_event_new(bufsize: c_int, rdev: *mut snd_midi_event) -> c_int;
}
extern "C" {
    pub fn snd_midi_event_free(dev: *mut snd_midi_event);
}
extern "C" {
    pub fn snd_midi_event_reset_encode(dev: *mut snd_midi_event);
}
extern "C" {
    pub fn snd_midi_event_reset_decode(dev: *mut snd_midi_event);
}
extern "C" {
    pub fn snd_midi_event_no_status(dev: *mut snd_midi_event, on: c_int);
}
// decode from event to bytes - return number of written bytes if success
