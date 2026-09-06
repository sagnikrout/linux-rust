//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mxs-lradc.h
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
// Freescale MXS Low Resolution Analog-to-Digital Converter driver
//
// Copyright (c) 2012 DENX Software Engineering, GmbH.
// Copyright (c) 2016 Ksenija Stanojevic <ksenija.stanojevic@gmail.com>
//
// Author: Marek Vasut <marex@denx.de>
//

pub const LRADC_MAX_DELAY_CHANS: c_int = 4;
pub const LRADC_MAX_MAPPED_CHANS: c_int = 8;
pub const LRADC_MAX_TOTAL_CHANS: c_int = 16;
pub const LRADC_DELAY_TIMER_HZ: c_int = 2000;
pub const LRADC_CTRL0: c_uint = 0x00;

pub const LRADC_CTRL1: c_uint = 0x10;

pub const LRADC_CTRL1_LRADC_IRQ_EN_OFFSET: c_int = 16;

pub const LRADC_CTRL1_MX28_LRADC_IRQ_MASK: c_uint = 0x1fff;
pub const LRADC_CTRL1_MX23_LRADC_IRQ_MASK: c_uint = 0x01ff;
pub const LRADC_CTRL1_LRADC_IRQ_OFFSET: c_int = 0;
pub const LRADC_CTRL2: c_uint = 0x20;
pub const LRADC_CTRL2_DIVIDE_BY_TWO_OFFSET: c_int = 24;

pub const LRADC_STATUS: c_uint = 0x40;

pub const LRADC_CH_NUM_SAMPLES_OFFSET: c_int = 24;

pub const LRADC_CH_VALUE_MASK: c_uint = 0x3ffff;
pub const LRADC_CH_VALUE_OFFSET: c_int = 0;

pub const LRADC_DELAY_TRIGGER_LRADCS_OFFSET: c_int = 24;

pub const LRADC_DELAY_TRIGGER_DELAYS_OFFSET: c_int = 16;

pub const LRADC_DELAY_LOOP_COUNT_OFFSET: c_int = 11;

pub const LRADC_DELAY_DELAY_MASK: c_uint = 0x7ff;
pub const LRADC_DELAY_DELAY_OFFSET: c_int = 0;

pub const LRADC_CTRL4: c_uint = 0x140;

pub const LRADC_RESOLUTION: c_int = 12;

pub const BUFFER_VCHANS_LIMITED: c_uint = 0x3f;
pub const BUFFER_VCHANS_ALL: c_uint = 0xff;
//
// Certain LRADC channels are shared between touchscreen
// and/or touch-buttons and generic LRADC block. Therefore when using
// either of these, these channels are not available for the regular
// sampling. The shared channels are as follows:
//
// CH0 -- Touch button #0
// CH1 -- Touch button #1
// CH2 -- Touch screen XPUL
// CH3 -- Touch screen YPLL
// CH4 -- Touch screen XNUL
// CH5 -- Touch screen YNLR
// CH6 -- Touch screen WIPER (5-wire only)
//
// The bit fields below represents which parts of the LRADC block are
// switched into special mode of operation. These channels can not
// be sampled as regular LRADC channels. The driver will refuse any
// attempt to sample these channels.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxs_lradc_id {
    IMX23_LRADC,
    IMX28_LRADC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxs_lradc_ts_wires {
    MXS_LRADC_TOUCHSCREEN_NONE = 0,
    MXS_LRADC_TOUCHSCREEN_4WIRE,
    MXS_LRADC_TOUCHSCREEN_5WIRE,
}

//
// struct mxs_lradc
// @soc: soc type (IMX23 or IMX28)
// @clk: 2 kHz clock for delay units
// @buffer_vchans: channels that can be used during buffered capture
// @touchscreen_wire: touchscreen type (4-wire or 5-wire)
// @use_touchbutton: button state (on or off)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_lradc {
    pub soc: mxs_lradc_id,
    pub clk: *mut clk,
    pub buffer_vchans: u8,
    pub touchscreen_wire: mxs_lradc_ts_wires,
    pub use_touchbutton: bool,
}
