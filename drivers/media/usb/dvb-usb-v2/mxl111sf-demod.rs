//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/mxl111sf-demod.h
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
// mxl111sf-demod.h - driver for the MaxLinear MXL111SF DVB-T demodulator
//
// Copyright (C) 2010-2014 Michael Krufky <mkrufky@linuxtv.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl111sf_demod_config {
    pub data): *mut *mut *mut int (read_reg)(struct mxl111sf_state state, u8 addr, u8,
    pub data): *mut *mut *mut int (write_reg)(struct mxl111sf_state state, u8 addr, u8,
    pub ctrl_reg_info): *mut mxl111sf_reg_ctrl_info,
}

