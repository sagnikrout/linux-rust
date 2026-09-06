//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/bcm/cygnus-ssp.h
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
// Copyright (C) 2014-2015 Broadcom Corporation
pub const CYGNUS_TDM_DAI_MAX_SLOTS: c_int = 16;
pub const CYGNUS_MAX_PLAYBACK_PORTS: c_int = 4;
pub const CYGNUS_MAX_CAPTURE_PORTS: c_int = 3;
pub const CYGNUS_MAX_I2S_PORTS: c_int = 3;

pub const CYGNUS_AUIDO_MAX_NUM_CLKS: c_int = 3;
pub const CYGNUS_SSP_FRAMEBITS_DIV: c_int = 1;
pub const CYGNUS_SSPMODE_I2S: c_int = 0;
pub const CYGNUS_SSPMODE_TDM: c_int = 1;

pub const CYGNUS_SSP_CLKSRC_PLL: c_int = 0;
// Max string length of our dt property names
pub const PROP_LEN_MAX: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ringbuf_regs {
    pub rdaddr: unsigned,
    pub wraddr: unsigned,
    pub baseaddr: unsigned,
    pub endaddr: unsigned,
    pub /: *mut *mut unsigned fmark; / freemark for play, fullmark for caputure,
    pub period_bytes: unsigned,
    pub buf_size: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cygnus_audio_port_type {
    PORT_TDM,
    PORT_SPDIF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cygnus_ssp_regs {
    pub i2s_stream_cfg: u32,
    pub i2s_cfg: u32,
    pub i2s_cap_stream_cfg: u32,
    pub i2s_cap_cfg: u32,
    pub i2s_mclk_cfg: u32,
    pub bf_destch_ctrl: u32,
    pub bf_destch_cfg: u32,
    pub bf_sourcech_ctrl: u32,
    pub bf_sourcech_cfg: u32,
    pub bf_sourcech_grp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cygnus_track_clk {
    pub cap_en: bool,
    pub play_en: bool,
    pub cap_clk_en: bool,
    pub play_clk_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cygnus_aio_port {
    pub portnum: c_int,
    pub mode: c_int,
    pub is_slave: bool,
    pub /: *mut *mut int streams_on; / will be 0 if both capture and play are off,
    pub fsync_width: c_int,
    pub port_type: c_int,
    pub mclk: u32,
    pub lrclk: u32,
    pub bit_per_frame: u32,
    pub pll_clk_num: u32,
    pub cygaud: *mut cygnus_audio,
    pub regs: cygnus_ssp_regs,
    pub play_rb_regs: ringbuf_regs,
    pub capture_rb_regs: ringbuf_regs,
    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,
    pub clk_trace: cygnus_track_clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cygnus_audio {
    pub portinfo: [cygnus_aio_port; CYGNUS_MAX_PORTS],
    pub irq_num: c_int,
    pub audio: *mut void __iomem,
    pub dev: *mut device,
    pub i2s_in: *mut void __iomem,
    pub audio_clk: [*mut clk; CYGNUS_AUIDO_MAX_NUM_CLKS],
    pub active_ports: c_int,
    pub vco_rate: c_ulong,
}

extern "C" {
    pub fn cygnus_soc_platform_unregister(dev: *mut device) -> c_int;
}
