//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/renesas/core.h
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
// SuperH Pin Function Controller support.
//
// Copyright (C) 2012  Renesas Solutions Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_pfc_pin_range {
    pub start: u16,
    pub end: u16,
}

extern "C" {
    pub fn sh_pfc_register_gpiochip(pfc: *mut sh_pfc) -> c_int;
}
extern "C" {
    pub fn sh_pfc_register_pinctrl(pfc: *mut sh_pfc) -> c_int;
}
extern "C" {
    pub fn sh_pfc_read_raw_reg(mapped_reg: *mut void __iomem, reg_width: c_uint) -> u32;
}
extern "C" {
    pub fn sh_pfc_read(pfc: *mut sh_pfc, reg: u32) -> u32;
}
extern "C" {
    pub fn sh_pfc_write(pfc: *mut sh_pfc, reg: u32, data: u32);
}
extern "C" {
    pub fn sh_pfc_get_pin_index(pfc: *mut sh_pfc, pin: c_uint) -> c_int;
}
extern "C" {
    pub fn sh_pfc_config_mux(pfc: *mut sh_pfc, mark: unsigned, pinmux_type: c_int) -> c_int;
}
