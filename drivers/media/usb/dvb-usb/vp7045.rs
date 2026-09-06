//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/vp7045.h
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
// Common header-file of the Linux driver for the TwinhanDTV Alpha/MagicBoxII
// USB2.0 DVB-T receiver.
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// Thanks to Twinhan who kindly provided hardware and information.
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

// vp7045 commands
// Twinhan Vendor requests
pub const TH_COMMAND_IN: c_uint = 0xC0;
pub const TH_COMMAND_OUT: c_uint = 0xC1;
// command bytes
pub const TUNER_REG_READ: c_uint = 0x03;
pub const TUNER_REG_WRITE: c_uint = 0x04;
pub const RC_VAL_READ: c_uint = 0x05;
pub const RC_NO_KEY: c_uint = 0x44;
pub const SET_TUNER_POWER: c_uint = 0x06;
pub const CHECK_TUNER_POWER: c_uint = 0x12;
pub const Tuner_Power_ON: c_int = 1;
pub const Tuner_Power_OFF: c_int = 0;
pub const GET_USB_SPEED: c_uint = 0x07;
pub const LOCK_TUNER_COMMAND: c_uint = 0x09;
pub const TUNER_SIGNAL_READ: c_uint = 0x0A;
// FX2 eeprom
pub const SET_EE_VALUE: c_uint = 0x10;
pub const GET_EE_VALUE: c_uint = 0x11;
pub const FX2_ID_ADDR: c_uint = 0x00;
pub const VID_MSB_ADDR: c_uint = 0x02;
pub const VID_LSB_ADDR: c_uint = 0x01;
pub const PID_MSB_ADDR: c_uint = 0x04;
pub const PID_LSB_ADDR: c_uint = 0x03;
pub const MAC_0_ADDR: c_uint = 0x07;
pub const MAC_1_ADDR: c_uint = 0x08;
pub const MAC_2_ADDR: c_uint = 0x09;
pub const MAC_3_ADDR: c_uint = 0x0a;
pub const MAC_4_ADDR: c_uint = 0x0b;
pub const MAC_5_ADDR: c_uint = 0x0c;
pub const RESET_FX2: c_uint = 0x13;
pub const FW_VERSION_READ: c_uint = 0x0B;
pub const VENDOR_STRING_READ: c_uint = 0x0C;
pub const PRODUCT_STRING_READ: c_uint = 0x0D;
pub const FW_BCD_VERSION_READ: c_uint = 0x14;
extern "C" {
    pub fn vp7045_fe_attach(d: *mut dvb_usb_device) -> *mut dvb_frontend;
}
extern "C" {
    pub fn vp7045_usb_op(d: *mut dvb_usb_device, cmd: u8, out: *mut u8, outlen: c_int, in: *mut u8, inlen: c_int, msec: c_int) -> c_int;
}
extern "C" {
    pub fn vp7045_read_reg(d: *mut dvb_usb_device, reg: u8) -> u8;
}
