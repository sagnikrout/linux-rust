//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ak4117.h
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
// Routines for Asahi Kasei AK4117
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
//
pub const AK4117_REG_PWRDN: c_uint = 0x00	/* power down */;
pub const AK4117_REG_CLOCK: c_uint = 0x01	/* clock control */;
pub const AK4117_REG_IO: c_uint = 0x02	/* input/output control */;
pub const AK4117_REG_INT0_MASK: c_uint = 0x03	/* interrupt0 mask */;
pub const AK4117_REG_INT1_MASK: c_uint = 0x04	/* interrupt1 mask */;
pub const AK4117_REG_RCS0: c_uint = 0x05	/* receiver status 0 */;
pub const AK4117_REG_RCS1: c_uint = 0x06	/* receiver status 1 */;
pub const AK4117_REG_RCS2: c_uint = 0x07	/* receiver status 2 */;
pub const AK4117_REG_RXCSB0: c_uint = 0x08	/* RX channel status byte 0 */;
pub const AK4117_REG_RXCSB1: c_uint = 0x09	/* RX channel status byte 1 */;
pub const AK4117_REG_RXCSB2: c_uint = 0x0a	/* RX channel status byte 2 */;
pub const AK4117_REG_RXCSB3: c_uint = 0x0b	/* RX channel status byte 3 */;
pub const AK4117_REG_RXCSB4: c_uint = 0x0c	/* RX channel status byte 4 */;
pub const AK4117_REG_Pc0: c_uint = 0x0d	/* burst preamble Pc byte 0 */;
pub const AK4117_REG_Pc1: c_uint = 0x0e	/* burst preamble Pc byte 1 */;
pub const AK4117_REG_Pd0: c_uint = 0x0f	/* burst preamble Pd byte 0 */;
pub const AK4117_REG_Pd1: c_uint = 0x10	/* burst preamble Pd byte 1 */;
pub const AK4117_REG_QSUB_ADDR: c_uint = 0x11	/* Q-subcode address + control */;
pub const AK4117_REG_QSUB_TRACK: c_uint = 0x12	/* Q-subcode track */;
pub const AK4117_REG_QSUB_INDEX: c_uint = 0x13	/* Q-subcode index */;
pub const AK4117_REG_QSUB_MINUTE: c_uint = 0x14	/* Q-subcode minute */;
pub const AK4117_REG_QSUB_SECOND: c_uint = 0x15	/* Q-subcode second */;
pub const AK4117_REG_QSUB_FRAME: c_uint = 0x16	/* Q-subcode frame */;
pub const AK4117_REG_QSUB_ZERO: c_uint = 0x17	/* Q-subcode zero */;
pub const AK4117_REG_QSUB_ABSMIN: c_uint = 0x18	/* Q-subcode absolute minute */;
pub const AK4117_REG_QSUB_ABSSEC: c_uint = 0x19	/* Q-subcode absolute second */;
pub const AK4117_REG_QSUB_ABSFRM: c_uint = 0x1a	/* Q-subcode absolute frame */;
// sizes

// AK4117_REG_PWRDN bits

// AK4117_REQ_CLOCK bits

// AK4117_REG_IO

// AK4117_REG_INT0_MASK & AK4117_REG_INT1_MASK

// AK4117_REG_RCS0

// AK4117_REG_RCS1

// AK4117_REG_RCS2

// flags for snd_ak4117_check_rate_and_errors()

pub const AK4117_CONTROLS: c_int = 13;
extern "C" {
    pub fn void(private_data: *mut ak4117_write_t)(void, addr: c_uchar, data: c_uchar) -> typedef;
}
extern "C" {
    pub fn char(private_data: *mut ak4117_read_t)(void, addr: c_uchar) -> typedef unsigned;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4117 {
    pub card: *mut snd_card,
    pub write: *mut *mut ak4117_write_t,
    pub read: *mut *mut ak4117_read_t,
    pub private_data: *mut *mut c_void,
    pub 1: unsigned int init:,
    pub lock: spinlock_t,
    pub regmap: [c_uchar; 5],
    pub kctls: [*mut snd_kcontrol; AK4117_CONTROLS],
    pub substream: *mut snd_pcm_substream,
    pub errors: [c_ulong; AK4117_NUM_ERRORS],
    pub rcs0: c_uchar,
    pub rcs1: c_uchar,
    pub rcs2: c_uchar,
    pub /: *mut *mut timer_list timer; / statistic timer,
    pub change_callback_private: *mut c_void,
    pub c1): *mut *mut *mut void (change_callback)(struct ak4117 ak4117, unsigned char c0, unsigned char,
}

extern "C" {
    pub fn snd_ak4117_reg_write(ak4117: *mut ak4117, reg: c_uchar, mask: c_uchar, val: c_uchar);
}
extern "C" {
    pub fn snd_ak4117_reinit(ak4117: *mut ak4117);
}
extern "C" {
    pub fn snd_ak4117_build(ak4117: *mut ak4117, capture_substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_ak4117_external_rate(ak4117: *mut ak4117) -> c_int;
}
extern "C" {
    pub fn snd_ak4117_check_rate_and_errors(ak4117: *mut ak4117, flags: c_uint) -> c_int;
}
