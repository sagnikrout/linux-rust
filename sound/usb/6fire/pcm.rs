//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/6fire/pcm.h
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
// Linux driver for TerraTec DMX 6Fire USB
//
// Author:	Torsten Schenk <torsten.schenk@zoho.com>
// Created:	Jan 01, 2011
// Copyright:	(C) Torsten Schenk
//

// maximum of EP_W_MAX_PACKET_SIZE[] (see firmware.c)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_urb {
    pub chip: *mut sfire_chip,
// BEGIN DO NOT SEPARATE
    pub instance: urb,
    pub packets: [usb_iso_packet_descriptor; PCM_N_PACKETS_PER_URB],
// END DO NOT SEPARATE
    pub buffer: *mut u8,
    pub peer: *mut pcm_urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_substream {
    pub lock: spinlock_t,
    pub instance: *mut snd_pcm_substream,
    pub active: bool,
    pub /: *mut *mut snd_pcm_uframes_t dma_off; / current position in alsa dma_area,
    pub /: *mut *mut snd_pcm_uframes_t period_off; / current position in current period,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_runtime {
    pub chip: *mut sfire_chip,
    pub instance: *mut snd_pcm,
    pub playback: pcm_substream,
    pub capture: pcm_substream,
    pub /: *mut *mut bool panic; / if set driver won't do anymore pcm on device,
    pub in_urbs: [pcm_urb; PCM_N_URBS],
    pub out_urbs: [pcm_urb; PCM_N_URBS],
    pub in_packet_size: c_int,
    pub out_packet_size: c_int,
    pub /: *mut *mut int in_n_analog; / number of analog channels soundcard sends,
    pub /: *mut *mut int out_n_analog; / number of analog channels soundcard receives,
    pub stream_mutex: mutex,
    pub /: *mut *mut u8 stream_state; / one of STREAM_XXX (pcm.c),
    pub /: *mut *mut u8 rate; / one of PCM_RATE_XXX,
    pub stream_wait_queue: wait_queue_head_t,
    pub stream_wait_cond: bool,
}

extern "C" {
    pub fn usb6fire_pcm_init(chip: *mut sfire_chip) -> c_int;
}
extern "C" {
    pub fn usb6fire_pcm_abort(chip: *mut sfire_chip);
}
extern "C" {
    pub fn usb6fire_pcm_destroy(chip: *mut sfire_chip);
}
