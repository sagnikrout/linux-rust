//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/pcm-indirect.h
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
// Helper functions for indirect PCM data transfer
//
// Copyright (c) by Takashi Iwai <tiwai@suse.de>
// Jaroslav Kysela <perex@perex.cz>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_indirect {
    pub /: *mut *mut unsigned int hw_buffer_size; / Byte size of hardware buffer,
    pub /: *mut *mut unsigned int hw_queue_size; / Max queue size of hw buffer (0 = buffer size),
    pub /: *mut *mut unsigned int hw_data; / Offset to next dst (or src) in hw ring buffer,
    pub /: *mut *mut unsigned int hw_io; / Ring buffer hw pointer,
    pub /: *mut *mut int hw_ready; / Bytes ready for play (or captured) in hw ring buffer,
    pub /: *mut *mut unsigned int sw_buffer_size; / Byte size of software buffer,
    pub /: *mut *mut unsigned int sw_data; / Offset to next dst (or src) in sw ring buffer,
    pub /: *mut *mut unsigned int sw_io; / Current software pointer in bytes,
    pub /: *mut *mut int sw_ready; / Bytes ready to be transferred to/from hw,
    pub /: *mut *mut snd_pcm_uframes_t appl_ptr; / Last seen appl_ptr,
}

//
// helper function for playback ack callback
//
// helper function for playback pointer callback
// ptr = current byte pointer
//
extern "C" {
    pub fn bytes_to_frames(_arg: substream->runtime, _arg: rec->sw_io) -> return;
}
//
// helper function for capture ack callback
//
// helper function for capture pointer callback,
// ptr = current byte pointer
//
extern "C" {
    pub fn bytes_to_frames(_arg: substream->runtime, _arg: rec->sw_io) -> return;
}
