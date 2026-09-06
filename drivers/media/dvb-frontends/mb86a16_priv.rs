//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mb86a16_priv.h
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
pub const MB86A16_TSOUT: c_uint = 0x00;

pub const MB86A16_FEC: c_uint = 0x01;

pub const MB86A16_AGC: c_uint = 0x02;

pub const MB86A16_SRATE1: c_uint = 0x03;

pub const MB86A16_SRATE2: c_uint = 0x04;

pub const MB86A16_SRATE3: c_uint = 0x05;

pub const MB86A16_VITERBI: c_uint = 0x06;
pub const MB86A16_FRAMESYNC: c_uint = 0x07;
pub const MB86A16_CRLFILTCOEF1: c_uint = 0x08;
pub const MB86A16_CRLFILTCOEF2: c_uint = 0x09;
pub const MB86A16_STRFILTCOEF1: c_uint = 0x0a;
pub const MB86A16_STRFILTCOEF2: c_uint = 0x0b;
pub const MB86A16_RESET: c_uint = 0x0c;
pub const MB86A16_STATUS: c_uint = 0x0d;
pub const MB86A16_AFCML: c_uint = 0x0e;
pub const MB86A16_AFCMH: c_uint = 0x0f;
pub const MB86A16_BERMON: c_uint = 0x10;
pub const MB86A16_BERTAB: c_uint = 0x11;
pub const MB86A16_BERLSB: c_uint = 0x12;
pub const MB86A16_BERMID: c_uint = 0x13;
pub const MB86A16_BERMSB: c_uint = 0x14;
pub const MB86A16_AGCM: c_uint = 0x15;
pub const MB86A16_DCC1: c_uint = 0x16;

pub const MB86A16_DCC2: c_uint = 0x17;

pub const MB86A16_DCC3: c_uint = 0x18;

pub const MB86A16_DCC4: c_uint = 0x19;

pub const MB86A16_DCC5: c_uint = 0x1a;

pub const MB86A16_DCC6: c_uint = 0x1b;

pub const MB86A16_DCC7: c_uint = 0x1c;

pub const MB86A16_DCC8: c_uint = 0x1d;

pub const MB86A16_DCCOUT: c_uint = 0x1e;

pub const MB86A16_TONEOUT1: c_uint = 0x1f;

pub const MB86A16_TONEOUT2: c_uint = 0x20;

pub const MB86A16_FREQ1: c_uint = 0x21;
pub const MB86A16_FREQ2: c_uint = 0x22;
pub const MB86A16_FREQ3: c_uint = 0x23;
pub const MB86A16_FREQ4: c_uint = 0x24;
pub const MB86A16_FREQSET: c_uint = 0x25;
pub const MB86A16_CNM: c_uint = 0x26;
pub const MB86A16_PORT0: c_uint = 0x27;
pub const MB86A16_PORT1: c_uint = 0x28;
pub const MB86A16_DRCFILT: c_uint = 0x29;
pub const MB86A16_AFC: c_uint = 0x2a;
pub const MB86A16_AFCEXL: c_uint = 0x2b;
pub const MB86A16_AFCEXH: c_uint = 0x2c;
pub const MB86A16_DAGC: c_uint = 0x2d;
pub const MB86A16_SEQMODE: c_uint = 0x32;
pub const MB86A16_S0S1T: c_uint = 0x33;
pub const MB86A16_S2S3T: c_uint = 0x34;
pub const MB86A16_S4S5T: c_uint = 0x35;
pub const MB86A16_CNTMR: c_uint = 0x36;
pub const MB86A16_SIG1: c_uint = 0x37;
pub const MB86A16_SIG2: c_uint = 0x38;
pub const MB86A16_VIMAG: c_uint = 0x39;
pub const MB86A16_VISET1: c_uint = 0x3a;
pub const MB86A16_VISET2: c_uint = 0x3b;
pub const MB86A16_VISET3: c_uint = 0x3c;
pub const MB86A16_FAGCS1: c_uint = 0x3d;
pub const MB86A16_FAGCS2: c_uint = 0x3e;
pub const MB86A16_FAGCS3: c_uint = 0x3f;
pub const MB86A16_FAGCS4: c_uint = 0x40;
pub const MB86A16_FAGCS5: c_uint = 0x41;
pub const MB86A16_FAGCS6: c_uint = 0x42;
pub const MB86A16_CRM: c_uint = 0x43;
pub const MB86A16_STRM: c_uint = 0x44;
pub const MB86A16_DAGCML: c_uint = 0x45;
pub const MB86A16_DAGCMH: c_uint = 0x46;
pub const MB86A16_QPSKTST: c_uint = 0x49;
pub const MB86A16_DISTMON: c_uint = 0x52;
pub const MB86A16_VERSION: c_uint = 0x7f;
