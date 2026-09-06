//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/sp2.h
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

//
// I2C address
// 0x40 (port 0)
// 0x41 (port 1)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp2_config {
// dvb_adapter to attach the ci to
    pub dvb_adap: *mut dvb_adapter,
// function ci_control handles the device specific ci ops
    pub ci_control: *mut c_void,
// priv is passed back to function ci_control
    pub priv: *mut c_void,
}

extern "C" {
    pub fn sp2_ci_slot_reset(en50221: *mut dvb_ca_en50221, slot: c_int) -> c_int;
}
extern "C" {
    pub fn sp2_ci_slot_shutdown(en50221: *mut dvb_ca_en50221, slot: c_int) -> c_int;
}
extern "C" {
    pub fn sp2_ci_slot_ts_enable(en50221: *mut dvb_ca_en50221, slot: c_int) -> c_int;
}
