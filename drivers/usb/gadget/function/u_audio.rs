//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_audio.h
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
// u_audio.h -- interface to USB gadget "ALSA sound card" utilities
//
// Copyright (C) 2016
// Author: Ruslan Bilovol <ruslan.bilovol@gmail.com>
//

//
// Same maximum frequency deviation on the slower side as in
// sound/usb/endpoint.c. Value is expressed in per-mil deviation.
//
pub const FBACK_SLOW_MAX: c_int = 250;
//
// Maximum frequency deviation on the faster side, default value for UAC1/2.
// Value is expressed in per-mil deviation.
// UAC2 provides the value as a parameter as it impacts the endpoint required
// bandwidth.
//
pub const FBACK_FAST_MAX: c_int = 5;
// Feature Unit parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_fu_params {
    pub /: *mut *mut int id; / Feature Unit ID,
    pub /: *mut *mut bool mute_present; / mute control enable,
    pub /: *mut *mut bool volume_present; / volume control enable,
    pub /: *mut *mut s16 volume_min; / min volume in 1/256 dB,
    pub /: *mut *mut s16 volume_max; / max volume in 1/256 dB,
    pub /: *mut *mut s16 volume_res; / volume resolution in 1/256 dB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uac_params {
// playback
    pub /: *mut *mut int p_chmask; / channel mask,
    pub /: *mut *mut int p_srates[UAC_MAX_RATES]; / available rates in Hz (0 terminated list),
    pub /: *mut *mut int p_ssize; / sample size,
    pub /: *mut *mut uac_fu_params p_fu; / Feature Unit parameters,
// capture
    pub /: *mut *mut int c_chmask; / channel mask,
    pub /: *mut *mut int c_srates[UAC_MAX_RATES]; / available rates in Hz (0 terminated list),
    pub /: *mut *mut int c_ssize; / sample size,
    pub /: *mut *mut uac_fu_params c_fu; / Feature Unit parameters,
// rates are dynamic, in uac_rtd_params
    pub /: *mut *mut int req_number; / number of preallocated requests,
    pub /: *mut *mut int fb_max; / upper frequency drift feedback limit per-mil,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g_audio {
    pub func: usb_function,
    pub gadget: *mut usb_gadget,
    pub in_ep: *mut usb_ep,
    pub out_ep: *mut usb_ep,
// feedback IN endpoint corresponding to out_ep
    pub in_ep_fback: *mut usb_ep,
// Max packet size for all in_ep possible speeds
    pub in_ep_maxpsize: c_uint,
// Max packet size for all out_ep possible speeds
    pub out_ep_maxpsize: c_uint,
// Notify UAC driver about control change
    pub cs): *mut *mut *mut int (notify)(struct g_audio g_audio, int unit_id, int,
// The ALSA Sound Card it represents on the USB-Client side
    pub uac: *mut snd_uac_chip,
    pub params: uac_params,
}

extern "C" {
    pub fn container_of(_arg: f, g_audio: struct, _arg: func) -> return;
}
//
// g_audio_setup - initialize one virtual ALSA sound card
// @g_audio: struct with filled params, in_ep_maxpsize, out_ep_maxpsize
// @pcm_name: the id string for a PCM instance of this sound card
// @card_name: name of this soundcard
//
// This sets up the single virtual ALSA sound card that may be exported by a
// gadget driver using this framework.
//
// Context: may sleep
//
// Returns zero on success, or a negative error on failure.
//
extern "C" {
    pub fn g_audio_cleanup(g_audio: *mut g_audio);
}
extern "C" {
    pub fn u_audio_start_capture(g_audio: *mut g_audio) -> c_int;
}
extern "C" {
    pub fn u_audio_stop_capture(g_audio: *mut g_audio);
}
extern "C" {
    pub fn u_audio_start_playback(g_audio: *mut g_audio) -> c_int;
}
extern "C" {
    pub fn u_audio_stop_playback(g_audio: *mut g_audio);
}
extern "C" {
    pub fn u_audio_get_capture_srate(audio_dev: *mut g_audio, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn u_audio_set_capture_srate(audio_dev: *mut g_audio, srate: c_int) -> c_int;
}
extern "C" {
    pub fn u_audio_get_playback_srate(audio_dev: *mut g_audio, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn u_audio_set_playback_srate(audio_dev: *mut g_audio, srate: c_int) -> c_int;
}
extern "C" {
    pub fn u_audio_get_volume(g_audio: *mut g_audio, playback: c_int, val: *mut i16) -> c_int;
}
extern "C" {
    pub fn u_audio_set_volume(g_audio: *mut g_audio, playback: c_int, val: i16) -> c_int;
}
extern "C" {
    pub fn u_audio_get_mute(g_audio: *mut g_audio, playback: c_int, val: *mut c_int) -> c_int;
}
extern "C" {
    pub fn u_audio_set_mute(g_audio: *mut g_audio, playback: c_int, val: c_int) -> c_int;
}
extern "C" {
    pub fn u_audio_suspend(g_audio: *mut g_audio);
}
