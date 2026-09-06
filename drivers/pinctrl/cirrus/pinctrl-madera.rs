//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/cirrus/pinctrl-madera.h
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
//
// Pinctrl for Cirrus Logic Madera codecs
//
// Copyright (C) 2016-2017 Cirrus Logic
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_pin_groups {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub n_pins: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_pin_chip {
    pub n_pins: c_uint,
    pub pin_groups: *const madera_pin_groups,
    pub n_pin_groups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct madera_pin_private {
    pub madera: *mut madera,
    pub /: *const *const *const madera_pin_chip chip; / chip-specific groups,
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
}
