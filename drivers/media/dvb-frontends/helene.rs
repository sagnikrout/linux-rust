//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/helene.h
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
// helene.h
//
// Sony HELENE DVB-S/S2/T/T2/C/C2/ISDB-T/S tuner driver (CXD2858ER)
//
// Copyright 2012 Sony Corporation
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum helene_xtal {
    SONY_HELENE_XTAL_16000, /* 16 MHz */
    SONY_HELENE_XTAL_20500, /* 20.5 MHz */
    SONY_HELENE_XTAL_24000, /* 24 MHz */
    SONY_HELENE_XTAL_41000 /* 41 MHz */
}

//
// struct helene_config - the configuration of 'Helene' tuner driver
// @i2c_address:	I2C address of the tuner
// @xtal_freq_mhz:	Oscillator frequency, MHz
// @set_tuner_priv:	Callback function private context
// @set_tuner_callback:	Callback function that notifies the parent driver
// which tuner is active now
// @xtal: Cristal frequency as described by &enum helene_xtal
// @fe: Frontend for which connects this tuner
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct helene_config {
    pub i2c_address: u8,
    pub xtal_freq_mhz: u8,
    pub set_tuner_priv: *mut c_void,
    pub int): *mut *mut *mut int (set_tuner_callback)(void ,,
    pub xtal: helene_xtal,
    pub fe: *mut dvb_frontend,
}

//
// helene_attach - Attach a helene tuner (terrestrial and cable standards)
//
// @fe: frontend to be attached
// @config: pointer to &struct helene_config with tuner configuration.
// @i2c: i2c adapter to use.
//
// return: FE pointer on success, NULL on failure.
//
// helene_attach_s - Attach a helene tuner (satellite standards)
//
// @fe: frontend to be attached
// @config: pointer to &struct helene_config with tuner configuration.
// @i2c: i2c adapter to use.
//
// return: FE pointer on success, NULL on failure.
//

