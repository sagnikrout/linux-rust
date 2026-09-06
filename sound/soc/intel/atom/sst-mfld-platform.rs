//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/atom/sst-mfld-platform.h
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
// sst_mfld_platform.h - Intel MID Platform driver header file
//
// Copyright (C) 2010 Intel Corp
// Author: Vinod Koul <vinod.koul@intel.com>
// Author: Harsha Priya <priya.harsha@intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

pub const SST_MONO: c_int = 1;
pub const SST_STEREO: c_int = 2;
pub const SST_MAX_CAP: c_int = 5;

pub const SST_MIN_PERIOD_BYTES: c_int = 32;

pub const SST_MIN_PERIODS: c_int = 2;

pub const SST_FIFO_SIZE: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_stream_info {
    pub str_id: c_int,
    pub arg: *mut c_void,
    pub arg): *mut *mut void (period_elapsed) (void,
    pub buffer_ptr: c_ulonglong,
    pub pcm_delay: c_ulonglong,
    pub sfreq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_drv_status {
    SST_PLATFORM_INIT = 1,
    SST_PLATFORM_STARTED,
    SST_PLATFORM_RUNNING,
    SST_PLATFORM_PAUSED,
    SST_PLATFORM_DROPPED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_stream_ops {
    STREAM_OPS_PLAYBACK = 0,
    STREAM_OPS_CAPTURE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_audio_device_type {
    SND_SST_DEVICE_HEADSET = 1,
    SND_SST_DEVICE_IHF,
    SND_SST_DEVICE_VIBRA,
    SND_SST_DEVICE_HAPTIC,
    SND_SST_DEVICE_CAPTURE,
    SND_SST_DEVICE_COMPRESS,
}

// PCM Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_pcm_params {
    pub /: *mut *mut u16 codec; / codec type,
    pub /: *mut *mut u8 num_chan; / 1=Mono, 2=Stereo,
    pub bit*/: *mut *mut u8 pcm_wd_sz; / 16/24 -,
    pub /: *mut *mut u32 reserved; / Bitrate in bits per second,
    pub /: *mut *mut u32 sfreq; / Sampling rate in Hz,
    pub ring_buffer_size: u32,
    pub samples*/: *mut *mut u32 period_count; / period elapsed in,
    pub ring_buffer_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_stream_params {
    pub result: u32,
    pub stream_id: u32,
    pub codec: u8,
    pub ops: u8,
    pub stream_type: u8,
    pub device_type: u8,
    pub sparams: sst_pcm_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_compress_cb {
    pub param: *mut c_void,
    pub param): *mut *mut void (compr_cb)(void,
    pub drain_cb_param: *mut c_void,
    pub param): *mut *mut void (drain_notify)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compress_sst_ops {
    pub name: *const c_char,
    pub cb): *mut *mut snd_sst_params str_params, sst_compress_cb,
    pub str_id): *mut *mut *mut int (stream_start)(struct device dev, unsigned int,
    pub str_id): *mut *mut *mut int (stream_drop)(struct device dev, unsigned int,
    pub str_id): *mut *mut *mut int (stream_drain)(struct device dev, unsigned int,
    pub str_id): *mut *mut *mut int (stream_partial_drain)(struct device dev, unsigned int,
    pub str_id): *mut *mut *mut int (stream_pause)(struct device dev, unsigned int,
    pub str_id): *mut *mut *mut int (stream_pause_release)(struct device dev, unsigned int,
    pub tstamp): *mut snd_compr_tstamp64,
    pub bytes): c_ulong,
    pub str_id): *mut *mut *mut int (close)(struct device dev, unsigned int,
    pub caps): *mut *mut int (get_caps)(struct snd_compr_caps,
    pub codec): *mut *mut int (get_codec_caps)(struct snd_compr_codec_caps,
    pub mdata): *mut snd_compr_metadata,
    pub state): *mut *mut *mut int (power)(struct device dev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ops {
    pub str_param): *mut *mut *mut int (open)(struct device dev, struct snd_sst_params,
    pub str_info): *mut *mut *mut int (stream_init)(struct device dev, struct pcm_stream_info,
    pub str_id): *mut *mut *mut int (stream_start)(struct device dev, int,
    pub str_id): *mut *mut *mut int (stream_drop)(struct device dev, int,
    pub str_id): *mut *mut *mut int (stream_pause)(struct device dev, int,
    pub str_id): *mut *mut *mut int (stream_pause_release)(struct device dev, int,
    pub str_info): *mut *mut *mut int (stream_read_tstamp)(struct device dev, struct pcm_stream_info,
    pub bytes): *mut *mut *mut int (send_byte_stream)(struct device dev, struct snd_sst_bytes_v2,
    pub str_id): *mut *mut *mut int (close)(struct device dev, unsigned int,
    pub state): *mut *mut *mut int (power)(struct device dev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_runtime_stream {
    pub stream_status: c_int,
    pub id: c_uint,
    pub bytes_written: usize,
    pub stream_info: pcm_stream_info,
    pub ops: *mut sst_ops,
    pub compr_ops: *mut compress_sst_ops,
    pub status_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_device {
    pub name: *mut c_char,
    pub dev: *mut device,
    pub ops: *mut sst_ops,
    pub pdev: *mut platform_device,
    pub compr_ops: *mut compress_sst_ops,
}

extern "C" {
    pub fn sst_dsp_init_v2_dpcm(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn sst_send_pipe_gains(dai: *mut snd_soc_dai, stream: c_int, mute: c_int) -> c_int;
}
extern "C" {
    pub fn send_ssp_cmd(dai: *mut snd_soc_dai, id: *const c_char, enable: bool) -> c_int;
}
extern "C" {
    pub fn sst_handle_vb_timer(dai: *mut snd_soc_dai, enable: bool) -> c_int;
}
extern "C" {
    pub fn sst_set_stream_status(stream: *mut sst_runtime_stream, state: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_algo_int_control_v2 {
    pub mc: soc_mixer_control,
    pub /: *mut *mut u16 module_id; / module identifieer,
    pub /: *mut *mut u16 pipe_id; / location info: pipe_id + instance_id,
    pub instance_id: u16,
    pub /: *mut *mut unsigned int value; / Value received is stored here,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_data {
    pub pdev: *mut platform_device,
    pub pdata: *mut sst_platform_data,
    pub byte_stream: *mut snd_sst_bytes_v2,
    pub lock: mutex,
    pub soc_card: *mut snd_soc_card,
    pub ssp_cmd: sst_cmd_sba_hw_set_ssp,
}

extern "C" {
    pub fn sst_register_dsp(dev: *mut sst_device) -> c_int;
}
extern "C" {
    pub fn sst_unregister_dsp(dev: *mut sst_device) -> c_int;
}
