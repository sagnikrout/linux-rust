//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/mt9t112.h
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
// mt9t112 Camera
//
// Copyright (C) 2009 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9t112_pll_divider {
    pub n: u8 m,,
    pub p7: u8 p1, p2, p3, p4, p5, p6,,
}

//
// struct mt9t112_platform_data - mt9t112 driver interface
// @flags:			Sensor media bus configuration.
// @divider:			Sensor PLL configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt9t112_platform_data {

    pub flags: u32,
    pub divider: mt9t112_pll_divider,
}
