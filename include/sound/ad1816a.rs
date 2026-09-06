//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ad1816a.h
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

pub const AD1816A_CHIP_STATUS: c_uint = 0x00;
pub const AD1816A_INDIR_ADDR: c_uint = 0x00;
pub const AD1816A_INTERRUPT_STATUS: c_uint = 0x01;
pub const AD1816A_INDIR_DATA_LOW: c_uint = 0x02;
pub const AD1816A_INDIR_DATA_HIGH: c_uint = 0x03;
pub const AD1816A_PIO_DEBUG: c_uint = 0x04;
pub const AD1816A_PIO_STATUS: c_uint = 0x05;
pub const AD1816A_PIO_DATA: c_uint = 0x06;
pub const AD1816A_RESERVED_7: c_uint = 0x07;
pub const AD1816A_PLAYBACK_CONFIG: c_uint = 0x08;
pub const AD1816A_CAPTURE_CONFIG: c_uint = 0x09;
pub const AD1816A_RESERVED_10: c_uint = 0x0a;
pub const AD1816A_RESERVED_11: c_uint = 0x0b;
pub const AD1816A_JOYSTICK_RAW_DATA: c_uint = 0x0c;
pub const AD1816A_JOYSTICK_CTRL: c_uint = 0x0d;
pub const AD1816A_JOY_POS_DATA_LOW: c_uint = 0x0e;
pub const AD1816A_JOY_POS_DATA_HIGH: c_uint = 0x0f;
pub const AD1816A_LOW_BYTE_TMP: c_uint = 0x00;
pub const AD1816A_INTERRUPT_ENABLE: c_uint = 0x01;
pub const AD1816A_EXTERNAL_CTRL: c_uint = 0x01;
pub const AD1816A_PLAYBACK_SAMPLE_RATE: c_uint = 0x02;
pub const AD1816A_CAPTURE_SAMPLE_RATE: c_uint = 0x03;
pub const AD1816A_VOICE_ATT: c_uint = 0x04;
pub const AD1816A_FM_ATT: c_uint = 0x05;
pub const AD1816A_I2S_1_ATT: c_uint = 0x06;
pub const AD1816A_I2S_0_ATT: c_uint = 0x07;
pub const AD1816A_PLAYBACK_BASE_COUNT: c_uint = 0x08;
pub const AD1816A_PLAYBACK_CURR_COUNT: c_uint = 0x09;
pub const AD1816A_CAPTURE_BASE_COUNT: c_uint = 0x0a;
pub const AD1816A_CAPTURE_CURR_COUNT: c_uint = 0x0b;
pub const AD1816A_TIMER_BASE_COUNT: c_uint = 0x0c;
pub const AD1816A_TIMER_CURR_COUNT: c_uint = 0x0d;
pub const AD1816A_MASTER_ATT: c_uint = 0x0e;
pub const AD1816A_CD_GAIN_ATT: c_uint = 0x0f;
pub const AD1816A_SYNTH_GAIN_ATT: c_uint = 0x10;
pub const AD1816A_VID_GAIN_ATT: c_uint = 0x11;
pub const AD1816A_LINE_GAIN_ATT: c_uint = 0x12;
pub const AD1816A_MIC_GAIN_ATT: c_uint = 0x13;
pub const AD1816A_PHONE_IN_GAIN_ATT: c_uint = 0x13;
pub const AD1816A_ADC_SOURCE_SEL: c_uint = 0x14;
pub const AD1816A_ADC_PGA: c_uint = 0x14;
pub const AD1816A_CHIP_CONFIG: c_uint = 0x20;
pub const AD1816A_DSP_CONFIG: c_uint = 0x21;
pub const AD1816A_FM_SAMPLE_RATE: c_uint = 0x22;
pub const AD1816A_I2S_1_SAMPLE_RATE: c_uint = 0x23;
pub const AD1816A_I2S_0_SAMPLE_RATE: c_uint = 0x24;
pub const AD1816A_RESERVED_37: c_uint = 0x25;
pub const AD1816A_PROGRAM_CLOCK_RATE: c_uint = 0x26;
pub const AD1816A_3D_PHAT_CTRL: c_uint = 0x27;
pub const AD1816A_PHONE_OUT_ATT: c_uint = 0x27;
pub const AD1816A_RESERVED_40: c_uint = 0x28;
pub const AD1816A_HW_VOL_BUT: c_uint = 0x29;
pub const AD1816A_DSP_MAILBOX_0: c_uint = 0x2a;
pub const AD1816A_DSP_MAILBOX_1: c_uint = 0x2b;
pub const AD1816A_POWERDOWN_CTRL: c_uint = 0x2c;
pub const AD1816A_TIMER_CTRL: c_uint = 0x2c;
pub const AD1816A_VERSION_ID: c_uint = 0x2d;
pub const AD1816A_RESERVED_46: c_uint = 0x2e;
pub const AD1816A_READY: c_uint = 0x80;
pub const AD1816A_PLAYBACK_IRQ_PENDING: c_uint = 0x80;
pub const AD1816A_CAPTURE_IRQ_PENDING: c_uint = 0x40;
pub const AD1816A_TIMER_IRQ_PENDING: c_uint = 0x20;
pub const AD1816A_PLAYBACK_ENABLE: c_uint = 0x01;
pub const AD1816A_PLAYBACK_PIO: c_uint = 0x02;
pub const AD1816A_CAPTURE_ENABLE: c_uint = 0x01;
pub const AD1816A_CAPTURE_PIO: c_uint = 0x02;
pub const AD1816A_FMT_LINEAR_8: c_uint = 0x00;
pub const AD1816A_FMT_ULAW_8: c_uint = 0x08;
pub const AD1816A_FMT_LINEAR_16_LIT: c_uint = 0x10;
pub const AD1816A_FMT_ALAW_8: c_uint = 0x18;
pub const AD1816A_FMT_LINEAR_16_BIG: c_uint = 0x30;
pub const AD1816A_FMT_ALL: c_uint = 0x38;
pub const AD1816A_FMT_STEREO: c_uint = 0x04;
pub const AD1816A_PLAYBACK_IRQ_ENABLE: c_uint = 0x8000;
pub const AD1816A_CAPTURE_IRQ_ENABLE: c_uint = 0x4000;
pub const AD1816A_TIMER_IRQ_ENABLE: c_uint = 0x2000;
pub const AD1816A_TIMER_ENABLE: c_uint = 0x0080;
pub const AD1816A_SRC_LINE: c_uint = 0x00;
pub const AD1816A_SRC_OUT: c_uint = 0x10;
pub const AD1816A_SRC_CD: c_uint = 0x20;
pub const AD1816A_SRC_SYNTH: c_uint = 0x30;
pub const AD1816A_SRC_VIDEO: c_uint = 0x40;
pub const AD1816A_SRC_MIC: c_uint = 0x50;
pub const AD1816A_SRC_MONO: c_uint = 0x50;
pub const AD1816A_SRC_PHONE_IN: c_uint = 0x60;
pub const AD1816A_SRC_MASK: c_uint = 0x70;
pub const AD1816A_CAPTURE_NOT_EQUAL: c_uint = 0x1000;
pub const AD1816A_WSS_ENABLE: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ad1816a {
    pub port: c_ulong,
    pub res_port: *mut resource,
    pub irq: c_int,
    pub dma1: c_int,
    pub dma2: c_int,
    pub hardware: c_ushort,
    pub version: c_ushort,
    pub lock: spinlock_t,
    pub mode: c_ushort,
    pub clock_freq: c_uint,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub timer: *mut snd_timer,
    pub image: [c_ushort; 48],
}

pub const AD1816A_HW_AUTO: c_int = 0;
pub const AD1816A_HW_AD1816A: c_int = 1;
pub const AD1816A_HW_AD1815: c_int = 2;
pub const AD1816A_HW_AD18MAX10: c_int = 3;
pub const AD1816A_MODE_PLAYBACK: c_uint = 0x01;
pub const AD1816A_MODE_CAPTURE: c_uint = 0x02;
pub const AD1816A_MODE_TIMER: c_uint = 0x04;

extern "C" {
    pub fn snd_ad1816a_pcm(chip: *mut snd_ad1816a, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_ad1816a_mixer(chip: *mut snd_ad1816a) -> c_int;
}
extern "C" {
    pub fn snd_ad1816a_timer(chip: *mut snd_ad1816a, device: c_int) -> c_int;
}

extern "C" {
    pub fn snd_ad1816a_suspend(chip: *mut snd_ad1816a);
}
extern "C" {
    pub fn snd_ad1816a_resume(chip: *mut snd_ad1816a);
}

