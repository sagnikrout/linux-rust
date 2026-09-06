//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ak4113.h
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
// Routines for Asahi Kasei AK4113
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// Copyright (c) by Pavel Hofman <pavel.hofman@ivitera.com>,
//
// AK4113 registers
// power down
pub const AK4113_REG_PWRDN: c_uint = 0x00;
// format control
pub const AK4113_REG_FORMAT: c_uint = 0x01;
// input/output control
pub const AK4113_REG_IO0: c_uint = 0x02;
// input/output control
pub const AK4113_REG_IO1: c_uint = 0x03;
// interrupt0 mask
pub const AK4113_REG_INT0_MASK: c_uint = 0x04;
// interrupt1 mask
pub const AK4113_REG_INT1_MASK: c_uint = 0x05;
// DAT mask & DTS select
pub const AK4113_REG_DATDTS: c_uint = 0x06;
// receiver status 0
pub const AK4113_REG_RCS0: c_uint = 0x07;
// receiver status 1
pub const AK4113_REG_RCS1: c_uint = 0x08;
// receiver status 2
pub const AK4113_REG_RCS2: c_uint = 0x09;
// RX channel status byte 0
pub const AK4113_REG_RXCSB0: c_uint = 0x0a;
// RX channel status byte 1
pub const AK4113_REG_RXCSB1: c_uint = 0x0b;
// RX channel status byte 2
pub const AK4113_REG_RXCSB2: c_uint = 0x0c;
// RX channel status byte 3
pub const AK4113_REG_RXCSB3: c_uint = 0x0d;
// RX channel status byte 4
pub const AK4113_REG_RXCSB4: c_uint = 0x0e;
// burst preamble Pc byte 0
pub const AK4113_REG_Pc0: c_uint = 0x0f;
// burst preamble Pc byte 1
pub const AK4113_REG_Pc1: c_uint = 0x10;
// burst preamble Pd byte 0
pub const AK4113_REG_Pd0: c_uint = 0x11;
// burst preamble Pd byte 1
pub const AK4113_REG_Pd1: c_uint = 0x12;
// Q-subcode address + control
pub const AK4113_REG_QSUB_ADDR: c_uint = 0x13;
// Q-subcode track
pub const AK4113_REG_QSUB_TRACK: c_uint = 0x14;
// Q-subcode index
pub const AK4113_REG_QSUB_INDEX: c_uint = 0x15;
// Q-subcode minute
pub const AK4113_REG_QSUB_MINUTE: c_uint = 0x16;
// Q-subcode second
pub const AK4113_REG_QSUB_SECOND: c_uint = 0x17;
// Q-subcode frame
pub const AK4113_REG_QSUB_FRAME: c_uint = 0x18;
// Q-subcode zero
pub const AK4113_REG_QSUB_ZERO: c_uint = 0x19;
// Q-subcode absolute minute
pub const AK4113_REG_QSUB_ABSMIN: c_uint = 0x1a;
// Q-subcode absolute second
pub const AK4113_REG_QSUB_ABSSEC: c_uint = 0x1b;
// Q-subcode absolute frame
pub const AK4113_REG_QSUB_ABSFRM: c_uint = 0x1c;
// sizes

// AK4113_REG_PWRDN bits
// Channel Status Select

// Block Start & C/U Output Mode

// Master Clock Operation Select

// Master Clock Operation Select

// Master Clock Frequency Select

// Master Clock Frequency Select

// 0 = power down, 1 = normal operation

// 0 = reset & initialize (except thisregister), 1 = normal operation

// AK4113_REQ_FORMAT bits
// V/TX Output select: 0 = Validity Flag Output, 1 = TX

// Audio Data Control

// Audio Data Control

// Audio Data Control

// Deemphasis Autodetect Enable (1 = enable)

// 32kHz-48kHz Deemphasis Control

// 32kHz-48kHz Deemphasis Control

// STDO: 16-bit, right justified

// STDO: 18-bit, right justified

// STDO: 20-bit, right justified

// STDO: 24-bit, right justified

// STDO: 24-bit, left justified

// STDO: I2S

// STDO: 24-bit, left justified; LRCLK, BICK = Input

// STDO: I2S;  LRCLK, BICK = Input

// AK4113_REG_IO0
// XTL1=0,XTL0=0 -> 11.2896Mhz; XTL1=0,XTL0=1 -> 12.288Mhz

// XTL1=1,XTL0=0 -> 24.576Mhz; XTL1=1,XTL0=1 -> use channel status

// Block Start Signal Output: 0 = U-bit, 1 = C-bit (req. BCU = 1)

// TX Output Enable (1 = enable)

// Output Through Data Selector for TX pin

// Output Through Data Selector for TX pin

// Output Through Data Selector for TX pin

// 11.2896 MHz ref. Xtal freq.

// 12.288 MHz ref. Xtal freq.

// 24.576 MHz ref. Xtal freq.

// AK4113_REG_IO1
// Interrupt 0 pin Hold

// Interrupt 0 pin Hold

// PLL Lock Time: 0 = 384/fs, 1 = 1/fs

// MCKO2 Output Select: 0 = CMx/OCKSx, 1 = Xtal

// MCKO2 Output Freq. Select: 0 = x1, 1 = x0.5  (req. XMCK = 1)

// Input Recovery Data Select

// Input Recovery Data Select

// Input Recovery Data Select

// AK4113_REG_INT0_MASK && AK4113_REG_INT1_MASK
// mask enable for QINT bit

// mask enable for AUTO bit

// mask enable for CINT bit

// mask enable for UNLOCK bit

// mask enable for V bit

// mask enable for STC bit

// mask enable for AUDN bit

// mask enable for PAR bit

// AK4113_REG_DATDTS
// DAT Start ID Counter

// DTS-CD 16-bit Sync Word Detect

// DTS-CD 14-bit Sync Word Detect

// mask enable for DAT bit (if 1, no INT1 effect

// mask enable for DAT bit (if 1, no INT0 effect

// AK4113_REG_RCS0
// Q-subcode buffer interrupt, 0 = no change, 1 = changed

// Non-PCM or DTS stream auto detection, 0 = no detect, 1 = detect

// channel status buffer interrupt, 0 = no change, 1 = change

// PLL lock status, 0 = lock, 1 = unlock

// Validity bit, 0 = valid, 1 = invalid

// sampling frequency or Pre-emphasis change, 0 = no detect, 1 = detect

// audio bit output, 0 = audio, 1 = non-audio

// parity error or biphase error status, 0 = no error, 1 = error

// AK4113_REG_RCS1
// sampling frequency detection

// Pre-emphasis detect, 0 = OFF, 1 = ON

// DAT Start ID Detect, 0 = no detect, 1 = detect

// DTS-CD bit audio stream detect, 0 = no detect, 1 = detect

// Non-PCM bit stream detection, 0 = no detect, 1 = detect

// AK4113_REG_RCS2
// CRC for Q-subcode, 0 = no error, 1 = error

// CRC for channel status, 0 = no error, 1 = error

// flags for snd_ak4113_check_rate_and_errors()

pub const AK4113_CONTROLS: c_int = 13;
extern "C" {
    pub fn char(private_data: *mut ak4113_read_t)(void, addr: c_uchar) -> typedef unsigned;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ak4113 {
    pub card: *mut snd_card,
    pub write: *mut ak4113_write_t,
    pub read: *mut ak4113_read_t,
    pub private_data: *mut c_void,
    pub wq_processing: core::sync::atomic::AtomicI32,
    pub reinit_mutex: mutex,
    pub lock: spinlock_t,
    pub regmap: [c_uchar; AK4113_WRITABLE_REGS],
    pub kctls: [*mut snd_kcontrol; AK4113_CONTROLS],
    pub substream: *mut snd_pcm_substream,
    pub errors: [c_ulong; AK4113_NUM_ERRORS],
    pub rcs0: c_uchar,
    pub rcs1: c_uchar,
    pub rcs2: c_uchar,
    pub work: delayed_work,
    pub check_flags: c_uint,
    pub change_callback_private: *mut c_void,
    pub c1): c_uchar,
}

extern "C" {
    pub fn snd_ak4113_reinit(ak4113: *mut ak4113);
}
extern "C" {
    pub fn snd_ak4113_external_rate(ak4113: *mut ak4113) -> c_int;
}
extern "C" {
    pub fn snd_ak4113_check_rate_and_errors(ak4113: *mut ak4113, flags: c_uint) -> c_int;
}

extern "C" {
    pub fn snd_ak4113_suspend(chip: *mut ak4113);
}
extern "C" {
    pub fn snd_ak4113_resume(chip: *mut ak4113);
}

