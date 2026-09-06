//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/touchscreen/tsc200x-core.h
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


// SPDX-License-Identifier: GPL-2.0
// control byte 1
pub const TSC200X_CMD: c_uint = 0x80;
pub const TSC200X_CMD_NORMAL: c_uint = 0x00;
pub const TSC200X_CMD_STOP: c_uint = 0x01;
pub const TSC200X_CMD_12BIT: c_uint = 0x04;
// control byte 0
pub const TSC200X_REG_READ: c_uint = 0x01 /* R/W access */;
pub const TSC200X_REG_PND0: c_uint = 0x02 /* Power Not Down Control */;

// configuration register 0
pub const TSC200X_CFR0_PRECHARGE_276US: c_uint = 0x0040;
pub const TSC200X_CFR0_STABTIME_1MS: c_uint = 0x0300;
pub const TSC200X_CFR0_CLOCK_1MHZ: c_uint = 0x1000;
pub const TSC200X_CFR0_RESOLUTION12: c_uint = 0x2000;
pub const TSC200X_CFR0_PENMODE: c_uint = 0x8000;

// bits common to both read and write of configuration register 0
pub const TSC200X_CFR0_RW_MASK: c_uint = 0x3fff;
// configuration register 1
pub const TSC200X_CFR1_BATCHDELAY_4MS: c_uint = 0x0003;

// configuration register 2
pub const TSC200X_CFR2_MAVE_Z: c_uint = 0x0004;
pub const TSC200X_CFR2_MAVE_Y: c_uint = 0x0008;
pub const TSC200X_CFR2_MAVE_X: c_uint = 0x0010;
pub const TSC200X_CFR2_AVG_7: c_uint = 0x0800;
pub const TSC200X_CFR2_MEDIUM_15: c_uint = 0x3000;

pub const MAX_12BIT: c_uint = 0xfff;
pub const TSC200X_DEF_X_FUZZ: c_int = 4;
pub const TSC200X_DEF_Y_FUZZ: c_int = 8;
pub const TSC200X_DEF_P_FUZZ: c_int = 2;
pub const TSC200X_DEF_RESISTOR: c_int = 280;
pub const TSC2005_SPI_MAX_SPEED_HZ: c_int = 10000000;
pub const TSC200X_PENUP_TIME_MS: c_int = 40;
