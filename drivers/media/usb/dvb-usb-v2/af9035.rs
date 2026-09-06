//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/af9035.h
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
// Afatech AF9035 DVB USB driver
//
// Copyright (C) 2009 Antti Palosaari <crope@iki.fi>
// Copyright (C) 2012 Antti Palosaari <crope@iki.fi>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val {
    pub reg: u32,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val_mask {
    pub reg: u32,
    pub val: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_req {
    pub cmd: u8,
    pub mbox: u8,
    pub wlen: u8,
    pub wbuf: *mut u8,
    pub rlen: u8,
    pub rbuf: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state {
pub const BUF_LEN: c_int = 64;
    pub buf: [u8; BUF_LEN],
    pub /: *mut *mut u8 seq; / packet sequence number,
    pub prechip_version: u8,
    pub chip_version: u8,
    pub chip_type: u16,
    pub eeprom: [u8; 256],
    pub no_eeprom: bool,
    pub ir_mode: u8,
    pub ir_type: u8,
    pub dual_mode:1: u8,
    pub no_read:1: u8,
    pub af9033_i2c_addr: [u8; 2],
    pub it930x_addresses: u8,
    pub af9033_config: [af9033_config; 2],
    pub ops: af9033_ops,
pub const AF9035_I2C_CLIENT_MAX: c_int = 4;
    pub i2c_client: [*mut i2c_client; AF9035_I2C_CLIENT_MAX],
    pub i2c_adapter_demod: *mut i2c_adapter,
    pub platform_device_tuner: [*mut platform_device; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct address_table {
    pub frontend_i2c_addr: u8,
    pub tuner_i2c_addr: u8,
    pub tuner_if_port: u8,
}

//
// eeprom is memory mapped as read only. Writing that memory mapped address
// will not corrupt eeprom.
//
// TS mode:
// 0  TS
// 1  DCA + PIP
// 3  PIP
// 5  DCA + PIP (AF9035 only)
// n  DCA
//
// Values 0, 3 and 5 are seen to this day. 0 for single TS and 3/5 for dual TS.
//
pub const EEPROM_BASE_AF9035: c_uint = 0x42f5;
pub const EEPROM_BASE_IT9135: c_uint = 0x4994;
pub const EEPROM_SHIFT: c_uint = 0x10;
pub const EEPROM_IR_MODE: c_uint = 0x18;
pub const EEPROM_TS_MODE: c_uint = 0x31;
pub const EEPROM_2ND_DEMOD_ADDR: c_uint = 0x32;
pub const EEPROM_IR_TYPE: c_uint = 0x34;
pub const EEPROM_1_IF_L: c_uint = 0x38;
pub const EEPROM_1_IF_H: c_uint = 0x39;
pub const EEPROM_1_TUNER_ID: c_uint = 0x3c;
pub const EEPROM_2_IF_L: c_uint = 0x48;
pub const EEPROM_2_IF_H: c_uint = 0x49;
pub const EEPROM_2_TUNER_ID: c_uint = 0x4c;
// USB commands
pub const CMD_MEM_RD: c_uint = 0x00;
pub const CMD_MEM_WR: c_uint = 0x01;
pub const CMD_I2C_RD: c_uint = 0x02;
pub const CMD_I2C_WR: c_uint = 0x03;
pub const CMD_IR_GET: c_uint = 0x18;
pub const CMD_FW_DL: c_uint = 0x21;
pub const CMD_FW_QUERYINFO: c_uint = 0x22;
pub const CMD_FW_BOOT: c_uint = 0x23;
pub const CMD_FW_DL_BEGIN: c_uint = 0x24;
pub const CMD_FW_DL_END: c_uint = 0x25;
pub const CMD_FW_SCATTER_WR: c_uint = 0x29;
pub const CMD_GENERIC_I2C_RD: c_uint = 0x2a;
pub const CMD_GENERIC_I2C_WR: c_uint = 0x2b;
