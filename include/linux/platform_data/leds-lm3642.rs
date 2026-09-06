//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/leds-lm3642.h
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
// Copyright (C) 2012 Texas Instruments
//
// Simple driver for Texas Instruments LM3642 LED driver chip
//
// Author: G.Shark Jeong <gshark.jeong@gmail.com>
// Daniel Jeong <daniel.jeong@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3642_torch_pin_enable {
    LM3642_TORCH_PIN_DISABLE = 0x00,
    LM3642_TORCH_PIN_ENABLE = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3642_strobe_pin_enable {
    LM3642_STROBE_PIN_DISABLE = 0x00,
    LM3642_STROBE_PIN_ENABLE = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm3642_tx_pin_enable {
    LM3642_TX_PIN_DISABLE = 0x00,
    LM3642_TX_PIN_ENABLE = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3642_platform_data {
    pub torch_pin: lm3642_torch_pin_enable,
    pub strobe_pin: lm3642_strobe_pin_enable,
    pub tx_pin: lm3642_tx_pin_enable,
}
