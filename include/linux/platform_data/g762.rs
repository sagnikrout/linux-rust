//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/g762.h
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
// Platform data structure for g762 fan controller driver
//
// Copyright (C) 2013, Arnaud EBALARD <arno@natisbad.org>
//
// Following structure can be used to set g762 driver platform specific data
// during board init. Note that passing a sparse structure is possible but
// will result in non-specified attributes to be set to default value, hence
// overloading those installed during boot (e.g. by u-boot).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct g762_platform_data {
    pub fan_startv: u32,
    pub fan_gear_mode: u32,
    pub pwm_polarity: u32,
    pub clk_freq: u32,
}
