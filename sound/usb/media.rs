//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/media.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// media.h - Media Controller specific ALSA driver code
//
// Copyright (c) 2019 Shuah Khan <shuah@kernel.org>
//
// This file adds Media Controller support to the ALSA driver
// to use the Media Controller API to share the tuner with DVB
// and V4L2 drivers that control the media device.
//
// The media device is created based on the existing quirks framework.
// Using this approach, the media controller API usage can be added for
// a specific device.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_ctl {
    pub media_dev: *mut media_device,
    pub media_entity: media_entity,
    pub intf_devnode: *mut media_intf_devnode,
    pub intf_link: *mut media_link,
    pub media_pad: media_pad,
    pub media_pipe: media_pipeline,
}

//
// One source pad each for SNDRV_PCM_STREAM_CAPTURE and
// SNDRV_PCM_STREAM_PLAYBACK. One for sink pad to link
// to AUDIO Source
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_mixer_ctl {
    pub media_dev: *mut media_device,
    pub media_entity: media_entity,
    pub intf_devnode: *mut media_intf_devnode,
    pub intf_link: *mut media_link,
    pub media_pad: [media_pad; MEDIA_MIXER_PAD_MAX],
    pub media_pipe: media_pipeline,
}

extern "C" {
    pub fn snd_media_device_delete(chip: *mut snd_usb_audio);
}
extern "C" {
    pub fn snd_media_stream_delete(subs: *mut snd_usb_substream);
}
extern "C" {
    pub fn snd_media_start_pipeline(subs: *mut snd_usb_substream) -> c_int;
}
extern "C" {
    pub fn snd_media_stop_pipeline(subs: *mut snd_usb_substream);
}

