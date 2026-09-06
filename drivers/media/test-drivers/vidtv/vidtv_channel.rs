//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_channel.h
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
// This file contains the code for a 'channel' abstraction.
//
// When vidtv boots, it will create some hardcoded channels.
// Their services will be concatenated to populate the SDT.
// Their programs will be concatenated to populate the PAT
// Their events will be concatenated to populate the EIT
// For each program in the PAT, a PMT section will be created
// The PMT section for a channel will be assigned its streams.
// Every stream will have its corresponding encoder polled to produce TS packets
// These packets may be interleaved by the mux and then delivered to the bridge
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

//
// struct vidtv_channel - A 'channel' abstraction
//
// When vidtv boots, it will create some hardcoded channels.
// Their services will be concatenated to populate the SDT.
// Their programs will be concatenated to populate the PAT
// For each program in the PAT, a PMT section will be created
// The PMT section for a channel will be assigned its streams.
// Every stream will have its corresponding encoder polled to produce TS packets
// These packets may be interleaved by the mux and then delivered to the bridge
//
// @name: name of the channel
// @transport_stream_id: a number to identify the TS, chosen at will.
// @service: A _single_ service. Will be concatenated into the SDT.
// @program_num: The link between PAT, PMT and SDT.
// @program: A _single_ program with one or more streams associated with it.
// Will be concatenated into the PAT.
// @streams: A stream loop used to populate the PMT section for 'program'
// @encoders: A encoder loop. There must be one encoder for each stream.
// @events: Optional event information. This will feed into the EIT.
// @next: Optionally chain this channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_channel {
    pub name: *mut c_char,
    pub transport_stream_id: u16,
    pub service: *mut vidtv_psi_table_sdt_service,
    pub program_num: u16,
    pub program: *mut vidtv_psi_table_pat_program,
    pub streams: *mut vidtv_psi_table_pmt_stream,
    pub encoders: *mut vidtv_encoder,
    pub events: *mut vidtv_psi_table_eit_event,
    pub next: *mut vidtv_channel,
}

//
// vidtv_channel_si_init - Init the PSI tables from the channels in the mux
// @m: The mux containing the channels.
//
extern "C" {
    pub fn vidtv_channel_si_init(m: *mut vidtv_mux) -> c_int;
}
extern "C" {
    pub fn vidtv_channel_si_destroy(m: *mut vidtv_mux);
}
//
// vidtv_channels_init - Init hardcoded, fake 'channels'.
// @m: The mux to store the channels into.
//
extern "C" {
    pub fn vidtv_channels_init(m: *mut vidtv_mux) -> c_int;
}
// vidtv_channel_s302m_init(struct vidtv_channel *head, u16 transport_stream_id);
extern "C" {
    pub fn vidtv_channels_destroy(m: *mut vidtv_mux);
}
