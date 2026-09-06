//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs5535audio/cs5535audio.h
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


// SPDX-License-Identifier: GPL-2.0

pub const CS5535AUDIO_MAX_DESCRIPTORS: c_int = 128;
// acc_codec bar0 reg addrs
pub const ACC_GPIO_STATUS: c_uint = 0x00;
pub const ACC_CODEC_STATUS: c_uint = 0x08;
pub const ACC_CODEC_CNTL: c_uint = 0x0C;
pub const ACC_IRQ_STATUS: c_uint = 0x12;
pub const ACC_BM0_CMD: c_uint = 0x20;
pub const ACC_BM1_CMD: c_uint = 0x28;
pub const ACC_BM0_PRD: c_uint = 0x24;
pub const ACC_BM1_PRD: c_uint = 0x2C;
pub const ACC_BM0_STATUS: c_uint = 0x21;
pub const ACC_BM1_STATUS: c_uint = 0x29;
pub const ACC_BM0_PNTR: c_uint = 0x60;
pub const ACC_BM1_PNTR: c_uint = 0x64;
// acc_codec bar0 reg bits
// ACC_IRQ_STATUS
pub const IRQ_STS: c_int = 0;
pub const WU_IRQ_STS: c_int = 1;
pub const BM0_IRQ_STS: c_int = 2;
pub const BM1_IRQ_STS: c_int = 3;
// ACC_BMX_STATUS

// ACC_BMX_CTL
pub const BM_CTL_EN: c_uint = 0x01;
pub const BM_CTL_PAUSE: c_uint = 0x03;
pub const BM_CTL_DIS: c_uint = 0x00;
pub const BM_CTL_BYTE_ORD_LE: c_uint = 0x00;
pub const BM_CTL_BYTE_ORD_BE: c_uint = 0x04;
// cs5535 specific ac97 codec register defines
pub const CMD_MASK: c_uint = 0xFF00FFFF;
pub const CMD_NEW: c_uint = 0x00010000;
pub const STS_NEW: c_uint = 0x00020000;
pub const PRM_RDY_STS: c_uint = 0x00800000;

pub const ACC_CODEC_CNTL_RD_CMD: c_uint = 0x80000000;
pub const ACC_CODEC_CNTL_LNK_SHUTDOWN: c_uint = 0x00040000;
pub const ACC_CODEC_CNTL_LNK_WRM_RST: c_uint = 0x00020000;
pub const PRD_JMP: c_uint = 0x2000;
pub const PRD_EOP: c_uint = 0x4000;
pub const PRD_EOT: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs5535audio_dma_ops {
    pub type: c_int,
    pub cs5535au): *mut *mut void (enable_dma)(struct cs5535audio,
    pub cs5535au): *mut *mut void (disable_dma)(struct cs5535audio,
    pub cs5535au): *mut *mut void (pause_dma)(struct cs5535audio,
    pub prd_addr): *mut *mut *mut void (setup_prd)(struct cs5535audio cs5535au, u32,
    pub cs5535au): *mut *mut u32 (read_prd)(struct cs5535audio,
    pub cs5535au): *mut *mut u32 (read_dma_pntr)(struct cs5535audio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs5535audio_dma_desc {
    pub addr: __le32,
    pub size: __le16,
    pub ctlreserved: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs5535audio_dma {
    pub ops: *const cs5535audio_dma_ops,
    pub desc_buf: snd_dma_buffer,
    pub substream: *mut snd_pcm_substream,
    pub buf_bytes: unsigned int buf_addr,,
    pub periods: unsigned int period_bytes,,
    pub saved_prd: u32,
    pub pcm_open_flag: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs5535audio {
    pub card: *mut snd_card,
    pub ac97: *mut snd_ac97,
    pub pcm: *mut snd_pcm,
    pub irq: c_int,
    pub pci: *mut pci_dev,
    pub port: c_ulong,
    pub reg_lock: spinlock_t,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub dmas: [cs5535audio_dma; NUM_CS5535AUDIO_DMAS],
}

extern "C" {
    pub fn olpc_quirks(card: *mut snd_card, ac97: *mut snd_ac97) -> c_int;
}
extern "C" {
    pub fn olpc_quirks_cleanup();
}
extern "C" {
    pub fn olpc_analog_input(ac97: *mut snd_ac97, on: c_int);
}
extern "C" {
    pub fn olpc_mic_bias(ac97: *mut snd_ac97, on: c_int);
}
// default to Analog Input off
// enable MIC Bias for recording
// disable Analog Input
// disable the MIC Bias (so the recording LED turns off)

extern "C" {
    pub fn snd_cs5535audio_pcm(cs5535audio: *mut cs5535audio) -> c_int;
}
