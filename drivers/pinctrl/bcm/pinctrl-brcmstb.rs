//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/bcm/pinctrl-brcmstb.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Header for Broadcom brcmstb GPIO based drivers
//
// Copyright (C) 2024-2025 Ivan T. Ivanov, Andrea della Porta
// Copyright (C) 2021-3 Raspberry Pi Ltd.
// Copyright (C) 2012 Chris Boot, Simon Arlott, Stephen Warren
//
// Based heavily on the BCM2835 GPIO & pinctrl driver, which was inspired by:
// pinctrl-nomadik.c, please see original file for copyright information
// pinctrl-tegra.c, please see original file for copyright information
//

pub const MUX_BIT_VALID: c_uint = 0x8000;
pub const PAD_BIT_INVALID: c_uint = 0xffff;

//
// AON pins are in the Always-On power domain. SGPIOs are also 'Safe'
// being 5V tolerant (necessary for the HDMI I2C pins), and can be driven
// while the power is off.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pin_regs {
    pub mux_bit: u16,
    pub pad_bit: u16,
}

//
// struct brcmstb_pin_funcs - pins provide their primary/alternate
// functions in this struct
// @func_mask: mask representing valid bits of the function selector
// in the registers
// @funcs: array of function identifiers
// @n_funcs: number of identifiers of the @funcs array above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_pin_funcs {
    pub func_mask: u32,
    pub funcs: *const u8,
    pub n_funcs: c_uint,
}

//
// struct brcmstb_pdata - specific data for a pinctrl chip implementation
// @pctl_desc: pin controller descriptor for this implementation
// @gpio_range: range of GPIOs served by this controller
// @pin_regs: array of register descriptors for each pin
// @pin_funcs: array of all possible assignable function for each pin
// @func_count: total number of functions
// @func_gpio: which function number is GPIO (usually 0)
// @func_names: an array listing all function names
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_pdata {
    pub pctl_desc: *const pinctrl_desc,
    pub gpio_range: *const pinctrl_gpio_range,
    pub pin_regs: *const pin_regs,
    pub pin_funcs: *const brcmstb_pin_funcs,
    pub func_count: c_uint,
    pub func_gpio: c_uint,
    pub func_names: *const *const c_char,
}

extern "C" {
    pub fn brcmstb_pinctrl_probe(pdev: *mut platform_device) -> c_int;
}
