//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/es1688.h
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
// Header file for ES488/ES1688
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

pub const ES1688_HW_AUTO: c_uint = 0x0000;
pub const ES1688_HW_688: c_uint = 0x0001;
pub const ES1688_HW_1688: c_uint = 0x0002;
pub const ES1688_HW_UNDEF: c_uint = 0x0003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_es1688 {
    pub card: *mut snd_card,
    pub /: *mut *mut unsigned long port; / port of ESS chip,
    pub res_port: *mut resource,
    pub /: *mut *mut unsigned long mpu_port; / MPU-401 port of ESS chip,
    pub /: *mut *mut int irq; / IRQ number of ESS chip,
    pub /: *mut *mut int mpu_irq; / MPU IRQ,
    pub /: *mut *mut int dma8; / 8-bit DMA,
    pub /: *mut *mut unsigned short version; / version of ESS chip,
    pub /: *mut *mut unsigned short hardware; / see to ES1688_HW_XXXX,
    pub trigger_value: c_ushort,
    pub pad: c_uchar,
    pub dma_size: c_uint,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub reg_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
}

// I/O ports

pub const e_s_s_ESS1688RESET: c_uint = 0x6;
pub const e_s_s_ESS1688READ: c_uint = 0xa;
pub const e_s_s_ESS1688WRITE: c_uint = 0xc;
pub const e_s_s_ESS1688COMMAND: c_uint = 0xc;
pub const e_s_s_ESS1688STATUS: c_uint = 0xc;
pub const e_s_s_ESS1688DATA_AVAIL: c_uint = 0xe;
pub const e_s_s_ESS1688DATA_AVAIL_16: c_uint = 0xf;
pub const e_s_s_ESS1688MIXER_ADDR: c_uint = 0x4;
pub const e_s_s_ESS1688MIXER_DATA: c_uint = 0x5;
pub const e_s_s_ESS1688OPL3_LEFT: c_uint = 0x0;
pub const e_s_s_ESS1688OPL3_RIGHT: c_uint = 0x2;
pub const e_s_s_ESS1688OPL3_BOTH: c_uint = 0x8;
pub const e_s_s_ESS1688ENABLE0: c_uint = 0x0;
pub const e_s_s_ESS1688ENABLE1: c_uint = 0x9;
pub const e_s_s_ESS1688ENABLE2: c_uint = 0xb;
pub const e_s_s_ESS1688INIT1: c_uint = 0x7;
pub const ES1688_DSP_CMD_DMAOFF: c_uint = 0xd0;
pub const ES1688_DSP_CMD_SPKON: c_uint = 0xd1;
pub const ES1688_DSP_CMD_SPKOFF: c_uint = 0xd3;
pub const ES1688_DSP_CMD_DMAON: c_uint = 0xd4;
pub const ES1688_PCM_DEV: c_uint = 0x14;
pub const ES1688_MIC_DEV: c_uint = 0x1a;
pub const ES1688_REC_DEV: c_uint = 0x1c;
pub const ES1688_MASTER_DEV: c_uint = 0x32;
pub const ES1688_FM_DEV: c_uint = 0x36;
pub const ES1688_CD_DEV: c_uint = 0x38;
pub const ES1688_AUX_DEV: c_uint = 0x3a;
pub const ES1688_SPEAKER_DEV: c_uint = 0x3c;
pub const ES1688_LINE_DEV: c_uint = 0x3e;
pub const ES1688_RECLEV_DEV: c_uint = 0xb4;
pub const ES1688_MIXS_MASK: c_uint = 0x17;
pub const ES1688_MIXS_MIC: c_uint = 0x00;
pub const ES1688_MIXS_MIC_MASTER: c_uint = 0x01;
pub const ES1688_MIXS_CD: c_uint = 0x02;
pub const ES1688_MIXS_AOUT: c_uint = 0x03;
pub const ES1688_MIXS_MIC1: c_uint = 0x04;
pub const ES1688_MIXS_REC_MIX: c_uint = 0x05;
pub const ES1688_MIXS_LINE: c_uint = 0x06;
pub const ES1688_MIXS_MASTER: c_uint = 0x07;
pub const ES1688_MIXS_MUTE: c_uint = 0x10;
//
extern "C" {
    pub fn snd_es1688_mixer_write(chip: *mut snd_es1688, reg: c_uchar, data: c_uchar);
}
extern "C" {
    pub fn snd_es1688_pcm(card: *mut snd_card, chip: *mut snd_es1688, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_es1688_mixer(card: *mut snd_card, chip: *mut snd_es1688) -> c_int;
}
extern "C" {
    pub fn snd_es1688_reset(chip: *mut snd_es1688) -> c_int;
}
