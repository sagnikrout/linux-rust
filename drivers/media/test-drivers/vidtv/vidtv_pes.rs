//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_pes.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Vidtv serves as a reference DVB driver and helps validate the existing APIs
// in the media subsystem. It can also aid developers working on userspace
// applications.
//
// This file contains the logic to translate the ES data for one access unit
// from an encoder into MPEG TS packets. It does so by first encapsulating it
// with a PES header and then splitting it into TS packets.
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

pub const PES_START_CODE_PREFIX: c_uint = 0x001 /* 00 00 01 */;
// Used when sending PTS, but not DTS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_pes_optional_pts {
    pub pts1: u8,
    pub pts2: __be16,
    pub pts3: __be16,
    pub __packed: },
// Used when sending both PTS and DTS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_pes_optional_pts_dts {
    pub pts1: u8,
    pub pts2: __be16,
    pub pts3: __be16,
    pub dts1: u8,
    pub dts2: __be16,
    pub dts3: __be16,
    pub __packed: },
// PES optional flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_pes_optional {
//
// These flags show which components are actually
// present in the "optional fields" in the optional PES
// header and which are not
//
// u16 two:2;  //0x2
// u16 PES_scrambling_control:2;
// u16 PES_priority:1;
// u16 data_alignment_indicator:1; // unused
// u16 copyright:1;
// u16 original_or_copy:1;
// u16 PTS_DTS:2;
// u16 ESCR:1;
// u16 ES_rate:1;
// u16 DSM_trick_mode:1;
// u16 additional_copy_info:1;
// u16 PES_CRC:1;
// u16 PES_extension:1;
//
    pub bitfield: __be16,
    pub length: u8,
    pub __packed: },
// The PES header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mpeg_pes {
    pub /: *mut *mut __be32 bitfield; / packet_start_code_prefix:24, stream_id: 8,
// after this field until the end of the PES data payload
    pub length: __be16,
    pub optional: [vidtv_pes_optional; ],
    pub __packed: },
//
// struct pes_header_write_args - Arguments to write a PES header.
// @dest_buf: The buffer to write into.
// @dest_offset: where to start writing in the dest_buffer.
// @dest_buf_sz: The size of the dest_buffer
// @encoder_id: Encoder id (see vidtv_encoder.h)
// @send_pts: Should we send PTS?
// @pts: PTS value to send.
// @send_dts: Should we send DTS?
// @dts: DTS value to send.
// @stream_id: The stream id to use. Ex: Audio streams (0xc0-0xdf), Video
// streams (0xe0-0xef).
// @n_pes_h_s_bytes: Padding bytes. Might be used by an encoder if needed, gets
// discarded by the decoder.
// @access_unit_len: The size of _one_ access unit (with any headers it might need)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pes_header_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub dest_buf_sz: u32,
    pub encoder_id: u32,
    pub send_pts: bool,
    pub pts: u64,
    pub send_dts: bool,
    pub dts: u64,
    pub stream_id: u16,
// might be used by an encoder if needed, gets discarded by decoder
    pub n_pes_h_s_bytes: u32,
    pub access_unit_len: u32,
}

//
// struct pes_ts_header_write_args - Arguments to write a TS header.
// @dest_buf: The buffer to write into.
// @dest_offset: where to start writing in the dest_buffer.
// @dest_buf_sz: The size of the dest_buffer
// @pid: The PID to use for the TS packets.
// @continuity_counter: Incremented on every new TS packet.
// @wrote_pes_header: Flag to indicate that the PES header was written
// @n_stuffing_bytes: Padding bytes. Might be used by an encoder if needed, gets
// discarded by the decoder.
// @pcr: counter driven by a 27Mhz clock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pes_ts_header_write_args {
    pub dest_buf: *mut c_void,
    pub dest_offset: u32,
    pub dest_buf_sz: u32,
    pub pid: u16,
    pub continuity_counter: *mut u8,
    pub wrote_pes_header: bool,
    pub n_stuffing_bytes: u32,
    pub pcr: u64,
}

//
// struct pes_write_args - Arguments for the packetizer.
// @dest_buf: The buffer to write into.
// @from: A pointer to the encoder buffer containing one access unit.
// @access_unit_len: The size of _one_ access unit (with any headers it might need)
// @dest_offset: where to start writing in the dest_buffer.
// @dest_buf_sz: The size of the dest_buffer
// @pid: The PID to use for the TS packets.
// @encoder_id: Encoder id (see vidtv_encoder.h)
// @continuity_counter: Incremented on every new TS packet.
// @stream_id: The stream id to use. Ex: Audio streams (0xc0-0xdf), Video
// streams (0xe0-0xef).
// @send_pts: Should we send PTS?
// @pts: PTS value to send.
// @send_dts: Should we send DTS?
// @dts: DTS value to send.
// @n_pes_h_s_bytes: Padding bytes. Might be used by an encoder if needed, gets
// discarded by the decoder.
// @pcr: counter driven by a 27Mhz clock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pes_write_args {
    pub dest_buf: *mut c_void,
    pub from: *mut c_void,
    pub access_unit_len: u32,
    pub dest_offset: u32,
    pub dest_buf_sz: u32,
    pub pid: u16,
    pub encoder_id: u32,
    pub continuity_counter: *mut u8,
    pub stream_id: u16,
    pub send_pts: bool,
    pub pts: u64,
    pub send_dts: bool,
    pub dts: u64,
    pub n_pes_h_s_bytes: u32,
    pub pcr: u64,
}

//
// vidtv_pes_write_into - Write a PES packet as MPEG-TS packets into a buffer.
// @args: The args to use when writing
//
// This function translate the ES data for one access unit
// from an encoder into MPEG TS packets. It does so by first encapsulating it
// with a PES header and then splitting it into TS packets.
//
// The data is then written into the buffer pointed to by 'args.buf'
//
// Return: The number of bytes written into the buffer. This is usually NOT
// equal to the size of the access unit, since we need space for PES headers, TS headers
// and padding bytes, if any.
//
extern "C" {
    pub fn vidtv_pes_write_into(args: *mut pes_write_args) -> u32;
}
