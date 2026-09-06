//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/dib0700.h
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
// Linux driver for devices based on the DiBcom DiB0700 USB bridge
//
// Copyright (C) 2005-6 DiBcom, SA
//

pub const REQUEST_SET_USB_XFER_LEN: c_uint = 0x0 /* valid only for firmware version */;
// higher than 1.21
pub const REQUEST_I2C_READ: c_uint = 0x2;
pub const REQUEST_I2C_WRITE: c_uint = 0x3;
pub const REQUEST_POLL_RC: c_uint = 0x4 /* deprecated in firmware v1.20 */;
pub const REQUEST_JUMPRAM: c_uint = 0x8;
pub const REQUEST_SET_CLOCK: c_uint = 0xB;
pub const REQUEST_SET_GPIO: c_uint = 0xC;
pub const REQUEST_ENABLE_VIDEO: c_uint = 0xF;
// 1 Byte: 4MSB(1 = enable streaming, 0 = disable streaming) 4LSB(Video Mode: 0 = MPEG2 188Bytes, 1 = Analog)
// 2 Byte: MPEG2 mode:  4MSB(1 = Master Mode, 0 = Slave Mode) 4LSB(Channel 1 = bit0, Channel 2 = bit1)
// 2 Byte: Analog mode: 4MSB(0 = 625 lines, 1 = 525 lines)    4LSB(     "                "           )
pub const REQUEST_SET_I2C_PARAM: c_uint = 0x10;
pub const REQUEST_SET_RC: c_uint = 0x11;
pub const REQUEST_NEW_I2C_READ: c_uint = 0x12;
pub const REQUEST_NEW_I2C_WRITE: c_uint = 0x13;
pub const REQUEST_GET_VERSION: c_uint = 0x15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dib0700_state {
    pub channel_state: u8,
    pub mt2060_if1: [u16; 2],
    pub rc_toggle: u8,
    pub rc_counter: u8,
    pub is_dib7000pc: u8,
    pub fw_use_new_i2c_api: u8,
    pub disable_streaming_master_mode: u8,
    pub fw_version: u32,
    pub nb_packet_buffer_size: u32,
    pub ): *mut *mut *mut int (read_status)(struct dvb_frontend , enum fe_status,
    pub fe): *mut *mut *mut int (sleep)(struct dvb_frontend,
    pub buf: [u8; 255],
    pub i2c_client_demod: *mut i2c_client,
    pub i2c_client_tuner: *mut i2c_client,
}

extern "C" {
    pub fn dib0700_ctrl_clock(d: *mut dvb_usb_device, clk_MHz: u32, clock_out_gp3: u8) -> c_int;
}
extern "C" {
    pub fn dib0700_rc_setup(d: *mut dvb_usb_device, intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn dib0700_streaming_ctrl(adap: *mut dvb_usb_adapter, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn dib0700_change_protocol(dev: *mut rc_dev, rc_proto: *mut u64) -> c_int;
}
extern "C" {
    pub fn dib0700_set_i2c_speed(d: *mut dvb_usb_device, scl_kHz: u16) -> c_int;
}
