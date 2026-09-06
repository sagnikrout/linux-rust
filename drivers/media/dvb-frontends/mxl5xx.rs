//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mxl5xx.h
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
// Driver for the MaxLinear MxL5xx family of tuners/demods
//
// Copyright (C) 2014-2015 Ralph Metzler <rjkm@metzlerbros.de>
// Marcus Metzler <mocm@metzlerbros.de>
// developed for Digital Devices GmbH
//
// based on code:
// Copyright (c) 2011-2013 MaxLinear, Inc. All rights reserved
// which was released under GPL V2
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl5xx_cfg {
    pub adr: u8,
    pub type: u8,
    pub cap: u32,
    pub clk: u32,
    pub ts_clk: u32,
    pub fw: *mut u8,
    pub fw_len: u32,
    pub len): *mut *mut *mut *mut int (fw_read)(void priv, u8 buf, u32,
    pub fw_priv: *mut c_void,
}

