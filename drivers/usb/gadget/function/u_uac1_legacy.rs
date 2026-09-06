//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_uac1_legacy.h
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
// u_uac1.h -- interface to USB gadget "ALSA AUDIO" utilities
//
// Copyright (C) 2008 Bryan Wu <cooloney@kernel.org>
// Copyright (C) 2008 Analog Devices, Inc
//

pub const UAC1_OUT_EP_MAX_PACKET_SIZE: c_int = 200;
pub const UAC1_REQ_COUNT: c_int = 256;
pub const UAC1_AUDIO_BUF_SIZE: c_int = 48000;
//
// This represents the USB side of an audio card device, managed by a USB
// function which provides control and stream interfaces.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudio_snd_dev {
    pub card: *mut gaudio,
    pub filp: *mut file,
    pub substream: *mut snd_pcm_substream,
    pub access: c_int,
    pub format: c_int,
    pub channels: c_int,
    pub rate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudio {
    pub func: usb_function,
    pub gadget: *mut usb_gadget,
// ALSA sound device interfaces
    pub control: gaudio_snd_dev,
    pub playback: gaudio_snd_dev,
    pub capture: gaudio_snd_dev,
// TODO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_uac1_legacy_opts {
    pub func_inst: usb_function_instance,
    pub req_buf_size: c_int,
    pub req_count: c_int,
    pub audio_buf_size: c_int,
    pub fn_play: *mut c_char,
    pub fn_cap: *mut c_char,
    pub fn_cntl: *mut c_char,
    pub bound:1: unsigned,
    pub lock: mutex,
    pub refcnt: c_int,
}

extern "C" {
    pub fn gaudio_setup(card: *mut gaudio) -> c_int;
}
extern "C" {
    pub fn gaudio_cleanup(the_card: *mut gaudio);
}
extern "C" {
    pub fn u_audio_playback(card: *mut gaudio, buf: *mut c_void, count: usize) -> usize;
}
extern "C" {
    pub fn u_audio_get_playback_channels(card: *mut gaudio) -> c_int;
}
extern "C" {
    pub fn u_audio_get_playback_rate(card: *mut gaudio) -> c_int;
}
