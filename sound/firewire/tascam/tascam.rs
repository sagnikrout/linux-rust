//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/tascam/tascam.h
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
// tascam.h - a part of driver for TASCAM FireWire series
//
// Copyright (c) 2015 Takashi Sakamoto
//

// Macro flag: #define SOUND_TASCAM_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_tscm_spec {
    pub name: *const *const c_char,
    pub has_adat: bool,
    pub has_spdif: bool,
    pub pcm_capture_analog_channels: c_uint,
    pub pcm_playback_analog_channels: c_uint,
    pub midi_capture_ports: c_uint,
    pub midi_playback_ports: c_uint,
}

pub const TSCM_MIDI_IN_PORT_MAX: c_int = 4;
pub const TSCM_MIDI_OUT_PORT_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_fw_async_midi_port {
    pub parent: *mut fw_device,
    pub work: work_struct,
    pub idling: bool,
    pub next_ktime: ktime_t,
    pub error: bool,
    pub transaction: fw_transaction,
    pub buf: [u8; 4],
    pub running_status: u8,
    pub on_sysex: bool,
    pub substream: *mut snd_rawmidi_substream,
    pub consume_bytes: c_int,
}

pub const SND_TSCM_QUEUE_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_tscm {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub spec: *const snd_tscm_spec,
    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_counter: c_uint,
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
// For MIDI message incoming transactions.
    pub async_handler: fw_address_handler,
    pub tx_midi_substreams: [*mut snd_rawmidi_substream; TSCM_MIDI_IN_PORT_MAX],
// For MIDI message outgoing transactions.
    pub out_ports: [snd_fw_async_midi_port; TSCM_MIDI_OUT_PORT_MAX],
// A cache of status information in tx isoc packets.
    pub state: [__be32; SNDRV_FIREWIRE_TASCAM_STATE_COUNT],
    pub hwdep: *mut snd_hwdep,
    pub queue: [snd_firewire_tascam_change; SND_TSCM_QUEUE_COUNT],
    pub pull_pos: c_uint,
    pub push_pos: c_uint,
    pub domain: amdtp_domain,
    pub need_long_tx_init_skip: bool,
}

pub const TSCM_ADDR_BASE: c_uint = 0xffff00000000ull;
pub const TSCM_OFFSET_FIRMWARE_REGISTER: c_uint = 0x0000;
pub const TSCM_OFFSET_FIRMWARE_FPGA: c_uint = 0x0004;
pub const TSCM_OFFSET_FIRMWARE_ARM: c_uint = 0x0008;
pub const TSCM_OFFSET_FIRMWARE_HW: c_uint = 0x000c;
pub const TSCM_OFFSET_ISOC_TX_CH: c_uint = 0x0200;
pub const TSCM_OFFSET_UNKNOWN: c_uint = 0x0204;
pub const TSCM_OFFSET_START_STREAMING: c_uint = 0x0208;
pub const TSCM_OFFSET_ISOC_RX_CH: c_uint = 0x020c;
pub const TSCM_OFFSET_ISOC_RX_ON: c_uint = 0x0210	/* Little conviction. */;
pub const TSCM_OFFSET_TX_PCM_CHANNELS: c_uint = 0x0214;
pub const TSCM_OFFSET_RX_PCM_CHANNELS: c_uint = 0x0218;
pub const TSCM_OFFSET_MULTIPLEX_MODE: c_uint = 0x021c;
pub const TSCM_OFFSET_ISOC_TX_ON: c_uint = 0x0220;
// Unknown				0x0224
pub const TSCM_OFFSET_CLOCK_STATUS: c_uint = 0x0228;
pub const TSCM_OFFSET_SET_OPTION: c_uint = 0x022c;
pub const TSCM_OFFSET_MIDI_TX_ON: c_uint = 0x0300;
pub const TSCM_OFFSET_MIDI_TX_ADDR_HI: c_uint = 0x0304;
pub const TSCM_OFFSET_MIDI_TX_ADDR_LO: c_uint = 0x0308;
pub const TSCM_OFFSET_LED_POWER: c_uint = 0x0404;
pub const TSCM_OFFSET_MIDI_RX_QUAD: c_uint = 0x4000;
// Although FE-8 supports the above registers, it has no I/O interfaces for
// audio samples and music messages. Otherwise it supports another notification
// for status and control message as well as LED brightening. The message
// consists of quadlet-aligned data up to 32 quadlets. The first byte of message
// is fixed to 0x40. The second byte is between 0x00 to 0x1f and represent each
// control:
// fader:	0x00-0x07
// button:	0x0d, 0x0e
// knob:	0x14-0x1b
// sensing:	0x0b
//
// The rest two bytes represent state of the controls; e.g. current value for
// fader and knob, bitmasks for button and sensing.
// Just after turning on, 32 quadlets messages with 0x00-0x1f are immediately
// sent in one transaction. After, several quadlets are sent in one transaction.
//
// TSCM_OFFSET_FE8_CTL_TX_ON		0x0310
// TSCM_OFFSET_FE8_CTL_TX_ADDR_HI	0x0314
// TSCM_OFFSET_FE8_CTL_TX_ADDR_LO	0x0318
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_tscm_clock {
    SND_TSCM_CLOCK_INTERNAL = 0,
    SND_TSCM_CLOCK_WORD	= 1,
    SND_TSCM_CLOCK_SPDIF	= 2,
    SND_TSCM_CLOCK_ADAT	= 3,
}

extern "C" {
    pub fn amdtp_tscm_set_parameters(s: *mut amdtp_stream, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_tscm_stream_get_rate(tscm: *mut snd_tscm, rate: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn snd_tscm_stream_init_duplex(tscm: *mut snd_tscm) -> c_int;
}
extern "C" {
    pub fn snd_tscm_stream_update_duplex(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_tscm_stream_destroy_duplex(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_tscm_stream_start_duplex(tscm: *mut snd_tscm, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_tscm_stream_stop_duplex(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_tscm_stream_lock_changed(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_tscm_stream_lock_try(tscm: *mut snd_tscm) -> c_int;
}
extern "C" {
    pub fn snd_tscm_stream_lock_release(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_fw_async_midi_port_init(port: *mut snd_fw_async_midi_port);
}
extern "C" {
    pub fn snd_tscm_transaction_register(tscm: *mut snd_tscm) -> c_int;
}
extern "C" {
    pub fn snd_tscm_transaction_reregister(tscm: *mut snd_tscm) -> c_int;
}
extern "C" {
    pub fn snd_tscm_transaction_unregister(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_tscm_proc_init(tscm: *mut snd_tscm);
}
extern "C" {
    pub fn snd_tscm_create_pcm_devices(tscm: *mut snd_tscm) -> c_int;
}
extern "C" {
    pub fn snd_tscm_create_midi_devices(tscm: *mut snd_tscm) -> c_int;
}
extern "C" {
    pub fn snd_tscm_create_hwdep_device(tscm: *mut snd_tscm) -> c_int;
}
