//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/kirkwood/kirkwood.h
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
// kirkwood.h
//
// (c) 2010 Arnaud Patard <apatard@mandriva.com>
//

pub const KIRKWOOD_RECORD_WIN: c_int = 0;
pub const KIRKWOOD_PLAYBACK_WIN: c_int = 1;
pub const KIRKWOOD_MAX_AUDIO_WIN: c_int = 2;

pub const KIRKWOOD_RECCTL: c_uint = 0x1000;

pub const KIRKWOOD_REC_BUF_ADDR: c_uint = 0x1004;
pub const KIRKWOOD_REC_BUF_SIZE: c_uint = 0x1008;
pub const KIRKWOOD_REC_BYTE_COUNT: c_uint = 0x100C;
pub const KIRKWOOD_PLAYCTL: c_uint = 0x1100;

pub const KIRKWOOD_PLAY_BUF_ADDR: c_uint = 0x1104;
pub const KIRKWOOD_PLAY_BUF_SIZE: c_uint = 0x1108;
pub const KIRKWOOD_PLAY_BYTE_COUNT: c_uint = 0x110C;
pub const KIRKWOOD_DCO_CTL: c_uint = 0x1204;

pub const KIRKWOOD_DCO_SPCR_STATUS: c_uint = 0x120c;

pub const KIRKWOOD_CLOCKS_CTRL: c_uint = 0x1230;

pub const KIRKWOOD_ERR_CAUSE: c_uint = 0x1300;
pub const KIRKWOOD_ERR_MASK: c_uint = 0x1304;
pub const KIRKWOOD_INT_CAUSE: c_uint = 0x1308;
pub const KIRKWOOD_INT_MASK: c_uint = 0x130C;

pub const KIRKWOOD_REC_BYTE_INT_COUNT: c_uint = 0x1310;
pub const KIRKWOOD_PLAY_BYTE_INT_COUNT: c_uint = 0x1314;
pub const KIRKWOOD_BYTE_INT_COUNT_MASK: c_uint = 0xffffff;
pub const KIRKWOOD_I2S_PLAYCTL: c_uint = 0x2508;
pub const KIRKWOOD_I2S_RECCTL: c_uint = 0x2408;

// Theses values come from the marvell alsa driver
// need to find where they come from
pub const KIRKWOOD_SND_MIN_PERIODS: c_int = 2;
pub const KIRKWOOD_SND_MAX_PERIODS: c_int = 16;
pub const KIRKWOOD_SND_MIN_PERIOD_BYTES: c_int = 256;
pub const KIRKWOOD_SND_MAX_PERIOD_BYTES: c_uint = 0x8000;

// KIRKWOOD_SND_MAX_PERIODS)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kirkwood_dma_data {
    pub io: *mut void __iomem,
    pub pll_config: *mut void __iomem,
    pub soc_control: *mut void __iomem,
    pub clk: *mut clk,
    pub extclk: *mut clk,
    pub ctl_play: u32,
    pub ctl_rec: u32,
    pub substream_play: *mut snd_pcm_substream,
    pub substream_rec: *mut snd_pcm_substream,
    pub irq: c_int,
    pub burst: c_int,
}
