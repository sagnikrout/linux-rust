//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/serio.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 1999-2002 Vojtech Pavlik
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//

//
// bit masks for use in "interrupt" flags (3rd argument)
//

//
// Serio types
//
pub const SERIO_XT: c_uint = 0x00;
pub const SERIO_8042: c_uint = 0x01;
pub const SERIO_RS232: c_uint = 0x02;
pub const SERIO_HIL_MLC: c_uint = 0x03;
pub const SERIO_PS_PSTHRU: c_uint = 0x05;
pub const SERIO_8042_XL: c_uint = 0x06;
//
// Serio protocols
//
pub const SERIO_UNKNOWN: c_uint = 0x00;
pub const SERIO_MSC: c_uint = 0x01;
pub const SERIO_SUN: c_uint = 0x02;
pub const SERIO_MS: c_uint = 0x03;
pub const SERIO_MP: c_uint = 0x04;
pub const SERIO_MZ: c_uint = 0x05;
pub const SERIO_MZP: c_uint = 0x06;
pub const SERIO_MZPP: c_uint = 0x07;
pub const SERIO_VSXXXAA: c_uint = 0x08;
pub const SERIO_SUNKBD: c_uint = 0x10;
pub const SERIO_WARRIOR: c_uint = 0x18;
pub const SERIO_SPACEORB: c_uint = 0x19;
pub const SERIO_MAGELLAN: c_uint = 0x1a;
pub const SERIO_SPACEBALL: c_uint = 0x1b;
pub const SERIO_GUNZE: c_uint = 0x1c;
pub const SERIO_IFORCE: c_uint = 0x1d;
pub const SERIO_STINGER: c_uint = 0x1e;
pub const SERIO_NEWTON: c_uint = 0x1f;
pub const SERIO_STOWAWAY: c_uint = 0x20;
pub const SERIO_H3600: c_uint = 0x21;
pub const SERIO_PS2SER: c_uint = 0x22;
pub const SERIO_TWIDKBD: c_uint = 0x23;
pub const SERIO_TWIDJOY: c_uint = 0x24;
pub const SERIO_HIL: c_uint = 0x25;
pub const SERIO_SNES232: c_uint = 0x26;
pub const SERIO_SEMTECH: c_uint = 0x27;
pub const SERIO_LKKBD: c_uint = 0x28;
pub const SERIO_ELO: c_uint = 0x29;
pub const SERIO_MICROTOUCH: c_uint = 0x30;
pub const SERIO_PENMOUNT: c_uint = 0x31;
pub const SERIO_TOUCHRIGHT: c_uint = 0x32;
pub const SERIO_TOUCHWIN: c_uint = 0x33;
pub const SERIO_TAOSEVM: c_uint = 0x34;
pub const SERIO_FUJITSU: c_uint = 0x35;
pub const SERIO_ZHENHUA: c_uint = 0x36;
pub const SERIO_INEXIO: c_uint = 0x37;
pub const SERIO_TOUCHIT213: c_uint = 0x38;
pub const SERIO_W8001: c_uint = 0x39;
pub const SERIO_DYNAPRO: c_uint = 0x3a;
pub const SERIO_HAMPSHIRE: c_uint = 0x3b;
pub const SERIO_PS2MULT: c_uint = 0x3c;
pub const SERIO_TSC40: c_uint = 0x3d;
pub const SERIO_WACOM_IV: c_uint = 0x3e;
pub const SERIO_EGALAX: c_uint = 0x3f;
pub const SERIO_PULSE8_CEC: c_uint = 0x40;
pub const SERIO_RAINSHADOW_CEC: c_uint = 0x41;
pub const SERIO_FSIA6B: c_uint = 0x42;
pub const SERIO_EXTRON_DA_HD_4K_PLUS: c_uint = 0x43;
