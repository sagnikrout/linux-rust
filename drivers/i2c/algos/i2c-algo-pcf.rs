//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/algos/i2c-algo-pcf.h
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
// --------------------------------------------------------------------
// i2c-pcf8584.h: PCF 8584 global defines
// --------------------------------------------------------------------
// Copyright (C) 1996 Simon G. Vogl
//
// --------------------------------------------------------------------
// With some changes from Frodo Looijaard <frodol@dds.nl>
pub const I2C_PCF8584_H: c_int = 1;
// ----- Control register bits ----------------------------------------
pub const I2C_PCF_PIN: c_uint = 0x80;
pub const I2C_PCF_ESO: c_uint = 0x40;
pub const I2C_PCF_ES1: c_uint = 0x20;
pub const I2C_PCF_ES2: c_uint = 0x10;
pub const I2C_PCF_ENI: c_uint = 0x08;
pub const I2C_PCF_STA: c_uint = 0x04;
pub const I2C_PCF_STO: c_uint = 0x02;
pub const I2C_PCF_ACK: c_uint = 0x01;

// ----- Status register bits -----------------------------------------
// #define I2C_PCF_PIN  0x80    as above
pub const I2C_PCF_INI: c_uint = 0x40   /* 1 if not initialized */;
pub const I2C_PCF_STS: c_uint = 0x20;
pub const I2C_PCF_BER: c_uint = 0x10;
pub const I2C_PCF_AD0: c_uint = 0x08;
pub const I2C_PCF_LRB: c_uint = 0x08;
pub const I2C_PCF_AAS: c_uint = 0x04;
pub const I2C_PCF_LAB: c_uint = 0x02;
pub const I2C_PCF_BB: c_uint = 0x01;
// ----- Chip clock frequencies ---------------------------------------
pub const I2C_PCF_CLK3: c_uint = 0x00;
pub const I2C_PCF_CLK443: c_uint = 0x10;
pub const I2C_PCF_CLK6: c_uint = 0x14;
pub const I2C_PCF_CLK: c_uint = 0x18;
pub const I2C_PCF_CLK12: c_uint = 0x1c;
// ----- transmission frequencies -------------------------------------
pub const I2C_PCF_TRNS90: c_uint = 0x00	/*  90 kHz */;
pub const I2C_PCF_TRNS45: c_uint = 0x01	/*  45 kHz */;
pub const I2C_PCF_TRNS11: c_uint = 0x02	/*  11 kHz */;
pub const I2C_PCF_TRNS15: c_uint = 0x03	/* 1.5 kHz */;
// ----- Access to internal registers according to ES1,ES2 ------------
// they are mapped to the data port ( a0 = 0 )
// available when ESO == 0 :
pub const I2C_PCF_OWNADR: c_int = 0;

