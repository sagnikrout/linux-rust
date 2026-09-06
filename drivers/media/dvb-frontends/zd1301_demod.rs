//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/zd1301_demod.h
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
// ZyDAS ZD1301 driver (demodulator)
//
// Copyright (C) 2015 Antti Palosaari <crope@iki.fi>
//

//
// struct zd1301_demod_platform_data - Platform data for the zd1301_demod driver
// @reg_priv: First argument of reg_read and reg_write callbacks.
// @reg_read: Register read callback.
// @reg_write: Register write callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd1301_demod_platform_data {
    pub reg_priv: *mut c_void,
    pub ): *mut *mut *mut int (reg_read)(void , u16, u8,
    pub u8): *mut *mut *mut int (reg_write)(void , u16,,
}

//
// zd1301_demod_get_dvb_frontend() - Get pointer to DVB frontend
// @pdev: Pointer to platform device
//
// Return: Pointer to DVB frontend which given platform device owns.
//
// zd1301_demod_get_i2c_adapter() - Get pointer to I2C adapter
// @pdev: Pointer to platform device
//
// Return: Pointer to I2C adapter which given platform device owns.
//

