//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/pcm_local.h
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
// pcm_local.h - a local header file for snd-pcm module.
//
// Copyright (c) Takashi Sakamoto <o-takashi@sakamocchi.jp>
//
extern "C" {
    pub fn snd_pcm_update_hw_ptr(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_pcm_playback_avail(_arg: substream->runtime) -> return;
}
extern "C" {
    pub fn snd_pcm_capture_avail(_arg: substream->runtime) -> return;
}
extern "C" {
    pub fn snd_pcm_playback_hw_avail(_arg: substream->runtime) -> return;
}
extern "C" {
    pub fn snd_pcm_capture_hw_avail(_arg: substream->runtime) -> return;
}

extern "C" {
    pub fn snd_pcm_timer_resolution_change(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_timer_init(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_timer_done(substream: *mut snd_pcm_substream);
}

extern "C" {
    pub fn __snd_pcm_xrun(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_group_init(group: *mut snd_pcm_group);
}
extern "C" {
    pub fn snd_pcm_sync_stop(substream: *mut snd_pcm_substream, sync_irq: bool);
}

// loop over all PCM substreams

