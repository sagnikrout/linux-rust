//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/via_i2c.h
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
// Copyright 1998-2009 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_i2c_stuff {
    pub /: *mut *mut u16 i2c_port; / GPIO or I2C port,
    pub /: *mut *mut u16 is_active; / Being used as I2C?,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

extern "C" {
    pub fn viafb_i2c_readbyte(adap: u8, slave_addr: u8, index: u8, pdata: *mut u8) -> c_int;
}
extern "C" {
    pub fn viafb_i2c_writebyte(adap: u8, slave_addr: u8, index: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn viafb_i2c_readbytes(adap: u8, slave_addr: u8, index: u8, buff: *mut u8, buff_len: c_int) -> c_int;
}
extern "C" {
    pub fn viafb_i2c_init() -> c_int;
}
extern "C" {
    pub fn viafb_i2c_exit();
}
