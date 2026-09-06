//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/lm3630a_bl.h
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
// Simple driver for Texas Instruments LM3630A LED Flash driver chip
// Copyright (C) 2012 Texas Instruments
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3630a_pwm_ctrl {
    LM3630A_PWM_DISABLE = 0x00,
    LM3630A_PWM_BANK_A,
    LM3630A_PWM_BANK_B,
    LM3630A_PWM_BANK_ALL,
    LM3630A_PWM_BANK_A_ACT_LOW = 0x05,
    LM3630A_PWM_BANK_B_ACT_LOW,
    LM3630A_PWM_BANK_ALL_ACT_LOW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3630a_leda_ctrl {
    LM3630A_LEDA_DISABLE = 0x00,
    LM3630A_LEDA_ENABLE = 0x04,
    LM3630A_LEDA_ENABLE_LINEAR = 0x14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3630a_ledb_ctrl {
    LM3630A_LEDB_DISABLE = 0x00,
    LM3630A_LEDB_ON_A = 0x01,
    LM3630A_LEDB_ENABLE = 0x02,
    LM3630A_LEDB_ENABLE_LINEAR = 0x0A,
}

pub const LM3630A_MAX_BRIGHTNESS: c_int = 255;
//
// @leda_label    : optional led a label.
// @leda_init_brt : led a init brightness. 4~255
// @leda_max_brt  : led a max brightness.  4~255
// @leda_ctrl     : led a disable, enable linear, enable exponential
// @ledb_label    : optional led b label.
// @ledb_init_brt : led b init brightness. 4~255
// @ledb_max_brt  : led b max brightness.  4~255
// @ledb_ctrl     : led b disable, enable linear, enable exponential
// @pwm_period    : pwm period
// @pwm_ctrl      : pwm disable, bank a or b, active high or low
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3630a_platform_data {
// led a config.
    pub leda_label: *const c_char,
    pub leda_init_brt: c_int,
    pub leda_max_brt: c_int,
    pub leda_ctrl: lm3630a_leda_ctrl,
// led b config.
    pub ledb_label: *const c_char,
    pub ledb_init_brt: c_int,
    pub ledb_max_brt: c_int,
    pub ledb_ctrl: lm3630a_ledb_ctrl,
// pwm config.
    pub pwm_period: c_uint,
    pub pwm_ctrl: lm3630a_pwm_ctrl,
}
