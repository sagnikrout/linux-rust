//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2841er_priv.h
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
// cxd2841er_priv.h
//
// Sony CXD2441ER digital demodulator driver internal definitions
//
// Copyright 2012 Sony Corporation
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//
pub const I2C_SLVX: c_int = 0;
pub const I2C_SLVT: c_int = 1;
pub const CXD2837ER_CHIP_ID: c_uint = 0xb1;
pub const CXD2838ER_CHIP_ID: c_uint = 0xb0;
pub const CXD2841ER_CHIP_ID: c_uint = 0xa7;
pub const CXD2843ER_CHIP_ID: c_uint = 0xa4;
pub const CXD2854ER_CHIP_ID: c_uint = 0xc1;
pub const CXD2841ER_DVBS_POLLING_INVL: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2841er_cnr_data {
    pub value: u32,
    pub cnr_x1000: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2841er_dvbt2_profile_t {
    DVBT2_PROFILE_ANY = 0,
    DVBT2_PROFILE_BASE = 1,
    DVBT2_PROFILE_LITE = 2
}
