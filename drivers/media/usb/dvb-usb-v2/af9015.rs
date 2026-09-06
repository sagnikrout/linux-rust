//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/af9015.h
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
// DVB USB Linux driver for Afatech AF9015 DVB-T USB2.0 receiver
//
// Copyright (C) 2007 Antti Palosaari <crope@iki.fi>
//
// Thanks to Afatech who kindly provided information.
//

pub const AF9015_I2C_EEPROM: c_uint = 0x50;
pub const AF9015_I2C_DEMOD: c_uint = 0x1c;
pub const AF9015_USB_TIMEOUT: c_int = 2000;
// EEPROM locations
pub const AF9015_EEPROM_IR_MODE: c_uint = 0x18;
pub const AF9015_EEPROM_IR_REMOTE_TYPE: c_uint = 0x34;
pub const AF9015_EEPROM_TS_MODE: c_uint = 0x31;
pub const AF9015_EEPROM_DEMOD2_I2C: c_uint = 0x32;
pub const AF9015_EEPROM_SAW_BW1: c_uint = 0x35;
pub const AF9015_EEPROM_XTAL_TYPE1: c_uint = 0x36;
pub const AF9015_EEPROM_SPEC_INV1: c_uint = 0x37;
pub const AF9015_EEPROM_IF1L: c_uint = 0x38;
pub const AF9015_EEPROM_IF1H: c_uint = 0x39;
pub const AF9015_EEPROM_MT2060_IF1L: c_uint = 0x3a;
pub const AF9015_EEPROM_MT2060_IF1H: c_uint = 0x3b;
pub const AF9015_EEPROM_TUNER_ID1: c_uint = 0x3c;
pub const AF9015_EEPROM_SAW_BW2: c_uint = 0x45;
pub const AF9015_EEPROM_XTAL_TYPE2: c_uint = 0x46;
pub const AF9015_EEPROM_SPEC_INV2: c_uint = 0x47;
pub const AF9015_EEPROM_IF2L: c_uint = 0x48;
pub const AF9015_EEPROM_IF2H: c_uint = 0x49;
pub const AF9015_EEPROM_MT2060_IF2L: c_uint = 0x4a;
pub const AF9015_EEPROM_MT2060_IF2H: c_uint = 0x4b;
pub const AF9015_EEPROM_TUNER_ID2: c_uint = 0x4c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct req_t {
    pub /: *mut *mut u8 cmd; / [0],
// seq */     /* [1]
    pub /: *mut *mut u8 i2c_addr; / [2],
    pub /: *mut *mut u16 addr; / [3|4],
    pub /: *mut *mut u8 mbox; / [5],
    pub /: *mut *mut u8 addr_len; / [6],
    pub /: *mut *mut u8 data_len; / [7],
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af9015_cmd {
    GET_CONFIG           = 0x10,
    DOWNLOAD_FIRMWARE    = 0x11,
    BOOT                 = 0x13,
    READ_MEMORY          = 0x20,
    WRITE_MEMORY         = 0x21,
    READ_WRITE_I2C       = 0x22,
    COPY_FIRMWARE        = 0x23,
    RECONNECT_USB        = 0x5a,
    WRITE_VIRTUAL_MEMORY = 0x26,
    GET_IR_CODE          = 0x27,
    READ_I2C,
    WRITE_I2C,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af9015_ir_mode {
    AF9015_IR_MODE_DISABLED = 0,
    AF9015_IR_MODE_HID,
    AF9015_IR_MODE_RLC,
    AF9015_IR_MODE_RC6,
    AF9015_IR_MODE_POLLING, /* just guess */
}

pub const BUF_LEN: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct af9015_state {
    pub regmap: *mut regmap,
    pub /: *mut *mut u8 buf[BUF_LEN]; / bulk USB control message,
    pub ir_mode: u8,
    pub rc_repeat: u8,
    pub rc_keycode: u32,
    pub rc_last: [u8; 4],
    pub rc_failed: bool,
    pub dual_mode: u8,
    pub /: *mut *mut u8 seq; / packet sequence number,
    pub mt2060_if1: [u16; 2],
    pub firmware_size: u16,
    pub firmware_checksum: u16,
    pub eeprom_sum: u32,
    pub af9013_pdata: [af9013_platform_data; 2],
    pub demod_i2c_client: [*mut i2c_client; 2],
    pub af9013_i2c_addr: [u8; 2],
    pub usb_ts_if_configured: [bool; 2],
// for demod callback override
    pub fe): *mut *mut int (set_frontend[2]) (struct dvb_frontend,
    pub status): *mut *mut *mut int (read_status[2]) (struct dvb_frontend fe, enum fe_status,
    pub fe): *mut *mut int (init[2]) (struct dvb_frontend,
    pub fe): *mut *mut int (sleep[2]) (struct dvb_frontend,
    pub fe): *mut *mut int (tuner_init[2]) (struct dvb_frontend,
    pub fe): *mut *mut int (tuner_sleep[2]) (struct dvb_frontend,
    pub fe_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum af9015_remote {
    AF9015_REMOTE_NONE                    = 0,
// 1 */	AF9015_REMOTE_A_LINK_DTU_M,
    AF9015_REMOTE_MSI_DIGIVOX_MINI_II_V3,
    AF9015_REMOTE_MYGICTV_U718,
    AF9015_REMOTE_DIGITTRADE_DVB_T,
// 5 */	AF9015_REMOTE_AVERMEDIA_KS,
}
