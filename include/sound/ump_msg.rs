//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ump_msg.h
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
// Universal MIDI Packet (UMP): Message Definitions
//
// MIDI 1.0 / 2.0 Status Code (4bit)
// MIDI 1.0 Channel Control (7bit)
// MIDI 1.0 / 2.0 System Messages (0xfx)
// MIDI 1.0 Realtime and SysEx status messages (0xfx)
//
// UMP Message Definitions
//
// MIDI 1.0 Note Off / Note On (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi1_msg_note {

    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub velocity:8: u32,

    pub velocity:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// MIDI 1.0 Poly Pressure (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi1_msg_paf {

    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub data:8: u32,

    pub data:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// MIDI 1.0 Control Change (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi1_msg_cc {

    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub index:8: u32,
    pub data:8: u32,

    pub data:8: u32,
    pub index:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// MIDI 1.0 Program Change (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi1_msg_program {

    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub program:8: u32,
    pub reserved:8: u32,

    pub reserved:8: u32,
    pub program:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// MIDI 1.0 Channel Pressure (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi1_msg_caf {

    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub data:8: u32,
    pub reserved:8: u32,

    pub reserved:8: u32,
    pub data:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// MIDI 1.0 Pitch Bend (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi1_msg_pitchbend {

    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub data_lsb:8: u32,
    pub data_msb:8: u32,

    pub data_msb:8: u32,
    pub data_lsb:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// System Common and Real Time messages (32bit); no channel field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_system_msg {

    pub type:4: u32,
    pub group:4: u32,
    pub status:8: u32,
    pub parm1:8: u32,
    pub parm2:8: u32,

    pub parm2:8: u32,
    pub parm1:8: u32,
    pub status:8: u32,
    pub group:4: u32,
    pub type:4: u32,

    pub __packed: },
// MIDI 1.0 UMP CVM (32bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ump_midi1_msg {
    pub note: snd_ump_midi1_msg_note,
    pub paf: snd_ump_midi1_msg_paf,
    pub cc: snd_ump_midi1_msg_cc,
    pub pg: snd_ump_midi1_msg_program,
    pub caf: snd_ump_midi1_msg_caf,
    pub pb: snd_ump_midi1_msg_pitchbend,
    pub system: snd_ump_system_msg,
    pub raw: u32,
}

// MIDI 2.0 Note Off / Note On (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_note {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub attribute_type:8: u32,
// 1
    pub velocity:16: u32,
    pub attribute_data:16: u32,

// 0
    pub attribute_type:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub attribute_data:16: u32,
    pub velocity:16: u32,

    pub __packed: },
// MIDI 2.0 Poly Pressure (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_paf {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub reserved:8: u32,
// 1
    pub data: u32,

// 0
    pub reserved:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 Per-Note Controller (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_pernote_cc {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub index:8: u32,
// 1
    pub data: u32,

// 0
    pub index:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 Per-Note Management (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_pernote_mgmt {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub flags:8: u32,
// 1
    pub reserved: u32,

// 0
    pub flags:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub reserved: u32,

    pub __packed: },
// MIDI 2.0 Control Change (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_cc {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub index:8: u32,
    pub reserved:8: u32,
// 1
    pub data: u32,

// 0
    pub reserved:8: u32,
    pub index:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 Registered Controller (RPN) / Assignable Controller (NRPN) (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_rpn {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub bank:8: u32,
    pub index:8: u32,
// 1
    pub data: u32,

// 0
    pub index:8: u32,
    pub bank:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 Program Change (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_program {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub reserved:15: u32,
    pub bank_valid:1: u32,
// 1
    pub program:8: u32,
    pub reserved2:8: u32,
    pub bank_msb:8: u32,
    pub bank_lsb:8: u32,

// 0
    pub bank_valid:1: u32,
    pub reserved:15: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub bank_lsb:8: u32,
    pub bank_msb:8: u32,
    pub reserved2:8: u32,
    pub program:8: u32,

    pub __packed: },
// MIDI 2.0 Channel Pressure (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_caf {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub reserved:16: u32,
// 1
    pub data: u32,

// 0
    pub reserved:16: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 Pitch Bend (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_pitchbend {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub reserved:16: u32,
// 1
    pub data: u32,

// 0
    pub reserved:16: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 Per-Note Pitch Bend (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_msg_pernote_pitchbend {

// 0
    pub type:4: u32,
    pub group:4: u32,
    pub status:4: u32,
    pub channel:4: u32,
    pub note:8: u32,
    pub reserved:8: u32,
// 1
    pub data: u32,

// 0
    pub reserved:8: u32,
    pub note:8: u32,
    pub channel:4: u32,
    pub status:4: u32,
    pub group:4: u32,
    pub type:4: u32,
// 1
    pub data: u32,

    pub __packed: },
// MIDI 2.0 UMP CVM (64bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ump_midi2_msg {
    pub note: snd_ump_midi2_msg_note,
    pub paf: snd_ump_midi2_msg_paf,
    pub pernote_cc: snd_ump_midi2_msg_pernote_cc,
    pub pernote_mgmt: snd_ump_midi2_msg_pernote_mgmt,
    pub cc: snd_ump_midi2_msg_cc,
    pub rpn: snd_ump_midi2_msg_rpn,
    pub pg: snd_ump_midi2_msg_program,
    pub caf: snd_ump_midi2_msg_caf,
    pub pb: snd_ump_midi2_msg_pitchbend,
    pub pernote_pb: snd_ump_midi2_msg_pernote_pitchbend,
    pub raw: [u32; 2],
}

// UMP Stream Message: Endpoint Discovery (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_ep_discovery {

// 0
    pub type:4: u32,
    pub format:2: u32,
    pub status:10: u32,
    pub ump_version_major:8: u32,
    pub ump_version_minor:8: u32,
// 1
    pub reserved:24: u32,
    pub filter_bitmap:8: u32,
// 2-3
    pub reserved2: [u32; 2],
// 0
    pub ump_version_minor:8: u32,
    pub ump_version_major:8: u32,
    pub status:10: u32,
    pub format:2: u32,
    pub type:4: u32,
// 1
    pub filter_bitmap:8: u32,
    pub reserved:24: u32,
// 2-3
    pub reserved2: [u32; 2],
    pub __packed: },
// UMP Stream Message: Endpoint Info Notification (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_ep_info {

// 0
    pub type:4: u32,
    pub format:2: u32,
    pub status:10: u32,
    pub ump_version_major:8: u32,
    pub ump_version_minor:8: u32,
// 1
    pub static_function_block:1: u32,
    pub num_function_blocks:7: u32,
    pub reserved:8: u32,
    pub protocol:8: u32,
    pub reserved2:6: u32,
    pub jrts:2: u32,
// 2-3
    pub reserved3: [u32; 2],
// 0
    pub ump_version_minor:8: u32,
    pub ump_version_major:8: u32,
    pub status:10: u32,
    pub format:2: u32,
    pub type:4: u32,
// 1
    pub jrts:2: u32,
    pub reserved2:6: u32,
    pub protocol:8: u32,
    pub reserved:8: u32,
    pub num_function_blocks:7: u32,
    pub static_function_block:1: u32,
// 2-3
    pub reserved3: [u32; 2],
    pub __packed: },
// UMP Stream Message: Device Info Notification (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_device_info {

// 0
    pub type:4: u32,
    pub format:2: u32,
    pub status:10: u32,
    pub reserved:16: u32,
// 1
    pub manufacture_id: u32,
// 2
    pub family_lsb: u8,
    pub family_msb: u8,
    pub model_lsb: u8,
    pub model_msb: u8,
// 3
    pub sw_revision: u32,

// 0
    pub reserved:16: u32,
    pub status:10: u32,
    pub format:2: u32,
    pub type:4: u32,
// 1
    pub manufacture_id: u32,
// 2
    pub model_msb: u8,
    pub model_lsb: u8,
    pub family_msb: u8,
    pub family_lsb: u8,
// 3
    pub sw_revision: u32,

    pub __packed: },
// UMP Stream Message: Stream Config Request / Notification (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_stream_cfg {

// 0
    pub type:4: u32,
    pub format:2: u32,
    pub status:10: u32,
    pub protocol:8: u32,
    pub reserved:6: u32,
    pub jrts:2: u32,
// 1-3
    pub reserved2: [u32; 3],
// 0
    pub jrts:2: u32,
    pub reserved:6: u32,
    pub protocol:8: u32,
    pub status:10: u32,
    pub format:2: u32,
    pub type:4: u32,
// 1-3
    pub reserved2: [u32; 3],
    pub __packed: },
// UMP Stream Message: Function Block Discovery (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_fb_discovery {

// 0
    pub type:4: u32,
    pub format:2: u32,
    pub status:10: u32,
    pub function_block_id:8: u32,
    pub filter:8: u32,
// 1-3
    pub reserved: [u32; 3],
// 0
    pub filter:8: u32,
    pub function_block_id:8: u32,
    pub status:10: u32,
    pub format:2: u32,
    pub type:4: u32,
// 1-3
    pub reserved: [u32; 3],
    pub __packed: },
// UMP Stream Message: Function Block Info Notification (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_fb_info {

// 0
    pub type:4: u32,
    pub format:2: u32,
    pub status:10: u32,
    pub active:1: u32,
    pub function_block_id:7: u32,
    pub reserved:2: u32,
    pub ui_hint:2: u32,
    pub midi_10:2: u32,
    pub direction:2: u32,
// 1
    pub first_group:8: u32,
    pub num_groups:8: u32,
    pub midi_ci_version:8: u32,
    pub sysex8_streams:8: u32,
// 2-3
    pub reserved2: [u32; 2],
// 0
    pub direction:2: u32,
    pub midi_10:2: u32,
    pub ui_hint:2: u32,
    pub reserved:2: u32,
    pub function_block_id:7: u32,
    pub active:1: u32,
    pub status:10: u32,
    pub format:2: u32,
    pub type:4: u32,
// 1
    pub sysex8_streams:8: u32,
    pub midi_ci_version:8: u32,
    pub num_groups:8: u32,
    pub first_group:8: u32,
// 2-3
    pub reserved2: [u32; 2],
    pub __packed: },
// UMP Stream Message: Function Block Name Notification (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_stream_msg_fb_name {

// 0
    pub type:4: u16,
    pub format:2: u16,
    pub status:10: u16,
    pub function_block_id: u8,
    pub name0: u8,
// 1-3
    pub name: [u8; 12],
// 0
    pub name0: u8,
    pub function_block_id: u8,
    pub status:10: u16,
    pub format:2: u16,
    pub type:4: u16,
// 1-3
    pub order: u8 name[12]; // FIXME: byte,

    pub __packed: },
// MIDI 2.0 Stream Messages (128bit)
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_ump_stream_msg {
    pub ep_discovery: snd_ump_stream_msg_ep_discovery,
    pub ep_info: snd_ump_stream_msg_ep_info,
    pub device_info: snd_ump_stream_msg_device_info,
    pub stream_cfg: snd_ump_stream_msg_stream_cfg,
    pub fb_discovery: snd_ump_stream_msg_fb_discovery,
    pub fb_info: snd_ump_stream_msg_fb_info,
    pub fb_name: snd_ump_stream_msg_fb_name,
    pub raw: [u32; 4],
}
