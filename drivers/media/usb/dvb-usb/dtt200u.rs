//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/dtt200u.h
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
// Common header file of Linux driver for the WideView/ Yakumo/ Hama
// Typhoon/ Yuan DVB-T USB2.0 receiver.
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

// guessed protocol description (reverse engineered):
// read
// 00 - USB type 0x02 for usb2.0, 0x01 for usb1.1
// 88 - locking 2 bytes (0x80 0x40 == no signal, 0x89 0x20 == nice signal)
//
pub const GET_SPEED: c_uint = 0x00;
pub const GET_TUNE_STATUS: c_uint = 0x81;
pub const GET_RC_CODE: c_uint = 0x84;
pub const GET_CONFIGURATION: c_uint = 0x88;
pub const GET_AGC: c_uint = 0x89;
pub const GET_SNR: c_uint = 0x8a;
pub const GET_VIT_ERR_CNT: c_uint = 0x8c;
pub const GET_RS_ERR_CNT: c_uint = 0x8d;
pub const GET_RS_UNCOR_BLK_CNT: c_uint = 0x8e;
// write
// 01 - init
// 02 - frequency (divided by 250000)
// 03 - bandwidth
// 04 - pid table (index pid(7:0) pid(12:8))
// 05 - reset the pid table
// 08 - transfer switch
//
pub const SET_INIT: c_uint = 0x01;
pub const SET_RF_FREQ: c_uint = 0x02;
pub const SET_BANDWIDTH: c_uint = 0x03;
pub const SET_PID_FILTER: c_uint = 0x04;
pub const RESET_PID_FILTER: c_uint = 0x05;
pub const SET_STREAMING: c_uint = 0x08;
extern "C" {
    pub fn dtt200u_fe_attach(d: *mut dvb_usb_device) -> *mut dvb_frontend;
}
