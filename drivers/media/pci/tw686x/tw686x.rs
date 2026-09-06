//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/tw686x/tw686x.h
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
// Copyright (C) 2015 VanguardiaSur - www.vanguardiasur.com.ar
//
// Copyright (C) 2015 Industrial Research Institute for Automation
// and Measurements PIAP
// Written by Krzysztof Ha?asa
//

pub const TYPE_MAX_CHANNELS: c_uint = 0x0f;
pub const TYPE_SECOND_GEN: c_uint = 0x10;
pub const TW686X_DEF_PHASE_REF: c_uint = 0x1518;
pub const TW686X_AUDIO_PAGE_MAX: c_int = 16;
pub const TW686X_AUDIO_PERIODS_MIN: c_int = 2;

pub const TW686X_DMA_MODE_MEMCPY: c_int = 0;
pub const TW686X_DMA_MODE_CONTIG: c_int = 1;
pub const TW686X_DMA_MODE_SG: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_format {
    pub name: *mut c_char,
    pub fourcc: c_uint,
    pub depth: c_uint,
    pub mode: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_dma_desc {
    pub phys: dma_addr_t,
    pub virt: *mut c_void,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_sg_desc {
// 3 MSBits for flags, 13 LSBits for length
    pub flags_length: __le32,
    pub phys: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_audio_buf {
    pub dma: dma_addr_t,
    pub virt: *mut c_void,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_v4l2_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_audio_channel {
    pub dev: *mut tw686x_dev,
    pub ss: *mut snd_pcm_substream,
    pub ch: c_uint,
    pub curr_bufs: [*mut tw686x_audio_buf; 2],
    pub dma_descs: [tw686x_dma_desc; 2],
    pub ptr: dma_addr_t,
    pub buf: [tw686x_audio_buf; TW686X_AUDIO_PAGE_MAX],
    pub buf_list: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_video_channel {
    pub dev: *mut tw686x_dev,
    pub vidq: vb2_queue,
    pub vidq_queued: list_head,
    pub device: *mut video_device,
    pub curr_bufs: [*mut tw686x_v4l2_buf; 2],
    pub dma_descs: [tw686x_dma_desc; 2],
    pub sg_descs: [*mut tw686x_sg_desc; 2],
    pub ctrl_handler: v4l2_ctrl_handler,
    pub format: *const tw686x_format,
    pub vb_mutex: mutex,
    pub qlock: spinlock_t,
    pub video_standard: v4l2_std_id,
    pub height: unsigned int width,,
    pub v_halve: unsigned int h_halve,,
    pub ch: c_uint,
    pub num: c_uint,
    pub fps: c_uint,
    pub input: c_uint,
    pub sequence: c_uint,
    pub pb: c_uint,
    pub no_signal: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_dma_ops {
    pub dev): *mut *mut int (setup)(struct tw686x_dev,
    pub pb): *mut *mut *mut int (alloc)(struct tw686x_video_channel vc, unsigned int,
    pub pb): *mut *mut *mut void (free)(struct tw686x_video_channel vc, unsigned int,
    pub pb): *mut *mut *mut void (buf_refill)(struct tw686x_video_channel vc, unsigned int,
    pub mem_ops: *const vb2_mem_ops,
    pub field: v4l2_field,
    pub hw_dma_mode: u32,
}

// struct tw686x_dev - global device status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw686x_dev {
//
// spinlock controlling access to the shared device registers
// (DMA enable/disable)
//
    pub lock: spinlock_t,
    pub v4l2_dev: v4l2_device,
    pub snd_card: *mut snd_card,
    pub name: [c_char; 32],
    pub type: c_uint,
    pub dma_mode: c_uint,
    pub pci_dev: *mut pci_dev,
    pub mmio: *mut __u32 __iomem,
    pub dma_ops: *const tw686x_dma_ops,
    pub video_channels: *mut tw686x_video_channel,
    pub audio_channels: *mut tw686x_audio_channel,
// Per-device audio parameters
    pub audio_rate: c_int,
    pub period_size: c_int,
    pub audio_enabled: c_int,
    pub dma_delay_timer: timer_list,
    pub /: *mut *mut u32 pending_dma_en; / must be protected by lock,
    pub /: *mut *mut u32 pending_dma_cmd; / must be protected by lock,
}

extern "C" {
    pub fn readl(reg: dev->mmio +) -> return;
}
// each channel has its own DMA SG table
extern "C" {
    pub fn tw686x_enable_channel(dev: *mut tw686x_dev, channel: c_uint);
}
extern "C" {
    pub fn tw686x_disable_channel(dev: *mut tw686x_dev, channel: c_uint);
}
extern "C" {
    pub fn tw686x_video_init(dev: *mut tw686x_dev) -> c_int;
}
extern "C" {
    pub fn tw686x_video_free(dev: *mut tw686x_dev);
}
extern "C" {
    pub fn tw686x_audio_init(dev: *mut tw686x_dev) -> c_int;
}
extern "C" {
    pub fn tw686x_audio_free(dev: *mut tw686x_dev);
}
