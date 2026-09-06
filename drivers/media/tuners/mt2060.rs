//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/mt2060.h
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
// Driver for Microtune MT2060 "Single chip dual conversion broadband tuner"
//
// Copyright (c) 2006 Olivier DANET <odanet@caramail.com>
//
// I2C address
// 0x60, ...
//
// struct mt2060_platform_data - Platform data for the mt2060 driver
// @clock_out: Clock output setting. 0 = off, 1 = CLK/4, 2 = CLK/2, 3 = CLK/1.
// @if1: First IF used [MHz]. 0 defaults to 1220.
// @i2c_write_max: Maximum number of bytes I2C adapter can write at once.
// 0 defaults to maximum.
// @dvb_frontend: DVB frontend.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2060_platform_data {
    pub clock_out: u8,
    pub if1: u16,
    pub i2c_write_max:5: c_uint,
    pub dvb_frontend: *mut dvb_frontend,
}

// configuration struct for mt2060_attach()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt2060_config {
    pub i2c_address: u8,
    pub /: *mut *mut u8 clock_out; / 0 = off, 1 = CLK/4, 2 = CLK/2, 3 = CLK/1,
}

extern "C" {
    pub fn mt2060_attach(fe: *mut dvb_frontend, i2c: *mut i2c_adapter, cfg: *mut mt2060_config, if1: u16) -> *mut dvb_frontend;
}

