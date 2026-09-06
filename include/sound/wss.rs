//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wss.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Definitions for CS4231 & InterWave chips & compatible chips
//

// defines for codec.mode
pub const WSS_MODE_NONE: c_uint = 0x0000;
pub const WSS_MODE_PLAY: c_uint = 0x0001;
pub const WSS_MODE_RECORD: c_uint = 0x0002;
pub const WSS_MODE_TIMER: c_uint = 0x0004;

// defines for codec.hardware
pub const WSS_HW_DETECT: c_uint = 0x0000	/* let CS4231 driver detect chip */;
pub const WSS_HW_DETECT3: c_uint = 0x0001	/* allow mode 3 */;
pub const WSS_HW_TYPE_MASK: c_uint = 0xff00	/* type mask */;
pub const WSS_HW_CS4231_MASK: c_uint = 0x0100	/* CS4231 serie */;
pub const WSS_HW_CS4231: c_uint = 0x0100	/* CS4231 chip */;
pub const WSS_HW_CS4231A: c_uint = 0x0101	/* CS4231A chip */;
pub const WSS_HW_AD1845: c_uint = 0x0102	/* AD1845 chip */;
pub const WSS_HW_CS4232_MASK: c_uint = 0x0200	/* CS4232 serie (has control ports) */;
pub const WSS_HW_CS4232: c_uint = 0x0200	/* CS4232 */;
pub const WSS_HW_CS4232A: c_uint = 0x0201	/* CS4232A */;
pub const WSS_HW_CS4236: c_uint = 0x0202	/* CS4236 */;
pub const WSS_HW_CS4236B_MASK: c_uint = 0x0400	/* CS4236B serie (has extended control regs) */;
pub const WSS_HW_CS4235: c_uint = 0x0400	/* CS4235 - Crystal Clear (tm) stereo enhancement */;
pub const WSS_HW_CS4236B: c_uint = 0x0401	/* CS4236B */;
pub const WSS_HW_CS4237B: c_uint = 0x0402	/* CS4237B - SRS 3D */;
pub const WSS_HW_CS4238B: c_uint = 0x0403	/* CS4238B - QSOUND 3D */;
pub const WSS_HW_CS4239: c_uint = 0x0404	/* CS4239 - Crystal Clear (tm) stereo enhancement */;
pub const WSS_HW_AD1848_MASK: c_uint = 0x0800	/* AD1848 serie (half duplex) */;
pub const WSS_HW_AD1847: c_uint = 0x0801	/* AD1847 chip */;
pub const WSS_HW_AD1848: c_uint = 0x0802	/* AD1848 chip */;
pub const WSS_HW_CS4248: c_uint = 0x0803	/* CS4248 chip */;
pub const WSS_HW_CMI8330: c_uint = 0x0804	/* CMI8330 chip */;
pub const WSS_HW_THINKPAD: c_uint = 0x0805	/* Thinkpad 360/750/755 */;
// compatible, but clones
pub const WSS_HW_INTERWAVE: c_uint = 0x1000	/* InterWave chip */;
pub const WSS_HW_OPL3SA2: c_uint = 0x1101	/* OPL3-SA2 chip, similar to cs4231 */;
pub const WSS_HW_OPTI93X: c_uint = 0x1102	/* Opti 930/931/933 */;
// defines for codec.hwshare

// IBM Thinkpad specific stuff
pub const AD1848_THINKPAD_CTL_PORT1: c_uint = 0x15e8;
pub const AD1848_THINKPAD_CTL_PORT2: c_uint = 0x15e9;
pub const AD1848_THINKPAD_CS4248_ENABLE_BIT: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wss {
    pub /: *mut *mut unsigned long port; / base i/o port,
    pub res_port: *mut resource,
    pub /: *mut *mut unsigned long cport; / control base i/o port (CS4236),
    pub res_cport: *mut resource,
    pub /: *mut *mut int irq; / IRQ line,
    pub /: *mut *mut int dma1; / playback DMA,
    pub /: *mut *mut int dma2; / record DMA,
    pub /: *mut *mut unsigned short version; / version of CODEC chip,
    pub /: *mut *mut unsigned short mode; / see to WSS_MODE_XXXX,
    pub /: *mut *mut unsigned short hardware; / see to WSS_HW_XXXX,
    pub /: *mut *mut unsigned short hwshare; / shared resources,
// daughter board) or dma1 == dma2
    pub /: *mut *mut thinkpad_flag:1; / Thinkpad CS4248 needs extra help,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub timer: *mut snd_timer,
    pub /: *mut *mut unsigned char image[32]; / registers image,
    pub /: *mut *mut unsigned char eimage[32]; / extended registers image,
    pub /: *mut *mut unsigned char cimage[16]; / control registers image,
    pub mce_bit: c_int,
    pub calibrate_mute: c_int,
    pub sw_3d_bit: c_int,
    pub p_dma_size: c_uint,
    pub c_dma_size: c_uint,
    pub reg_lock: spinlock_t,
    pub mce_mutex: mutex,
    pub open_mutex: mutex,
    pub runtime): *mut *mut int (rate_constraint) (struct snd_pcm_runtime,
    pub pdfr): c_uchar,
    pub cdfr): c_uchar,
    pub start): *mut *mut *mut void (trigger) (struct snd_wss chip, unsigned int what, int,

    pub chip): *mut *mut void (suspend) (struct snd_wss,
    pub chip): *mut *mut void (resume) (struct snd_wss,

    pub dma_private_data: *mut c_void,
    pub dma): *mut *mut void dma_private_data, int,
    pub dma): *mut *mut void dma_private_data, int,
}

// exported functions
extern "C" {
    pub fn snd_wss_out(chip: *mut snd_wss, reg: c_uchar, val: c_uchar);
}
extern "C" {
    pub fn snd_wss_in(chip: *mut snd_wss, reg: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn snd_cs4236_ext_in(chip: *mut snd_wss, reg: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn snd_wss_mce_up(chip: *mut snd_wss);
}
extern "C" {
    pub fn snd_wss_mce_down(chip: *mut snd_wss);
}
extern "C" {
    pub fn snd_wss_overrange(chip: *mut snd_wss);
}
extern "C" {
    pub fn snd_wss_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_wss_timer(chip: *mut snd_wss, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_wss_mixer(chip: *mut snd_wss) -> c_int;
}
extern "C" {
    pub fn snd_cs4236_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs4236_mixer(chip: *mut snd_wss) -> c_int;
}
//
// mixer library
//

