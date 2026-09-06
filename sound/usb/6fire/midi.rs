//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/6fire/midi.h
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
// Linux driver for TerraTec DMX 6Fire USB
//
// Author:	Torsten Schenk <torsten.schenk@zoho.com>
// Created:	Jan 01, 2011
// Copyright:	(C) Torsten Schenk
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct midi_runtime {
    pub chip: *mut sfire_chip,
    pub instance: *mut snd_rawmidi,
    pub in: *mut snd_rawmidi_substream,
    pub in_active: c_char,
    pub in_lock: spinlock_t,
    pub out_lock: spinlock_t,
    pub out: *mut snd_rawmidi_substream,
    pub out_urb: urb,
    pub /: *mut *mut u8 out_serial; / serial number of out packet,
    pub out_buffer: *mut u8,
    pub buffer_offset: c_int,
    pub length): *mut *mut *mut *mut void (in_received)(struct midi_runtime rt, u8 data, int,
}

extern "C" {
    pub fn usb6fire_midi_init(chip: *mut sfire_chip) -> c_int;
}
extern "C" {
    pub fn usb6fire_midi_abort(chip: *mut sfire_chip);
}
extern "C" {
    pub fn usb6fire_midi_destroy(chip: *mut sfire_chip);
}
