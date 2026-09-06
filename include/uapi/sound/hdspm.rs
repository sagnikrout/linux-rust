//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/hdspm.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright (C) 2003 Winfried Ritsch (IEM)
// based on hdsp.h from Thomas Charbonnel (thomas@undata.org)
//

// Maximum channels is 64 even on 56Mode you have 64playbacks to matrix
pub const HDSPM_MAX_CHANNELS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_io_type {
    MADI,
    MADIface,
    AIO,
    AES32,
    RayDAT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_speed {
    ss,
    ds,
    qs
}

// -------------------- IOCTL Peak/RMS Meters --------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_peak_rms {
    pub input_peaks: [__u32; 64],
    pub playback_peaks: [__u32; 64],
    pub output_peaks: [__u32; 64],
    pub input_rms: [__u64; 64],
    pub playback_rms: [__u64; 64],
    pub output_rms: [__u64; 64],
    pub /: *mut *mut __u8 speed; / enum {ss, ds, qs},
    pub status2: c_int,
}

// ------------ CONFIG block IOCTL ----------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_config {
    pub pref_sync_ref: c_uchar,
    pub wordclock_sync_check: c_uchar,
    pub madi_sync_check: c_uchar,
    pub system_sample_rate: c_uint,
    pub autosync_sample_rate: c_uint,
    pub system_clock_mode: c_uchar,
    pub clock_source: c_uchar,
    pub autosync_ref: c_uchar,
    pub line_out: c_uchar,
    pub passthru: c_uint,
    pub analog_out: c_uint,
}

//
// If there's a TCO (TimeCode Option) board installed,
// there are further options and status data available.
// The hdspm_ltc structure contains the current SMPTE
// timecode and some status information and can be
// obtained via SNDRV_HDSPM_IOCTL_GET_LTC or in the
// hdspm_status struct.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_ltc_format {
    format_invalid,
    fps_24,
    fps_25,
    fps_2997,
    fps_30
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_ltc_frame {
    frame_invalid,
    drop_frame,
    full_frame
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_ltc_input_format {
    ntsc,
    pal,
    no_video
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_ltc {
    pub ltc: c_uint,
    pub format: hdspm_ltc_format,
    pub frame: hdspm_ltc_frame,
    pub input_format: hdspm_ltc_input_format,
}

//
// The status data reflects the device's current state
// as determined by the card's configuration and
// connection status.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_sync {
    hdspm_sync_no_lock = 0,
    hdspm_sync_lock = 1,
    hdspm_sync_sync = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_madi_input {
    hdspm_input_optical = 0,
    hdspm_input_coax = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_madi_channel_format {
    hdspm_format_ch_64 = 0,
    hdspm_format_ch_56 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_madi_frame_format {
    hdspm_frame_48 = 0,
    hdspm_frame_96 = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdspm_syncsource {
    syncsource_wc = 0,
    syncsource_madi = 1,
    syncsource_tco = 2,
    syncsource_sync = 3,
    syncsource_none = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_status {
    pub /: *mut *mut __u8 card_type; / enum hdspm_io_type,
    pub autosync_source: hdspm_syncsource,
    pub card_clock: __u64,
    pub master_period: __u32,
    pub /: *mut *mut __u8 sync_wc; / enum hdspm_sync,
    pub /: *mut *mut __u8 sync_madi; / enum hdspm_sync,
    pub /: *mut *mut __u8 sync_tco; / enum hdspm_sync,
    pub /: *mut *mut __u8 sync_in; / enum hdspm_sync,
    pub /: *mut *mut __u8 madi_input; / enum hdspm_madi_input,
    pub /: *mut *mut __u8 channel_format; / enum hdspm_madi_channel_format,
    pub /: *mut *mut __u8 frame_format; / enum hdspm_madi_frame_format,
    pub madi: },
    pub card_specific: },
}

//
// Get information about the card and its add-ons.
//
pub const HDSPM_ADDON_TCO: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_version {
    pub /: *mut *mut __u8 card_type; / enum hdspm_io_type,
    pub cardname: [c_char; 20],
    pub serial: c_uint,
    pub firmware_rev: c_ushort,
    pub addons: c_int,
}

// ------------- get Matrix Mixer IOCTL ---------------
// MADI mixer: 64inputs+64playback in 64outputs = 8192 => *4Byte =
// 32768 Bytes
//
// organisation is 64 channelfader in a continuous memory block
// equivalent to hardware definition, maybe for future feature of mmap of
// them
//
// each of 64 outputs has 64 infader and 64 outfader:

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_channelfader {
    pub in: [c_uint; HDSPM_MIXER_CHANNELS],
    pub pb: [c_uint; HDSPM_MIXER_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_mixer {
    pub ch: [hdspm_channelfader; HDSPM_MIXER_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdspm_mixer_ioctl {
    pub mixer: *mut hdspm_mixer,
}

// use indirect access due to the limit of ioctl bit size

