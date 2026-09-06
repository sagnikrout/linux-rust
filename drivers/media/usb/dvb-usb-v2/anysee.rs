//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/anysee.h
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
// DVB USB Linux driver for Anysee E30 DVB-C & DVB-T USB2.0 receiver
//
// Copyright (C) 2007 Antti Palosaari <crope@iki.fi>
//
// TODO:
// - add smart card reader support for Conditional Access (CA)
//
// Card reader in Anysee is nothing more than ISO 7816 card reader.
// There is no hardware CAM in any Anysee device sold.
// In my understanding it should be implemented by making own module
// for ISO 7816 card reader, like dvb_ca_en50221 is implemented. This
// module registers serial interface that can be used to communicate
// with any ISO 7816 smart card.
//
// Any help according to implement serial smart card reader support
// is highly welcome!
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd {
    CMD_I2C_READ            = 0x33,
    CMD_I2C_WRITE           = 0x31,
    CMD_REG_READ            = 0xb0,
    CMD_REG_WRITE           = 0xb1,
    CMD_STREAMING_CTRL      = 0x12,
    CMD_LED_AND_IR_CTRL     = 0x16,
    CMD_GET_IR_CODE         = 0x41,
    CMD_GET_HW_INFO         = 0x19,
    CMD_SMARTCARD           = 0x34,
    CMD_CI                  = 0x37,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anysee_state {
    pub buf: [u8; 64],
    pub seq: u8,
    pub /: *mut *mut u8 hw; / PCB ID,
pub const ANYSEE_I2C_CLIENT_MAX: c_int = 1;
    pub i2c_client: [*mut i2c_client; ANYSEE_I2C_CLIENT_MAX],
    pub /: *mut *mut u8 fe_id:1; / frondend ID,
    pub has_ci:1: u8,
    pub has_tda18212:1: u8,
    pub ci_attached:1: u8,
    pub ci: dvb_ca_en50221,
    pub /: *mut *mut unsigned long ci_cam_ready; / jiffies,
}

pub const REG_IOA: c_uint = 0x80 /* Port A (bit addressable) */;
pub const REG_IOB: c_uint = 0x90 /* Port B (bit addressable) */;
pub const REG_IOC: c_uint = 0xa0 /* Port C (bit addressable) */;
pub const REG_IOD: c_uint = 0xb0 /* Port D (bit addressable) */;
pub const REG_IOE: c_uint = 0xb1 /* Port E (NOT bit addressable) */;
pub const REG_OEA: c_uint = 0xb2 /* Port A Output Enable */;
pub const REG_OEB: c_uint = 0xb3 /* Port B Output Enable */;
pub const REG_OEC: c_uint = 0xb4 /* Port C Output Enable */;
pub const REG_OED: c_uint = 0xb5 /* Port D Output Enable */;
pub const REG_OEE: c_uint = 0xb6 /* Port E Output Enable */;

//
// USB API description (reverse engineered)
//
