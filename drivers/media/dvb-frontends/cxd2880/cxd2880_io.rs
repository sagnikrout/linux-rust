//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_io.h
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
// cxd2880_io.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// register I/O interface definitions
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_io_tgt {
    CXD2880_IO_TGT_SYS,
    CXD2880_IO_TGT_DMD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_reg_value {
    pub addr: u8,
    pub value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_io {
    pub size): *mut *mut u8 data, u32,
    pub size): *const *const u8 data, u32,
    pub data): u8,
    pub if_object: *mut c_void,
    pub i2c_address_sys: u8,
    pub i2c_address_demod: u8,
    pub slave_select: u8,
    pub user: *mut c_void,
}
