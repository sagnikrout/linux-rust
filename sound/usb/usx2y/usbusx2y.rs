//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/usbusx2y.h
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


// SPDX-License-Identifier: GPL-2.0

pub const NRURBS: c_int = 2;
// Default value used for nr of packs per urb.
// 1 to 4 have been tested ok on uhci.
// To use 3 on ohci, you'd need a patch:
// look for "0000425-linux-2.6.9-rc4-mm1_ohci-hcd.patch.gz" on
// "https://bugtrack.alsa-project.org/alsa-bug/bug_view_page.php?bug_id=0000425"
//
// 1, 2 and 4 work out of the box on ohci, if I recall correctly.
// Bigger is safer operation, smaller gives lower latencies.
//
pub const USX2Y_NRPACKS: c_int = 4;
pub const USX2Y_NRPACKS_MAX: c_int = 1024;
// If your system works ok with this module's parameter
// nrpacks set to 1, you might as well comment
// this define out, and thereby produce smaller, faster code.
// You'd also set USX2Y_NRPACKS to 1 then.
//
pub const USX2Y_NRPACKS_VARIABLE: c_int = 1;

pub const URBS_ASYNC_SEQ: c_int = 10;
pub const URB_DATA_LEN_ASYNC_SEQ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usx2y_async_seq {
    pub urb: [*mut urb; URBS_ASYNC_SEQ],
    pub buffer: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usx2y_urb_seq {
    pub submitted: c_int,
    pub len: c_int,
    pub __counted_by(len): *mut *mut urb urb[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usx2ydev {
    pub dev: *mut usb_device,
    pub card_index: c_int,
    pub stride: c_int,
    pub in04_urb: *mut urb,
    pub in04_buf: *mut c_void,
    pub in04_last: [c_char; 24],
    pub in04_int_calls: c_uint,
    pub us04: *mut snd_usx2y_urb_seq,
    pub in04_wait_queue: wait_queue_head_t,
    pub as04: snd_usx2y_async_seq,
    pub chip_status: c_int,
    pub pcm_mutex: mutex,
    pub us428ctls_sharedmem: *mut us428ctls_sharedmem,
    pub wait_iso_frame: c_int,
    pub us428ctls_wait_queue_head: wait_queue_head_t,
    pub hwdep_pcm_shm: *mut snd_usx2y_hwdep_pcm_shm,
    pub subs: [*mut snd_usx2y_substream; 4],
    pub prepare_subs: *mut *mut snd_usx2y_substream  volatile,
    pub prepare_wait_queue: wait_queue_head_t,
    pub midi_list: list_head,
    pub pcm_devs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usx2y_substream {
    pub usx2y: *mut usx2ydev,
    pub pcm_substream: *mut snd_pcm_substream,
    pub endpoint: c_int,
    pub /: *mut *mut unsigned int maxpacksize; / max packet size in bytes,
    pub state: core::sync::atomic::AtomicI32,
pub const STATE_STOPPED: c_int = 0;
pub const STATE_STARTING1: c_int = 1;
pub const STATE_STARTING2: c_int = 2;
pub const STATE_STARTING3: c_int = 3;
pub const STATE_PREPARED: c_int = 4;
pub const STATE_PRERUNNING: c_int = 6;
pub const STATE_RUNNING: c_int = 8;
    pub /: *mut *mut int hwptr; / free frame position in the buffer (only for playback),
    pub /: *mut *mut int hwptr_done; / processed frame position in the buffer,
    pub /: *mut *mut int transfer_done; / processed frames since last period update,
    pub /: *mut *mut *mut urb urb[NRURBS]; / data urb table,
    pub completed_urb: *mut urb,
    pub /: *mut *mut *mut char tmpbuf; / temporary buffer for playback,
}

extern "C" {
    pub fn usx2y_audio_create(card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn usx2y_async_seq04_init(usx2y: *mut usx2ydev) -> c_int;
}
extern "C" {
    pub fn usx2y_in04_init(usx2y: *mut usx2ydev) -> c_int;
}

