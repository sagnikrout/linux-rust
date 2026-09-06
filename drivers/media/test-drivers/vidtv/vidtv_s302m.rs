//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_s302m.h
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
// This file contains the code for an AES3 (also known as AES/EBU) encoder.
// It is based on EBU Tech 3250 and SMPTE 302M technical documents.
//
// This encoder currently supports 16bit AES3 subframes using 16bit signed
// integers.
//
// Note: AU stands for Access Unit, and AAU stands for Audio Access Unit
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

// see SMPTE 302M 2007 clause 7.3
pub const VIDTV_S302M_BUF_SZ: c_int = 65024;
// see ETSI TS 102 154 v.1.2.1 clause 7.3.5
pub const VIDTV_S302M_FORMAT_IDENTIFIER: c_uint = 0x42535344;
//
// struct vidtv_s302m_ctx - s302m encoder context.
// @enc: A pointer to the containing encoder structure.
// @frame_index: The current frame in a block
// @au_count: The total number of access units encoded up to now
// @last_duration: Duration of the tone currently being played
// @note_offset: Position at the music tone array
// @last_tone: Tone currently being played
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_s302m_ctx {
    pub enc: *mut vidtv_encoder,
    pub frame_index: u32,
    pub au_count: u32,
    pub last_duration: c_int,
    pub note_offset: c_uint,
    pub last_tone: musical_notes,
}

//
// struct vidtv_smpte_s302m_es - s302m MPEG Elementary Stream header.
//
// See SMPTE 302M 2007 table 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_smpte_s302m_es {
//
// audio_packet_size:16;
// num_channels:2;
// channel_identification:8;
// bits_per_sample:2; // 0x0 for 16bits
// zero:4;
//
    pub bitfield: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_s302m_frame_16 {
    pub data: [u8; 5],
    pub __packed: },
//
// struct vidtv_s302m_encoder_init_args - Args for the s302m encoder.
//
// @name: A name to identify this particular instance
// @src_buf: The source buffer, encoder will default to a sine wave if this is NULL.
// @src_buf_sz: The size of the source buffer.
// @es_pid: The MPEG Elementary Stream PID to use.
// @sync: Attempt to synchronize audio with this video encoder, if not NULL.
// @last_sample_cb: A callback called when the encoder runs out of data.
// @head: Add to this chain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_s302m_encoder_init_args {
    pub name: *mut c_char,
    pub src_buf: *mut c_void,
    pub src_buf_sz: u32,
    pub es_pid: u16,
    pub sync: *mut vidtv_encoder,
    pub sample_no): *mut *mut void (last_sample_cb)(u32,
    pub head: *mut vidtv_encoder,
}

// vidtv_s302m_encoder_init(struct vidtv_s302m_encoder_init_args args);
extern "C" {
    pub fn vidtv_s302m_encoder_destroy(encoder: *mut vidtv_encoder);
}
