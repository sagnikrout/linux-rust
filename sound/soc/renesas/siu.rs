//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/renesas/siu.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// siu.h - ALSA SoC driver for Renesas SH7343, SH7722 SIU peripheral.
//
// Copyright (C) 2009-2010 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
// Copyright (C) 2006 Carlos Munoz <carlos@kenati.com>
// Common kernel and user-space firmware-building defines and types

// PRAM program array size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siu_spb_param {
    pub /: *mut *mut __u32 ab1a; / input FIFO address,
    pub /: *mut *mut __u32 ab0a; / output FIFO address,
    pub /: *mut *mut __u32 dir; / 0=the ather except CPUOUTPUT, 1=CPUINPUT,
    pub /: *mut *mut __u32 event; / SPB program starting conditions,
    pub /: *mut *mut __u32 stfifo; / STFIFO register setting value,
    pub /: *mut *mut __u32 trdat; / TRDAT register setting value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siu_firmware {
    pub yram_fir_coeff: [__u32; YRAM_FIR_SIZE],
    pub pram0: [__u32; PRAM0_SIZE],
    pub pram1: [__u32; PRAM1_SIZE],
    pub yram0: [__u32; YRAM0_SIZE],
    pub yram1: [__u32; YRAM1_SIZE],
    pub yram2: [__u32; YRAM2_SIZE],
    pub yram3: [__u32; YRAM3_SIZE],
    pub yram4: [__u32; YRAM4_SIZE],
    pub spbpar_num: __u32,
    pub spbpar: [siu_spb_param; 32],
}

// SIU ports: only one can be used at a time
// SIU clock configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siu_info {
    pub dev: *mut device,
    pub port_id: c_int,
    pub pram: *mut u32 __iomem,
    pub xram: *mut u32 __iomem,
    pub yram: *mut u32 __iomem,
    pub reg: *mut u32 __iomem,
    pub fw: siu_firmware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siu_stream {
    pub work: work_struct,
    pub substream: *mut snd_pcm_substream,
    pub format: snd_pcm_format_t,
    pub buf_bytes: usize,
    pub period_bytes: usize,
    pub /: *mut *mut int cur_period; / Period currently in dma,
    pub volume: u32,
    pub /: *mut *mut snd_pcm_sframes_t xfer_cnt; / Number of frames,
    pub /: *mut *mut u8 rw_flg; / transfer status,
// DMA status
    pub /: *mut *mut *mut dma_chan chan; / DMA channel,
    pub tx_desc: *mut dma_async_tx_descriptor,
    pub cookie: dma_cookie_t,
    pub param: sh_dmae_slave,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct siu_port {
    pub /: *mut *mut unsigned long play_cap; / Used to track full duplex,
    pub pcm: *mut snd_pcm,
    pub playback: siu_stream,
    pub capture: siu_stream,
    pub /: *mut *mut u32 stfifo; / STFIFO value from firmware,
    pub /: *mut *mut u32 trdat; / TRDAT value from firmware,
}

// Register access
extern "C" {
    pub fn __raw_readl(_arg: addr) -> return;
}
// SIU registers

extern "C" {
    pub fn siu_init_port(port: c_int, port_info: *mut siu_port, card: *mut snd_card) -> c_int;
}
extern "C" {
    pub fn siu_free_port(port_info: *mut siu_port);
}

