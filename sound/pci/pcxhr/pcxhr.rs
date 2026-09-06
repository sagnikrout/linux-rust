//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/pcxhr/pcxhr.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Digigram pcxhr soundcards
//
// main header file
//
// Copyright (c) 2004 by Digigram <alsa@digigram.com>
//

pub const PCXHR_DRIVER_VERSION: c_uint = 0x000906	/* 0.9.6 */;

pub const PCXHR_MAX_CARDS: c_int = 6;
pub const PCXHR_PLAYBACK_STREAMS: c_int = 4;

// transfer granularity of pipes and the dsp time (MBOX4)
pub const PCXHR_GRANULARITY_MIN: c_int = 96;
// TODO : granularity could be 64 or 128

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcxhr_clock_type {
    PCXHR_CLOCK_TYPE_INTERNAL = 0,
    PCXHR_CLOCK_TYPE_WORD_CLOCK,
    PCXHR_CLOCK_TYPE_AES_SYNC,
    PCXHR_CLOCK_TYPE_AES_1,
    PCXHR_CLOCK_TYPE_AES_2,
    PCXHR_CLOCK_TYPE_AES_3,
    PCXHR_CLOCK_TYPE_AES_4,
    PCXHR_CLOCK_TYPE_MAX = PCXHR_CLOCK_TYPE_AES_4,
    HR22_CLOCK_TYPE_INTERNAL = PCXHR_CLOCK_TYPE_INTERNAL,
    HR22_CLOCK_TYPE_AES_SYNC,
    HR22_CLOCK_TYPE_AES_1,
    HR22_CLOCK_TYPE_MAX = HR22_CLOCK_TYPE_AES_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcxhr_mgr {
    pub num_cards: c_uint,
    pub chip: [*mut snd_pcxhr; PCXHR_MAX_CARDS],
    pub pci: *mut pci_dev,
    pub irq: c_int,
    pub granularity: c_int,
// card access with 1 mem bar and 2 io bar's
    pub port: [c_ulong; 3],
// share the name
    pub /: *mut *mut char name[40]; / name of this soundcard,
    pub prmh: *mut pcxhr_rmh,
    pub /: *mut *mut mutex lock; / interrupt lock,
    pub /: *mut *mut mutex msg_lock; / message lock,
    pub /: *mut *mut mutex setup_mutex; / mutex used in hw_params, open and close,
    pub /: *mut *mut mutex mixer_mutex; / mutex for mixer,
// hardware interface
    pub /: *mut *mut unsigned int dsp_loaded; / bit flags of loaded dsp indices,
    pub /: *mut *mut unsigned int dsp_version; / read from embedded once firmware is loaded,
    pub playback_chips: c_int,
    pub capture_chips: c_int,
    pub fw_file_set: c_int,
    pub firmware_num: c_int,
    pub is_hr_stereo:1: c_uint,
    pub /: *mut *mut unsigned int board_has_aes1:1; / if 1 board has AES1 plug and SRC,
    pub /: *mut *mut unsigned int board_has_analog:1; / if 0 the board is digital only,
    pub /: *mut *mut unsigned int board_has_mic:1; / if 1 the board has microphone input,
    pub /: *mut *mut unsigned int board_aes_in_192k:1;/ if 1 the aes input plugs do support 192kHz,
    pub /: *mut *mut unsigned int mono_capture:1; / if 1 the board does mono capture,
    pub /: *mut *mut unsigned int capture_ltc:1; / if 1 the board captures LTC input,
    pub hostport: snd_dma_buffer,
    pub /: *mut *mut pcxhr_clock_type use_clock_type; / clock type selected by mixer,
    pub /: *mut *mut pcxhr_clock_type cur_clock_type; / current clock type synced,
    pub sample_rate: c_int,
    pub ref_count_rate: c_int,
    pub /: *mut *mut int timer_toggle; / timer interrupt toggles between the two values 0x200 and 0x300,
    pub /: *mut *mut int dsp_time_last; / the last dsp time (read by interrupt),
    pub /: *mut *mut int dsp_time_err; / dsp time errors,
    pub /: *mut *mut unsigned int src_it_dsp; / dsp interrupt source,
    pub /: *mut *mut unsigned int io_num_reg_cont; / backup of IO_NUM_REG_CONT,
    pub /: *mut *mut unsigned int codec_speed; / speed mode of the codecs,
    pub /: *mut *mut unsigned int sample_rate_real; / current real sample rate,
    pub last_reg_stat: c_int,
    pub async_err_stream_xrun: c_int,
    pub async_err_pipe_xrun: c_int,
    pub async_err_other_last: c_int,
    pub /: *mut *mut unsigned char xlx_cfg; / copy of PCXHR_XLX_CFG register,
    pub /: *mut *mut unsigned char xlx_selmic; / copy of PCXHR_XLX_SELMIC register,
    pub /: *mut *mut unsigned char dsp_reset; / copy of PCXHR_DSP_RESET register,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcxhr_stream_status {
    PCXHR_STREAM_STATUS_FREE,
    PCXHR_STREAM_STATUS_OPEN,
    PCXHR_STREAM_STATUS_SCHEDULE_RUN,
    PCXHR_STREAM_STATUS_STARTED,
    PCXHR_STREAM_STATUS_RUNNING,
    PCXHR_STREAM_STATUS_SCHEDULE_STOP,
    PCXHR_STREAM_STATUS_STOPPED,
    PCXHR_STREAM_STATUS_PAUSED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcxhr_stream {
    pub substream: *mut snd_pcm_substream,
    pub format: snd_pcm_format_t,
    pub pipe: *mut pcxhr_pipe,
    pub /: *mut *mut pcxhr_stream_status status; / free, open, running, draining, pause,
    pub /: *mut *mut u_int64_t timer_abs_periods; / timer: samples elapsed since TRIGGER_START (multiple of period_size),
    pub /: *mut *mut u_int32_t timer_period_frag; / timer: samples elapsed since last call to snd_pcm_period_elapsed (0..period_size),
    pub /: *mut *mut u_int32_t timer_buf_periods; / nb of periods in the buffer that have already elapsed,
    pub /: *mut *mut int timer_is_synced; / if(0) : timer needs to be resynced with real hardware pointer,
    pub channels: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcxhr_pipe_status {
    PCXHR_PIPE_UNDEFINED,
    PCXHR_PIPE_DEFINED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcxhr_pipe {
    pub status: pcxhr_pipe_status,
    pub /: *mut *mut int is_capture; / this is a capture pipe,
    pub /: *mut *mut int first_audio; / first audio num,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcxhr {
    pub card: *mut snd_card,
    pub mgr: *mut pcxhr_mgr,
    pub /: *mut *mut int chip_idx; / zero based,
    pub /: *mut *mut *mut snd_pcm pcm; / PCM,
    pub /: *mut *mut pcxhr_pipe playback_pipe; / 1 stereo pipe only,
    pub /: *mut *mut pcxhr_pipe capture_pipe[2]; / 1 stereo or 2 mono pipes,
    pub playback_stream: [pcxhr_stream; PCXHR_PLAYBACK_STREAMS],
    pub /: *mut *mut pcxhr_stream capture_stream[2]; / 1 stereo or 2 mono streams,
    pub nb_streams_play: c_int,
    pub nb_streams_capt: c_int,
    pub /: *mut *mut int analog_playback_active[2]; / Mixer : Master Playback !mute,
    pub /: *mut *mut int analog_playback_volume[2]; / Mixer : Master Playback Volume,
    pub /: *mut *mut int analog_capture_volume[2]; / Mixer : Master Capture Volume,
    pub digital_playback_active: [c_int; PCXHR_PLAYBACK_STREAMS][2],
    pub digital_playback_volume: [c_int; PCXHR_PLAYBACK_STREAMS][2],
    pub /: *mut *mut int digital_capture_volume[2]; / Mixer : Digital Capture Volume,
    pub /: *mut *mut int monitoring_active[2]; / Mixer : Monitoring Active,
    pub /: *mut *mut int monitoring_volume[2]; / Mixer : Monitoring Volume,
    pub /: *mut *mut int audio_capture_source; / Mixer : Audio Capture Source,
    pub /: *mut *mut int mic_volume; / used by cards with MIC only,
    pub /: *mut *mut int mic_boost; / used by cards with MIC only,
    pub /: *mut *mut int mic_active; / used by cards with MIC only,
    pub /: *mut *mut int analog_capture_active; / used by cards with MIC only,
    pub /: *mut *mut int phantom_power; / used by cards with MIC only,
    pub /: *mut *mut unsigned char aes_bits[5]; / Mixer : IEC958_AES bits,
}

// exported
extern "C" {
    pub fn pcxhr_create_pcm(chip: *mut snd_pcxhr) -> c_int;
}
extern "C" {
    pub fn pcxhr_set_clock(mgr: *mut pcxhr_mgr, rate: c_uint) -> c_int;
}
