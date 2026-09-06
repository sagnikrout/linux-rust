//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/bebob/bebob.h
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
// bebob.h - a part of driver for BeBoB based devices
//
// Copyright (c) 2013-2014 Takashi Sakamoto
//

// Macro flag: #define SOUND_BEBOB_H_INCLUDED

// basic register addresses on DM1000/DM1100/DM1500
pub const BEBOB_ADDR_REG_INFO: c_uint = 0xffffc8020000ULL;
pub const BEBOB_ADDR_REG_REQ: c_uint = 0xffffc8021000ULL;
pub const SND_BEBOB_STRM_FMT_ENTRIES: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_bebob_stream_formation {
    pub pcm: c_uint,
    pub midi: c_uint,
}

// this is a lookup table for index of stream formations
// device specific operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_bebob_clock_type {
    SND_BEBOB_CLOCK_TYPE_INTERNAL = 0,
    SND_BEBOB_CLOCK_TYPE_EXTERNAL,
    SND_BEBOB_CLOCK_TYPE_SYT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_bebob_clock_spec {
    pub num: c_uint,
    pub labels: *const *const c_char,
    pub types: *const snd_bebob_clock_type,
    pub id): *mut *mut *mut int (get)(struct snd_bebob bebob, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_bebob_rate_spec {
    pub rate): *mut *mut *mut int (get)(struct snd_bebob bebob, unsigned int,
    pub rate): *mut *mut *mut int (set)(struct snd_bebob bebob, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_bebob_meter_spec {
    pub num: c_uint,
    pub labels: *const *const c_char,
    pub size): *mut *mut *mut *mut int (get)(struct snd_bebob bebob, u32 target, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_bebob_spec {
    pub clock: *const snd_bebob_clock_spec,
    pub rate: *const snd_bebob_rate_spec,
    pub meter: *const snd_bebob_meter_spec,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_bebob_quirk {
    SND_BEBOB_QUIRK_INITIAL_DISCONTINUOUS_DBC = (1 << 0),
    SND_BEBOB_QUIRK_WRONG_DBC		  = (1 << 1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_bebob {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub card_index: c_int,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub spec: *const snd_bebob_spec,
    pub enumerations.: unsigned int quirks; // Combination of snd_bebob_quirk,
    pub midi_input_ports: c_uint,
    pub midi_output_ports: c_uint,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub out_conn: cmp_connection,
    pub in_conn: cmp_connection,
    pub substreams_counter: c_uint,
    pub sync_input_plug: c_int,
// for uapi
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
// for M-Audio special devices
    pub maudio_special_quirk: *mut c_void,
    pub domain: amdtp_domain,
}

// AV/C Audio Subunit Specification 1.0 (Oct 2000, 1394TA)
//
// AVC command extensions, AV/C Unit and Subunit, Revision 17
// (Nov 2003, BridgeCo)
//
pub const AVC_BRIDGECO_ADDR_BYTES: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avc_bridgeco_plug_dir {
    AVC_BRIDGECO_PLUG_DIR_IN	= 0x00,
    AVC_BRIDGECO_PLUG_DIR_OUT	= 0x01
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avc_bridgeco_plug_mode {
    AVC_BRIDGECO_PLUG_MODE_UNIT		= 0x00,
    AVC_BRIDGECO_PLUG_MODE_SUBUNIT		= 0x01,
    AVC_BRIDGECO_PLUG_MODE_FUNCTION_BLOCK	= 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avc_bridgeco_plug_unit {
    AVC_BRIDGECO_PLUG_UNIT_ISOC	= 0x00,
    AVC_BRIDGECO_PLUG_UNIT_EXT	= 0x01,
    AVC_BRIDGECO_PLUG_UNIT_ASYNC	= 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum avc_bridgeco_plug_type {
    AVC_BRIDGECO_PLUG_TYPE_ISOC	= 0x00,
    AVC_BRIDGECO_PLUG_TYPE_ASYNC	= 0x01,
    AVC_BRIDGECO_PLUG_TYPE_MIDI	= 0x02,
    AVC_BRIDGECO_PLUG_TYPE_SYNC	= 0x03,
    AVC_BRIDGECO_PLUG_TYPE_ANA	= 0x04,
    AVC_BRIDGECO_PLUG_TYPE_DIG	= 0x05,
    AVC_BRIDGECO_PLUG_TYPE_ADDITION	= 0x06
}

// for AMDTP streaming
extern "C" {
    pub fn snd_bebob_stream_get_rate(bebob: *mut snd_bebob, rate: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn snd_bebob_stream_set_rate(bebob: *mut snd_bebob, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_bebob_stream_discover(bebob: *mut snd_bebob) -> c_int;
}
extern "C" {
    pub fn snd_bebob_stream_init_duplex(bebob: *mut snd_bebob) -> c_int;
}
extern "C" {
    pub fn snd_bebob_stream_start_duplex(bebob: *mut snd_bebob) -> c_int;
}
extern "C" {
    pub fn snd_bebob_stream_stop_duplex(bebob: *mut snd_bebob);
}
extern "C" {
    pub fn snd_bebob_stream_destroy_duplex(bebob: *mut snd_bebob);
}
extern "C" {
    pub fn snd_bebob_stream_lock_changed(bebob: *mut snd_bebob);
}
extern "C" {
    pub fn snd_bebob_stream_lock_try(bebob: *mut snd_bebob) -> c_int;
}
extern "C" {
    pub fn snd_bebob_stream_lock_release(bebob: *mut snd_bebob);
}
extern "C" {
    pub fn snd_bebob_proc_init(bebob: *mut snd_bebob);
}
extern "C" {
    pub fn snd_bebob_create_midi_devices(bebob: *mut snd_bebob) -> c_int;
}
extern "C" {
    pub fn snd_bebob_create_pcm_devices(bebob: *mut snd_bebob) -> c_int;
}
extern "C" {
    pub fn snd_bebob_create_hwdep_device(bebob: *mut snd_bebob) -> c_int;
}
// model specific operations
extern "C" {
    pub fn snd_bebob_maudio_special_discover(bebob: *mut snd_bebob, is1814: bool) -> c_int;
}
extern "C" {
    pub fn snd_bebob_maudio_load_firmware(unit: *mut fw_unit) -> c_int;
}
