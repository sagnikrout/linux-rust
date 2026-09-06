//! Automatically rewritten from C Header to Rust Module
//! Source: sound/virtio/virtio_pcm.h
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
// virtio-snd: Virtio sound device
// Copyright (C) 2021 OpenSynergy GmbH
//

//
// struct virtio_pcm_substream - VirtIO PCM substream.
// @snd: VirtIO sound device.
// @nid: Function group node identifier.
// @sid: Stream identifier.
// @direction: Stream data flow direction (SNDRV_PCM_STREAM_XXX).
// @features: Stream VirtIO feature bit map (1 << VIRTIO_SND_PCM_F_XXX).
// @substream: Kernel ALSA substream.
// @pcm_indirect: Kernel indirect pcm structure.
// @hw: Kernel ALSA substream hardware descriptor.
// @elapsed_period: Kernel work to handle the elapsed period state.
// @lock: Spinlock that protects fields shared by interrupt handlers and
// substream operators.
// @buffer_bytes: Current buffer size in bytes.
// @hw_ptr: Substream hardware pointer value in bytes [0 ... buffer_bytes).
// @xfer_enabled: Data transfer state (0 - off, 1 - on).
// @xfer_xrun: Data underflow/overflow state (0 - no xrun, 1 - xrun).
// @stopped: True if the substream is stopped and must be released on the device
// side.
// @suspended: True if the substream is suspended and must be reconfigured on
// the device side at resume.
// @msgs: Allocated I/O messages.
// @nmsgs: Number of allocated I/O messages.
// @msg_count: Number of pending I/O messages in the virtqueue.
// @msg_empty: Notify when msg_count is zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pcm_substream {
    pub snd: *mut virtio_snd,
    pub nid: u32,
    pub sid: u32,
    pub direction: u32,
    pub features: u32,
    pub substream: *mut snd_pcm_substream,
    pub pcm_indirect: snd_pcm_indirect,
    pub hw: snd_pcm_hardware,
    pub elapsed_period: work_struct,
    pub lock: spinlock_t,
    pub buffer_bytes: usize,
    pub hw_ptr: usize,
    pub xfer_enabled: bool,
    pub xfer_xrun: bool,
    pub stopped: bool,
    pub suspended: bool,
    pub msgs: *mut virtio_pcm_msg,
    pub nmsgs: c_uint,
    pub msg_count: c_uint,
    pub msg_empty: wait_queue_head_t,
}

//
// struct virtio_pcm_stream - VirtIO PCM stream.
// @substreams: VirtIO substreams belonging to the stream.
// @nsubstreams: Number of substreams.
// @chmaps: Kernel channel maps belonging to the stream.
// @nchmaps: Number of channel maps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pcm_stream {
    pub substreams: *mut virtio_pcm_substream,
    pub nsubstreams: u32,
    pub chmaps: *mut snd_pcm_chmap_elem,
    pub nchmaps: u32,
}

//
// struct virtio_pcm - VirtIO PCM device.
// @list: VirtIO PCM list entry.
// @nid: Function group node identifier.
// @pcm: Kernel PCM device.
// @streams: VirtIO PCM streams (playback and capture).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pcm {
    pub list: list_head,
    pub nid: u32,
    pub pcm: *mut snd_pcm,
    pub 1]: virtio_pcm_stream streams[SNDRV_PCM_STREAM_LAST +,
}

extern "C" {
    pub fn virtsnd_pcm_validate(vdev: *mut virtio_device) -> c_int;
}
extern "C" {
    pub fn virtsnd_pcm_parse_cfg(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_pcm_build_devs(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_pcm_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);
}
extern "C" {
    pub fn virtsnd_pcm_tx_notify_cb(vqueue: *mut virtqueue);
}
extern "C" {
    pub fn virtsnd_pcm_rx_notify_cb(vqueue: *mut virtqueue);
}
extern "C" {
    pub fn virtsnd_pcm_msg_free(vss: *mut virtio_pcm_substream);
}
extern "C" {
    pub fn virtsnd_pcm_msg_pending_num(vss: *mut virtio_pcm_substream) -> c_uint;
}
