//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-gpio.h
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
// cx18 gpio functions
//
// Derived from ivtv-gpio.h
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//
extern "C" {
    pub fn cx18_gpio_init(cx: *mut cx18);
}
extern "C" {
    pub fn cx18_gpio_register(cx: *mut cx18, hw: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx18_gpio_reset_type {
    CX18_GPIO_RESET_I2C     = 0,
    CX18_GPIO_RESET_Z8F0811 = 1,
    CX18_GPIO_RESET_XC2028  = 2,
}

extern "C" {
    pub fn cx18_reset_tuner_gpio(dev: *mut c_void, component: c_int, cmd: c_int, value: c_int) -> c_int;
}
