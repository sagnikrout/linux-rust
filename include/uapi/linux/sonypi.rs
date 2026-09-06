//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sonypi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Sony Programmable I/O Control Device driver for VAIO
//
// Copyright (C) 2001-2005 Stelian Pop <stelian@popies.net>
//
// Copyright (C) 2005 Narayanan R S <nars@kadamba.org>
// Copyright (C) 2001-2002 Alcôve <www.alcove.com>
//
// Copyright (C) 2001 Michael Ashley <m.ashley@unsw.edu.au>
//
// Copyright (C) 2001 Junichi Morita <jun1m@mars.dti.ne.jp>
//
// Copyright (C) 2000 Takaya Kinjo <t-kinjo@tc4.so-net.ne.jp>
//
// Copyright (C) 2000 Andrew Tridgell <tridge@valinux.com>
//
// Earlier work by Werner Almesberger, Paul `Rusty' Russell and Paul Mackerras.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//

// events the user application reading /dev/sonypi can use
pub const SONYPI_EVENT_IGNORE: c_int = 0;
pub const SONYPI_EVENT_JOGDIAL_DOWN: c_int = 1;
pub const SONYPI_EVENT_JOGDIAL_UP: c_int = 2;
pub const SONYPI_EVENT_JOGDIAL_DOWN_PRESSED: c_int = 3;
pub const SONYPI_EVENT_JOGDIAL_UP_PRESSED: c_int = 4;
pub const SONYPI_EVENT_JOGDIAL_PRESSED: c_int = 5;

pub const SONYPI_EVENT_CAPTURE_PRESSED: c_int = 7;

pub const SONYPI_EVENT_CAPTURE_PARTIALPRESSED: c_int = 9;
pub const SONYPI_EVENT_CAPTURE_PARTIALRELEASED: c_int = 10;
pub const SONYPI_EVENT_FNKEY_ESC: c_int = 11;
pub const SONYPI_EVENT_FNKEY_F1: c_int = 12;
pub const SONYPI_EVENT_FNKEY_F2: c_int = 13;
pub const SONYPI_EVENT_FNKEY_F3: c_int = 14;
pub const SONYPI_EVENT_FNKEY_F4: c_int = 15;
pub const SONYPI_EVENT_FNKEY_F5: c_int = 16;
pub const SONYPI_EVENT_FNKEY_F6: c_int = 17;
pub const SONYPI_EVENT_FNKEY_F7: c_int = 18;
pub const SONYPI_EVENT_FNKEY_F8: c_int = 19;
pub const SONYPI_EVENT_FNKEY_F9: c_int = 20;
pub const SONYPI_EVENT_FNKEY_F10: c_int = 21;
pub const SONYPI_EVENT_FNKEY_F11: c_int = 22;
pub const SONYPI_EVENT_FNKEY_F12: c_int = 23;
pub const SONYPI_EVENT_FNKEY_1: c_int = 24;
pub const SONYPI_EVENT_FNKEY_2: c_int = 25;
pub const SONYPI_EVENT_FNKEY_D: c_int = 26;
pub const SONYPI_EVENT_FNKEY_E: c_int = 27;
pub const SONYPI_EVENT_FNKEY_F: c_int = 28;
pub const SONYPI_EVENT_FNKEY_S: c_int = 29;
pub const SONYPI_EVENT_FNKEY_B: c_int = 30;
pub const SONYPI_EVENT_BLUETOOTH_PRESSED: c_int = 31;
pub const SONYPI_EVENT_PKEY_P1: c_int = 32;
pub const SONYPI_EVENT_PKEY_P2: c_int = 33;
pub const SONYPI_EVENT_PKEY_P3: c_int = 34;
pub const SONYPI_EVENT_BACK_PRESSED: c_int = 35;
pub const SONYPI_EVENT_LID_CLOSED: c_int = 36;
pub const SONYPI_EVENT_LID_OPENED: c_int = 37;
pub const SONYPI_EVENT_BLUETOOTH_ON: c_int = 38;
pub const SONYPI_EVENT_BLUETOOTH_OFF: c_int = 39;
pub const SONYPI_EVENT_HELP_PRESSED: c_int = 40;
pub const SONYPI_EVENT_FNKEY_ONLY: c_int = 41;
pub const SONYPI_EVENT_JOGDIAL_FAST_DOWN: c_int = 42;
pub const SONYPI_EVENT_JOGDIAL_FAST_UP: c_int = 43;
pub const SONYPI_EVENT_JOGDIAL_FAST_DOWN_PRESSED: c_int = 44;
pub const SONYPI_EVENT_JOGDIAL_FAST_UP_PRESSED: c_int = 45;
pub const SONYPI_EVENT_JOGDIAL_VFAST_DOWN: c_int = 46;
pub const SONYPI_EVENT_JOGDIAL_VFAST_UP: c_int = 47;
pub const SONYPI_EVENT_JOGDIAL_VFAST_DOWN_PRESSED: c_int = 48;
pub const SONYPI_EVENT_JOGDIAL_VFAST_UP_PRESSED: c_int = 49;
pub const SONYPI_EVENT_ZOOM_PRESSED: c_int = 50;
pub const SONYPI_EVENT_THUMBPHRASE_PRESSED: c_int = 51;
pub const SONYPI_EVENT_MEYE_FACE: c_int = 52;
pub const SONYPI_EVENT_MEYE_OPPOSITE: c_int = 53;
pub const SONYPI_EVENT_MEMORYSTICK_INSERT: c_int = 54;
pub const SONYPI_EVENT_MEMORYSTICK_EJECT: c_int = 55;
pub const SONYPI_EVENT_ANYBUTTON_RELEASED: c_int = 56;
pub const SONYPI_EVENT_BATTERY_INSERT: c_int = 57;
pub const SONYPI_EVENT_BATTERY_REMOVE: c_int = 58;
pub const SONYPI_EVENT_FNKEY_RELEASED: c_int = 59;
pub const SONYPI_EVENT_WIRELESS_ON: c_int = 60;
pub const SONYPI_EVENT_WIRELESS_OFF: c_int = 61;
pub const SONYPI_EVENT_ZOOM_IN_PRESSED: c_int = 62;
pub const SONYPI_EVENT_ZOOM_OUT_PRESSED: c_int = 63;
pub const SONYPI_EVENT_CD_EJECT_PRESSED: c_int = 64;
pub const SONYPI_EVENT_MODEKEY_PRESSED: c_int = 65;
pub const SONYPI_EVENT_PKEY_P4: c_int = 66;
pub const SONYPI_EVENT_PKEY_P5: c_int = 67;
pub const SONYPI_EVENT_SETTINGKEY_PRESSED: c_int = 68;
pub const SONYPI_EVENT_VOLUME_INC_PRESSED: c_int = 69;
pub const SONYPI_EVENT_VOLUME_DEC_PRESSED: c_int = 70;
pub const SONYPI_EVENT_BRIGHTNESS_PRESSED: c_int = 71;
pub const SONYPI_EVENT_MEDIA_PRESSED: c_int = 72;
pub const SONYPI_EVENT_VENDOR_PRESSED: c_int = 73;
// get/set brightness

// get battery full capacity/remaining capacity

// get battery flags: battery1/battery2/ac adapter present
pub const SONYPI_BFLAGS_B1: c_uint = 0x01;
pub const SONYPI_BFLAGS_B2: c_uint = 0x02;
pub const SONYPI_BFLAGS_AC: c_uint = 0x04;

// get/set bluetooth subsystem state on/off

// get/set fan state on/off

// get temperature (C)

