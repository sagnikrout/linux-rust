//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/ttusb2.h
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
// DVB USB compliant linux driver for Technotrend DVB USB boxes and clones
// (e.g. Pinnacle 400e DVB-S USB2.0).
//
// Copyright (c) 2002 Holger Waechtler <holger@convergence.de>
// Copyright (c) 2003 Felix Domke <tmbinc@elitedvb.net>
// Copyright (C) 2005-6 Patrick Boettcher <pb@linuxtv.de>
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//
// TTUSB protocol
//
// always to messages (out/in)
// out message:
// 0xaa <id> <cmdbyte> <datalen> <data...>
//
// in message (complete block is always 0x40 bytes long)
// 0x55 <id> <cmdbyte> <datalen> <data...>
//
// id is incremented for each transaction
//
pub const CMD_DSP_DOWNLOAD: c_uint = 0x13;
// out data: <byte>[28]
// last block must be empty
pub const CMD_DSP_BOOT: c_uint = 0x14;
// out data: nothing
pub const CMD_POWER: c_uint = 0x15;
// out data: <on=1/off=0>
pub const CMD_LNB: c_uint = 0x16;
// out data: <power=1> <18V=0,13V=1> <tone> <??=1> <??=1>
pub const CMD_GET_VERSION: c_uint = 0x17;
// in  data: <version_byte>[5]
pub const CMD_DISEQC: c_uint = 0x18;
// out data: <master=0xff/burst=??> <cmdlen> <cmdbytes>[cmdlen]
pub const CMD_PID_ENABLE: c_uint = 0x22;
// out data: <index> <type: ts=1/sec=2> <pid msb> <pid lsb>
pub const CMD_PID_DISABLE: c_uint = 0x23;
// out data: <index>
pub const CMD_FILTER_ENABLE: c_uint = 0x24;
// out data: <index> <pid_idx> <filter>[12] <mask>[12]
pub const CMD_FILTER_DISABLE: c_uint = 0x25;
// out data: <index>
pub const CMD_GET_DSP_VERSION: c_uint = 0x26;
// in  data: <version_byte>[28]
pub const CMD_I2C_XFER: c_uint = 0x31;
// out data: <addr << 1> <sndlen> <rcvlen> <data>[sndlen]
// in  data: <addr << 1> <sndlen> <rcvlen> <data>[rcvlen]
pub const CMD_I2C_BITRATE: c_uint = 0x32;
// out data: <default=0>
