//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2841er.h
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
// cxd2841er.h
//
// Sony CXD2441ER digital demodulator driver public definitions
//
// Copyright 2012 Sony Corporation
// Copyright (C) 2014 NetUP Inc.
// Copyright (C) 2014 Sergey Kozlov <serjk@netup.ru>
// Copyright (C) 2014 Abylay Ospan <aospan@netup.ru>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2841er_xtal {
    SONY_XTAL_20500, /* 20.5 MHz */
    SONY_XTAL_24000, /* 24 MHz */
    SONY_XTAL_41000 /* 41 MHz */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2841er_config {
    pub i2c_addr: u8,
    pub xtal: cxd2841er_xtal,
    pub flags: u32,
}

