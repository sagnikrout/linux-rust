//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/oss/seq_oss_event.h
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
// seq_oss_event.h - OSS event queue record
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

pub const SHORT_EVENT_SIZE: c_int = 4;
pub const LONG_EVENT_SIZE: c_int = 8;
// short event (4bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_short {
    pub code: c_uchar,
    pub parm1: c_uchar,
    pub dev: c_uchar,
    pub parm2: c_uchar,
}

// short note events (4bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_note {
    pub code: c_uchar,
    pub chn: c_uchar,
    pub note: c_uchar,
    pub vel: c_uchar,
}

// long timer events (8bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_timer {
    pub code: c_uchar,
    pub cmd: c_uchar,
    pub dummy2: unsigned char dummy1,,
    pub time: c_uint,
}

// long extended events (8bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_extended {
    pub code: c_uchar,
    pub cmd: c_uchar,
    pub dev: c_uchar,
    pub chn: c_uchar,
    pub p4: unsigned char p1, p2, p3,,
}

// long channel events (8bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_long {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub cmd: c_uchar,
    pub chn: c_uchar,
    pub p2: unsigned char p1,,
    pub val: c_ushort,
}

// channel voice events (8bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_voice {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub cmd: c_uchar,
    pub chn: c_uchar,
    pub parm: unsigned char note,,
    pub dummy: c_ushort,
}

// sysex events (8bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evrec_sysex {
    pub code: c_uchar,
    pub dev: c_uchar,
    pub buf: [c_uchar; 6],
}

// event record
#[repr(C)]
#[derive(Copy, Clone)]
pub union evrec {
    pub s: evrec_short,
    pub n: evrec_note,
    pub l: evrec_long,
    pub v: evrec_voice,
    pub t: evrec_timer,
    pub e: evrec_extended,
    pub x: evrec_sysex,
    pub echo: c_uint,
    pub c: [c_uchar; LONG_EVENT_SIZE],
}

extern "C" {
    pub fn snd_seq_oss_process_timer_event(rec: *mut seq_oss_timer, q: *mut evrec) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_event_input(ev: *mut snd_seq_event, direct: c_int, private_data: *mut c_void, atomic: c_int, hop: c_int) -> c_int;
}
