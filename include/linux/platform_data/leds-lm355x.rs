//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/leds-lm355x.h
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
// Simple driver for Texas Instruments LM355x LED driver chip
//
// Author: G.Shark Jeong <gshark.jeong@gmail.com>
// Daniel Jeong <daniel.jeong@ti.com>
//

// lm3554 : strobe def. on
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm355x_strobe {
    LM355x_PIN_STROBE_DISABLE = 0x00,
    LM355x_PIN_STROBE_ENABLE = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm355x_torch {
    LM355x_PIN_TORCH_DISABLE = 0,
    LM3554_PIN_TORCH_ENABLE = 0x80,
    LM3556_PIN_TORCH_ENABLE = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm355x_tx2 {
    LM355x_PIN_TX_DISABLE = 0,
    LM3554_PIN_TX_ENABLE = 0x20,
    LM3556_PIN_TX_ENABLE = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm355x_ntc {
    LM355x_PIN_NTC_DISABLE = 0,
    LM3554_PIN_NTC_ENABLE = 0x08,
    LM3556_PIN_NTC_ENABLE = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lm355x_pmode {
    LM355x_PMODE_DISABLE = 0,
    LM355x_PMODE_ENABLE = 0x04,
}

//
// struct lm3554_platform_data
// @pin_strobe: strobe input
// @pin_torch : input pin
// lm3554-tx1/torch/gpio1
// lm3556-torch
// @pin_tx2   : input pin
// lm3554-envm/tx2/gpio2
// lm3556-tx pin
// @ntc_pin  : output pin
// lm3554-ledi/ntc
// lm3556-temp pin
// @pass_mode : pass mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm355x_platform_data {
    pub pin_strobe: lm355x_strobe,
    pub pin_tx1: lm355x_torch,
    pub pin_tx2: lm355x_tx2,
    pub ntc_pin: lm355x_ntc,
    pub pass_mode: lm355x_pmode,
}
