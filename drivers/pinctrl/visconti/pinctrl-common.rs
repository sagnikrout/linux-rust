//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/visconti/pinctrl-common.h
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
// Copyright (c) 2020 TOSHIBA CORPORATION
// Copyright (c) 2020 Toshiba Electronic Devices & Storage Corporation
// Copyright (c) 2020 Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
//
// PIN

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_desc_pin {
    pub pin: pinctrl_pin_desc,
    pub dsel_offset: c_uint,
    pub dsel_shift: c_uint,
    pub pude_offset: c_uint,
    pub pudsel_offset: c_uint,
    pub pud_shift: c_uint,
}

// Group

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_mux {
    pub offset: c_uint,
    pub mask: c_uint,
    pub val: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_pin_group {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub nr_pins: c_uint,
    pub mux: visconti_mux,
}

// MUX
#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_pin_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub nr_groups: c_uint,
}

// chip dependent data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct visconti_pinctrl_devdata {
    pub pins: *const visconti_desc_pin,
    pub nr_pins: c_uint,
    pub groups: *const visconti_pin_group,
    pub nr_groups: c_uint,
    pub functions: *const visconti_pin_function,
    pub nr_functions: c_uint,
    pub gpio_mux: *const visconti_mux,
    pub base): *mut *mut void (unlock)(void __iomem,
}
