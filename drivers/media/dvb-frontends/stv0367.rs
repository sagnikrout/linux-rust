//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0367.h
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
// stv0367.h
//
// Driver for ST STV0367 DVB-T & DVB-C demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2010,2011 NetUP Inc.
// Copyright (C) 2010,2011 Igor M. Liplianin <liplianin@netup.ru>
//

pub const STV0367_ICSPEED_53125: c_int = 53125000;
pub const STV0367_ICSPEED_58000: c_int = 58000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0367_config {
    pub demod_address: u8,
    pub xtal: u32,
    pub if_khz;/*4500*/: *mut u32,
    pub if_iq_mode: c_int,
    pub ts_mode: c_int,
    pub clk_pol: c_int,
}

