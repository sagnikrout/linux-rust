//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/gp8psk-fe.h
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
// gp8psk_fe driver
//

// gp8psk commands
pub const GET_8PSK_CONFIG: c_uint = 0x80    /* in */;
pub const SET_8PSK_CONFIG: c_uint = 0x81;
pub const I2C_WRITE: c_uint = 0x83;
pub const I2C_READ: c_uint = 0x84;
pub const ARM_TRANSFER: c_uint = 0x85;
pub const TUNE_8PSK: c_uint = 0x86;
pub const GET_SIGNAL_STRENGTH: c_uint = 0x87    /* in */;
pub const LOAD_BCM4500: c_uint = 0x88;
pub const BOOT_8PSK: c_uint = 0x89    /* in */;
pub const START_INTERSIL: c_uint = 0x8A    /* in */;
pub const SET_LNB_VOLTAGE: c_uint = 0x8B;
pub const SET_22KHZ_TONE: c_uint = 0x8C;
pub const SEND_DISEQC_COMMAND: c_uint = 0x8D;
pub const SET_DVB_MODE: c_uint = 0x8E;
pub const SET_DN_SWITCH: c_uint = 0x8F;
pub const GET_SIGNAL_LOCK: c_uint = 0x90    /* in */;
pub const GET_FW_VERS: c_uint = 0x92;
pub const GET_SERIAL_NUMBER: c_uint = 0x93    /* in */;
pub const USE_EXTRA_VOLT: c_uint = 0x94;
pub const GET_FPGA_VERS: c_uint = 0x95;
pub const CW3K_INIT: c_uint = 0x9d;
// PSK_configuration bits
pub const bm8pskStarted: c_uint = 0x01;
pub const bm8pskFW_Loaded: c_uint = 0x02;
pub const bmIntersilOn: c_uint = 0x04;
pub const bmDVBmode: c_uint = 0x08;
pub const bm22kHz: c_uint = 0x10;
pub const bmSEL18V: c_uint = 0x20;
pub const bmDCtuned: c_uint = 0x40;
pub const bmArmed: c_uint = 0x80;
// Satellite modulation modes

// firmware revision id's
pub const GP8PSK_FW_REV1: c_uint = 0x020604;
pub const GP8PSK_FW_REV2: c_uint = 0x020704;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gp8psk_fe_ops {
    pub blen): *mut *mut *mut *mut int (in)(void priv, u8 req, u16 value, u16 index, u8 b, int,
    pub blen): *mut *mut *mut *mut int (out)(void priv, u8 req, u16 value, u16 index, u8 b, int,
    pub priv): *mut *mut int (reload)(void,
}
