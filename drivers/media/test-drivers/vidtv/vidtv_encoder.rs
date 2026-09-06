//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_encoder.h
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
// This file contains a generic encoder type that can provide data for a stream
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vidtv_encoder_id {
// add IDs here when implementing new encoders
    S302M,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_access_unit {
    pub num_samples: u32,
    pub pts: u64,
    pub dts: u64,
    pub nbytes: u32,
    pub offset: u32,
    pub next: *mut vidtv_access_unit,
}

// Some musical notes, used by a tone generator. Values are in Hz
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musical_notes {
    NOTE_SILENT = 0,

    NOTE_C_2 = 65,
    NOTE_CS_2 = 69,
    NOTE_D_2 = 73,
    NOTE_DS_2 = 78,
    NOTE_E_2 = 82,
    NOTE_F_2 = 87,
    NOTE_FS_2 = 93,
    NOTE_G_2 = 98,
    NOTE_GS_2 = 104,
    NOTE_A_2 = 110,
    NOTE_AS_2 = 117,
    NOTE_B_2 = 123,
    NOTE_C_3 = 131,
    NOTE_CS_3 = 139,
    NOTE_D_3 = 147,
    NOTE_DS_3 = 156,
    NOTE_E_3 = 165,
    NOTE_F_3 = 175,
    NOTE_FS_3 = 185,
    NOTE_G_3 = 196,
    NOTE_GS_3 = 208,
    NOTE_A_3 = 220,
    NOTE_AS_3 = 233,
    NOTE_B_3 = 247,
    NOTE_C_4 = 262,
    NOTE_CS_4 = 277,
    NOTE_D_4 = 294,
    NOTE_DS_4 = 311,
    NOTE_E_4 = 330,
    NOTE_F_4 = 349,
    NOTE_FS_4 = 370,
    NOTE_G_4 = 392,
    NOTE_GS_4 = 415,
    NOTE_A_4 = 440,
    NOTE_AS_4 = 466,
    NOTE_B_4 = 494,
    NOTE_C_5 = 523,
    NOTE_CS_5 = 554,
    NOTE_D_5 = 587,
    NOTE_DS_5 = 622,
    NOTE_E_5 = 659,
    NOTE_F_5 = 698,
    NOTE_FS_5 = 740,
    NOTE_G_5 = 784,
    NOTE_GS_5 = 831,
    NOTE_A_5 = 880,
    NOTE_AS_5 = 932,
    NOTE_B_5 = 988,
    NOTE_C_6 = 1047,
    NOTE_CS_6 = 1109,
    NOTE_D_6 = 1175,
    NOTE_DS_6 = 1245,
    NOTE_E_6 = 1319,
    NOTE_F_6 = 1397,
    NOTE_FS_6 = 1480,
    NOTE_G_6 = 1568,
    NOTE_GS_6 = 1661,
    NOTE_A_6 = 1760,
    NOTE_AS_6 = 1865,
    NOTE_B_6 = 1976,
    NOTE_C_7 = 2093
}

//
// struct vidtv_encoder - A generic encoder type.
// @id: So we can cast to a concrete implementation when needed.
// @name: Usually the same as the stream name.
// @encoder_buf: The encoder internal buffer for the access units.
// @encoder_buf_sz: The encoder buffer size, in bytes
// @encoder_buf_offset: Our byte position in the encoder buffer.
// @sample_count: How many samples we have encoded in total.
// @access_units: encoder payload units, used for clock references
// @src_buf: The source of raw data to be encoded, encoder might set a
// default if null.
// @src_buf_sz: size of @src_buf.
// @src_buf_offset: Our position in the source buffer.
// @is_video_encoder: Whether this a video encoder (as opposed to audio)
// @ctx: Encoder-specific state.
// @stream_id: Examples: Audio streams (0xc0-0xdf), Video streams
// (0xe0-0xef).
// @es_pid: The TS PID to use for the elementary stream in this encoder.
// @encode: Prepare enough AUs for the given amount of time.
// @clear: Clear the encoder output.
// @sync: Attempt to synchronize with this encoder.
// @sampling_rate_hz: The sampling rate (or fps, if video) used.
// @last_sample_cb: Called when the encoder runs out of data.This is
// so the source can read data in a
// piecemeal fashion instead of having to
// provide it all at once.
// @destroy: Destroy this encoder, freeing allocated resources.
// @next: Next in the chain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_encoder {
    pub id: vidtv_encoder_id,
    pub name: *mut c_char,
    pub encoder_buf: *mut u8,
    pub encoder_buf_sz: u32,
    pub encoder_buf_offset: u32,
    pub sample_count: u64,
    pub access_units: *mut vidtv_access_unit,
    pub src_buf: *mut c_void,
    pub src_buf_sz: u32,
    pub src_buf_offset: u32,
    pub is_video_encoder: bool,
    pub ctx: *mut c_void,
    pub stream_id: __be16,
    pub es_pid: __be16,
    pub e): *mut *mut *mut void (encode)(struct vidtv_encoder,
    pub e): *mut *mut u32 (clear)(struct vidtv_encoder,
    pub sync: *mut vidtv_encoder,
    pub sampling_rate_hz: u32,
    pub sample_no): *mut *mut void (last_sample_cb)(u32,
    pub e): *mut *mut void (destroy)(struct vidtv_encoder,
    pub next: *mut vidtv_encoder,
}
