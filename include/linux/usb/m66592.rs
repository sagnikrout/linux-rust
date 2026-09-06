//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/m66592.h
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
// M66592 driver platform data
//
// Copyright (C) 2009  Renesas Solutions Corp.
//
pub const M66592_PLATDATA_XTAL_12MHZ: c_uint = 0x01;
pub const M66592_PLATDATA_XTAL_24MHZ: c_uint = 0x02;
pub const M66592_PLATDATA_XTAL_48MHZ: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m66592_platdata {
// one = on chip controller, zero = external controller
    pub on_chip:1: unsigned,
// one = big endian, zero = little endian
    pub endian:1: unsigned,
// (external controller only) M66592_PLATDATA_XTAL_nnMHZ
    pub xtal:2: unsigned,
// (external controller only) one = 3.3V, zero = 1.5V
    pub vif:1: unsigned,
// (external controller only) set one = WR0_N shorted to WR1_N
    pub wr0_shorted_to_wr1:1: unsigned,
}
