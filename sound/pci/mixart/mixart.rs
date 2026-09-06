//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/mixart/mixart.h
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
// Driver for Digigram miXart soundcards
//
// main header file
//
// Copyright (c) 2003 by Digigram <alsa@digigram.com>
//

pub const MIXART_DRIVER_VERSION: c_uint = 0x000100	/* 0.1.0 */;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_uid {
    pub object_id: u32,
    pub desc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_area {
    pub phys: c_ulong,
    pub virt: *mut void __iomem,
    pub res: *mut resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_route {
    pub connected: c_uchar,
    pub phase_inv: c_uchar,
    pub volume: c_int,
}

// firmware status codes
pub const MIXART_MOTHERBOARD_XLX_INDEX: c_int = 0;
pub const MIXART_MOTHERBOARD_ELF_INDEX: c_int = 1;
pub const MIXART_AESEBUBOARD_XLX_INDEX: c_int = 2;

pub const MIXART_MAX_CARDS: c_int = 4;
pub const MSG_FIFO_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_mgr {
    pub num_cards: c_uint,
    pub chip: [*mut snd_mixart; MIXART_MAX_CARDS],
    pub pci: *mut pci_dev,
    pub irq: c_int,
// memory-maps
    pub mem: [mem_area; 2],
// one and only blocking message or notification may be pending
    pub pending_event: u32,
    pub msg_sleep: wait_queue_head_t,
// messages fifo
    pub msg_fifo: [u32; MSG_FIFO_SIZE],
    pub msg_fifo_readptr: c_int,
    pub msg_fifo_writeptr: c_int,
    pub /: *mut *mut atomic_t msg_processed; / number of messages to be processed in irq thread,
    pub /: *mut *mut mutex lock; / interrupt lock,
    pub /: *mut *mut mutex msg_lock; / mailbox lock,
    pub /: *mut *mut mutex setup_mutex; / mutex used in hw_params, open and close,
// hardware interface
    pub /: *mut *mut unsigned int dsp_loaded; / bit flags of loaded dsp indices,
    pub /: *mut *mut unsigned int board_type; / read from embedded once elf file is loaded, 250 = miXart8, 251 = with AES, 252 = with Cobranet,
    pub flowinfo: snd_dma_buffer,
    pub bufferinfo: snd_dma_buffer,
    pub uid_console_manager: mixart_uid,
    pub sample_rate: c_int,
    pub ref_count_rate: c_int,
    pub /: *mut *mut mutex mixer_mutex; / mutex for mixer,
}

pub const MIXART_STREAM_STATUS_FREE: c_int = 0;
pub const MIXART_STREAM_STATUS_OPEN: c_int = 1;
pub const MIXART_STREAM_STATUS_RUNNING: c_int = 2;
pub const MIXART_STREAM_STATUS_DRAINING: c_int = 3;
pub const MIXART_STREAM_STATUS_PAUSE: c_int = 4;
pub const MIXART_PLAYBACK_STREAMS: c_int = 4;
pub const MIXART_CAPTURE_STREAMS: c_int = 1;
pub const MIXART_PCM_ANALOG: c_int = 0;
pub const MIXART_PCM_DIGITAL: c_int = 1;
pub const MIXART_PCM_TOTAL: c_int = 2;

pub const MIXART_NOTIFY_CARD_MASK: c_uint = 0xF000;
pub const MIXART_NOTIFY_CARD_OFFSET: c_int = 12;
pub const MIXART_NOTIFY_PCM_MASK: c_uint = 0x0F00;
pub const MIXART_NOTIFY_PCM_OFFSET: c_int = 8;
pub const MIXART_NOTIFY_CAPT_MASK: c_uint = 0x0080;
pub const MIXART_NOTIFY_SUBS_MASK: c_uint = 0x007F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_stream {
    pub substream: *mut snd_pcm_substream,
    pub pipe: *mut mixart_pipe,
    pub pcm_number: c_int,
    pub /: *mut *mut int status; / nothing, running, draining,
    pub /: *mut *mut u64 abs_period_elapsed; / last absolute stream position where period_elapsed was called (multiple of runtime->period_size),
    pub /: *mut *mut u32 buf_periods; / periods counter in the buffer (< runtime->periods),
    pub /: *mut *mut u32 buf_period_frag; / defines with buf_periods the exact position in the buffer (< runtime->period_size),
    pub channels: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mixart_pipe_status {
    PIPE_UNDEFINED,
    PIPE_STOPPED,
    PIPE_RUNNING,
    PIPE_CLOCK_SET
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mixart_pipe {
    pub /: *mut *mut mixart_uid group_uid; / id of the pipe, as returned by embedded,
    pub stream_count: c_int,
    pub /: *mut *mut mixart_uid uid_left_connector; / UID's for the audio connectors,
    pub uid_right_connector: mixart_uid,
    pub status: mixart_pipe_status,
    pub /: *mut *mut int references; / number of subs openned,
    pub /: *mut *mut int monitoring; / pipe used for monitoring issue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mixart {
    pub card: *mut snd_card,
    pub mgr: *mut mixart_mgr,
    pub /: *mut *mut int chip_idx; / zero based,
    pub /: *mut *mut *mut snd_hwdep hwdep; / DSP loader, only for the first card,
    pub /: *mut *mut *mut snd_pcm pcm; / PCM analog i/o,
    pub /: *mut *mut *mut snd_pcm pcm_dig; / PCM digital i/o,
// allocate stereo pipe for instance
    pub pipe_in_ana: mixart_pipe,
    pub pipe_out_ana: mixart_pipe,
// if AES/EBU daughter board is available, additional pipes possible on pcm_dig
    pub pipe_in_dig: mixart_pipe,
    pub pipe_out_dig: mixart_pipe,
    pub /: *mut *mut mixart_stream playback_stream[MIXART_PCM_TOTAL][MIXART_PLAYBACK_STREAMS]; / 0 = pcm, 1 = pcm_dig,
    pub /: *mut *mut mixart_stream capture_stream[MIXART_PCM_TOTAL]; / 0 = pcm, 1 = pcm_dig,
// UID's for the physical io's
    pub uid_out_analog_physio: mixart_uid,
    pub uid_in_analog_physio: mixart_uid,
    pub /: *mut *mut int analog_playback_active[2]; / Mixer : Master Playback active (!mute),
    pub /: *mut *mut int analog_playback_volume[2]; / Mixer : Master Playback Volume,
    pub /: *mut *mut int analog_capture_volume[2]; / Mixer : Master Capture Volume,
    pub output)*streams][stereo]*/: *mut *mut *mut int digital_playback_active[2MIXART_PLAYBACK_STREAMS][2]; / Mixer : Digital Playback Active [(analog+AES,
    pub output)*streams][stereo]*/: *mut *mut *mut int digital_playback_volume[2MIXART_PLAYBACK_STREAMS][2]; / Mixer : Digital Playback Volume [(analog+AES,
    pub /: *mut *mut int digital_capture_volume[2][2]; / Mixer : Digital Capture Volume [analog+AES output][stereo],
    pub /: *mut *mut int monitoring_active[2]; / Mixer : Monitoring Active,
    pub /: *mut *mut int monitoring_volume[2]; / Mixer : Monitoring Volume,
}

// exported
extern "C" {
    pub fn snd_mixart_create_pcm(chip: *mut *mut snd_mixart) -> c_int;
}
extern "C" {
    pub fn snd_mixart_kill_ref_pipe(mgr: *mut mixart_mgr, pipe: *mut mixart_pipe, monitoring: c_int) -> c_int;
}
