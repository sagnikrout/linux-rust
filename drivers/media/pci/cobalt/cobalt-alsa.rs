//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cobalt/cobalt-alsa.h
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
// ALSA interface to cobalt PCM capture streams
//
// Copyright 2014-2015 Cisco Systems, Inc. and/or its affiliates.
// All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_cobalt_card {
    pub s: *mut cobalt_stream,
    pub sc: *mut snd_card,
    pub capture_transfer_done: c_uint,
    pub hwptr_done_capture: c_uint,
    pub alsa_record_cnt: unsigned,
    pub capture_pcm_substream: *mut snd_pcm_substream,
    pub pb_size: c_uint,
    pub pb_count: c_uint,
    pub pb_pos: c_uint,
    pub pb_filled: unsigned,
    pub alsa_pb_channel: bool,
    pub alsa_playback_cnt: unsigned,
    pub playback_pcm_substream: *mut snd_pcm_substream,
}

extern "C" {
    pub fn cobalt_alsa_init(s: *mut cobalt_stream) -> c_int;
}
extern "C" {
    pub fn cobalt_alsa_exit(s: *mut cobalt_stream);
}
