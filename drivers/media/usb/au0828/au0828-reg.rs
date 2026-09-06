//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/au0828/au0828-reg.h
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
// Driver for the Auvitek USB bridge
//
// Copyright (c) 2008 Steven Toth <stoth@linuxtv.org>
//
// We'll start to rename these registers once we have a better
// understanding of their meaning.
//
pub const REG_000: c_uint = 0x000;
pub const REG_001: c_uint = 0x001;
pub const REG_002: c_uint = 0x002;
pub const REG_003: c_uint = 0x003;
pub const AU0828_SENSORCTRL_100: c_uint = 0x100;
pub const AU0828_SENSORCTRL_VBI_103: c_uint = 0x103;
// I2C registers
pub const AU0828_I2C_TRIGGER_200: c_uint = 0x200;
pub const AU0828_I2C_STATUS_201: c_uint = 0x201;
pub const AU0828_I2C_CLK_DIVIDER_202: c_uint = 0x202;
pub const AU0828_I2C_DEST_ADDR_203: c_uint = 0x203;
pub const AU0828_I2C_WRITE_FIFO_205: c_uint = 0x205;
pub const AU0828_I2C_READ_FIFO_209: c_uint = 0x209;
pub const AU0828_I2C_MULTIBYTE_MODE_2FF: c_uint = 0x2ff;
// Audio registers
pub const AU0828_AUDIOCTRL_50C: c_uint = 0x50C;
pub const REG_600: c_uint = 0x600;
//
// Here are constants for values associated with the above registers
// I2C Trigger (Reg 0x200)
pub const AU0828_I2C_TRIGGER_WRITE: c_uint = 0x01;
pub const AU0828_I2C_TRIGGER_READ: c_uint = 0x20;
pub const AU0828_I2C_TRIGGER_HOLD: c_uint = 0x40;
// I2C Status (Reg 0x201)
pub const AU0828_I2C_STATUS_READ_DONE: c_uint = 0x01;
pub const AU0828_I2C_STATUS_NO_READ_ACK: c_uint = 0x02;
pub const AU0828_I2C_STATUS_WRITE_DONE: c_uint = 0x04;
pub const AU0828_I2C_STATUS_NO_WRITE_ACK: c_uint = 0x08;
pub const AU0828_I2C_STATUS_BUSY: c_uint = 0x10;
// I2C Clock Divider (Reg 0x202)
pub const AU0828_I2C_CLK_250KHZ: c_uint = 0x07;
pub const AU0828_I2C_CLK_100KHZ: c_uint = 0x14;
pub const AU0828_I2C_CLK_30KHZ: c_uint = 0x40;
pub const AU0828_I2C_CLK_20KHZ: c_uint = 0x60;
