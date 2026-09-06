//! Automatically rewritten from C Header to Rust Module
//! Source: sound/firewire/oxfw/oxfw.h
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
// oxfw.h - a part of driver for OXFW970/971 based devices
//
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_oxfw_quirk {
// Postpone transferring packets during handling asynchronous transaction. As a result,
// next isochronous packet includes more events than one packet can include.
    SND_OXFW_QUIRK_JUMBO_PAYLOAD = 0x01,
// The dbs field of CIP header in tx packet is wrong.
    SND_OXFW_QUIRK_WRONG_DBS = 0x02,
// Blocking transmission mode is used.
    SND_OXFW_QUIRK_BLOCKING_TRANSMISSION = 0x04,
// Stanton SCS1.d and SCS1.m support unique transaction.
    SND_OXFW_QUIRK_SCS_TRANSACTION = 0x08,
// Apogee Duet FireWire ignores data blocks in packet with NO_INFO for audio data
// processing, while output level meter moves. Any value in syt field of packet takes
// the device to process audio data even if the value is invalid in a point of
// IEC 61883-1/6.
    SND_OXFW_QUIRK_IGNORE_NO_INFO_PACKET = 0x10,
// Loud Technologies Mackie Onyx 1640i seems to configure OXFW971 ASIC so that it decides
// event frequency according to events in received isochronous packets. The device looks to
// performs media clock recovery voluntarily. In the recovery, the packets with NO_INFO
// are ignored, thus driver should transfer packets with timestamp.
    SND_OXFW_QUIRK_VOLUNTARY_RECOVERY = 0x20,
// Miglia Harmony Audio does not support AV/C Stream Format Information command.
    SND_OXFW_QUIRK_STREAM_FORMAT_INFO_UNSUPPORTED = 0x40,
// Miglia Harmony Audio transmits CIP in which the value of dbc field expresses the number
// of accumulated payload quadlets including the packet.
    SND_OXFW_QUIRK_DBC_IS_TOTAL_PAYLOAD_QUADLETS = 0x80,
}

// This is an arbitrary number for convinience.
pub const SND_OXFW_STREAM_FORMAT_ENTRIES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_oxfw {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,
// The combination of snd_oxfw_quirk enumeration-constants.
    pub quirks: c_uint,
    pub has_output: bool,
    pub has_input: bool,
    pub tx_stream_formats: [*mut u8; SND_OXFW_STREAM_FORMAT_ENTRIES],
    pub rx_stream_formats: [*mut u8; SND_OXFW_STREAM_FORMAT_ENTRIES],
    pub assumed: bool,
    pub out_conn: cmp_connection,
    pub in_conn: cmp_connection,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_count: c_uint,
    pub midi_input_ports: c_uint,
    pub midi_output_ports: c_uint,
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
    pub spec: *mut c_void,
    pub domain: amdtp_domain,
}

//
// AV/C Stream Format Information Specification 1.1 Working Draft
// (Apr 2005, 1394TA)
//
extern "C" {
    pub fn avc_stream_get_format(_arg: unit, _arg: dir, _arg: pid, _arg: buf, _arg: len, _arg: 0xff) -> return;
}
extern "C" {
    pub fn avc_stream_get_format(_arg: unit, _arg: dir, _arg: pid, _arg: buf, _arg: len, _arg: eid) -> return;
}
//
// AV/C Digital Interface Command Set General Specification 4.2
// (Sep 2004, 1394TA)
//
extern "C" {
    pub fn snd_oxfw_stream_init_duplex(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_stream_start_duplex(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_stream_stop_duplex(oxfw: *mut snd_oxfw);
}
extern "C" {
    pub fn snd_oxfw_stream_destroy_duplex(oxfw: *mut snd_oxfw);
}
extern "C" {
    pub fn snd_oxfw_stream_update_duplex(oxfw: *mut snd_oxfw);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_oxfw_stream_formation {
    pub rate: c_uint,
    pub pcm: c_uint,
    pub midi: c_uint,
}

extern "C" {
    pub fn snd_oxfw_stream_discover(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_stream_lock_changed(oxfw: *mut snd_oxfw);
}
extern "C" {
    pub fn snd_oxfw_stream_lock_try(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_stream_lock_release(oxfw: *mut snd_oxfw);
}
extern "C" {
    pub fn snd_oxfw_create_pcm(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_proc_init(oxfw: *mut snd_oxfw);
}
extern "C" {
    pub fn snd_oxfw_create_midi(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_create_hwdep(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_add_spkr(oxfw: *mut snd_oxfw, is_lacie: bool) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_scs1x_add(oxfw: *mut snd_oxfw) -> c_int;
}
extern "C" {
    pub fn snd_oxfw_scs1x_update(oxfw: *mut snd_oxfw);
}
