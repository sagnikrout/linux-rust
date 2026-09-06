//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/rawmidi.h
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
// Abstract layer for MIDI v1.0 stream
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

//
// Raw MIDI interface
//
pub const SNDRV_RAWMIDI_DEVICES: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_ops {
    pub substream): *mut *mut *mut int (open) (struct snd_rawmidi_substream,
    pub substream): *mut *mut *mut int (close) (struct snd_rawmidi_substream,
    pub up): *mut *mut *mut void (trigger) (struct snd_rawmidi_substream  substream, int,
    pub substream): *mut *mut *mut void (drain) (struct snd_rawmidi_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_global_ops {
    pub rmidi): *mut *mut *mut int (dev_register) (struct snd_rawmidi,
    pub rmidi): *mut *mut *mut int (dev_unregister) (struct snd_rawmidi,
    pub info): *mut snd_seq_port_info,
    pub argp): *mut void __user,
    pub buf): *mut snd_info_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_runtime {
    pub substream: *mut snd_rawmidi_substream,
    pub /: *mut *mut oss: 1; / OSS compatible mode,
// midi stream buffer
    pub /: *mut *mut *mut unsigned char buffer; / buffer for MIDI data,
    pub /: *mut *mut size_t buffer_size; / size of buffer,
    pub /: *mut *mut size_t appl_ptr; / application pointer,
    pub /: *mut *mut size_t hw_ptr; / hardware pointer,
    pub /: *mut *mut size_t avail_min; / min avail for wakeup,
    pub /: *mut *mut size_t avail; / max used buffer for wakeup,
    pub /: *mut *mut size_t xruns; / over/underruns counter,
    pub /: *mut *mut size_t align; / alignment (0 = byte stream, 3 = UMP),
    pub /: *mut *mut int buffer_ref; / buffer reference count,
// misc
    pub sleep: wait_queue_head_t,
// event handler (new bytes, input only)
    pub substream): *mut *mut void (event)(struct snd_rawmidi_substream,
// defers calls to event [input] or ops->trigger [output]
    pub event_work: work_struct,
// private data
    pub private_data: *mut c_void,
    pub substream): *mut *mut void (private_free)(struct snd_rawmidi_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_substream {
    pub /: *mut *mut list_head list; / list of all substream for given stream,
    pub /: *mut *mut int stream; / direction,
    pub /: *mut *mut int number; / substream number,
    pub /: *mut *mut bool opened; / open flag,
    pub /: *mut *mut bool append; / append flag (merge more streams),
    pub /: *mut *mut bool active_sensing; / send active sensing when close,
    pub /: *mut *mut unsigned int framing; / whether to frame input data,
    pub /: *mut *mut unsigned int clock_type; / clock source to use for input framing,
    pub /: *mut *mut int use_count; / use counter (for output),
    pub /: *mut *mut bool inactive; / inactive substream (for UMP legacy),
    pub bytes: usize,
    pub lock: spinlock_t,
    pub rmidi: *mut snd_rawmidi,
    pub pstr: *mut snd_rawmidi_str,
    pub name: [c_char; 32],
    pub runtime: *mut snd_rawmidi_runtime,
    pub pid: *mut pid,
// hardware layer
    pub ops: *const snd_rawmidi_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_file {
    pub rmidi: *mut snd_rawmidi,
    pub input: *mut snd_rawmidi_substream,
    pub output: *mut snd_rawmidi_substream,
    pub /: *mut *mut unsigned int user_pversion; / supported protocol version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_str {
    pub substream_count: c_uint,
    pub substream_opened: c_uint,
    pub substreams: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi {
    pub card: *mut snd_card,
    pub list: list_head,
    pub /: *mut *mut unsigned int device; / device number,
    pub /: *mut *mut unsigned int info_flags; / SNDRV_RAWMIDI_INFO_XXXX,
    pub tied_device: c_uint,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
    pub ossreg: c_int,

    pub ops: *const snd_rawmidi_global_ops,
    pub streams: [snd_rawmidi_str; 2],
    pub private_data: *mut c_void,
    pub rmidi): *mut *mut void (private_free) (struct snd_rawmidi,
    pub open_mutex: mutex,
    pub open_wait: wait_queue_head_t,
    pub dev: *mut device,
    pub proc_entry: *mut snd_info_entry,

    pub seq_dev: *mut snd_seq_device,

}

// main rawmidi functions
// internal
extern "C" {
    pub fn snd_rawmidi_free(rmidi: *mut snd_rawmidi) -> c_int;
}
// callbacks
extern "C" {
    pub fn snd_rawmidi_transmit_empty(substream: *mut snd_rawmidi_substream) -> c_int;
}
extern "C" {
    pub fn snd_rawmidi_transmit_ack(substream: *mut snd_rawmidi_substream, count: c_int) -> c_int;
}
extern "C" {
    pub fn snd_rawmidi_proceed(substream: *mut snd_rawmidi_substream) -> c_int;
}
// main midi functions
extern "C" {
    pub fn snd_rawmidi_info_select(card: *mut snd_card, info: *mut snd_rawmidi_info) -> c_int;
}
extern "C" {
    pub fn snd_rawmidi_drop_output(substream: *mut snd_rawmidi_substream) -> c_int;
}
extern "C" {
    pub fn snd_rawmidi_drain_output(substream: *mut snd_rawmidi_substream) -> c_int;
}
extern "C" {
    pub fn snd_rawmidi_drain_input(substream: *mut snd_rawmidi_substream) -> c_int;
}
// non-nested version
extern "C" {
    pub fn snd_rawmidi_kernel_open_nested(_arg: rmidi, _arg: subdevice, _arg: mode, _arg: rfile, _arg: 0) -> return;
}
extern "C" {
    pub fn snd_rawmidi_kernel_release_nested(_arg: rfile, _arg: 0) -> return;
}
// set up the tied devices
// tied_device field keeps the device+1 (so that 0 being unknown)
