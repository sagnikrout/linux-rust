//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/isp1301.h
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
// NXP ISP1301 USB transceiver driver
//
// Copyright (C) 2012 Roland Stigge <stigge@antcom.de>
//

// I2C Register definitions:
pub const ISP1301_I2C_MODE_CONTROL_1: c_uint = 0x04	/* u8 read, set, +1 clear */;

pub const MC1_MASK: c_uint = 0x7f;
pub const ISP1301_I2C_MODE_CONTROL_2: c_uint = 0x12	/* u8 read, set, +1 clear */;

pub const ISP1301_I2C_OTG_CONTROL_1: c_uint = 0x06	/* u8 read, set, +1 clear */;

pub const ISP1301_I2C_OTG_CONTROL_2: c_uint = 0x10	/* u8 readonly */;

pub const ISP1301_I2C_INTERRUPT_SOURCE: c_uint = 0x8;
pub const ISP1301_I2C_INTERRUPT_LATCH: c_uint = 0xA;
pub const ISP1301_I2C_INTERRUPT_FALLING: c_uint = 0xC;
pub const ISP1301_I2C_INTERRUPT_RISING: c_uint = 0xE;

