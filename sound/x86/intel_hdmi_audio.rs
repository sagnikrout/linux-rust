//! Automatically rewritten from C Header to Rust Module
//! Source: sound/x86/intel_hdmi_audio.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2016 Intel Corporation
// Authors:	Sailaja Bandarupalli <sailaja.bandarupalli@intel.com>
// Ramesh Babu K V	<ramesh.babu@intel.com>
// Vaibhav Agarwal <vaibhav.agarwal@intel.com>
// Jerome Anand <jerome.anand@intel.com>
//

pub const MAX_PB_STREAMS: c_int = 1;
pub const MAX_CAP_STREAMS: c_int = 0;
pub const BYTES_PER_WORD: c_uint = 0x4;

//
// CEA speaker placement:
//
// FL  FLC   FC   FRC   FR
//
// LFE
//
// RL  RLC   RC   RRC   RR
//
// The Left/Right Surround channel _notions_ LS/RS in SMPTE 320M
// corresponds to CEA RL/RR; The SMPTE channel _assignment_ C/LFE is
// swapped to CEA LFE/FC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cea_speaker_placement {
    FL  = (1 <<  0),        /* Front Left           */
    FC  = (1 <<  1),        /* Front Center         */
    FR  = (1 <<  2),        /* Front Right          */
    FLC = (1 <<  3),        /* Front Left Center    */
    FRC = (1 <<  4),        /* Front Right Center   */
    RL  = (1 <<  5),        /* Rear Left            */
    RC  = (1 <<  6),        /* Rear Center          */
    RR  = (1 <<  7),        /* Rear Right           */
    RLC = (1 <<  8),        /* Rear Left Center     */
    RRC = (1 <<  9),        /* Rear Right Center    */
    LFE = (1 << 10),        /* Low Frequency Effect */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cea_channel_speaker_allocation {
    pub ca_index: c_int,
    pub speakers: [c_int; 8],
// derived values, just for convenience
    pub channels: c_int,
    pub spk_mask: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_map_table {
    pub /: *mut *mut unsigned char map; / ALSA API channel map position,
    pub /: *mut *mut unsigned char cea_slot; / CEA slot value,
    pub /: *mut *mut int spk_mask; / speaker position bit mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_stream_info {
    pub substream: *mut snd_pcm_substream,
    pub substream_refcount: c_int,
}

//
// struct snd_intelhad - intelhad driver structure
//
// @card: ptr to hold card details
// @connected: the monitor connection status
// @stream_info: stream information
// @eld: holds ELD info
// @curr_buf: pointer to hold current active ring buf
// @valid_buf_cnt: ring buffer count for stream
// @had_spinlock: driver lock
// @aes_bits: IEC958 status bits
// @buff_done: id of current buffer done intr
// @dev: platform device handle
// @chmap: holds channel map info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_intelhad {
    pub card_ctx: *mut snd_intelhad_card,
    pub connected: bool,
    pub stream_info: pcm_stream_info,
    pub eld: [c_uchar; HDMI_MAX_ELD_BYTES],
    pub dp_output: bool,
    pub aes_bits: c_uint,
    pub had_spinlock: spinlock_t,
    pub dev: *mut device,
    pub chmap: *mut snd_pcm_chmap,
    pub tmds_clock_speed: c_int,
    pub link_rate: c_int,
    pub /: *mut *mut int port; / fixed,
    pub /: *mut *mut int pipe; / can change dynamically,
// ring buffer (BD) position index
    pub bd_head: c_uint,
// PCM buffer position indices
    pub /: *mut *mut unsigned int pcmbuf_head; / being processed,
    pub /: *mut *mut unsigned int pcmbuf_filled; / to be filled,
    pub /: *mut *mut unsigned int num_bds; / number of BDs,
    pub /: *mut *mut unsigned int period_bytes; / PCM period size in bytes,
// internal stuff
    pub /: *mut *mut aud_cfg aud_config; / AUD_CONFIG reg value cache,
    pub hdmi_audio_wq: work_struct,
    pub /: *mut *mut mutex mutex; / for protecting chmap and eld,
    pub jack: *mut snd_jack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_intelhad_card {
    pub card: *mut snd_card,
    pub dev: *mut device,
// internal stuff
    pub irq: c_int,
    pub mmio_start: *mut void __iomem,
    pub num_pipes: c_int,
    pub num_ports: c_int,
    pub /: *mut *mut snd_intelhad pcm_ctx[3]; / one for each port,
}
