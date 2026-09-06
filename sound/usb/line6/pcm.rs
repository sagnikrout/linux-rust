//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/line6/pcm.h
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

//
pub const LINE6_ISO_PACKETS: c_int = 1;
// in a "full speed" device (such as the PODxt Pro) this means 1ms,
// for "high speed" it's 1/8ms
//
pub const LINE6_ISO_INTERVAL: c_int = 1;
pub const LINE6_IMPULSE_DEFAULT_PERIOD: c_int = 100;
//

//
// ) PCM playback and capture via ALSA
// ) software monitoring (for devices without hardware monitoring)
// ) optional impulse response measurement
//
// stream types
// misc bit flags for PCM operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct line6_pcm_properties {
    pub capture_hw: snd_pcm_hardware playback_hw,,
    pub rates: snd_pcm_hw_constraint_ratdens,
    pub bytes_per_channel: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct line6_pcm_stream {
// allocated URBs
    pub urbs: *mut urb,
// Temporary buffer;
// Since the packet size is not known in advance, this buffer is
// large enough to store maximum size packets.
//
    pub buffer: *mut c_uchar,
// Free frame position in the buffer.
    pub pos: snd_pcm_uframes_t,
// Count processed bytes;
// This is modulo period size (to determine when a period is finished).
//
    pub bytes: unsigned,
// Counter to create desired sample rate
    pub count: unsigned,
// period size in bytes
    pub period: unsigned,
// Processed frame position in the buffer;
// The contents of the ring buffer have been consumed by the USB
// subsystem (i.e., sent to the USB device) up to this position.
//
    pub pos_done: snd_pcm_uframes_t,
// Bit mask of active URBs
    pub active_urbs: c_ulong,
// Bit mask of URBs currently being unlinked
    pub unlink_urbs: c_ulong,
// Spin lock to protect updates of the buffer positions (not contents)
//
    pub lock: spinlock_t,
// Bit flags for operational stream types
    pub opened: c_ulong,
// Bit flags for running stream types
    pub running: c_ulong,
    pub last_frame: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_line6_pcm {
// Pointer back to the Line 6 driver data structure
    pub line6: *mut usb_line6,
// Properties.
    pub properties: *mut line6_pcm_properties,
// ALSA pcm stream
    pub pcm: *mut snd_pcm,
// protection to state changes of in/out streams
    pub state_mutex: mutex,
// Capture and playback streams
    pub in: line6_pcm_stream,
    pub out: line6_pcm_stream,
// Previously captured frame (for software monitoring)
    pub prev_fbuf: *mut c_uchar,
// Size of previously captured frame (for software monitoring/sync)
    pub prev_fsize: c_int,
// Maximum size of USB packet
    pub max_packet_size_in: c_int,
    pub max_packet_size_out: c_int,
// PCM playback volume (left and right)
    pub volume_playback: [c_int; 2],
// PCM monitor volume
    pub volume_monitor: c_int,
// Volume of impulse response test signal (if zero, test is disabled)
    pub impulse_volume: c_int,
// Period of impulse response test signal
    pub impulse_period: c_int,
// Counter for impulse response test signal
    pub impulse_count: c_int,
// Several status bits (see LINE6_FLAG_*)
    pub flags: c_ulong,
}

extern "C" {
    pub fn snd_line6_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn snd_line6_prepare(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_line6_hw_free(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_line6_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
}
extern "C" {
    pub fn line6_pcm_disconnect(line6pcm: *mut snd_line6_pcm);
}
extern "C" {
    pub fn line6_pcm_release(line6pcm: *mut snd_line6_pcm, type: c_int);
}
