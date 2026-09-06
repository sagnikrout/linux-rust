//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/dibusb.h
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
// Header file for all dibusb-based-receivers.
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

//
// protocol of all dibusb related devices
//
// bulk msg to/from endpoint 0x01
//
// general structure:
// request_byte parameter_bytes
//
pub const DIBUSB_REQ_START_READ: c_uint = 0x00;
pub const DIBUSB_REQ_START_DEMOD: c_uint = 0x01;
//
// i2c read
// bulk write: 0x02 ((7bit i2c_addr << 1) | 0x01) register_bytes length_word
// bulk read:  byte_buffer (length_word bytes)
//
pub const DIBUSB_REQ_I2C_READ: c_uint = 0x02;
//
// i2c write
// bulk write: 0x03 (7bit i2c_addr << 1) register_bytes value_bytes
//
pub const DIBUSB_REQ_I2C_WRITE: c_uint = 0x03;
//
// polling the value of the remote control
// bulk write: 0x04
// bulk read:  byte_buffer (5 bytes)
//
pub const DIBUSB_REQ_POLL_REMOTE: c_uint = 0x04;
// additional status values for Hauppauge Remote Control Protocol
pub const DIBUSB_RC_HAUPPAUGE_KEY_PRESSED: c_uint = 0x01;
pub const DIBUSB_RC_HAUPPAUGE_KEY_EMPTY: c_uint = 0x03;
// streaming mode:
// bulk write: 0x05 mode_byte
//
// mode_byte is mostly 0x00
//
pub const DIBUSB_REQ_SET_STREAMING_MODE: c_uint = 0x05;
// interrupt the internal read loop, when blocking
pub const DIBUSB_REQ_INTR_READ: c_uint = 0x06;
// io control
// 0x07 cmd_byte param_bytes
//
// param_bytes can be up to 32 bytes
//
// cmd_byte function    parameter name
// 0x00     power mode
// 0x00      sleep
// 0x01      wakeup
//
// 0x01     enable streaming
// 0x02     disable streaming
//
pub const DIBUSB_REQ_SET_IOCTL: c_uint = 0x07;
// IOCTL commands
// change the power mode in firmware
pub const DIBUSB_IOCTL_CMD_POWER_MODE: c_uint = 0x00;
pub const DIBUSB_IOCTL_POWER_SLEEP: c_uint = 0x00;
pub const DIBUSB_IOCTL_POWER_WAKEUP: c_uint = 0x01;
// modify streaming of the FX2
pub const DIBUSB_IOCTL_CMD_ENABLE_STREAM: c_uint = 0x01;
pub const DIBUSB_IOCTL_CMD_DISABLE_STREAM: c_uint = 0x02;
// Max transfer size done by I2C transfer functions
pub const MAX_XFER_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibusb_state {
    pub ops: dib_fe_xfer_ops,
    pub mt2060_present: c_int,
    pub tuner_addr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dibusb_device_state {
// for RC5 remote control
    pub old_toggle: c_int,
    pub last_repeat_count: c_int,
}

extern "C" {
    pub fn dibusb_dib3000mc_frontend_attach(: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dibusb_dib3000mc_tuner_attach(: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dibusb_streaming_ctrl(: *mut dvb_usb_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dibusb_pid_filter(: *mut dvb_usb_adapter, _arg: c_int, _arg: u16, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dibusb_pid_filter_ctrl(: *mut dvb_usb_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dibusb2_0_streaming_ctrl(: *mut dvb_usb_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dibusb_power_ctrl(: *mut dvb_usb_device, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn dibusb2_0_power_ctrl(: *mut dvb_usb_device, _arg: c_int) -> c_int;
}
pub const DEFAULT_RC_INTERVAL: c_int = 150;
// #define DEFAULT_RC_INTERVAL 100000
extern "C" {
    pub fn dibusb_rc_query(: *mut dvb_usb_device, : *mut u32, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn dibusb_read_eeprom_byte(: *mut dvb_usb_device, _arg: u8, : *mut u8) -> c_int;
}
