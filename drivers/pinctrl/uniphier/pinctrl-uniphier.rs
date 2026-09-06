//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/uniphier/pinctrl-uniphier.h
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
// Copyright (C) 2015-2017 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//

// input enable control register bit
pub const UNIPHIER_PIN_IECTRL_SHIFT: c_int = 0;
pub const UNIPHIER_PIN_IECTRL_BITS: c_int = 3;

// drive strength control register number

pub const UNIPHIER_PIN_DRVCTRL_BITS: c_int = 9;

// drive control type

pub const UNIPHIER_PIN_DRV_TYPE_BITS: c_int = 3;

// pull-up / pull-down register number

pub const UNIPHIER_PIN_PUPDCTRL_BITS: c_int = 9;

// direction of pull register

pub const UNIPHIER_PIN_PULL_DIR_BITS: c_int = 3;

pub const UNIPHIER_PIN_IECTRL_EXIST: c_int = 0;
// drive control type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniphier_pin_drv_type {
    UNIPHIER_PIN_DRV_1BIT,		/* 2 level control: 4/8 mA */
    UNIPHIER_PIN_DRV_2BIT,		/* 4 level control: 8/12/16/20 mA */
    UNIPHIER_PIN_DRV_3BIT,		/* 8 level control: 4/5/7/9/11/12/14/16 mA */
    UNIPHIER_PIN_DRV_FIXED4,	/* fixed to 4mA */
    UNIPHIER_PIN_DRV_FIXED5,	/* fixed to 5mA */
    UNIPHIER_PIN_DRV_FIXED8,	/* fixed to 8mA */
    UNIPHIER_PIN_DRV_NONE,		/* no support (input only pin) */
}

// direction of pull register (no pin supports bi-directional pull biasing)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniphier_pin_pull_dir {
    UNIPHIER_PIN_PULL_UP,		/* pull-up or disabled */
    UNIPHIER_PIN_PULL_DOWN,		/* pull-down or disabled */
    UNIPHIER_PIN_PULL_UP_FIXED,	/* always pull-up */
    UNIPHIER_PIN_PULL_DOWN_FIXED,	/* always pull-down */
    UNIPHIER_PIN_PULL_NONE,		/* no pull register */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_pinctrl_group {
    pub name: *const c_char,
    pub pins: *const unsigned,
    pub num_pins: unsigned,
    pub muxvals: *const c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_pinmux_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub num_groups: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_pinctrl_socdata {
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub groups: *const uniphier_pinctrl_group,
    pub groups_count: c_int,
    pub functions: *const uniphier_pinmux_function,
    pub functions_count: c_int,
    pub gpio_offset): *mut *mut int (get_gpio_muxval)(unsigned int pin, unsigned int,
    pub caps: c_uint,

}

