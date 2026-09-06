//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/vt8500/pinctrl-wmt.h
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
// Pinctrl driver for the Wondermedia SoC's
//
// Copyright (c) 2013 Tony Prisk <linux@prisktech.co.nz>
//

// VT8500 has no enable register in the extgpio bank.
pub const NO_REG: c_uint = 0xFFFF;

// Encode/decode the bank/bit pairs into a pin value

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmt_pinctrl_bank_registers {
    pub reg_en: u32,
    pub reg_dir: u32,
    pub reg_data_out: u32,
    pub reg_data_in: u32,
    pub reg_pull_en: u32,
    pub reg_pull_cfg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmt_pinctrl_group {
    pub name: *const c_char,
    pub pins: *const c_uint,
    pub npins: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmt_pinctrl_data {
    pub dev: *mut device,
    pub pctl_dev: *mut pinctrl_dev,
// must be initialized before calling wmt_pinctrl_probe
    pub base: *mut void __iomem,
    pub banks: *const wmt_pinctrl_bank_registers,
    pub pins: *const pinctrl_pin_desc,
    pub groups: *const *const c_char,
    pub nbanks: u32,
    pub npins: u32,
    pub ngroups: u32,
    pub gpio_chip: gpio_chip,
    pub gpio_range: pinctrl_gpio_range,
}
