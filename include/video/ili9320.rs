//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/ili9320.h
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


// SPDX-License-Identifier: GPL-2.0-only
// include/video/ili9320.c
//
// ILI9320 LCD controller configuration control.
//
// Copyright 2007 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// http://armlinux.simtec.co.uk
//

// Register contents definitions.

// SPI interface definitions

// platform data to pass configuration from lcd
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ili9320_suspend {
    ILI9320_SUSPEND_OFF,
    ILI9320_SUSPEND_DEEP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ili9320_platdata {
    pub hsize: c_ushort,
    pub vsize: c_ushort,
    pub suspend: ili9320_suspend,
// set the reset line, 0 = reset asserted, 1 = normal
    pub val): *mut *mut void (reset)(unsigned int,
    pub entry_mode: c_ushort,
    pub display2: c_ushort,
    pub display3: c_ushort,
    pub display4: c_ushort,
    pub rgb_if1: c_ushort,
    pub rgb_if2: c_ushort,
    pub interface2: c_ushort,
    pub interface3: c_ushort,
    pub interface4: c_ushort,
    pub interface5: c_ushort,
    pub interface6: c_ushort,
}
