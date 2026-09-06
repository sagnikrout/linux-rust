//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs4349.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC CS4349 codec driver
//
// Copyright 2015 Cirrus Logic, Inc.
//
// Author: Tim Howe <Tim.Howe@cirrus.com>
//
// CS4349 registers addresses
pub const CS4349_CHIPID: c_uint = 0x01	/* Device and Rev ID, Read Only */;
pub const CS4349_MODE: c_uint = 0x02	/* Mode Control */;
pub const CS4349_VMI: c_uint = 0x03	/* Volume, Mixing, Inversion Control */;
pub const CS4349_MUTE: c_uint = 0x04	/* Mute Control */;
pub const CS4349_VOLA: c_uint = 0x05	/* DAC Channel A Volume Control */;
pub const CS4349_VOLB: c_uint = 0x06	/* DAC Channel B Volume Control */;
pub const CS4349_RMPFLT: c_uint = 0x07	/* Ramp and Filter Control */;
pub const CS4349_MISC: c_uint = 0x08	/* Power Down,Freeze Control,Pop Stop*/;
pub const CS4349_I2C_INCR: c_uint = 0x80;
// Device and Revision ID
pub const CS4349_REVA: c_uint = 0xF0	/* Rev A */;
pub const CS4349_REVB: c_uint = 0xF1	/* Rev B */;
pub const CS4349_REVC2: c_uint = 0xFF	/* Rev C2 */;
// PDN_DONE Poll Maximum
// If soft ramp is set it will take much longer to power down
// the system.
//
pub const PDN_POLL_MAX: c_int = 900;
// Bitfield Definitions
// CS4349_MODE
// (Digital Interface Format, De-Emphasis Control, Functional Mode

pub const DIF_LEFT_JST: c_uint = 0x00;
pub const DIF_I2S: c_uint = 0x01;
pub const DIF_RGHT_JST16: c_uint = 0x02;
pub const DIF_RGHT_JST24: c_uint = 0x03;
pub const DIF_TDM0: c_uint = 0x04;
pub const DIF_TDM1: c_uint = 0x05;
pub const DIF_TDM2: c_uint = 0x06;
pub const DIF_TDM3: c_uint = 0x07;
pub const DIF_MASK: c_uint = 0x70;

pub const DEM_MASK: c_uint = 0x0C;
pub const NO_DEM: c_uint = 0x00;
pub const DEM_441: c_uint = 0x04;
pub const DEM_48K: c_uint = 0x08;
pub const DEM_32K: c_uint = 0x0C;
pub const FM_AUTO: c_uint = 0x00;
pub const FM_SNGL: c_uint = 0x01;
pub const FM_DBL: c_uint = 0x02;
pub const FM_QUAD: c_uint = 0x03;
pub const FM_SNGL_MIN: c_int = 30000;
pub const FM_SNGL_MAX: c_int = 54000;
pub const FM_DBL_MAX: c_int = 108000;
pub const FM_QUAD_MAX: c_int = 216000;
pub const FM_MASK: c_uint = 0x03;
// CS4349_VMI (VMI = Volume, Mixing and Inversion Controls)

// INVERT_A only available for Left Jstfd, Right Jstfd16 and Right Jstfd24

// INVERT_B only available for Left Jstfd, Right Jstfd16 and Right Jstfd24

pub const MUTEAB: c_uint = 0x00;
pub const MUTEA_RIGHTB: c_uint = 0x01;
pub const MUTEA_LEFTB: c_uint = 0x02;
pub const MUTEA_SUMLRDIV2B: c_uint = 0x03;
pub const RIGHTA_MUTEB: c_uint = 0x04;
pub const RIGHTA_RIGHTB: c_uint = 0x05;
pub const RIGHTA_LEFTB: c_uint = 0x06;
pub const RIGHTA_SUMLRDIV2B: c_uint = 0x07;
pub const LEFTA_MUTEB: c_uint = 0x08;
pub const LEFTA_RIGHTB: c_uint = 0x09	/* Default */;
pub const LEFTA_LEFTB: c_uint = 0x0A;
pub const LEFTA_SUMLRDIV2B: c_uint = 0x0B;
pub const SUMLRDIV2A_MUTEB: c_uint = 0x0C;
pub const SUMLRDIV2A_RIGHTB: c_uint = 0x0D;
pub const SUMLRDIV2A_LEFTB: c_uint = 0x0E;
pub const SUMLRDIV2_AB: c_uint = 0x0F;
pub const CHMIX_MASK: c_uint = 0x0F;
// CS4349_MUTE

pub const MUTE_AB_MASK: c_uint = 0x18;
// CS4349_RMPFLT (Ramp and Filter Control)

pub const IMMDT_CHNG: c_uint = 0x31;
pub const ZEROCRSS: c_uint = 0x71;
pub const SOFT_RMP: c_uint = 0xB1;
pub const SFTRMP_ZEROCRSS: c_uint = 0xF1;
pub const SR_ZC_MASK: c_uint = 0xC0;
// CS4349_MISC

