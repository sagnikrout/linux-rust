//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/sophgo/pinctrl-cv18xx.h
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
// Copyright (C) 2024 Inochi Amaoto <inochiama@outlook.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cv1800_pin_io_type {
    IO_TYPE_1V8_ONLY = 0,
    IO_TYPE_1V8_OR_3V3 = 1,
    IO_TYPE_AUDIO = 2,
    IO_TYPE_ETH = 3
}

pub const CV1800_PINCONF_AREA_SYS: c_int = 0;
pub const CV1800_PINCONF_AREA_RTC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_pinmux {
    pub offset: u16,
    pub area: u8,
    pub max: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_pinmux2 {
    pub offset: u16,
    pub area: u8,
    pub max: u8,
    pub pfunc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_pinconf {
    pub offset: u16,
    pub area: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_pin {
    pub pin: sophgo_pin,
    pub power_domain: u8,
    pub mux: cv1800_pinmux,
    pub mux2: cv1800_pinmux2,
    pub conf: cv1800_pinconf,
}

pub const PIN_POWER_STATE_1V8: c_int = 1800;
pub const PIN_POWER_STATE_3V3: c_int = 3300;
extern "C" {
    pub fn FIELD_GET(_arg: CV1800_PIN_IO_TYPE, _arg: pin->pin.flags) -> return;
}

