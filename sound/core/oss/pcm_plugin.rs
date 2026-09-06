//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/oss/pcm_plugin.h
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
// Digital Audio (Plugin interface) abstract layer
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_pcm_plugin_action {
    INIT = 0,
    PREPARE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_channel_area {
    pub /: *mut *mut *mut void addr; / base address of channel samples,
    pub /: *mut *mut unsigned int first; / offset to first sample in bits,
    pub /: *mut *mut unsigned int step; / samples distance in bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_plugin_channel {
    pub /: *mut *mut *mut void aptr; / pointer to the allocated area,
    pub area: snd_pcm_channel_area,
    pub /: *mut *mut snd_pcm_uframes_t frames; / allocated frames,
    pub /: *mut *mut unsigned int enabled:1; / channel need to be processed,
    pub /: *mut *mut unsigned int wanted:1; / channel is wanted,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_plugin_format {
    pub format: snd_pcm_format_t,
    pub rate: c_uint,
    pub channels: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_plugin {
    pub /: *const *const *const char name; / plug-in name,
    pub stream: c_int,
    pub /: *mut *mut snd_pcm_plugin_format src_format; / source format,
    pub /: *mut *mut snd_pcm_plugin_format dst_format; / destination format,
    pub /: *mut *mut int src_width; / sample width in bits,
    pub /: *mut *mut int dst_width; / sample width in bits,
    pub access: snd_pcm_access_t,
    pub dst_frames): *mut *mut *mut snd_pcm_sframes_t (src_frames)(struct snd_pcm_plugin plugin, snd_pcm_uframes_t,
    pub src_frames): *mut *mut *mut snd_pcm_sframes_t (dst_frames)(struct snd_pcm_plugin plugin, snd_pcm_uframes_t,
    pub channels): *mut snd_pcm_plugin_channel,
    pub frames): snd_pcm_uframes_t,
    pub data): c_ulong,
    pub prev: *mut snd_pcm_plugin,
    pub next: *mut snd_pcm_plugin,
    pub plug: *mut snd_pcm_substream,
    pub private_data: *mut c_void,
    pub plugin): *mut *mut void (private_free)(struct snd_pcm_plugin,
    pub buf: *mut c_char,
    pub buf_frames: snd_pcm_uframes_t,
    pub buf_channels: *mut snd_pcm_plugin_channel,
    pub extra_data: [c_char; ],
}

extern "C" {
    pub fn snd_pcm_plugin_free(plugin: *mut snd_pcm_plugin) -> c_int;
}
extern "C" {
    pub fn snd_pcm_plug_alloc(plug: *mut snd_pcm_substream, frames: snd_pcm_uframes_t) -> c_int;
}
extern "C" {
    pub fn snd_pcm_plug_client_size(handle: *mut snd_pcm_substream, drv_size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
}
extern "C" {
    pub fn snd_pcm_plug_slave_size(handle: *mut snd_pcm_substream, clt_size: snd_pcm_uframes_t) -> snd_pcm_sframes_t;
}

extern "C" {
    pub fn snd_pcm_plugin_append(plugin: *mut snd_pcm_plugin) -> c_int;
}

