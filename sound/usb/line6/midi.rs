//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/line6/midi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
//

pub const MIDI_BUFFER_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_line6_midi {
// Pointer back to the Line 6 driver data structure
    pub line6: *mut usb_line6,
// MIDI substream for receiving (or NULL if not active)
    pub substream_receive: *mut snd_rawmidi_substream,
// MIDI substream for transmitting (or NULL if not active)
    pub substream_transmit: *mut snd_rawmidi_substream,
// Number of currently active MIDI send URBs
    pub num_active_send_urbs: c_int,
// Spin lock to protect MIDI buffer handling
    pub lock: spinlock_t,
// Wait queue for MIDI transmission
    pub send_wait: wait_queue_head_t,
// Buffer for incoming MIDI stream
    pub midibuf_in: midi_buffer,
// Buffer for outgoing MIDI stream
    pub midibuf_out: midi_buffer,
}

extern "C" {
    pub fn line6_init_midi(line6: *mut usb_line6) -> c_int;
}
