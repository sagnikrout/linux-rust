//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/stv0900.h
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
// stv0900.h
//
// Driver for ST STV0900 satellite demodulator IC.
//
// Copyright (C) ST Microelectronics.
// Copyright (C) 2009 NetUP Inc.
// Copyright (C) 2009 Igor M. Liplianin <liplianin@netup.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_reg {
    pub addr: u16,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stv0900_config {
    pub demod_address: u8,
    pub demod_mode: u8,
    pub xtal: u32,
    pub /: *mut *mut u8 clkmode;/ 0 for CLKI, 2 for XTALI,
    pub diseqc_mode: u8,
    pub path1_mode: u8,
    pub path2_mode: u8,
    pub ts_config_regs: *mut stv0900_reg,
    pub /: *mut *mut u8 tun1_maddress;/ 0, 1, 2, 3 for 0xc0, 0xc2, 0xc4, 0xc6,
    pub tun2_maddress: u8,
    pub /: *mut *mut u8 tun1_adc;/ 1 for stv6110, 2 for stb6100,
    pub tun2_adc: u8,
    pub /: *mut *mut u8 tun1_type;/ for now 3 for stb6100 auto, else - software,
    pub tun2_type: u8,
// Set device param to start dma
    pub is_punctured): *mut *mut *mut int (set_ts_params)(struct dvb_frontend fe, int,
// Hook for Lock LED
    pub offon): *mut *mut *mut void (set_lock_led)(struct dvb_frontend fe, int,
}

