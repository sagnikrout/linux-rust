//! Automatically rewritten from C Header to Rust Module
//! Source: sound/virtio/virtio_card.h
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
// struct virtio_snd_queue - Virtqueue wrapper structure.
// @lock: Used to synchronize access to a virtqueue.
// @vqueue: Underlying virtqueue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd_queue {
    pub lock: spinlock_t,
    pub vqueue: *mut virtqueue,
}

//
// struct virtio_kctl - VirtIO control element.
// @kctl: ALSA control element.
// @items: Items for the ENUMERATED element type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_kctl {
    pub kctl: *mut snd_kcontrol,
    pub items: *mut virtio_snd_ctl_enum_item,
}

//
// struct virtio_snd - VirtIO sound card device.
// @vdev: Underlying virtio device.
// @queues: Virtqueue wrappers.
// @card: ALSA sound card.
// @ctl_msgs: Pending control request list.
// @event_msgs: Device events.
// @pcm_list: VirtIO PCM device list.
// @jacks: VirtIO jacks.
// @njacks: Number of jacks.
// @substreams: VirtIO PCM substreams.
// @nsubstreams: Number of PCM substreams.
// @chmaps: VirtIO channel maps.
// @nchmaps: Number of channel maps.
// @kctl_infos: VirtIO control element information.
// @kctls: VirtIO control elements.
// @nkctls: Number of control elements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_snd {
    pub vdev: *mut virtio_device,
    pub queues: [virtio_snd_queue; VIRTIO_SND_VQ_MAX],
    pub card: *mut snd_card,
    pub ctl_msgs: list_head,
    pub event_msgs: *mut virtio_snd_event,
    pub pcm_list: list_head,
    pub jacks: *mut virtio_jack,
    pub njacks: u32,
    pub substreams: *mut virtio_pcm_substream,
    pub nsubstreams: u32,
    pub chmaps: *mut virtio_snd_chmap_info,
    pub nchmaps: u32,
    pub kctl_infos: *mut virtio_snd_ctl_info,
    pub kctls: *mut virtio_kctl,
    pub nkctls: u32,
}

// Message completion timeout in milliseconds (module parameter).
extern "C" {
    pub fn virtsnd_tx_queue(_arg: vss->snd) -> return;
}
extern "C" {
    pub fn virtsnd_rx_queue(_arg: vss->snd) -> return;
}
extern "C" {
    pub fn virtsnd_jack_parse_cfg(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_jack_build_devs(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_chmap_parse_cfg(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_chmap_build_devs(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_kctl_parse_cfg(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_kctl_build_devs(snd: *mut virtio_snd) -> c_int;
}
extern "C" {
    pub fn virtsnd_kctl_event(snd: *mut virtio_snd, event: *mut virtio_snd_event);
}
