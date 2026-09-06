//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/motu/motu.h
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
// motu.h - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

// Macro flag: #define SOUND_FIREWIRE_MOTU_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_motu_packet_format {
    pub midi_flag_offset: c_uchar,
    pub midi_byte_offset: c_uchar,
    pub pcm_byte_offset: c_uchar,
    pub msg_chunks: c_uchar,
    pub pcm_chunks: [c_uchar; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdtp_motu_cache {
    pub event_offsets: *mut c_uint,
    pub size: c_uint,
    pub tail: c_uint,
    pub tx_cycle_count: c_uint,
    pub head: c_uint,
    pub rx_cycle_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_motu {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,
// Model dependent information.
    pub spec: *const snd_motu_spec,
// For packet streaming
    pub tx_packet_formats: snd_motu_packet_format,
    pub rx_packet_formats: snd_motu_packet_format,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,
    pub substreams_counter: c_uint,
// For notification.
    pub async_handler: fw_address_handler,
    pub msg: u32,
// For uapi
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
    pub hwdep: *mut snd_hwdep,
    pub domain: amdtp_domain,
    pub cache: amdtp_motu_cache,
    pub message_parser: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_motu_spec_flags {
    SND_MOTU_SPEC_RX_MIDI_2ND_Q	= 0x0001,
    SND_MOTU_SPEC_RX_MIDI_3RD_Q	= 0x0002,
    SND_MOTU_SPEC_TX_MIDI_2ND_Q	= 0x0004,
    SND_MOTU_SPEC_TX_MIDI_3RD_Q	= 0x0008,
    SND_MOTU_SPEC_REGISTER_DSP	= 0x0010,
    SND_MOTU_SPEC_COMMAND_DSP	= 0x0020,
}

pub const SND_MOTU_CLOCK_RATE_COUNT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_motu_clock_source {
    SND_MOTU_CLOCK_SOURCE_INTERNAL,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_A,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_B,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_A,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_B,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX,
    SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR,
    SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC,
    SND_MOTU_CLOCK_SOURCE_SPH,
    SND_MOTU_CLOCK_SOURCE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_motu_protocol_version {
    SND_MOTU_PROTOCOL_V1,
    SND_MOTU_PROTOCOL_V2,
    SND_MOTU_PROTOCOL_V3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_motu_spec {
    pub name: *const *const c_char,
    pub protocol_version: snd_motu_protocol_version,
// The combination of snd_motu_spec_flags enumeration-constants.
    pub flags: c_uint,
    pub tx_fixed_pcm_chunks: [c_uchar; 3],
    pub rx_fixed_pcm_chunks: [c_uchar; 3],
}

extern "C" {
    pub fn snd_motu_transaction_register(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_transaction_reregister(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_transaction_unregister(motu: *mut snd_motu);
}
extern "C" {
    pub fn snd_motu_stream_init_duplex(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_stream_destroy_duplex(motu: *mut snd_motu);
}
extern "C" {
    pub fn snd_motu_stream_cache_packet_formats(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_stream_start_duplex(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_stream_stop_duplex(motu: *mut snd_motu);
}
extern "C" {
    pub fn snd_motu_stream_lock_try(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_stream_lock_release(motu: *mut snd_motu);
}
extern "C" {
    pub fn snd_motu_proc_init(motu: *mut snd_motu);
}
extern "C" {
    pub fn snd_motu_create_pcm_devices(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_create_midi_devices(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_create_hwdep_device(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_protocol_v1_cache_packet_formats(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_protocol_v2_cache_packet_formats(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_protocol_v3_cache_packet_formats(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_protocol_v2_get_clock_rate(_arg: motu, _arg: rate) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v3_get_clock_rate(_arg: motu, _arg: rate) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v1_get_clock_rate(_arg: motu, _arg: rate) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v2_set_clock_rate(_arg: motu, _arg: rate) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v3_set_clock_rate(_arg: motu, _arg: rate) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v1_set_clock_rate(_arg: motu, _arg: rate) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v2_get_clock_source(_arg: motu, _arg: source) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v3_get_clock_source(_arg: motu, _arg: source) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v1_get_clock_source(_arg: motu, _arg: source) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v2_switch_fetching_mode(_arg: motu, _arg: enable) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v3_switch_fetching_mode(_arg: motu, _arg: enable) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v1_switch_fetching_mode(_arg: motu, _arg: enable) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v2_cache_packet_formats(_arg: motu) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v3_cache_packet_formats(_arg: motu) -> return;
}
extern "C" {
    pub fn snd_motu_protocol_v1_cache_packet_formats(_arg: motu) -> return;
}
extern "C" {
    pub fn snd_motu_register_dsp_message_parser_new(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_register_dsp_message_parser_init(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_register_dsp_message_parser_count_event(motu: *mut snd_motu) -> c_uint;
}
extern "C" {
    pub fn snd_motu_register_dsp_message_parser_copy_event(motu: *mut snd_motu, event: *mut u32) -> bool;
}
extern "C" {
    pub fn snd_motu_command_dsp_message_parser_new(motu: *mut snd_motu) -> c_int;
}
extern "C" {
    pub fn snd_motu_command_dsp_message_parser_init(motu: *mut snd_motu, sfc: cip_sfc) -> c_int;
}
