//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/digi00x/digi00x.h
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
// digi00x.h - a part of driver for Digidesign Digi 002/003 family
//
// Copyright (c) 2014-2015 Takashi Sakamoto
//

// Macro flag: #define SOUND_DIGI00X_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dg00x {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub tx_stream: amdtp_stream,
    pub tx_resources: fw_iso_resources,
    pub rx_stream: amdtp_stream,
    pub rx_resources: fw_iso_resources,
    pub substreams_counter: c_uint,
// for uapi
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
// For asynchronous messages.
    pub async_handler: fw_address_handler,
    pub msg: u32,
// Console models have additional MIDI ports for control surface.
    pub is_console: bool,
    pub domain: amdtp_domain,
}

pub const DG00X_ADDR_BASE: c_uint = 0xffffe0000000ull;
pub const DG00X_OFFSET_STREAMING_STATE: c_uint = 0x0000;
pub const DG00X_OFFSET_STREAMING_SET: c_uint = 0x0004;
// unknown but address in host space	0x0008
// For LSB of the address		0x000c
// unknown				0x0010
pub const DG00X_OFFSET_MESSAGE_ADDR: c_uint = 0x0014;
// For LSB of the address		0x0018
// unknown				0x001c
// unknown				0x0020
// not used			0x0024--0x00ff
pub const DG00X_OFFSET_ISOC_CHANNELS: c_uint = 0x0100;
// unknown				0x0104
// unknown				0x0108
// unknown				0x010c
pub const DG00X_OFFSET_LOCAL_RATE: c_uint = 0x0110;
pub const DG00X_OFFSET_EXTERNAL_RATE: c_uint = 0x0114;
pub const DG00X_OFFSET_CLOCK_SOURCE: c_uint = 0x0118;
pub const DG00X_OFFSET_OPT_IFACE_MODE: c_uint = 0x011c;
// unknown				0x0120
// Mixer control on/off			0x0124
// unknown				0x0128
pub const DG00X_OFFSET_DETECT_EXTERNAL: c_uint = 0x012c;
// unknown				0x0138
pub const DG00X_OFFSET_MMC: c_uint = 0x0400;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dg00x_rate {
    SND_DG00X_RATE_44100 = 0,
    SND_DG00X_RATE_48000,
    SND_DG00X_RATE_88200,
    SND_DG00X_RATE_96000,
    SND_DG00X_RATE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dg00x_clock {
    SND_DG00X_CLOCK_INTERNAL = 0,
    SND_DG00X_CLOCK_SPDIF,
    SND_DG00X_CLOCK_ADAT,
    SND_DG00X_CLOCK_WORD,
    SND_DG00X_CLOCK_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dg00x_optical_mode {
    SND_DG00X_OPT_IFACE_MODE_ADAT = 0,
    SND_DG00X_OPT_IFACE_MODE_SPDIF,
    SND_DG00X_OPT_IFACE_MODE_COUNT,
}

pub const DOT_MIDI_IN_PORTS: c_int = 1;
pub const DOT_MIDI_OUT_PORTS: c_int = 2;
extern "C" {
    pub fn amdtp_dot_reset(s: *mut amdtp_stream);
}
extern "C" {
    pub fn snd_dg00x_transaction_register(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_transaction_reregister(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_transaction_unregister(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_stream_set_local_rate(dg00x: *mut snd_dg00x, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_stream_init_duplex(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_stream_start_duplex(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_stream_stop_duplex(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_stream_update_duplex(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_stream_destroy_duplex(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_stream_lock_changed(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_stream_lock_try(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_stream_lock_release(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_proc_init(dg00x: *mut snd_dg00x);
}
extern "C" {
    pub fn snd_dg00x_create_pcm_devices(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_create_midi_devices(dg00x: *mut snd_dg00x) -> c_int;
}
extern "C" {
    pub fn snd_dg00x_create_hwdep_device(dg00x: *mut snd_dg00x) -> c_int;
}
