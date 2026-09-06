//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ak4114.h
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
// Routines for Asahi Kasei AK4114
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
//
// AK4114 registers
pub const AK4114_REG_PWRDN: c_uint = 0x00	/* power down */;
pub const AK4114_REG_FORMAT: c_uint = 0x01	/* format control */;
pub const AK4114_REG_IO0: c_uint = 0x02	/* input/output control */;
pub const AK4114_REG_IO1: c_uint = 0x03	/* input/output control */;
pub const AK4114_REG_INT0_MASK: c_uint = 0x04	/* interrupt0 mask */;
pub const AK4114_REG_INT1_MASK: c_uint = 0x05	/* interrupt1 mask */;
pub const AK4114_REG_RCS0: c_uint = 0x06	/* receiver status 0 */;
pub const AK4114_REG_RCS1: c_uint = 0x07	/* receiver status 1 */;
pub const AK4114_REG_RXCSB0: c_uint = 0x08	/* RX channel status byte 0 */;
pub const AK4114_REG_RXCSB1: c_uint = 0x09	/* RX channel status byte 1 */;
pub const AK4114_REG_RXCSB2: c_uint = 0x0a	/* RX channel status byte 2 */;
pub const AK4114_REG_RXCSB3: c_uint = 0x0b	/* RX channel status byte 3 */;
pub const AK4114_REG_RXCSB4: c_uint = 0x0c	/* RX channel status byte 4 */;
pub const AK4114_REG_TXCSB0: c_uint = 0x0d	/* TX channel status byte 0 */;
pub const AK4114_REG_TXCSB1: c_uint = 0x0e	/* TX channel status byte 1 */;
pub const AK4114_REG_TXCSB2: c_uint = 0x0f	/* TX channel status byte 2 */;
pub const AK4114_REG_TXCSB3: c_uint = 0x10	/* TX channel status byte 3 */;
pub const AK4114_REG_TXCSB4: c_uint = 0x11	/* TX channel status byte 4 */;
pub const AK4114_REG_Pc0: c_uint = 0x12	/* burst preamble Pc byte 0 */;
pub const AK4114_REG_Pc1: c_uint = 0x13	/* burst preamble Pc byte 1 */;
pub const AK4114_REG_Pd0: c_uint = 0x14	/* burst preamble Pd byte 0 */;
pub const AK4114_REG_Pd1: c_uint = 0x15	/* burst preamble Pd byte 1 */;
pub const AK4114_REG_QSUB_ADDR: c_uint = 0x16	/* Q-subcode address + control */;
pub const AK4114_REG_QSUB_TRACK: c_uint = 0x17	/* Q-subcode track */;
pub const AK4114_REG_QSUB_INDEX: c_uint = 0x18	/* Q-subcode index */;
pub const AK4114_REG_QSUB_MINUTE: c_uint = 0x19	/* Q-subcode minute */;
pub const AK4114_REG_QSUB_SECOND: c_uint = 0x1a	/* Q-subcode second */;
pub const AK4114_REG_QSUB_FRAME: c_uint = 0x1b	/* Q-subcode frame */;
pub const AK4114_REG_QSUB_ZERO: c_uint = 0x1c	/* Q-subcode zero */;
pub const AK4114_REG_QSUB_ABSMIN: c_uint = 0x1d	/* Q-subcode absolute minute */;
pub const AK4114_REG_QSUB_ABSSEC: c_uint = 0x1e	/* Q-subcode absolute second */;
pub const AK4114_REG_QSUB_ABSFRM: c_uint = 0x1f	/* Q-subcode absolute frame */;
// sizes

// AK4117_REG_PWRDN bits

// AK4114_REQ_FORMAT bits

// AK4114_REG_IO0

// AK4114_REG_IO1

// AK4114_REG_INT0_MASK && AK4114_REG_INT1_MASK

// AK4114_REG_RCS0

// AK4114_REG_RCS1

// flags for snd_ak4114_check_rate_and_errors()

pub const AK4114_CONTROLS: c_int = 15;
extern "C" {
    pub fn void(private_data: *mut ak4114_write_t)(void, addr: c_uchar, data: c_uchar) -> typedef;
}
extern "C" {
    pub fn char(private_data: *mut ak4114_read_t)(void, addr: c_uchar) -> typedef unsigned;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4114 {
    pub card: *mut snd_card,
    pub write: *mut *mut ak4114_write_t,
    pub read: *mut *mut ak4114_read_t,
    pub private_data: *mut *mut c_void,
    pub wq_processing: core::sync::atomic::AtomicI32,
    pub reinit_mutex: mutex,
    pub lock: spinlock_t,
    pub regmap: [c_uchar; 6],
    pub txcsb: [c_uchar; 5],
    pub kctls: [*mut snd_kcontrol; AK4114_CONTROLS],
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub errors: [c_ulong; AK4114_NUM_ERRORS],
    pub rcs0: c_uchar,
    pub rcs1: c_uchar,
    pub work: delayed_work,
    pub check_flags: c_uint,
    pub change_callback_private: *mut c_void,
    pub c1): *mut *mut *mut void (change_callback)(struct ak4114 ak4114, unsigned char c0, unsigned char,
}

extern "C" {
    pub fn snd_ak4114_reg_write(ak4114: *mut ak4114, reg: c_uchar, mask: c_uchar, val: c_uchar);
}
extern "C" {
    pub fn snd_ak4114_reinit(ak4114: *mut ak4114);
}
extern "C" {
    pub fn snd_ak4114_external_rate(ak4114: *mut ak4114) -> c_int;
}
extern "C" {
    pub fn snd_ak4114_check_rate_and_errors(ak4114: *mut ak4114, flags: c_uint) -> c_int;
}

extern "C" {
    pub fn snd_ak4114_suspend(chip: *mut ak4114);
}
extern "C" {
    pub fn snd_ak4114_resume(chip: *mut ak4114);
}

