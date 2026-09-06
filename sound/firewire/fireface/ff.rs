//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/fireface/ff.h
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
// ff.h - a part of driver for RME Fireface series
//
// Copyright (c) 2015-2017 Takashi Sakamoto
//

// Macro flag: #define SOUND_FIREFACE_H_INCLUDED

pub const SND_FF_MAXIMIM_MIDI_QUADS: c_int = 9;
pub const SND_FF_IN_MIDI_PORTS: c_int = 2;
pub const SND_FF_OUT_MIDI_PORTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ff_unit_version {
    SND_FF_UNIT_VERSION_FF800	= 0x000001,
    SND_FF_UNIT_VERSION_FF400	= 0x000002,
    SND_FF_UNIT_VERSION_UFX		= 0x000003,
    SND_FF_UNIT_VERSION_UCX		= 0x000004,
    SND_FF_UNIT_VERSION_802		= 0x000005,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ff_stream_mode {
    SND_FF_STREAM_MODE_LOW = 0,
    SND_FF_STREAM_MODE_MID,
    SND_FF_STREAM_MODE_HIGH,
    SND_FF_STREAM_MODE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ff_spec {
    pub pcm_capture_channels: [c_uint; SND_FF_STREAM_MODE_COUNT],
    pub pcm_playback_channels: [c_uint; SND_FF_STREAM_MODE_COUNT],
    pub midi_in_ports: c_uint,
    pub midi_out_ports: c_uint,
    pub protocol: *const snd_ff_protocol,
    pub midi_high_addr: u64,
    pub midi_addr_range: u8,
    pub midi_rx_addrs: [u64; SND_FF_OUT_MIDI_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ff {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub unit_version: snd_ff_unit_version,
    pub spec: *const snd_ff_spec,
// To handle MIDI tx.
    pub tx_midi_substreams: [*mut snd_rawmidi_substream; SND_FF_IN_MIDI_PORTS],
    pub async_handler: fw_address_handler,
// TO handle MIDI rx.
    pub rx_midi_substreams: [*mut snd_rawmidi_substream; SND_FF_OUT_MIDI_PORTS],
    pub on_sysex: [bool; SND_FF_OUT_MIDI_PORTS],
    pub msg_buf: [__le32; SND_FF_OUT_MIDI_PORTS][SND_FF_MAXIMIM_MIDI_QUADS],
    pub rx_midi_work: [work_struct; SND_FF_OUT_MIDI_PORTS],
    pub transactions: [fw_transaction; SND_FF_OUT_MIDI_PORTS],
    pub next_ktime: [ktime_t; SND_FF_OUT_MIDI_PORTS],
    pub rx_midi_error: [bool; SND_FF_OUT_MIDI_PORTS],
    pub rx_bytes: [c_uint; SND_FF_OUT_MIDI_PORTS],
    pub substreams_counter: c_uint,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
    pub domain: amdtp_domain,
    pub msg_parser: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ff_clock_src {
    SND_FF_CLOCK_SRC_INTERNAL,
    SND_FF_CLOCK_SRC_SPDIF,
    SND_FF_CLOCK_SRC_ADAT1,
    SND_FF_CLOCK_SRC_ADAT2,
    SND_FF_CLOCK_SRC_WORD,
    SND_FF_CLOCK_SRC_LTC,
// TODO: perhaps TCO exists.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ff_protocol {
    pub msg_parser_size: usize,
    pub ff): *mut *mut bool (has_msg)(struct snd_ff,
    pub count): *mut *mut *mut *mut long (copy_msg_to_user)(struct snd_ff ff, char __user buf, long,
    pub tstamp): size_t length, u32,
    pub port): c_uint,
    pub src): *mut snd_ff_clock_src,
    pub enable): *mut *mut *mut int (switch_fetching_mode)(struct snd_ff ff, bool,
    pub rate): *mut *mut *mut int (allocate_resources)(struct snd_ff ff, unsigned int,
    pub rate): *mut *mut *mut int (begin_session)(struct snd_ff ff, unsigned int,
    pub ff): *mut *mut void (finish_session)(struct snd_ff,
    pub buffer): *mut *mut *mut void (dump_status)(struct snd_ff ff, struct snd_info_buffer,
}

extern "C" {
    pub fn snd_ff_transaction_register(ff: *mut snd_ff) -> c_int;
}
extern "C" {
    pub fn snd_ff_transaction_reregister(ff: *mut snd_ff) -> c_int;
}
extern "C" {
    pub fn snd_ff_transaction_unregister(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_stream_init_duplex(ff: *mut snd_ff) -> c_int;
}
extern "C" {
    pub fn snd_ff_stream_destroy_duplex(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_stream_start_duplex(ff: *mut snd_ff, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_ff_stream_stop_duplex(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_stream_update_duplex(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_stream_lock_changed(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_stream_lock_try(ff: *mut snd_ff) -> c_int;
}
extern "C" {
    pub fn snd_ff_stream_lock_release(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_proc_init(ff: *mut snd_ff);
}
extern "C" {
    pub fn snd_ff_create_midi_devices(ff: *mut snd_ff) -> c_int;
}
extern "C" {
    pub fn snd_ff_create_pcm_devices(ff: *mut snd_ff) -> c_int;
}
extern "C" {
    pub fn snd_ff_create_hwdep_devices(ff: *mut snd_ff) -> c_int;
}
