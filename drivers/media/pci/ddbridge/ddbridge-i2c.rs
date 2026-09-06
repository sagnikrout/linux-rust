//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ddbridge/ddbridge-i2c.h
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
// ddbridge-i2c.h: Digital Devices bridge i2c driver
//
// Copyright (C) 2010-2017 Digital Devices GmbH
// Ralph Metzler <rjkm@metzlerbros.de>
// Marcus Metzler <mocm@metzlerbros.de>
//

//
extern "C" {
    pub fn ddb_i2c_release(dev: *mut ddb);
}
extern "C" {
    pub fn ddb_i2c_init(dev: *mut ddb) -> c_int;
}
//
extern "C" {
    pub fn i2c_write(_arg: adap, _arg: adr, _arg: msg, _arg: 3) -> return;
}
extern "C" {
    pub fn i2c_write(_arg: adap, _arg: adr, _arg: msg, _arg: 2) -> return;
}
extern "C" {
    pub fn i2c_read_regs16(_arg: adapter, _arg: adr, _arg: reg, _arg: val, _arg: 1) -> return;
}
extern "C" {
    pub fn i2c_read_regs(_arg: adapter, _arg: adr, _arg: reg, _arg: val, _arg: 1) -> return;
}
