//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vidtv/vidtv_mux.h
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
// This file contains the muxer logic for TS packets from different
// elementary streams.
//
// Loosely based on libavcodec/mpegtsenc.c
//
// Copyright (C) 2020 Daniel W. S. Almeida
//

//
// struct vidtv_mux_timing - Timing related information
//
// This is used to decide when PCR or PSI packets should be sent. This will also
// provide storage for the clock, which is used to compute the value for the PCR.
//
// @start_jiffies: The value of 'jiffies' when we started the mux thread.
// @current_jiffies: The value of 'jiffies' for the current iteration.
// @past_jiffies: The value of 'jiffies' for the past iteration.
// @clk: A 27Mhz clock from which we will drive the PCR. Updated proportionally
// on every iteration.
// @pcr_period_usecs: How often we should send PCR packets.
// @si_period_usecs: How often we should send PSI packets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mux_timing {
    pub start_jiffies: u64,
    pub current_jiffies: u64,
    pub past_jiffies: u64,
    pub clk: u64,
    pub pcr_period_usecs: u64,
    pub si_period_usecs: u64,
}

//
// struct vidtv_mux_si - Store the PSI context.
//
// This is used to store the PAT, PMT sections and SDT in use by the muxer.
//
// The muxer acquire these by looking into the hardcoded channels in
// vidtv_channel and then periodically sends the TS packets for them>
//
// @pat: The PAT in use by the muxer.
// @pmt_secs: The PMT sections in use by the muxer. One for each program in the PAT.
// @sdt: The SDT in use by the muxer.
// @nit: The NIT in use by the muxer.
// @eit: the EIT in use by the muxer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mux_si {
// the SI tables
    pub pat: *mut vidtv_psi_table_pat,
    pub /: *mut *mut *mut *mut vidtv_psi_table_pmt pmt_secs; / the PMT sections,
    pub sdt: *mut vidtv_psi_table_sdt,
    pub nit: *mut vidtv_psi_table_nit,
    pub eit: *mut vidtv_psi_table_eit,
}

//
// struct vidtv_mux_pid_ctx - Store the context for a given TS PID.
// @pid: The TS PID.
// @cc: The continuity counter for this PID. It is incremented on every TS
// pack and it will wrap around at 0xf0. If the decoder notices a sudden jump in
// this counter this will trigger a discontinuity state.
// @h: This is embedded in a hash table, mapping pid -> vidtv_mux_pid_ctx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mux_pid_ctx {
    pub pid: u16,
    pub /: *mut *mut u8 cc; / continuity counter,
    pub h: hlist_node,
}

//
// struct vidtv_mux - A muxer abstraction loosely based in libavcodec/mpegtsenc.c
// @fe: The frontend structure allocated by the muxer.
// @dev: pointer to struct device.
// @timing: Keeps track of timing related information.
// @mux_rate_kbytes_sec: The bit rate for the TS, in kbytes.
// @pid_ctx: A hash table to keep track of per-PID metadata.
// @on_new_packets_available_cb: A callback to inform of new TS packets ready.
// @mux_buf: A pointer to a buffer for this muxer. TS packets are stored there
// and then passed on to the bridge driver.
// @mux_buf_sz: The size for 'mux_buf'.
// @mux_buf_offset: The current offset into 'mux_buf'.
// @channels: The channels associated with this muxer.
// @si: Keeps track of the PSI context.
// @num_streamed_pcr: Number of PCR packets streamed.
// @num_streamed_si: The number of PSI packets streamed.
// @mpeg_thread: Thread responsible for the muxer loop.
// @streaming: whether 'mpeg_thread' is running.
// @pcr_pid: The TS PID used for the PSI packets. All channels will share the
// same PCR.
// @transport_stream_id: The transport stream ID
// @network_id: The network ID
// @network_name: The network name
// @priv: Private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mux {
    pub fe: *mut dvb_frontend,
    pub dev: *mut device,
    pub timing: vidtv_mux_timing,
    pub mux_rate_kbytes_sec: u32,
    pub 3): DECLARE_HASHTABLE(pid_ctx,,
    pub npackets): *mut *mut *mut *mut void (on_new_packets_available_cb)(void priv, u8 buf, u32,
    pub mux_buf: *mut u8,
    pub mux_buf_sz: u32,
    pub mux_buf_offset: u32,
    pub channels: *mut vidtv_channel,
    pub si: vidtv_mux_si,
    pub num_streamed_pcr: u64,
    pub num_streamed_si: u64,
    pub mpeg_thread: work_struct,
    pub streaming: bool,
    pub pcr_pid: u16,
    pub transport_stream_id: u16,
    pub network_id: u16,
    pub network_name: *mut c_char,
    pub priv: *mut c_void,
}

//
// struct vidtv_mux_init_args - Arguments used to inix the muxer.
// @mux_rate_kbytes_sec: The bit rate for the TS, in kbytes.
// @on_new_packets_available_cb: A callback to inform of new TS packets ready.
// @mux_buf_sz: The size for 'mux_buf'.
// @pcr_period_usecs: How often we should send PCR packets.
// @si_period_usecs: How often we should send PSI packets.
// @pcr_pid: The TS PID used for the PSI packets. All channels will share the
// same PCR.
// @transport_stream_id: The transport stream ID
// @channels: an optional list of channels to use
// @network_id: The network ID
// @network_name: The network name
// @priv: Private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidtv_mux_init_args {
    pub mux_rate_kbytes_sec: u32,
    pub npackets): *mut *mut *mut *mut void (on_new_packets_available_cb)(void priv, u8 buf, u32,
    pub mux_buf_sz: u32,
    pub pcr_period_usecs: u64,
    pub si_period_usecs: u64,
    pub pcr_pid: u16,
    pub transport_stream_id: u16,
    pub channels: *mut vidtv_channel,
    pub network_id: u16,
    pub network_name: *mut c_char,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn vidtv_mux_destroy(m: *mut vidtv_mux);
}
extern "C" {
    pub fn vidtv_mux_start_thread(m: *mut vidtv_mux);
}
extern "C" {
    pub fn vidtv_mux_stop_thread(m: *mut vidtv_mux);
}
