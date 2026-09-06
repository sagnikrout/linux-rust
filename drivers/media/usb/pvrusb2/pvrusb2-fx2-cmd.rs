//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-fx2-cmd.h
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
// Copyright (C) 2007 Michael Krufky <mkrufky@linuxtv.org>
//
pub const FX2CMD_MEM_WRITE_DWORD: c_uint = 0x01u;
pub const FX2CMD_MEM_READ_DWORD: c_uint = 0x02u;
pub const FX2CMD_HCW_ZILOG_RESET: c_uint = 0x10u /* 1=reset 0=release */;
pub const FX2CMD_MEM_READ_64BYTES: c_uint = 0x28u;
pub const FX2CMD_REG_WRITE: c_uint = 0x04u;
pub const FX2CMD_REG_READ: c_uint = 0x05u;
pub const FX2CMD_MEMSEL: c_uint = 0x06u;
pub const FX2CMD_I2C_WRITE: c_uint = 0x08u;
pub const FX2CMD_I2C_READ: c_uint = 0x09u;
pub const FX2CMD_GET_USB_SPEED: c_uint = 0x0bu;
pub const FX2CMD_STREAMING_ON: c_uint = 0x36u;
pub const FX2CMD_STREAMING_OFF: c_uint = 0x37u;
pub const FX2CMD_FWPOST1: c_uint = 0x52u;
// These 2 only exist on Model 160xxx
pub const FX2CMD_HCW_DEMOD_RESET_PIN: c_uint = 0xd4u;
pub const FX2CMD_HCW_MAKO_SLEEP_PIN: c_uint = 0xd5u;
pub const FX2CMD_POWER_OFF: c_uint = 0xdcu;
pub const FX2CMD_POWER_ON: c_uint = 0xdeu;
pub const FX2CMD_DEEP_RESET: c_uint = 0xddu;
pub const FX2CMD_GET_EEPROM_ADDR: c_uint = 0xebu;
pub const FX2CMD_GET_IR_CODE: c_uint = 0xecu;
pub const FX2CMD_HCW_DEMOD_RESETIN: c_uint = 0xf0u;
pub const FX2CMD_HCW_DTV_STREAMING_ON: c_uint = 0xf1u;
pub const FX2CMD_HCW_DTV_STREAMING_OFF: c_uint = 0xf2u;
pub const FX2CMD_ONAIR_DTV_STREAMING_ON: c_uint = 0xa0u;
pub const FX2CMD_ONAIR_DTV_STREAMING_OFF: c_uint = 0xa1u;
pub const FX2CMD_ONAIR_DTV_POWER_ON: c_uint = 0xa2u;
pub const FX2CMD_ONAIR_DTV_POWER_OFF: c_uint = 0xa3u;
