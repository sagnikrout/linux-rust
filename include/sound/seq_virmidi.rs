//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/seq_virmidi.h
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
// Virtual Raw MIDI client on Sequencer
// Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>,
// Jaroslav Kysela <perex@perex.cz>
//

//
// device file instance:
// This instance is created at each time the midi device file is
// opened.  Each instance has its own input buffer and MIDI parser
// (buffer), and is associated with the device instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_virmidi {
    pub list: list_head,
    pub seq_mode: c_int,
    pub client: c_int,
    pub port: c_int,
    pub trigger: bool,
    pub parser: *mut snd_midi_event,
    pub event: snd_seq_event,
    pub rdev: *mut snd_virmidi_dev,
    pub substream: *mut snd_rawmidi_substream,
    pub output_work: work_struct,
}

//
// device record:
// Each virtual midi device has one device instance.  It contains
// common information and the linked-list of opened files,
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_virmidi_dev {
    pub /: *mut *mut *mut snd_card card; / associated card,
    pub /: *mut *mut *mut snd_rawmidi rmidi; / rawmidi device,
    pub /: *mut *mut int seq_mode; / SNDRV_VIRMIDI_XXX,
    pub /: *mut *mut int device; / sequencer device,
    pub /: *mut *mut int client; / created/attached client,
    pub /: *mut *mut int port; / created/attached port,
    pub /: *mut *mut *mut unsigned int flags; / SNDRV_VIRMIDI_,
    pub filelist_sem: rw_semaphore,
    pub filelist: list_head,
}

// sequencer mode:
// ATTACH = input/output events from midi device are routed to the
// attached sequencer port.  sequencer port is not created
// by virmidi itself.
// the input to rawmidi must be processed by passing the
// incoming events via snd_virmidi_receive()
// DISPATCH = input/output events are routed to subscribers.
// sequencer port is created in virmidi.
//
pub const SNDRV_VIRMIDI_SEQ_NONE: c_int = 0;
pub const SNDRV_VIRMIDI_SEQ_ATTACH: c_int = 1;
pub const SNDRV_VIRMIDI_SEQ_DISPATCH: c_int = 2;
extern "C" {
    pub fn snd_virmidi_new(card: *mut snd_card, device: c_int, rrmidi: *mut snd_rawmidi) -> c_int;
}
