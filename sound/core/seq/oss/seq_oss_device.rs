//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/oss/seq_oss_device.h
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
// OSS compatible sequencer driver
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

// max. applications
pub const SNDRV_SEQ_OSS_MAX_CLIENTS: c_int = 16;
pub const SNDRV_SEQ_OSS_MAX_SYNTH_DEVS: c_int = 16;
pub const SNDRV_SEQ_OSS_MAX_MIDI_DEVS: c_int = 32;
// version
pub const SNDRV_SEQ_OSS_MAJOR_VERSION: c_int = 0;
pub const SNDRV_SEQ_OSS_MINOR_VERSION: c_int = 1;
pub const SNDRV_SEQ_OSS_TINY_VERSION: c_int = 8;

// device and proc interface name

//
// type definitions
//
pub type reltime_t = c_uint;
pub type abstime_t = c_uint;
//
// synthesizer channel information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_oss_chinfo {
    pub vel: int note,,
}

//
// synthesizer information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_oss_synthinfo {
    pub arg: snd_seq_oss_arg,
    pub ch: *mut seq_oss_chinfo,
    pub nr_voices: c_int,
    pub opened: c_int,
    pub is_midi: c_int,
    pub midi_mapped: c_int,
}

//
// sequencer client information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_oss_devinfo {
    pub /: *mut *mut int index; / application index,
    pub /: *mut *mut int cseq; / sequencer client number,
    pub /: *mut *mut int port; / sequencer port number,
    pub /: *mut *mut int queue; / sequencer queue number,
    pub /: *mut *mut snd_seq_addr addr; / address of this device,
    pub /: *mut *mut int seq_mode; / sequencer mode,
    pub /: *mut *mut int file_mode; / file access,
// midi device table
    pub max_mididev: c_int,
// synth device table
    pub max_synthdev: c_int,
    pub synths: [seq_oss_synthinfo; SNDRV_SEQ_OSS_MAX_SYNTH_DEVS],
    pub synth_opened: c_int,
// output queue
    pub writeq: *mut seq_oss_writeq,
// midi input queue
    pub readq: *mut seq_oss_readq,
// timer
    pub timer: *mut seq_oss_timer,
}

//
// function prototypes
//
// create/delete OSS sequencer client
extern "C" {
    pub fn snd_seq_oss_create_client() -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_delete_client() -> c_int;
}
// device file interface
extern "C" {
    pub fn snd_seq_oss_open(file: *mut file, level: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_release(dp: *mut seq_oss_devinfo);
}
extern "C" {
    pub fn snd_seq_oss_ioctl(dp: *mut seq_oss_devinfo, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_read(dev: *mut seq_oss_devinfo, buf: *mut char __user, count: c_int) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_write(dp: *mut seq_oss_devinfo, buf: *const char __user, count: c_int, opt: *mut file) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_poll(dp: *mut seq_oss_devinfo, file: *mut file, wait: *mut *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn snd_seq_oss_reset(dp: *mut seq_oss_devinfo);
}
// proc interface
extern "C" {
    pub fn snd_seq_oss_system_info_read(buf: *mut snd_info_buffer);
}
extern "C" {
    pub fn snd_seq_oss_midi_info_read(buf: *mut snd_info_buffer);
}
extern "C" {
    pub fn snd_seq_oss_synth_info_read(buf: *mut snd_info_buffer);
}
extern "C" {
    pub fn snd_seq_oss_readq_info_read(q: *mut seq_oss_readq, buf: *mut snd_info_buffer);
}
// file mode macros

// dispatch event
extern "C" {
    pub fn snd_seq_kernel_client_dispatch(_arg: dp->cseq, _arg: ev, _arg: atomic, _arg: hop) -> return;
}
// ioctl for writeq
extern "C" {
    pub fn snd_seq_kernel_client_ioctl(_arg: dp->cseq, _arg: type, _arg: arg) -> return;
}
// fill the addresses in header
