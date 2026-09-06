//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max5970.h
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
//
// Device driver for regulators in MAX5970 and MAX5978 IC
//
// Copyright (c) 2022 9elements GmbH
//
// Author: Patrick Rudolph <patrick.rudolph@9elements.com>
//

pub const MAX5970_NUM_SWITCHES: c_int = 2;
pub const MAX5978_NUM_SWITCHES: c_int = 1;
pub const MAX5970_NUM_LEDS: c_int = 4;

pub const MAX5970_REG_MON_RANGE: c_uint = 0x18;
pub const MAX5970_MON_MASK: c_uint = 0x3;

pub const MAX5970_MON_MAX_RANGE_UV: c_int = 16000000;

pub const MAX5970_FAST2SLOW_RATIO: c_int = 200;
pub const MAX5970_REG_STATUS0: c_uint = 0x31;

pub const MAX5970_REG_STATUS1: c_uint = 0x32;
pub const STATUS1_PROT_MASK: c_uint = 0x3;

pub const STATUS1_PROT_SHUTDOWN: c_int = 0;
pub const STATUS1_PROT_CLEAR_PG: c_int = 1;
pub const STATUS1_PROT_ALERT_ONLY: c_int = 2;
pub const MAX5970_REG_STATUS2: c_uint = 0x33;
pub const MAX5970_IRNG_MASK: c_uint = 0x3;

pub const MAX5970_REG_STATUS3: c_uint = 0x34;

pub const MAX5970_REG_FAULT0: c_uint = 0x35;

pub const MAX5970_REG_FAULT1: c_uint = 0x36;

pub const MAX5970_REG_FAULT2: c_uint = 0x37;

pub const MAX5970_REG_CHXEN: c_uint = 0x3b;

pub const MAX5970_REG_LED_FLASH: c_uint = 0x43;
pub const MAX_REGISTERS: c_uint = 0x49;
pub const ADC_MASK: c_uint = 0x3FF;
