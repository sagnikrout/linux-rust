//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/mxl111sf-tuner.h
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
// mxl111sf-tuner.h - driver for the MaxLinear MXL111SF CMOS tuner
//
// Copyright (C) 2010-2014 Michael Krufky <mkrufky@linuxtv.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl_if_freq {

    MXL_IF_LO    = 0x00, /* other IF < 9MHz */

    MXL_IF_4_0   = 0x01, /* 4.0   MHz */
    MXL_IF_4_5   = 0x02, /* 4.5   MHz */
    MXL_IF_4_57  = 0x03, /* 4.57  MHz */
    MXL_IF_5_0   = 0x04, /* 5.0   MHz */
    MXL_IF_5_38  = 0x05, /* 5.38  MHz */
    MXL_IF_6_0   = 0x06, /* 6.0   MHz */
    MXL_IF_6_28  = 0x07, /* 6.28  MHz */
    MXL_IF_7_2   = 0x08, /* 7.2   MHz */
    MXL_IF_35_25 = 0x09, /* 35.25 MHz */
    MXL_IF_36    = 0x0a, /* 36    MHz */
    MXL_IF_36_15 = 0x0b, /* 36.15 MHz */
    MXL_IF_44    = 0x0c, /* 44    MHz */

    MXL_IF_HI    = 0x0f, /* other IF > 35 MHz and < 45 MHz */

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl111sf_tuner_config {
    pub if_freq: mxl_if_freq,
    pub invert_spectrum:1: c_uint,
    pub data): *mut *mut *mut int (read_reg)(struct mxl111sf_state state, u8 addr, u8,
    pub data): *mut *mut *mut int (write_reg)(struct mxl111sf_state state, u8 addr, u8,
    pub ctrl_reg_info): *mut mxl111sf_reg_ctrl_info,
    pub onoff): *mut *mut *mut int (top_master_ctrl)(struct mxl111sf_state state, int,
    pub fe): *mut *mut int (ant_hunt)(struct dvb_frontend,
}

// ------------------------------------------------------------------------

