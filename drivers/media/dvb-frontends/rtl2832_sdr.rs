//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/rtl2832_sdr.h
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
// Realtek RTL2832U SDR driver
//
// Copyright (C) 2013 Antti Palosaari <crope@iki.fi>
//
// GNU Radio plugin "gr-kernel" for device usage will be on:
// http://git.linuxtv.org/anttip/gr-kernel.git
//

//
// struct rtl2832_sdr_platform_data - Platform data for the rtl2832_sdr driver
// @clk: Clock frequency (4000000, 16000000, 25000000, 28800000).
// @tuner: Used tuner model.
// @regmap: pointer to &struct regmap.
// @dvb_frontend: rtl2832 DVB frontend.
// @v4l2_subdev: Tuner v4l2 controls.
// @dvb_usb_device: DVB USB interface for USB streaming.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl2832_sdr_platform_data {
    pub clk: u32,
//
// XXX: This list must be kept sync with dvb_usb_rtl28xxu USB IF driver.
//
pub const RTL2832_SDR_TUNER_FC2580: c_uint = 0x21;
pub const RTL2832_SDR_TUNER_TUA9001: c_uint = 0x24;
pub const RTL2832_SDR_TUNER_FC0012: c_uint = 0x26;
pub const RTL2832_SDR_TUNER_E4000: c_uint = 0x27;
pub const RTL2832_SDR_TUNER_FC0013: c_uint = 0x29;
pub const RTL2832_SDR_TUNER_R820T: c_uint = 0x2a;
pub const RTL2832_SDR_TUNER_R828D: c_uint = 0x2b;
    pub tuner: u8,
    pub regmap: *mut regmap,
    pub dvb_frontend: *mut dvb_frontend,
    pub v4l2_subdev: *mut v4l2_subdev,
    pub dvb_usb_device: *mut dvb_usb_device,
}
