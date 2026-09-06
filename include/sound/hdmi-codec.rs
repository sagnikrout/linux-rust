//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hdmi-codec.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// hdmi-codec.h - HDMI Codec driver API
//
// Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
//
// Author: Jyri Sarha <jsarha@ti.com>
//

//
// Protocol between ASoC cpu-dai and HDMI-encoder
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_codec_daifmt {
    pub fmt: },
    pub bit_clk_inv:1: c_uint,
    pub frame_clk_inv:1: c_uint,
    pub bit_clk_provider:1: c_uint,
    pub frame_clk_provider:1: c_uint,
// bit_fmt could be standard PCM format or
// IEC958 encoded format. ALSA IEC958 plugin will pass
// IEC958_SUBFRAME format to the underneath driver.
//
    pub bit_fmt: snd_pcm_format_t,
}

//
// HDMI audio parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_codec_params {
    pub cea: hdmi_audio_infoframe,
    pub iec: snd_aes_iec958,
    pub sample_rate: c_int,
    pub sample_width: c_int,
    pub channels: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_codec_ops {
//
// Called when ASoC starts an audio stream setup.
// Optional
//
    pub data): *mut *mut *mut int (audio_startup)(struct device dev, void,
//
// Configures HDMI-encoder for audio stream.
// Having either prepare or hw_params is mandatory.
//
    pub hparms): *mut hdmi_codec_params,
//
// Configures HDMI-encoder for audio stream. Can be called
// multiple times for each setup.
//
// Having either prepare or hw_params is mandatory.
//
    pub hparms): *mut hdmi_codec_params,
//
// Shuts down the audio stream.
// Mandatory
//
    pub data): *mut *mut *mut void (audio_shutdown)(struct device dev, void,
//
// Mute/unmute HDMI audio stream.
// Optional
//
    pub direction): bool enable, int,
//
// Provides EDID-Like-Data from connected HDMI device.
// Optional
//
    pub len): *mut *mut uint8_t buf, size_t,
//
// Getting DAI ID
// Optional
//
    pub data): *mut c_void,
//
// Hook callback function to handle connector plug event.
// Optional
//
    pub codec_dev): *mut device,
}

// HDMI codec initalization data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_codec_pdata {
    pub ops: *const hdmi_codec_ops,
    pub i2s_formats: u64,
    pub i2s:1: c_uint,
    pub no_i2s_playback:1: c_uint,
    pub no_i2s_capture:1: c_uint,
    pub spdif:1: c_uint,
    pub no_spdif_playback:1: c_uint,
    pub no_spdif_capture:1: c_uint,
    pub no_capture_mute:1: c_uint,
    pub max_i2s_channels: c_int,
    pub data: *mut c_void,
}

