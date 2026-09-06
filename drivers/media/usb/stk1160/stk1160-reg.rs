//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/stk1160/stk1160-reg.h
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
// STK1160 driver
//
// Copyright (C) 2012 Ezequiel Garcia
// <elezegarcia--a.t--gmail.com>
//
// Based on Easycap driver by R.M. Thomas
// Copyright (C) 2010 R.M. Thomas
// <rmthomas--a.t--sciolus.org>
//
// GPIO Control
pub const STK1160_GCTRL: c_uint = 0x000;
// Remote Wakeup Control
pub const STK1160_RMCTL: c_uint = 0x00c;
// Power-on Strapping Data
pub const STK1160_POSVA: c_uint = 0x010;
pub const STK1160_POSV_L: c_uint = 0x010;
pub const STK1160_POSV_M: c_uint = 0x011;
pub const STK1160_POSV_H: c_uint = 0x012;

//
// Decoder Control Register:
// This byte controls capture start/stop
// with bit #7 (0x?? OR 0x80 to activate).
//
pub const STK1160_DCTRL: c_uint = 0x100;
//
// Decimation Control Register:
// Byte 104: Horizontal Decimation Line Unit Count
// Byte 105: Vertical Decimation Line Unit Count
// Byte 106: Decimation Control
// Bit 0 - Horizontal Decimation Control
// 0 Horizontal decimation is disabled.
// 1 Horizontal decimation is enabled.
// Bit 1 - Decimates Half or More Column
// 0 Decimates less than half from original column,
// send count unit (0x105) before each unit skipped.
// 1 Decimates half or more from original column,
// skip count unit (0x105) before each unit sent.
// Bit 2 - Vertical Decimation Control
// 0 Vertical decimation is disabled.
// 1 Vertical decimation is enabled.
// Bit 3 - Vertical Greater or Equal to Half
// 0 Decimates less than half from original row,
// send count unit (0x105) before each unit skipped.
// 1 Decimates half or more from original row,
// skip count unit (0x105) before each unit sent.
// Bit 4 - Decimation Unit
// 0 Decimation will work with 2 rows or columns per unit.
// 1 Decimation will work with 4 rows or columns per unit.
//
pub const STK1160_DMCTRL_H_UNITS: c_uint = 0x104;
pub const STK1160_DMCTRL_V_UNITS: c_uint = 0x105;
pub const STK1160_DMCTRL: c_uint = 0x106;

// Capture Frame Start Position
pub const STK116_CFSPO: c_uint = 0x110;
pub const STK116_CFSPO_STX_L: c_uint = 0x110;
pub const STK116_CFSPO_STX_H: c_uint = 0x111;
pub const STK116_CFSPO_STY_L: c_uint = 0x112;
pub const STK116_CFSPO_STY_H: c_uint = 0x113;
// Capture Frame End Position
pub const STK116_CFEPO: c_uint = 0x114;
pub const STK116_CFEPO_ENX_L: c_uint = 0x114;
pub const STK116_CFEPO_ENX_H: c_uint = 0x115;
pub const STK116_CFEPO_ENY_L: c_uint = 0x116;
pub const STK116_CFEPO_ENY_H: c_uint = 0x117;
// Serial Interface Control
pub const STK1160_SICTL: c_uint = 0x200;
pub const STK1160_SICTL_CD: c_uint = 0x202;
pub const STK1160_SICTL_SDA: c_uint = 0x203;
// Serial Bus Write
pub const STK1160_SBUSW: c_uint = 0x204;
pub const STK1160_SBUSW_WA: c_uint = 0x204;
pub const STK1160_SBUSW_WD: c_uint = 0x205;
// Serial Bus Read
pub const STK1160_SBUSR: c_uint = 0x208;
pub const STK1160_SBUSR_RA: c_uint = 0x208;
pub const STK1160_SBUSR_RD: c_uint = 0x209;
// Alternate Serial Interface Control
pub const STK1160_ASIC: c_uint = 0x2fc;
// PLL Select Options
pub const STK1160_PLLSO: c_uint = 0x018;
// PLL Frequency Divider
pub const STK1160_PLLFD: c_uint = 0x01c;
// Timing Generator
pub const STK1160_TIGEN: c_uint = 0x300;
// Timing Control Parameter
pub const STK1160_TICTL: c_uint = 0x350;
// AC97 Audio Control
pub const STK1160_AC97CTL_0: c_uint = 0x500;
pub const STK1160_AC97CTL_1: c_uint = 0x504;

// Use [0:6] bits of register 0x504 to set codec command address
pub const STK1160_AC97_ADDR: c_uint = 0x504;
// Use [16:31] bits of register 0x500 to set codec command data
pub const STK1160_AC97_CMD: c_uint = 0x502;
// Audio I2S Interface
pub const STK1160_I2SCTL: c_uint = 0x50c;
// EEPROM Interface
pub const STK1160_EEPROM_SZ: c_uint = 0x5f0;
