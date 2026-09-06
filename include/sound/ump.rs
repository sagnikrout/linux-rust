//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ump.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Universal MIDI Packet (UMP) Support
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_group {
    pub /: *mut *mut int group; / group index (0-based),
    pub /: *mut *mut unsigned int dir_bits; / directions,
    pub /: *mut *mut bool active; / activeness,
    pub /: *mut *mut bool valid; / valid group (referred by blocks),
    pub /: *mut *mut bool is_midi1; / belongs to a MIDI1 FB,
    pub /: *mut *mut char name[64]; / group name,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_endpoint {
    pub /: *mut *mut snd_rawmidi core; / raw UMP access,
    pub info: snd_ump_endpoint_info,
    pub /: *const *const *const snd_ump_ops ops; / UMP ops set by the driver,
    pub /: *mut *mut *mut snd_rawmidi_substream substreams[2]; / opened substreams,
    pub private_data: *mut c_void,
    pub ump): *mut *mut void (private_free)(struct snd_ump_endpoint,
// UMP Stream message processing
    pub /: *mut *mut u32 stream_wait_for; / expected stream message status,
    pub /: *mut *mut bool stream_finished; / set when message has been processed,
    pub /: *mut *mut bool parsed; / UMP / FB parse finished?,
    pub /: *mut *mut bool no_process_stream; / suppress UMP stream messages handling,
    pub stream_wait: wait_queue_head_t,
    pub stream_rfile: snd_rawmidi_file,
    pub /: *mut *mut list_head block_list; / list of snd_ump_block objects,
// intermediate buffer for UMP input
    pub input_buf: [u32; 4],
    pub input_buf_head: c_int,
    pub input_pending: c_int,
    pub open_mutex: mutex,
    pub /: *mut *mut snd_ump_group groups[SNDRV_UMP_MAX_GROUPS]; / table of groups,
    pub legacy_locks: [spinlock_t; 2],
    pub legacy_rmidi: *mut snd_rawmidi,
    pub legacy_substreams: [*mut snd_rawmidi_substream; 2][SNDRV_UMP_MAX_GROUPS],
    pub legacy_mapping: [c_uchar; SNDRV_UMP_MAX_GROUPS],
// for legacy output; need to open the actual substream unlike input
    pub legacy_out_opens: c_int,
    pub legacy_out_rfile: snd_rawmidi_file,
    pub out_cvts: *mut ump_cvt_to_ump,

    pub seq_dev: *mut snd_seq_device,
    pub seq_ops: *const snd_seq_ump_ops,
    pub seq_client: *mut c_void,

}

// ops filled by UMP drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_ops {
    pub dir): *mut *mut *mut int (open)(struct snd_ump_endpoint ump, int,
    pub dir): *mut *mut *mut void (close)(struct snd_ump_endpoint ump, int,
    pub up): *mut *mut *mut void (trigger)(struct snd_ump_endpoint ump, int dir, int,
    pub dir): *mut *mut *mut void (drain)(struct snd_ump_endpoint ump, int,
}

// ops filled by sequencer binding
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_ump_ops {
    pub words): *const *const u32 data, int,
    pub ump): *mut *mut int (notify_ep_change)(struct snd_ump_endpoint,
    pub fb): *mut snd_ump_block,
    pub ump): *mut *mut int (switch_protocol)(struct snd_ump_endpoint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_block {
    pub info: snd_ump_block_info,
    pub ump: *mut snd_ump_endpoint,
    pub private_data: *mut c_void,
    pub blk): *mut *mut void (private_free)(struct snd_ump_block,
    pub list: list_head,
}

extern "C" {
    pub fn snd_ump_parse_endpoint(ump: *mut snd_ump_endpoint) -> c_int;
}
extern "C" {
    pub fn snd_ump_receive(ump: *mut snd_ump_endpoint, buffer: *const u32, count: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ump_transmit(ump: *mut snd_ump_endpoint, buffer: *mut u32, count: c_int) -> c_int;
}

extern "C" {
    pub fn snd_ump_receive_ump_val(ump: *mut snd_ump_endpoint, val: u32) -> c_int;
}
extern "C" {
    pub fn snd_ump_switch_protocol(ump: *mut snd_ump_endpoint, protocol: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_ump_update_group_attrs(ump: *mut snd_ump_endpoint);
}
//
// Some definitions for UMP
//
// MIDI 2.0 Message Type
// MIDI 2.0 SysEx / Data Status; same values for both 7-bit and 8-bit SysEx
// UMP Utility Type Status (type 0x0)
// UMP Stream Message Status (type 0xf)
// UMP Endpoint Discovery filter bitmap
// UMP Function Block Discovery filter bitmap
// UMP Endpoint Info capability bits (used for protocol request/notify, too)
// UMP EP / FB name string format; same as SysEx string handling
//
// Helpers for retrieving / filling bits from UMP
//
// get the message type (4bit) from a UMP packet (header)
// get the group number (0-based, 4bit) from a UMP packet (header)
// get the MIDI status code (4bit) from a UMP packet (header)
// get the MIDI channel number (0-based, 4bit) from a UMP packet (header)
// get the MIDI status + channel combo byte (8bit) from a UMP packet (header)
// compose a UMP packet (header) from type, group and status values
// get SysEx message status (for both 7 and 8bits) from a UMP packet (header)
// get SysEx message length (for both 7 and 8bits) from a UMP packet (header)
// For Stream Messages

