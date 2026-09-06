//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/line6/playback.h
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
// When the TonePort is used with jack in full duplex mode and the outputs are
// not connected, the software monitor produces an ugly noise since everything
// written to the output buffer (i.e., the input signal) will be repeated in
// the next period (sounds like a delay effect). As a workaround, the output
// buffer is cleared after the data have been read, but there must be a better
// solution. Until one is found, this workaround can be used to fix the
// problem.
//
pub const USE_CLEAR_BUFFER_WORKAROUND: c_int = 1;
extern "C" {
    pub fn line6_create_audio_out_urbs(line6pcm: *mut snd_line6_pcm) -> c_int;
}
extern "C" {
    pub fn line6_submit_audio_out_all_urbs(line6pcm: *mut snd_line6_pcm) -> c_int;
}
