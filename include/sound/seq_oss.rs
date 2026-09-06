//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/seq_oss.h
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
// Copyright (C) 1998,99 Takashi Iwai
//

//
// argument structure for synthesizer operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_oss_arg {
// given by OSS sequencer
    pub /: *mut *mut int app_index; / application unique index,
    pub /: *mut *mut int file_mode; / file mode - see below,
    pub /: *mut *mut int seq_mode; / sequencer mode - see below,
// following must be initialized in open callback
    pub /: *mut *mut snd_seq_addr addr; / opened port address,
    pub /: *mut *mut *mut void private_data; / private data for lowlevel drivers,
// note-on event passing mode: initially given by OSS seq,
// but configurable by drivers - see below
//
    pub event_passing: c_int,
}

//
// synthesizer operation callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_oss_callback {
    pub owner: *mut module,
    pub closure): *mut *mut *mut int (open)(struct snd_seq_oss_arg p, void,
    pub p): *mut *mut int (close)(struct snd_seq_oss_arg,
    pub arg): *mut *mut *mut int (ioctl)(struct snd_seq_oss_arg p, unsigned int cmd, unsigned long,
    pub count): *const *const *const *const int (load_patch)(struct snd_seq_oss_arg p, int format, char __user buf, int offs, int,
    pub p): *mut *mut int (reset)(struct snd_seq_oss_arg,
    pub data): *mut *mut *mut int (raw_event)(struct snd_seq_oss_arg p, unsigned char,
}

// flag: file_mode
pub const SNDRV_SEQ_OSS_FILE_ACMODE: c_int = 3;
pub const SNDRV_SEQ_OSS_FILE_READ: c_int = 1;
pub const SNDRV_SEQ_OSS_FILE_WRITE: c_int = 2;
pub const SNDRV_SEQ_OSS_FILE_NONBLOCK: c_int = 4;
// flag: seq_mode
pub const SNDRV_SEQ_OSS_MODE_SYNTH: c_int = 0;
pub const SNDRV_SEQ_OSS_MODE_MUSIC: c_int = 1;
// flag: event_passing

// default control rate: fixed
pub const SNDRV_SEQ_OSS_CTRLRATE: c_int = 100;
// default max queue length: configurable by module option
pub const SNDRV_SEQ_OSS_MAX_QLEN: c_int = 1024;
//
// data pointer to snd_seq_register_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_oss_reg {
    pub type: c_int,
    pub subtype: c_int,
    pub nvoices: c_int,
    pub oper: snd_seq_oss_callback,
    pub private_data: *mut c_void,
}

// device id

