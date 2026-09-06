//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs8427.h
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
// Routines for Cirrus Logic CS8427
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
//

pub const CS8427_BASE_ADDR: c_uint = 0x10	/* base I2C address */;
pub const CS8427_REG_AUTOINC: c_uint = 0x80	/* flag - autoincrement */;
pub const CS8427_REG_CONTROL1: c_uint = 0x01;
pub const CS8427_REG_CONTROL2: c_uint = 0x02;
pub const CS8427_REG_DATAFLOW: c_uint = 0x03;
pub const CS8427_REG_CLOCKSOURCE: c_uint = 0x04;
pub const CS8427_REG_SERIALINPUT: c_uint = 0x05;
pub const CS8427_REG_SERIALOUTPUT: c_uint = 0x06;
pub const CS8427_REG_INT1STATUS: c_uint = 0x07;
pub const CS8427_REG_INT2STATUS: c_uint = 0x08;
pub const CS8427_REG_INT1MASK: c_uint = 0x09;
pub const CS8427_REG_INT1MODEMSB: c_uint = 0x0a;
pub const CS8427_REG_INT1MODELSB: c_uint = 0x0b;
pub const CS8427_REG_INT2MASK: c_uint = 0x0c;
pub const CS8427_REG_INT2MODEMSB: c_uint = 0x0d;
pub const CS8427_REG_INT2MODELSB: c_uint = 0x0e;
pub const CS8427_REG_RECVCSDATA: c_uint = 0x0f;
pub const CS8427_REG_RECVERRORS: c_uint = 0x10;
pub const CS8427_REG_RECVERRMASK: c_uint = 0x11;
pub const CS8427_REG_CSDATABUF: c_uint = 0x12;
pub const CS8427_REG_UDATABUF: c_uint = 0x13;
pub const CS8427_REG_QSUBCODE: c_uint = 0x14	/* 0x14-0x1d (10 bytes) */;
pub const CS8427_REG_OMCKRMCKRATIO: c_uint = 0x1e;
pub const CS8427_REG_CORU_DATABUF: c_uint = 0x20	/* 24 byte buffer area */;
pub const CS8427_REG_ID_AND_VER: c_uint = 0x7f;
// CS8427_REG_CONTROL1 bits

// CS8427_REQ_CONTROL2 bits

// CS8427_REG_DATAFLOW

// CS8427_REG_CLOCKSOURCE

// CS8427_REG_SERIALINPUT

// CS8427_REG_SERIALOUTPUT

// CS8427_REG_INT1STATUS

// CS8427_REG_INT2STATUS

// CS8427_REG_INT1MODEMSB && CS8427_REG_INT1MODELSB
// bits are defined in CS8427_REG_INT1STATUS
// CS8427_REG_INT2MODEMSB && CS8427_REG_INT2MODELSB
// bits are defined in CS8427_REG_INT2STATUS
pub const CS8427_INTMODERISINGMSB: c_int = 0;
pub const CS8427_INTMODERESINGLSB: c_int = 0;
pub const CS8427_INTMODEFALLINGMSB: c_int = 0;
pub const CS8427_INTMODEFALLINGLSB: c_int = 1;
pub const CS8427_INTMODELEVELMSB: c_int = 1;
pub const CS8427_INTMODELEVELLSB: c_int = 0;
// CS8427_REG_RECVCSDATA

pub const CS8427_AUXSHIFT: c_int = 4;

// CS8427_REG_RECVERRORS
// CS8427_REG_RECVERRMASK for CS8427_RERR

// CS8427_REG_CSDATABUF

// CS8427_REG_UDATABUF

// CS8427_REG_ID_AND_VER

pub const CS8427_IDSHIFT: c_int = 4;

pub const CS8427_VERSHIFT: c_int = 0;
pub const CS8427_VER8427A: c_uint = 0x71;
extern "C" {
    pub fn snd_cs8427_init(bus: *mut snd_i2c_bus, device: *mut snd_i2c_device) -> c_int;
}
extern "C" {
    pub fn snd_cs8427_iec958_active(cs8427: *mut snd_i2c_device, active: c_int) -> c_int;
}
extern "C" {
    pub fn snd_cs8427_iec958_pcm(cs8427: *mut snd_i2c_device, rate: c_uint) -> c_int;
}
