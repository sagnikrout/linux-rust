//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/lx6464es/lx6464es.h
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
// -*- linux-c -*-
//
// ALSA driver for the digigram lx6464es interface
//
// Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
//

// Interrupt or CancelIrp)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lx_stream_status {
    LX_STREAM_STATUS_FREE,
// LX_STREAM_STATUS_OPEN,
    LX_STREAM_STATUS_SCHEDULE_RUN,
// LX_STREAM_STATUS_STARTED,
    LX_STREAM_STATUS_RUNNING,
    LX_STREAM_STATUS_SCHEDULE_STOP,
// LX_STREAM_STATUS_STOPPED,
// LX_STREAM_STATUS_PAUSED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lx_stream {
    pub stream: *mut snd_pcm_substream,
    pub frame_pos: snd_pcm_uframes_t,
    pub draining: *mut *mut lx_stream_status status; / free, open, running,,
// pause
    pub is_capture:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lx6464es {
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub irq: c_int,
    pub mac_address: [u8; 6],
    pub /: *mut *mut mutex lock; / interrupt lock,
    pub open: *mut *mut mutex setup_mutex; / mutex used in hw_params,,
// and close
// ports
    pub /: *mut *mut unsigned long port_plx; / io port (size=256),
    pub /: *mut *mut *mut void __iomem port_plx_remapped; / remapped plx port,
    pub (32-bit,: *mut *mut *mut void __iomem port_dsp_bar; / memory port,
// non-prefetchable,
// size=8K)
// messaging
    pub /: *mut *mut mutex msg_lock; / message lock,
    pub rmh: lx_rmh,
    pub irqsrc: u32,
// configuration
    pub 2: uint freq_ratio :,
    pub 1: uint playback_mute :,
    pub hardware_running: [c_uint; 2],
    pub from: *mut *mut u32 board_sample_rate; / sample rate read,
// board
    pub /: *mut *mut u16 pcm_granularity; / board blocksize,
// dma
    pub capture_dma_buf: snd_dma_buffer,
    pub playback_dma_buf: snd_dma_buffer,
// pcm
    pub pcm: *mut snd_pcm,
// streams
    pub capture_stream: lx_stream,
    pub playback_stream: lx_stream,
}
