//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/usbtv/usbtv.h
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


//
// Copyright (c) 2013 Lubomir Rintel
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL").
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Fushicai USBTV007 Audio-Video Grabber Driver
//
// No physical hardware was harmed running Windows during the
// reverse-engineering activity
//

// Hardware.
pub const USBTV_VIDEO_ENDP: c_uint = 0x81;
pub const USBTV_AUDIO_ENDP: c_uint = 0x83;
pub const USBTV_BASE: c_uint = 0xc000;
pub const USBTV_CONTROL_REG: c_int = 11;
pub const USBTV_REQUEST_REG: c_int = 12;
// Number of concurrent isochronous urbs submitted.
// Higher numbers was seen to overly saturate the USB bus.
pub const USBTV_ISOC_TRANSFERS: c_int = 16;
pub const USBTV_ISOC_PACKETS: c_int = 8;
pub const USBTV_CHUNK_SIZE: c_int = 256;
pub const USBTV_CHUNK: c_int = 240;
pub const USBTV_AUDIO_URBSIZE: c_int = 20480;
pub const USBTV_AUDIO_HDRSIZE: c_int = 4;
pub const USBTV_AUDIO_BUFFER: c_int = 65536;
// Chunk header.

// parameters for supported TV norms
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtv_norm_params {
    pub norm: v4l2_std_id,
    pub cap_height: int cap_width,,
}

// A single videobuf2 frame buffer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtv_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

// Per-device structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbtv {
    pub dev: *mut device,
    pub udev: *mut usb_device,
// video
    pub v4l2_dev: v4l2_device,
    pub ctrl: v4l2_ctrl_handler,
    pub vdev: video_device,
    pub vb2q: vb2_queue,
    pub v4l2_lock: mutex,
    pub vb2q_lock: mutex,
// List of videobuf2 buffers protected by a lock.
    pub buflock: spinlock_t,
    pub bufs: list_head,
// Number of currently processed frame, useful find
// out when a new one begins.
    pub frame_id: u32,
    pub chunks_done: c_int,
    pub input: },
    pub norm: v4l2_std_id,
    pub height: int width,,
    pub n_chunks: c_int,
    pub iso_size: c_int,
    pub last_odd: c_int,
    pub sequence: c_uint,
    pub isoc_urbs: [*mut urb; USBTV_ISOC_TRANSFERS],
// audio
    pub snd: *mut snd_card,
    pub snd_substream: *mut snd_pcm_substream,
    pub snd_stream: core::sync::atomic::AtomicI32,
    pub snd_trigger: work_struct,
    pub snd_bulk_urb: *mut urb,
    pub snd_buffer_pos: usize,
    pub snd_period_pos: usize,
}

extern "C" {
    pub fn usbtv_set_regs(usbtv: *mut usbtv, regs[][2]: u16, size: c_int) -> c_int;
}
extern "C" {
    pub fn usbtv_video_init(usbtv: *mut usbtv) -> c_int;
}
extern "C" {
    pub fn usbtv_video_free(usbtv: *mut usbtv);
}
extern "C" {
    pub fn usbtv_audio_init(usbtv: *mut usbtv) -> c_int;
}
extern "C" {
    pub fn usbtv_audio_free(usbtv: *mut usbtv);
}
extern "C" {
    pub fn usbtv_audio_suspend(usbtv: *mut usbtv);
}
extern "C" {
    pub fn usbtv_audio_resume(usbtv: *mut usbtv);
}
