//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/sp2_priv.h
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
// CIMaX SP2/HF CI driver
//
// Copyright (C) 2014 Olli Salonen <olli.salonen@iki.fi>
//

// state struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp2 {
    pub status: c_int,
    pub client: *mut i2c_client,
    pub dvb_adap: *mut dvb_adapter,
    pub ca: dvb_ca_en50221,
    pub module_access_type: c_int,
    pub next_status_checked_time: c_ulong,
    pub priv: *mut c_void,
    pub ci_control: *mut c_void,
}

pub const SP2_CI_ATTR_ACS: c_uint = 0x00;
pub const SP2_CI_IO_ACS: c_uint = 0x04;
pub const SP2_CI_WR: c_int = 0;
pub const SP2_CI_RD: c_int = 1;
// Module control register (0x00 module A, 0x09 module B) bits
pub const SP2_MOD_CTL_DET: c_uint = 0x01;
pub const SP2_MOD_CTL_AUTO: c_uint = 0x02;
pub const SP2_MOD_CTL_ACS0: c_uint = 0x04;
pub const SP2_MOD_CTL_ACS1: c_uint = 0x08;
pub const SP2_MOD_CTL_HAD: c_uint = 0x10;
pub const SP2_MOD_CTL_TSIEN: c_uint = 0x20;
pub const SP2_MOD_CTL_TSOEN: c_uint = 0x40;
pub const SP2_MOD_CTL_RST: c_uint = 0x80;
