//! Automatically rewritten from C Header to Rust Module
//! Source: sound/ppc/snd_ps3.h
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
// Audio support for PS3
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// All rights reserved.
// Copyright 2006, 2007 Sony Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ps3_out_channel {
    SND_PS3_OUT_SPDIF_0,
    SND_PS3_OUT_SPDIF_1,
    SND_PS3_OUT_SERIAL_0,
    SND_PS3_OUT_DEVS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ps3_dma_filltype {
    SND_PS3_DMA_FILLTYPE_FIRSTFILL,
    SND_PS3_DMA_FILLTYPE_RUNNING,
    SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL,
    SND_PS3_DMA_FILLTYPE_SILENT_RUNNING
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_ps3_ch {
    SND_PS3_CH_L = 0,
    SND_PS3_CH_R = 1,
    SND_PS3_CH_MAX = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ps3_avsetting_info {
    pub /: *mut *mut uint32_t avs_audio_ch; / fixed,
    pub avs_audio_rate: u32,
    pub avs_audio_width: u32,
    pub /: *mut *mut uint32_t avs_audio_format; / fixed,
    pub /: *mut *mut uint32_t avs_audio_source; / fixed,
    pub avs_cs_info: [c_uchar; 8],
}

//
// PS3 audio 'card' instance
// there should be only ONE hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ps3_card_info {
    pub ps3_dev: *mut ps3_system_bus_device,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub substream: *mut snd_pcm_substream,
// hvc info
    pub audio_lpar_addr: u64,
    pub audio_lpar_size: u64,
// registers
    pub mapped_mmio_vaddr: *mut void __iomem,
// irq
    pub audio_irq_outlet: u64,
    pub irq_no: c_uint,
// remember avsetting
    pub avs: snd_ps3_avsetting_info,
// dma buffer management
    pub dma_lock: spinlock_t,
// dma_lock start
    pub /: *mut *mut *mut void  dma_start_vaddr[2]; / 0 for L, 1 for R,
    pub dma_start_bus_addr: [dma_addr_t; 2],
    pub dma_buffer_size: usize,
    pub dma_last_transfer_vaddr: [*mut *mut c_void; 2],
    pub dma_next_transfer_vaddr: [*mut *mut c_void; 2],
    pub silent: c_int,
// dma_lock end
    pub running: c_int,
// null buffer
    pub null_buffer_start_vaddr: *mut c_void,
    pub null_buffer_start_dma_addr: dma_addr_t,
// start delay
    pub start_delay: c_uint,
}

// PS3 audio DMAC block size in bytes

// one stage (stereo)  of audio FIFO in bytes

// how many stages the fifo have

// fifo size 128 bytes * 8 stages * stereo (2ch)

// PS3 audio DMAC max block count in one dma shot = 128 (0x80) blocks

