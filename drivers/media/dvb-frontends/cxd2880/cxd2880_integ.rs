//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_integ.h
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


// SPDX-License-Identifier: GPL-2.0
//
// cxd2880_integ.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// integration layer common interface
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

pub const CXD2880_TNRDMD_WAIT_INIT_TIMEOUT: c_int = 500;
pub const CXD2880_TNRDMD_WAIT_INIT_INTVL: c_int = 10;
pub const CXD2880_TNRDMD_WAIT_AGC_STABLE: c_int = 100;
extern "C" {
    pub fn cxd2880_integ_init(tnr_dmd: *mut cxd2880_tnrdmd) -> c_int;
}
extern "C" {
    pub fn cxd2880_integ_cancel(tnr_dmd: *mut cxd2880_tnrdmd) -> c_int;
}
// tnr_dmd);
