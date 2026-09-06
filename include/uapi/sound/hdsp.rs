//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/hdsp.h
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
// Copyright (C) 2003 Thomas Charbonnel (thomas@undata.org)
//

pub const HDSP_MATRIX_MIXER_SIZE: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HDSP_IO_Type {
    Digiface,
    Multiface,
    H9652,
    H9632,
    RPM,
    Undefined,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdsp_peak_rms {
    pub input_peaks: [__u32; 26],
    pub playback_peaks: [__u32; 26],
    pub output_peaks: [__u32; 28],
    pub input_rms: [__u64; 26],
    pub playback_rms: [__u64; 26],
// These are only used for H96xx cards
    pub output_rms: [__u64; 26],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdsp_config_info {
    pub pref_sync_ref: c_uchar,
    pub wordclock_sync_check: c_uchar,
    pub spdif_sync_check: c_uchar,
    pub adatsync_sync_check: c_uchar,
    pub adat_sync_check: [c_uchar; 3],
    pub spdif_in: c_uchar,
    pub spdif_out: c_uchar,
    pub spdif_professional: c_uchar,
    pub spdif_emphasis: c_uchar,
    pub spdif_nonaudio: c_uchar,
    pub spdif_sample_rate: c_uint,
    pub system_sample_rate: c_uint,
    pub autosync_sample_rate: c_uint,
    pub system_clock_mode: c_uchar,
    pub clock_source: c_uchar,
    pub autosync_ref: c_uchar,
    pub line_out: c_uchar,
    pub passthru: c_uchar,
    pub da_gain: c_uchar,
    pub ad_gain: c_uchar,
    pub phone_gain: c_uchar,
    pub xlr_breakout_cable: c_uchar,
    pub analog_extension_board: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdsp_firmware {
    pub /: *mut *mut *mut void firmware_data; / 24413 x 4 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdsp_version {
    pub io_type: HDSP_IO_Type,
    pub firmware_rev: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdsp_mixer {
    pub matrix: [c_ushort; HDSP_MATRIX_MIXER_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdsp_9632_aeb {
    pub aebi: c_int,
    pub aebo: c_int,
}

